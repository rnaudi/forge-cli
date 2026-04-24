use std::fmt;
use std::path::{Path, PathBuf};
use std::time::Duration;

use reqwest::blocking::{Client, RequestBuilder};
use reqwest::redirect::Policy;
use reqwest::Method;

use crate::config::{RegistryAuth, RegistryRequirement, SecretRequirement, ToolRequirement};
use crate::environment::Environment;
use crate::report::{Category, CheckResult, Status};

pub trait ToolResolver {
    fn command_exists(&self, command: &str, environment: &dyn Environment) -> bool;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct PathToolResolver;

impl ToolResolver for PathToolResolver {
    fn command_exists(&self, command: &str, environment: &dyn Environment) -> bool {
        if command.trim().is_empty() {
            return false;
        }

        if has_path_separator(command) {
            return is_executable(Path::new(command));
        }

        let Some(path_value) = environment.var_os("PATH") else {
            return false;
        };

        std::env::split_paths(&path_value).any(|directory| {
            command_candidates(&directory, command, environment)
                .iter()
                .any(|candidate| is_executable(candidate))
        })
    }
}

pub fn check_tools(
    requirements: &[ToolRequirement],
    environment: &dyn Environment,
    resolver: &dyn ToolResolver,
) -> Vec<CheckResult> {
    requirements
        .iter()
        .map(|tool| {
            if resolver.command_exists(&tool.command, environment) {
                CheckResult::ok(
                    Category::Tools,
                    tool.name.clone(),
                    format!("command `{}` found on PATH", tool.command),
                )
            } else {
                let mut detail = format!("command `{}` was not found on PATH", tool.command);
                if let Some(hint) = &tool.install_hint {
                    if !hint.trim().is_empty() {
                        detail.push_str("; ");
                        detail.push_str(hint);
                    }
                }

                CheckResult::new(
                    failure_status(tool.required),
                    Category::Tools,
                    &tool.name,
                    detail,
                )
            }
        })
        .collect()
}

pub fn check_secrets(
    requirements: &[SecretRequirement],
    environment: &dyn Environment,
) -> Vec<CheckResult> {
    requirements
        .iter()
        .map(|secret| {
            if env_value_is_present(environment, &secret.env) {
                CheckResult::ok(
                    Category::Secrets,
                    secret.env.clone(),
                    "environment variable is set",
                )
            } else {
                CheckResult::new(
                    failure_status(secret.required),
                    Category::Secrets,
                    &secret.env,
                    "environment variable is not set or is empty",
                )
            }
        })
        .collect()
}

pub trait RegistryClient {
    fn check(
        &self,
        registry: &RegistryRequirement,
        auth: Option<&ResolvedAuth>,
    ) -> Result<RegistryResponse, RegistryProbeError>;
}

#[derive(Debug, Clone)]
pub struct HttpRegistryClient {
    timeout: Duration,
}

impl Default for HttpRegistryClient {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(5),
        }
    }
}

impl HttpRegistryClient {
    pub fn new(timeout: Duration) -> Self {
        Self { timeout }
    }

    fn send(
        &self,
        method: Method,
        registry: &RegistryRequirement,
        auth: Option<&ResolvedAuth>,
    ) -> Result<RegistryResponse, RegistryProbeError> {
        let client = Client::builder()
            .redirect(Policy::none())
            .build()
            .map_err(|_error| RegistryProbeError::ClientUnavailable)?;

        let request = client.request(method, &registry.url).timeout(self.timeout);
        let response = apply_auth(request, auth)
            .send()
            .map_err(RegistryProbeError::from_reqwest)?;

        Ok(RegistryResponse {
            status: response.status().as_u16(),
        })
    }
}

