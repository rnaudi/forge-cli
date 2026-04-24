# Security and Protected Data

This project is security-sensitive because its purpose is to name and validate
local bootstrap requirements, including secret environment variables and private
registry access.

## Defaults

- Do not commit secrets, tokens, credentials, private keys, or protected data.
- `forge.bootstrap.toml` may name secret environment variables, but must not
  contain secret values.
- `forge` may read secret values transiently from the process environment for
  validation or registry authentication.
- `forge` must never print, persist, log, or commit secret values.
- Reports may include safe identities such as tool names, environment-variable
  names, and registry URLs.

## Bootstrap Requirements

This repository's own MVP bootstrap does not require private credentials.

Safe commands without secrets:

```bash
cargo build
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -- doctor
```

When another repository uses `forge`, it should document required credential
names in `forge.bootstrap.toml` and keep acquisition instructions in that
repository's docs or approved internal access process.

## Protected Data

Tests and fixtures in this repository must use synthetic values only. Do not use
real tokens, private registry URLs, production logs, or customer data as test
fixtures.

If future tests need representative registry behavior, use local HTTP fixtures
or scrubbed synthetic endpoints.

## Agent Rules

Agents working in this repository must:

- validate secret presence by name only
- avoid printing environment values
- avoid adding real credentials to docs, tests, fixtures, logs, or commits
- ask before running commands that mutate external registries or access private
  systems

## References

- `docs/spec/02-forge-doctor.md`
- `docs/designs/0001-config-format-and-secret-handling.md`
- `forge.bootstrap.toml`
