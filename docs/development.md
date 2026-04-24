# Development Guide

This project is `forge-cli`, a Rust command-line tool for making repository
bootstrap requirements explicit and locally checkable.

## Toolchain

- Rust stable toolchain
- Cargo
- `jj` for version control
- Beads (`bd`) for issue tracking

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

## Mechanical Quality Gate

Run the full local quality gate before commit:

```bash
cargo fmt
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

Also run the relevant smoke command when behavior changes. For the current
checkout:

```bash
cargo run -- doctor
```

## Guide Quality Gate

Mechanical checks are necessary but not enough. Before returning work as ready
for human approval, review, or commit, apply the relevant repo guides:

- Rust code changes: `docs/rust.md`
- test changes or test gaps: `docs/testing.md`
- docs and comments: `docs/documentation.md`
- implementation review: `docs/process/code-review.md`
- spec or ADR changes: `docs/process/planning.md#reviewing-specs-and-adrs`
- secret, credential, registry, or protected-data changes: `docs/security.md`

The handoff should mention which gates ran and any gate that was skipped or
could not run.

## Testing Strategy

See `docs/testing.md` for test design rules.

Project-specific expectations:

- unit tests cover config parsing, validation decisions, report status
  aggregation, and exit-code behavior
- tests must not require real secret values or access to a private registry
- registry reachability tests should use local or mocked HTTP endpoints
- every change that affects command behavior should be traceable to a spec or
  ADR update

## Coding Conventions

See `docs/rust.md` for Rust style.

Project-specific expectations:

- use `clap` for command parsing
- keep command handlers small and delegate checks to testable modules
- treat missing required checks as failures, optional missing checks as
  warnings, and successful checks as OK
- never print, persist, log, or commit secret values; reports may include secret
  names, but not secret contents

## Environment

`forge` reads bootstrap requirements from `forge.bootstrap.toml` in the current
working directory.

The first milestone supports:

- local tool presence checks
- environment-variable presence checks
- generic HTTP registry reachability/auth-readiness checks

`forge` does not manage secrets, fetch secret values, or write shell profiles.

## Workflow

Use `jj` for version control and Beads (`bd`) for issue tracking. The
repository keeps the concrete adapter in `workflows/jj-bd.md`.

Follow the repository lifecycle:

```text
understand -> plan/design -> approve -> implement -> review -> commit
```

See also:

- `workflows/bootstrap.md` for fresh-checkout setup
- `workflows/ci.md` for the quality gate
- `docs/rust.md` for Rust style
- `docs/testing.md` for testing style
- `docs/documentation.md` for prose and code-comment style
- `docs/security.md` for secret-handling rules

## Troubleshooting

Add concrete setup failures here as they are discovered. Keep secret examples to
names only and never include secret values.
