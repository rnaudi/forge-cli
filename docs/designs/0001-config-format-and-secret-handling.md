---
id: 0001
title: Config Format and Secret Handling
status: implemented
tags: [configuration, security, cli]
created: 2026-04-24
accepted: 2026-04-24
implemented: 2026-04-24
superseded-by:
---

# Design 0001: Config Format and Secret Handling

## Status

Implemented on 2026-04-24.

## Summary

`forge-cli` will use a repository-owned `forge.bootstrap.toml` file for
bootstrap requirements and will treat secrets as environment-variable references
only. Secret values may be read transiently for validation or HTTP auth, but
must never be printed, persisted, logged, or committed.

## Context

The first milestone needs enough structure to check local tools, required
environment variables, and private registry readiness. The config must be easy
for humans and agents to review in a repository while keeping actual secret
material outside source control.

## Goals / Non-goals

### Goals

- Use a simple, reviewable config format checked into the repository.
- Support required and optional local tool checks.
- Support required and optional environment-variable presence checks.
- Support generic HTTP/private registry checks suitable for Artifactory.
- Make secret handling conservative and explicit.
- Keep the first implementation small enough to test thoroughly.

### Non-goals

- Fetch or manage secret values.
- Write shell profiles or mutate local environments.
- Model every registry ecosystem.
- Add remote config discovery.
- Add machine-readable output in the first milestone.

## Decision

Use `forge.bootstrap.toml` at the repository root as the initial config file.
The file describes requirement names, whether they are required, and safe
metadata. It may contain environment-variable names, but not secret values.

Initial shape:

```toml
[[tools]]
name = "cargo"
command = "cargo"
required = true

[[secrets]]
name = "ARTIFACTORY_TOKEN"
env = "ARTIFACTORY_TOKEN"
required = true

[[registries]]
name = "internal-artifactory"
url = "https://artifactory.example.com/artifactory/api/system/ping"
required = true
auth = { type = "bearer", token_env = "ARTIFACTORY_TOKEN" }
```

Design choices:

- TOML is used because it is readable, comment-friendly, and common in Rust
  projects.
- The command looks for `forge.bootstrap.toml` in the current working directory
  in the first milestone.
- Secret config stores names and environment-variable references only.
- Missing required secrets fail checks; missing optional secrets warn.
- Registry auth material is read from the environment only when needed for the
  request and is never included in report details.
- Unknown config fields should fail validation so misspelled requirements do
  not silently pass.

## Alternatives Considered

### YAML

- Benefits: common for CI and infrastructure configuration.
- Drawbacks: more complex parsing edge cases and less aligned with Rust project
  conventions than TOML.

### JSON

- Benefits: strict syntax and easy machine generation.
- Drawbacks: less pleasant for hand-authored repository configuration and no
  native comments.

### Secrets Embedded in Config

- Benefits: simpler implementation for registry requests.
- Drawbacks: violates the core security rule and risks committing secret values.
  Rejected.

### Ecosystem-Specific Registry Models First

- Benefits: could provide richer diagnostics for Cargo, npm, Maven, or other
  registries.
- Drawbacks: expands scope before the generic readiness workflow is proven.
  Rejected for the first milestone.

## Implementation Plan

- **Phase 1: Project scaffold** - create a Rust Cargo binary named `forge` with
  `clap`, TOML config parsing, and test dependencies.
- **Phase 2: Config model** - deserialize `forge.bootstrap.toml`, reject unknown
  fields, and validate required identifiers.
- **Phase 3: Local checks** - implement tool checks and environment-variable
  presence checks without printing secret values.
- **Phase 4: Registry checks** - implement bounded generic HTTP checks with
  optional environment-backed auth.
- **Phase 5: Reporting and exits** - render OK/WARN/ERR reports and return
  non-zero when required checks fail.
- **Phase 6: Tests and quality gate** - add focused unit/integration tests and
  run `cargo fmt`, `cargo test`, and `cargo clippy --all-targets --all-features
  -- -D warnings`.

## Consequences

### Positive

- The repository gains a clear, reviewable bootstrap contract.
- Secret values stay outside source control and terminal output.
- The initial CLI remains small enough for focused tests.
- A generic HTTP registry check works for Artifactory-like systems without
  hardcoding one package ecosystem.

### Negative

- Current-working-directory config discovery is simple but less flexible than
  searching parent directories or supporting `--config`.
- Generic HTTP checks may provide less targeted remediation than
  ecosystem-specific registry diagnostics.
- Environment-only secret handling depends on users or agents setting variables
  before running checks.

## Open Questions

- Should a later milestone search parent directories for `forge.bootstrap.toml`?
- Should `forge` eventually provide JSON output for CI or agent consumption?
- Should auth support be limited to bearer and basic auth until a concrete need
  appears?

## Future Work

- Machine-readable report output.
- Parent-directory config discovery or explicit `--config`.
- Ecosystem-specific registry adapters.
- Optional documentation drift checks against onboarding docs.

## References

- `docs/spec/02-forge-doctor.md`
- `docs/architecture.md`
- `docs/development.md`
