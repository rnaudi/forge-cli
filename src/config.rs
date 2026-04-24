use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde::Deserialize;

pub const CONFIG_FILE: &str = "forge.bootstrap.toml";

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BootstrapConfig {
    #[serde(default)]
    pub tools: Vec<ToolRequirement>,
    #[serde(default)]
    pub secrets: Vec<SecretRequirement>,
    #[serde(default)]
    pub registries: Vec<RegistryRequirement>,
}

impl BootstrapConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        let mut errors = Vec::new();

        for (index, tool) in self.tools.iter().enumerate() {
            require_non_empty(&tool.name, format!("tools[{index}].name"), &mut errors);
            require_non_empty(
                &tool.command,
                format!("tools[{index}].command"),
                &mut errors,
            );
        }

        for (index, secret) in self.secrets.iter().enumerate() {
            require_non_empty(&secret.name, format!("secrets[{index}].name"), &mut errors);
            require_non_empty(&secret.env, format!("secrets[{index}].env"), &mut errors);
        }

        for (index, registry) in self.registries.iter().enumerate() {
            require_non_empty(
                &registry.name,
                format!("registries[{index}].name"),
                &mut errors,
            );
            require_non_empty(
                &registry.url,
                format!("registries[{index}].url"),
                &mut errors,
            );
            validate_registry_url(
                &registry.url,
                format!("registries[{index}].url"),
                &mut errors,
            );

            if let Some(auth) = &registry.auth {
                validate_auth(auth, index, &mut errors);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(ConfigError::Invalid { errors })
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolRequirement {
    pub name: String,
    pub command: String,
    #[serde(default = "default_required")]
    pub required: bool,
    #[serde(default)]
    pub install_hint: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretRequirement {
    pub name: String,
    pub env: String,
    #[serde(default = "default_required")]
    pub required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegistryRequirement {
    pub name: String,
    pub url: String,
    #[serde(default = "default_required")]
    pub required: bool,
    #[serde(default)]
    pub auth: Option<RegistryAuth>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase", deny_unknown_fields)]
pub enum RegistryAuth {
    Bearer {
        token_env: String,
    },
    Basic {
        username_env: String,
        password_env: String,
    },
}

#[derive(Debug)]
pub enum ConfigError {
    Missing { path: PathBuf },
    Read { path: PathBuf, kind: io::ErrorKind },
    Parse { path: PathBuf },
    Invalid { errors: Vec<String> },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing { path } => write!(formatter, "{} was not found", path.display()),
            Self::Read { path, kind } => {
                write!(formatter, "could not read {}: {kind}", path.display())
            }
            Self::Parse { path } => write!(formatter, "{} is not valid TOML", path.display()),
            Self::Invalid { errors } => {
                write!(formatter, "invalid bootstrap config: {}", errors.join("; "))
            }
        }
    }
}

impl std::error::Error for ConfigError {}

pub fn load_config(path: &Path) -> Result<BootstrapConfig, ConfigError> {
    let content = fs::read_to_string(path).map_err(|error| match error.kind() {
        io::ErrorKind::NotFound => ConfigError::Missing {
            path: path.to_path_buf(),
        },
        kind => ConfigError::Read {
            path: path.to_path_buf(),
            kind,
        },
    })?;

    let config =
        toml::from_str::<BootstrapConfig>(&content).map_err(|_error| ConfigError::Parse {
            path: path.to_path_buf(),
        })?;

    config.validate()?;

    Ok(config)
}

fn default_required() -> bool {
    true
}

fn require_non_empty(value: &str, field: String, errors: &mut Vec<String>) {
    if value.trim().is_empty() {
        errors.push(format!("{field} must not be empty"));
    }
}

fn validate_auth(auth: &RegistryAuth, registry_index: usize, errors: &mut Vec<String>) {
    match auth {
        RegistryAuth::Bearer { token_env } => {
            require_non_empty(
                token_env,
                format!("registries[{registry_index}].auth.token_env"),
                errors,
            );
        }
        RegistryAuth::Basic {
            username_env,
            password_env,
        } => {
            require_non_empty(
                username_env,
                format!("registries[{registry_index}].auth.username_env"),
                errors,
            );
            require_non_empty(
                password_env,
                format!("registries[{registry_index}].auth.password_env"),
                errors,
            );
        }
    }
}

fn validate_registry_url(value: &str, field: String, errors: &mut Vec<String>) {
    let Ok(url) = reqwest::Url::parse(value) else {
        errors.push(format!("{field} must be a valid HTTP or HTTPS URL"));
        return;
    };

    if !matches!(url.scheme(), "http" | "https") {
        errors.push(format!("{field} must use the http or https scheme"));
    }

    if url.host_str().is_none() {
        errors.push(format!("{field} must include a host"));
    }

    if !url.username().is_empty() || url.password().is_some() {
        errors.push(format!("{field} must not include credentials"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_required_to_true() {
        let config: BootstrapConfig = toml::from_str(
            r#"
                [[tools]]
                name = "Cargo"
                command = "cargo"

                [[secrets]]
                name = "Token"
                env = "TOKEN"

                [[registries]]
                name = "Registry"
                url = "https://registry.example.test/ping"
            "#,
        )
        .expect("parse config");

        assert!(config.tools[0].required);
        assert!(config.secrets[0].required);
        assert!(config.registries[0].required);
    }

    #[test]
    fn rejects_unknown_fields() {
        let error = toml::from_str::<BootstrapConfig>(
            r#"
                [[tools]]
                name = "Cargo"
                command = "cargo"
                unexpected = true
            "#,
        )
        .expect_err("reject unknown field");

        assert!(error.to_string().contains("unknown field"));
    }

    #[test]
    fn rejects_registry_urls_with_credentials() {
        let config: BootstrapConfig = toml::from_str(
            r#"
                [[registries]]
                name = "Registry"
                url = "https://user:password@registry.example.test/ping"
            "#,
        )
        .expect("parse config");

        let error = config.validate().expect_err("reject credentials");

        assert!(error.to_string().contains("must not include credentials"));
        assert!(!error.to_string().contains("password"));
    }
}
