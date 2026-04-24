# Development Guide

This project is `forge-cli`, a Rust command-line tool for making repository
bootstrap requirements explicit and locally checkable.

## Toolchain

- Rust stable toolchain
- Cargo
- `jj` for version control

## Build

```bash
cargo build
```

## Test

```bash
cargo test
```

## Fast Feedback Loop

```bash
cargo fmt --check
cargo test
```

## Run Locally

During development, run the binary through Cargo:

```bash
cargo run -- doctor
cargo run -- secrets check
cargo run -- registries check
```

After building, the installed command name is:

```bash
forge doctor
```

## Tidy

Run the full local quality gate before commit:

```bash
cargo fmt
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

## Testing Strategy

- Unit tests cover config parsing, validation decisions, report status
  aggregation, and exit-code behavior.
- Tests must not require real secret values or access to a private registry.
- Registry reachability tests should use local or mocked HTTP endpoints.
- Every change that affects command behavior should be traceable to a spec or
  design doc update.

## Coding Conventions

- Use `clap` for command parsing.
- Keep command handlers small and delegate checks to testable modules.
- Treat missing required checks as failures, optional missing checks as
  warnings, and successful checks as OK.
- Never print, persist, log, or commit secret values. Reports may include secret
  names, but not secret contents.

## Environment

`forge` reads bootstrap requirements from `forge.bootstrap.toml` in the current
working directory.

The first milestone supports:

- local tool presence checks
- environment-variable presence checks
- generic HTTP registry reachability/auth-readiness checks

`forge` does not manage secrets, fetch secret values, or write shell profiles.

## Workflow

Use `jj` for version control. The repository keeps the default `jj`/`bd`
workflow adapter in `workflows/jj-bd.md`, but `bd` is not required for the first
MVP implementation.

Start using `bd` once follow-up work needs tracking beyond this initial MVP.

Follow the repository lifecycle:

```text
understand -> plan/design -> approve -> implement -> review -> commit
```

See also:

- `workflows/bootstrap.md` for fresh-checkout setup
- `workflows/ci.md` for the quality gate
- `docs/security.md` for secret-handling rules

## Troubleshooting

Add concrete setup failures here as they are discovered. Keep secret examples to
names only and never include secret values.
