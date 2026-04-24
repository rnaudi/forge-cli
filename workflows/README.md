# Workflows

This directory holds concrete tool adapters for the generic process in
`docs/process/`.

Use this directory for commands and tool-specific behavior such as:

- version-control commands
- issue-tracker commands
- bootstrap commands
- CI and release commands
- local automation commands
- project-specific workflow substitutions

Keep durable process rules in `docs/process/`. Keep concrete commands here.

## Default Adapter

- [bootstrap.md](bootstrap.md): expected project bootstrap contract
- [ci.md](ci.md): green-main, tidy, and low-noise automation guidance
- [jj-bd.md](jj-bd.md): Jujutsu for version control and optional `bd` local
  issue tracking

Executable GitHub Actions workflows live in `.github/workflows/`.

If a project uses different tools, add or replace an adapter instead of
rewriting every process document.
