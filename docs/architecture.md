# Project Architecture

`forge-cli` is a Rust command-line tool that helps humans and coding agents
bootstrap real software projects safely. It reads a repository-owned bootstrap
configuration, checks local setup requirements, and prints a readiness report
without exposing secret values.

## System Context

Primary users are:

- humans starting work in a repository
- coding agents preparing to build, test, or modify a repository
- maintainers who want onboarding requirements kept explicit in source control

External systems are intentionally minimal in the first milestone:

- the local operating system and `PATH`
- process environment variables
- generic HTTP endpoints for private registry readiness checks

## High-Level Architecture

The first implementation is a single Rust binary crate named `forge`.

Expected internal boundaries:

- CLI layer: parses commands with `clap` and dispatches handlers.
- Config layer: loads and validates `forge.bootstrap.toml`.
- Check layer: runs tool, secret, and registry checks.
- Reporting layer: renders readable terminal output with OK/WARN/ERR statuses.
- Exit layer: maps check results to process exit codes.

The binary should keep command parsing separate from check execution so the core
behavior remains easy to test without invoking a subprocess.

## Command Surface

Initial commands:

- `forge doctor`: runs all configured checks and prints one readiness report.
- `forge secrets check`: checks configured environment-variable requirements.
- `forge registries check`: checks configured private registry URLs.

The initial config file is `forge.bootstrap.toml`.

## Request / Data Flow

Representative `forge doctor` flow:

1. User runs `forge doctor` from a repository checkout.
2. The CLI layer parses the command.
3. The config layer reads `forge.bootstrap.toml`.
4. The check layer validates required tools, required environment variables, and
   configured registry URLs.
5. The reporting layer prints statuses and human-readable details without
   secret values.
6. The exit layer returns success when required checks pass and non-zero when
   any required check fails.

## Major Components

### CLI Layer

Owns command definitions and user-facing command names. It should avoid business
logic beyond selecting the requested check set.

### Config Layer

Owns deserialization and validation of `forge.bootstrap.toml`. Unknown fields
should be handled deliberately so configuration mistakes do not silently change
bootstrap behavior.

### Check Layer

Owns local probes:

- tool checks inspect executable availability
- secret checks inspect environment-variable presence only
- registry checks use generic HTTP reachability/auth-readiness requests

### Reporting Layer

Owns stable terminal report formatting. Reports can include requirement names,
URLs, and status messages. Reports must never include secret values.

## Data Model

The core data shapes are:

- `BootstrapConfig`: project bootstrap requirements from TOML.
- `ToolRequirement`: command expected to be available locally.
- `SecretRequirement`: environment-variable name and whether it is required.
- `RegistryRequirement`: URL and optional auth environment-variable names.
- `CheckResult`: status, requirement identity, and safe detail message.
- `ReadinessReport`: aggregate check results and final exit decision.

## Operational Concerns

Security constraints are central:

- secret values are never printed, persisted, logged, or committed
- config may name secret environment variables but must not contain secret
  values
- failures should explain what is missing without revealing sensitive contents

Network behavior should stay bounded and predictable. Registry checks should use
short timeouts and report unreachable endpoints clearly.

## Build and Runtime Boundaries

The first milestone is one deployable binary installed as `forge`.

Cargo is the build system:

- build: `cargo build`
- test: `cargo test`
- lint: `cargo clippy --all-targets --all-features -- -D warnings`

## References

- `docs/spec/02-forge-doctor.md`
- `docs/designs/0001-config-format-and-secret-handling.md`
- `docs/development.md`
