use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "forge",
    version,
    about = "Check repository bootstrap readiness"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: ForgeCommand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Subcommand)]
pub enum ForgeCommand {
    /// Run all configured bootstrap checks.
    Doctor,
    /// Validate bootstrap configuration.
    Config {
        #[command(subcommand)]
        command: ConfigCommand,
    },
    /// Check configured secret requirements.
    Secrets {
        #[command(subcommand)]
        command: SecretsCommand,
    },
    /// Check configured private registry requirements.
    Registries {
        #[command(subcommand)]
        command: RegistriesCommand,
    },
}

impl ForgeCommand {
    pub fn scope(self) -> CheckScope {
        match self {
            Self::Doctor => CheckScope::All,
            Self::Config {
                command: ConfigCommand::Validate,
            } => CheckScope::Config,
            Self::Secrets {
                command: SecretsCommand::Check,
            } => CheckScope::Secrets,
            Self::Registries {
                command: RegistriesCommand::Check,
            } => CheckScope::Registries,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Subcommand)]
pub enum ConfigCommand {
    /// Validate forge.bootstrap.toml without running live checks.
    Validate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Subcommand)]
pub enum SecretsCommand {
    /// Check required environment variables without printing values.
    Check,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Subcommand)]
pub enum RegistriesCommand {
    /// Check configured registry URLs for reachability and auth readiness.
    Check,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckScope {
    All,
    Config,
    Secrets,
    Registries,
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::*;

    #[test]
    fn parses_doctor_command() {
        let cli = Cli::parse_from(["forge", "doctor"]);

        assert_eq!(cli.command.scope(), CheckScope::All);
    }

    #[test]
    fn parses_config_validate_command() {
        let cli = Cli::parse_from(["forge", "config", "validate"]);

        assert_eq!(cli.command.scope(), CheckScope::Config);
    }

    #[test]
    fn parses_secrets_check_command() {
        let cli = Cli::parse_from(["forge", "secrets", "check"]);

        assert_eq!(cli.command.scope(), CheckScope::Secrets);
    }

    #[test]
    fn parses_registries_check_command() {
        let cli = Cli::parse_from(["forge", "registries", "check"]);

        assert_eq!(cli.command.scope(), CheckScope::Registries);
    }
}