impl RegistryClient for HttpRegistryClient {
    fn check(
        &self,
        registry: &RegistryRequirement,
        auth: Option<&ResolvedAuth>,
    ) -> Result<RegistryResponse, RegistryProbeError> {
        let response = self.send(Method::HEAD, registry, auth)?;

        if response.status == 405 {
            self.send(Method::GET, registry, auth)
        } else {
            Ok(response)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistryResponse {
    pub status: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum RegistryProbeError {
    #[error("request timed out")]
    Timeout,
    #[error("could not connect to registry")]
    Unreachable,
    #[error("HTTP client could not be initialized")]
    ClientUnavailable,
    #[error("request failed")]
    RequestFailed,
}

impl RegistryProbeError {
    fn from_reqwest(error: reqwest::Error) -> Self {
        if error.is_timeout() {
            Self::Timeout
        } else if error.is_connect() {
            Self::Unreachable
        } else {
            Self::RequestFailed
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct SecretValue(String);

impl SecretValue {
    fn new(value: String) -> Self {
        Self(value)
    }

    fn expose(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for SecretValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("<redacted>")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedAuth {
    Bearer {
        token: SecretValue,
    },
    Basic {
        username: SecretValue,
        password: SecretValue,
    },
}

pub fn check_registries(
    requirements: &[RegistryRequirement],
    environment: &dyn Environment,
    client: &dyn RegistryClient,
) -> Vec<CheckResult> {
    requirements
        .iter()
        .map(|registry| {
            let auth = match resolve_auth(&registry.auth, environment) {
                Ok(auth) => auth,
                Err(problems) => {
                    return CheckResult::new(
                        failure_status(registry.required),
                        Category::Registries,
                        registry_identity(registry),
                        describe_auth_problems(&problems),
                    );
                }
            };

            match client.check(registry, auth.as_ref()) {
                Ok(response) => registry_response_result(registry, response),
                Err(error) => CheckResult::new(
                    failure_status(registry.required),
                    Category::Registries,
                    registry_identity(registry),
                    error.to_string(),
                ),
            }
        })
        .collect()
}

fn registry_response_result(
    registry: &RegistryRequirement,
    response: RegistryResponse,
) -> CheckResult {
    if (200..400).contains(&response.status) {
        return CheckResult::ok(
            Category::Registries,
            registry_identity(registry),
            format!("registry reachable (HTTP {})", response.status),
        );
    }

    let detail = if matches!(response.status, 401 | 403) && registry.auth.is_some() {
        format!(
            "registry rejected configured authentication (HTTP {})",
            response.status
        )
    } else if matches!(response.status, 401 | 403) {
        format!(
            "registry requires authentication or denied access (HTTP {})",
            response.status
        )
    } else {
        format!("registry returned HTTP {}", response.status)
    };

    CheckResult::new(
        failure_status(registry.required),
        Category::Registries,
        registry_identity(registry),
        detail,
    )
}

fn resolve_auth(
    auth: &Option<RegistryAuth>,
    environment: &dyn Environment,
) -> Result<Option<ResolvedAuth>, Vec<AuthEnvProblem>> {
    let Some(auth) = auth else {
        return Ok(None);
    };

    match auth {
        RegistryAuth::Bearer { token_env } => secret_from_env(environment, token_env)
            .map(|token| Some(ResolvedAuth::Bearer { token }))
            .map_err(|problem| vec![problem]),
        RegistryAuth::Basic {
            username_env,
            password_env,
        } => {
            let username = secret_from_env(environment, username_env);
            let password = secret_from_env(environment, password_env);

            match (username, password) {
                (Ok(username), Ok(password)) => {
                    Ok(Some(ResolvedAuth::Basic { username, password }))
                }
                (username, password) => {
                    let mut problems = Vec::new();
                    if let Err(problem) = username {
                        problems.push(problem);
                    }
                    if let Err(problem) = password {
                        problems.push(problem);
                    }
                    Err(problems)
                }
            }
        }
    }
}

fn secret_from_env(
    environment: &dyn Environment,
    name: &str,
) -> Result<SecretValue, AuthEnvProblem> {
    let Some(value) = environment.var_os(name) else {
        return Err(AuthEnvProblem::missing(name));
    };

    if value.as_os_str().is_empty() {
        return Err(AuthEnvProblem::missing(name));
    }

    value
        .into_string()
        .map(SecretValue::new)
        .map_err(|_value| AuthEnvProblem::not_unicode(name))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AuthEnvProblem {
    env: String,
    kind: AuthEnvProblemKind,
}

impl AuthEnvProblem {
    fn missing(env: &str) -> Self {
        Self {
            env: env.to_owned(),
            kind: AuthEnvProblemKind::MissingOrEmpty,
        }
    }

    fn not_unicode(env: &str) -> Self {
        Self {
            env: env.to_owned(),
            kind: AuthEnvProblemKind::NotUnicode,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum AuthEnvProblemKind {
    MissingOrEmpty,
    NotUnicode,
}

fn describe_auth_problems(problems: &[AuthEnvProblem]) -> String {
    let missing = problems
        .iter()
        .filter(|problem| problem.kind == AuthEnvProblemKind::MissingOrEmpty)
        .map(|problem| problem.env.as_str())
        .collect::<Vec<_>>();
    let not_unicode = problems
        .iter()
        .filter(|problem| problem.kind == AuthEnvProblemKind::NotUnicode)
        .map(|problem| problem.env.as_str())
        .collect::<Vec<_>>();

    let mut details = Vec::new();
    if !missing.is_empty() {
        details.push(format!(
            "auth environment variable {} is missing or empty",
            missing.join(", ")
        ));
    }
    if !not_unicode.is_empty() {
        details.push(format!(
            "auth environment variable {} is not valid Unicode",
            not_unicode.join(", ")
        ));
    }

    details.join("; ")
}

fn apply_auth(request: RequestBuilder, auth: Option<&ResolvedAuth>) -> RequestBuilder {
    match auth {
        Some(ResolvedAuth::Bearer { token }) => request.bearer_auth(token.expose()),
        Some(ResolvedAuth::Basic { username, password }) => {
            request.basic_auth(username.expose(), Some(password.expose()))
        }
        None => request,
    }
}

fn registry_identity(registry: &RegistryRequirement) -> String {
    format!("{} ({})", registry.name, registry.url)
}

fn failure_status(required: bool) -> Status {
    if required {
        Status::Err
    } else {
        Status::Warn
    }
}

fn env_value_is_present(environment: &dyn Environment, name: &str) -> bool {
    environment
        .var_os(name)
        .is_some_and(|value| !value.as_os_str().is_empty())
}

fn has_path_separator(command: &str) -> bool {
    command.contains('/') || command.contains('\\')
}

fn command_candidates(
    directory: &Path,
    command: &str,
    environment: &dyn Environment,
) -> Vec<PathBuf> {
    #[cfg(windows)]
    {
        if Path::new(command).extension().is_some() {
            return vec![directory.join(command)];
        }

        let path_ext = environment
            .var_os("PATHEXT")
            .unwrap_or_else(|| std::ffi::OsString::from(".COM;.EXE;.BAT;.CMD"));

        path_ext
            .to_string_lossy()
            .split(';')
            .filter(|extension| !extension.is_empty())
            .map(|extension| directory.join(format!("{command}{extension}")))
            .collect()
    }

    #[cfg(not(windows))]
    {
        let _ = environment;
        vec![directory.join(command)]
    }
}

fn is_executable(path: &Path) -> bool {
    let Ok(metadata) = path.metadata() else {
        return false;
    };

    if !metadata.is_file() {
        return false;
    }

    is_executable_file(&metadata)
}

#[cfg(unix)]
fn is_executable_file(metadata: &std::fs::Metadata) -> bool {
    use std::os::unix::fs::PermissionsExt;

    metadata.permissions().mode() & 0o111 != 0
}

#[cfg(not(unix))]
fn is_executable_file(_metadata: &std::fs::Metadata) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::collections::HashMap;
    use std::ffi::OsString;
    use std::fs;

    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt;

    use tempfile::tempdir;

    use crate::config::{RegistryAuth, RegistryRequirement, SecretRequirement, ToolRequirement};
    use crate::environment::Environment;
    use crate::report::Status;

    use super::*;

    #[derive(Default)]
    struct TestEnvironment {
        values: HashMap<String, OsString>,
    }

    impl TestEnvironment {
        fn with_var(mut self, name: &str, value: impl Into<OsString>) -> Self {
            self.values.insert(name.to_owned(), value.into());
            self
        }
    }

    impl Environment for TestEnvironment {
        fn var_os(&self, name: &str) -> Option<OsString> {
            self.values.get(name).cloned()
        }
    }

    #[test]
    fn required_missing_secret_should_be_error_without_leaking_value() {
        let secrets = vec![SecretRequirement {
            name: "Token".to_owned(),
            env: "FORGE_TEST_TOKEN".to_owned(),
            required: true,
        }];
        let environment =
            TestEnvironment::default().with_var("UNRELATED_SECRET", "do-not-print-this");

        let results = check_secrets(&secrets, &environment);
        let detail = format!("{results:?}");

        assert_eq!(results[0].status, Status::Err);
        assert!(!detail.contains("do-not-print-this"));
    }

    #[test]
    fn optional_missing_secret_should_be_warning() {
        let secrets = vec![SecretRequirement {
            name: "Token".to_owned(),
            env: "FORGE_TEST_TOKEN".to_owned(),
            required: false,
        }];

        let results = check_secrets(&secrets, &TestEnvironment::default());

        assert_eq!(results[0].status, Status::Warn);
    }

    #[test]
    fn path_tool_resolver_finds_executable_on_path() {
        let directory = tempdir().expect("create temp dir");
        let tool_path = directory.path().join("forge-test-tool");
        fs::write(&tool_path, "#!/bin/sh\n").expect("write tool");

        #[cfg(unix)]
        {
            let mut permissions = fs::metadata(&tool_path)
                .expect("read metadata")
                .permissions();
            permissions.set_mode(0o755);
            fs::set_permissions(&tool_path, permissions).expect("set executable bit");
        }

        let environment =
            TestEnvironment::default().with_var("PATH", directory.path().as_os_str().to_owned());

        assert!(PathToolResolver.command_exists("forge-test-tool", &environment));
    }

    #[test]
    fn missing_required_tool_should_be_error() {
        let tools = vec![ToolRequirement {
            name: "Missing".to_owned(),
            command: "definitely-missing".to_owned(),
            required: true,
            install_hint: None,
        }];

        let results = check_tools(&tools, &TestEnvironment::default(), &PathToolResolver);

        assert_eq!(results[0].status, Status::Err);
    }

    struct FakeRegistryClient {
        status: u16,
        calls: Cell<usize>,
    }

    impl RegistryClient for FakeRegistryClient {
        fn check(
            &self,
            _registry: &RegistryRequirement,
            _auth: Option<&ResolvedAuth>,
        ) -> Result<RegistryResponse, RegistryProbeError> {
            self.calls.set(self.calls.get() + 1);
            Ok(RegistryResponse {
                status: self.status,
            })
        }
    }

    fn required_registry(auth: Option<RegistryAuth>) -> RegistryRequirement {
        RegistryRequirement {
            name: "Registry".to_owned(),
            url: "https://registry.example.test/ping".to_owned(),
            required: true,
            auth,
        }
    }

    #[test]
    fn reachable_registry_should_be_ok() {
        let registry = required_registry(None);
        let client = FakeRegistryClient {
            status: 204,
            calls: Cell::new(0),
        };

        let results = check_registries(&[registry], &TestEnvironment::default(), &client);

        assert_eq!(results[0].status, Status::Ok);
        assert_eq!(client.calls.get(), 1);
    }

    #[test]
    fn registry_missing_auth_env_should_fail_without_request() {
        let registry = required_registry(Some(RegistryAuth::Bearer {
            token_env: "FORGE_TEST_TOKEN".to_owned(),
        }));
        let client = FakeRegistryClient {
            status: 200,
            calls: Cell::new(0),
        };

        let results = check_registries(&[registry], &TestEnvironment::default(), &client);

        assert_eq!(results[0].status, Status::Err);
        assert_eq!(client.calls.get(), 0);
        assert!(results[0].detail.contains("FORGE_TEST_TOKEN"));
    }

    #[test]
    fn registry_rejected_auth_should_be_error_without_secret_value() {
        let registry = required_registry(Some(RegistryAuth::Bearer {
            token_env: "FORGE_TEST_TOKEN".to_owned(),
        }));
        let environment = TestEnvironment::default().with_var("FORGE_TEST_TOKEN", "secret-value");
        let client = FakeRegistryClient {
            status: 401,
            calls: Cell::new(0),
        };

        let results = check_registries(&[registry], &environment, &client);
        let debug = format!("{results:?}");

        assert_eq!(results[0].status, Status::Err);
        assert!(!debug.contains("secret-value"));
    }
}
