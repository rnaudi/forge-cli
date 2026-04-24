# Bootstrap Workflow

This file describes the smallest repeatable setup path for a fresh `forge-cli`
checkout.

## Required Tools

- Rust stable toolchain
- Cargo
- `jj` for version control

`bd` is the intended follow-up tracker, but it is not required to build, test,
or run the MVP.

## Fresh Checkout

From the repository root:

```bash
cargo build
cargo test
cargo run -- doctor
```

The expected local readiness check is:

```bash
forge doctor
```

During development, use:

```bash
cargo run -- doctor
```

## Agent Behavior

Agents should be able to:

- detect missing tools with `forge doctor`
- report missing credentials without printing secret values
- run `cargo test` for functional validation
- avoid destructive environment changes unless explicitly approved

## Keep It Boring

Bootstrap should install and verify the toolchain. It should not hide complex
business logic, deploy production by default, or become a private framework.
