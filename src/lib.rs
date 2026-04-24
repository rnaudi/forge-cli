pub mod checks;
pub mod cli;
pub mod config;
pub mod environment;
pub mod report;

use std::path::Path;

use checks::{check_registries, check_secrets, check_tools, HttpRegistryClient, PathToolResolver};
use cli::{CheckScope, Cli};
use config::{load_config, CONFIG_FILE};
use environment::SystemEnvironment;
use report::{Category, CheckResult, ReadinessReport};

pub fn run(cli: Cli) -> ReadinessReport {
    let environment = SystemEnvironment;
    let tool_resolver = PathToolResolver;
    let registry_client = HttpRegistryClient::default();

    run_with_dependencies(
        cli.command.scope(),
        Path::new(CONFIG_FILE),
        &environment,
        &tool_resolver,
        &registry_client,
    )
}

pub fn run_with_dependencies(
    scope: CheckScope,
    config_path: &Path,
    environment: &dyn environment::Environment,
    tool_resolver: &dyn checks::ToolResolver,
    registry_client: &dyn checks::RegistryClient,
) -> ReadinessReport {
    let config = match load_config(config_path) {
        Ok(config) => config,
        Err(error) => {
            return ReadinessReport::new(vec![CheckResult::err(
                Category::Config,
                config_path.display().to_string(),
                error.to_string(),
            )]);
        }
    };

    if matches!(scope, CheckScope::Config) {
        return ReadinessReport::new(vec![CheckResult::ok(
            Category::Config,
            config_path.display().to_string(),
            "bootstrap config is valid",
        )]);
    }

    let mut results = Vec::new();

    if matches!(scope, CheckScope::All) {
        results.extend(check_tools(&config.tools, environment, tool_resolver));
    }

    if matches!(scope, CheckScope::All | CheckScope::Secrets) {
        results.extend(check_secrets(&config.secrets, environment));
    }

    if matches!(scope, CheckScope::All | CheckScope::Registries) {
        results.extend(check_registries(
            &config.registries,
            environment,
            registry_client,
        ));
    }

    ReadinessReport::new(results)
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::collections::HashMap;
    use std::ffi::OsString;
    use std::fs;

    use tempfile::tempdir;

    use crate::checks::{
        RegistryClient, RegistryProbeError, RegistryResponse, ResolvedAuth, ToolResolver,
    };
    use crate::cli::CheckScope;
    use crate::environment::Environment;

    use super::*;

    #[derive(Default)]
    struct TestEnvironment {
        values: HashMap<String, OsString>,
    }

    impl TestEnvironment {
        fn with_var(mut self, name: &str, value: &str) -> Self {
            self.values.insert(name.to_owned(), OsString::from(value));
            self
        }
    }

    impl Environment for TestEnvironment {
        fn var_os(&self, name: &str) -> Option<OsString> {
            self.values.get(name).cloned()
        }
    }

    struct MissingToolResolver;

    impl ToolResolver for MissingToolResolver {
        fn command_exists(&self, _command: &str, _environment: &dyn Environment) -> bool {
            false
        }
    }

    struct PanickingEnvironment;

    impl Environment for PanickingEnvironment {
        fn var_os(&self, name: &str) -> Option<OsString> {
            panic!("config validation should not read environment variable {name}");
        }
    }

    struct PanickingToolResolver;

    impl ToolResolver for PanickingToolResolver {
        fn command_exists(&self, command: &str, _environment: &dyn Environment) -> bool {
            panic!("config validation should not resolve tool command {command}");
        }
    }

    struct PanickingRegistryClient;

    impl RegistryClient for PanickingRegistryClient {
        fn check(
            &self,
            registry: &config::RegistryRequirement,
            _auth: Option<&ResolvedAuth>,
        ) -> Result<RegistryResponse, RegistryProbeError> {
            panic!(
                "config validation should not probe registry {}",
                registry.name
            );
        }
    }

    struct FakeRegistryClient {
        calls: Cell<usize>,
    }

    impl RegistryClient for FakeRegistryClient {
        fn check(
            &self,
            _registry: &config::RegistryRequirement,
            _auth: Option<&ResolvedAuth>,
        ) -> Result<RegistryResponse, RegistryProbeError> {
            self.calls.set(self.calls.get() + 1);
            Ok(RegistryResponse { status: 200 })
        }
    }

    #[test]
    fn secret_subset_should_not_run_tool_or_registry_checks() {
        let directory = tempdir().expect("create temp dir");
        let config_path = directory.path().join(CONFIG_FILE);
        fs::write(
            &config_path,
            r#"
                [[tools]]
                name = "Missing tool"
                command = "definitely-not-installed"
                required = true

                [[secrets]]
                name = "Token"
                env = "FORGE_TEST_TOKEN"
                required = true

                [[registries]]
                name = "Registry"
                url = "https://registry.example.test/ping"
                required = true
            "#,
        )
        .expect("write config");

        let environment = TestEnvironment::default().with_var("FORGE_TEST_TOKEN", "secret-value");
        let registry_client = FakeRegistryClient {
            calls: Cell::new(0),
        };

        let report = run_with_dependencies(
            CheckScope::Secrets,
            &config_path,
            &environment,
            &MissingToolResolver,
            &registry_client,
        );

        assert_eq!(report.exit_code(), 0);
        assert_eq!(registry_client.calls.get(), 0);
        assert_eq!(report.results().len(), 1);
        assert!(!report.render().contains("secret-value"));
    }

    #[test]
    fn config_validate_should_not_run_live_checks() {
        let directory = tempdir().expect("create temp dir");
        let config_path = directory.path().join(CONFIG_FILE);
        fs::write(
            &config_path,
            r#"
                [[tools]]
                name = "Missing tool"
                command = "definitely-not-installed"
                required = true

                [[secrets]]
                name = "Token"
                env = "FORGE_TEST_TOKEN"
                required = true

                [[registries]]
                name = "Registry"
                url = "https://registry.example.test/ping"
                required = true
            "#,
        )
        .expect("write config");

        let report = run_with_dependencies(
            CheckScope::Config,
            &config_path,
            &PanickingEnvironment,
            &PanickingToolResolver,
            &PanickingRegistryClient,
        );

        assert_eq!(report.exit_code(), 0);
        assert_eq!(
            report.results(),
            &[CheckResult::ok(
                Category::Config,
                config_path.display().to_string(),
                "bootstrap config is valid",
            )]
        );
    }

    #[test]
    fn config_validate_should_report_invalid_config_without_live_checks() {
        let directory = tempdir().expect("create temp dir");
        let config_path = directory.path().join(CONFIG_FILE);
        fs::write(
            &config_path,
            r#"
                [[registries]]
                name = "Registry"
                url = "https://secret-value@registry.example.test/ping"
            "#,
        )
        .expect("write config");

        let report = run_with_dependencies(
            CheckScope::Config,
            &config_path,
            &PanickingEnvironment,
            &PanickingToolResolver,
            &PanickingRegistryClient,
        );

        let rendered = report.render();

        assert_eq!(report.exit_code(), 1);
        assert!(rendered.contains("ERR"));
        assert!(rendered.contains("must not include credentials"));
        assert!(!rendered.contains("secret-value"));
    }

    #[test]
    fn missing_config_should_return_error_report() {
        let directory = tempdir().expect("create temp dir");
        let config_path = directory.path().join(CONFIG_FILE);

        let report = run_with_dependencies(
            CheckScope::All,
            &config_path,
            &TestEnvironment::default(),
            &MissingToolResolver,
            &FakeRegistryClient {
                calls: Cell::new(0),
            },
        );

        assert_eq!(report.exit_code(), 1);
        assert!(report.render().contains("ERR"));
        assert!(report.render().contains("was not found"));
    }
}
