# forge-cli

`forge-cli` is a Rust command-line tool for making repository bootstrap
requirements explicit and locally checkable.

The installed command is `forge`.

The first milestone focuses on:

- reading `forge.bootstrap.toml`
- checking required local tools
- checking required environment variables without printing secret values
- checking generic HTTP/private registry reachability
- printing a readable OK/WARN/ERR readiness report
- exiting non-zero when required checks fail

`forge` does not manage secrets, fetch secret values, write shell profiles, or
try to support every registry ecosystem in its initial scope.

## Operating Biases

- Keep bootstrap requirements in the repo.
- Prefer executable checks over setup lore.
- Report missing access without exposing secret values.
- Keep workflow defaults local-first and version-controlled.

## Commands

Initial commands:

```bash
forge doctor
forge config validate
forge secrets check
forge registries check
```

## Development

Start with:

- `AGENTS.md` for the project lifecycle and agent entry point
- `docs/development.md` for Rust/Cargo commands
- `docs/architecture.md` for the system shape
- `docs/spec/02-forge-doctor.md` for first-milestone behavior
- `docs/designs/0001-config-format-and-secret-handling.md` for the initial
  config and secret-handling decision

The default workflow remains:

- version control: `jj`
- tracking: Beads (`bd`)
