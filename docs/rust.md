# Rust Style

This document is the Rust style guide for `forge-cli`.

Use it with `docs/development.md`, `docs/testing.md`, and the relevant spec or
ADR for the change.

## Tooling

Before committing Rust changes, run:

```bash
cargo fmt
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

CI runs the same gate with `cargo fmt --check`.

## Module Boundaries

- Keep `src/main.rs` thin. It should parse the CLI, call library code, print the
  report, and exit.
- Keep `src/cli.rs` focused on `clap` command definitions and command-to-scope
  mapping.
- Keep config loading and validation in `src/config.rs`.
- Keep probes in `src/checks.rs`.
- Keep terminal rendering and exit aggregation in `src/report.rs`.
- Keep environment access behind traits when tests need control over local
  state.

Command parsing should not contain business logic. Command handlers should
delegate to functions that can be tested without spawning a subprocess.

## Ownership and Borrowing

- Prefer borrowed parameters: `&str` over `String`, `&[T]` over `Vec<T>`, and
  `&Path` over `PathBuf` unless ownership is required.
- Clone only when the clone expresses real ownership. Avoid defensive clones to
  appease the borrow checker before checking whether a borrow is simpler.
- Passing small `Copy` values by value is fine.
- Avoid collecting intermediate `Vec`s when an iterator can express the flow
  clearly.
- Prefer straightforward code over clever lifetime gymnastics. If a small owned
  value makes the boundary clearer, use it deliberately.

## Error Handling

- Return `Result<T, E>` for fallible operations.
- Use `thiserror` for project error types.
- Do not use `unwrap()` or `expect()` in production paths.
- Panics are acceptable only for impossible internal invariants, and the message
  must explain the invariant.
- User-facing errors should be actionable and safe to print.
- Config errors should identify the safe field or path, not dump raw input when
  that input might contain sensitive material.

## Secret Safety

The central security rule applies to Rust code:

```text
never print, persist, log, or commit secret values
```

- Do not derive or implement `Debug` for secret-bearing types unless the output
  is redacted.
- Do not include environment-variable values in error messages.
- Reports may include secret names, environment-variable names, and safe
  requirement names.
- Tests that use fake secret values must assert those values do not appear in
  rendered output.
- Avoid adding logging until there is an ADR for what can be logged safely.

## Types and APIs

- Use explicit domain types when they make illegal states harder to represent.
- Keep structs small and purpose-specific. Avoid "options bag" structs until
  repeated call sites justify them.
- Prefer enums for closed status sets such as OK/WARN/ERR.
- Keep trait boundaries at resource edges: environment, filesystem, network,
  process execution, and clock access.
- Use `dyn Trait` when tests or runtime polymorphism need it. Prefer generics
  only when they make ownership or performance clearer.

## Comments and Documentation

- Comments should explain intent, invariants, or non-obvious constraints.
- Avoid comments that restate the code.
- Public APIs that become part of the crate contract should have rustdoc.
- Every TODO needs a Beads issue ID or should not be committed.
- Follow `docs/documentation.md` for prose and code-comment style.

## Dependencies

- Add dependencies only when they remove meaningful complexity.
- Prefer dependencies already used by the project.
- For new dependencies, consider maintenance, license, transitive dependency
  load, and whether the standard library is enough.
- Security-sensitive dependency decisions should usually have an ADR.

## Review Checklist

- Does the change preserve the spec or ADR contract?
- Is the CLI layer still thin?
- Are resource boundaries testable?
- Are required failures represented as errors, not panics?
- Are secret values impossible to leak through `Debug`, `Display`, reports, or
  tests?
- Does clippy pass without suppressions?
