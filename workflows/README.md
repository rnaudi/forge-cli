# Workflows

This directory holds concrete tool adapters for the generic process in
`docs/process/`.

Use this directory for commands and tool-specific behavior such as:

- version-control commands
- issue-tracker commands
- local automation commands
- project-specific workflow substitutions

Keep durable process rules in `docs/process/`. Keep concrete commands here.

## Default Adapter

- [jj-bd.md](jj-bd.md): Jujutsu for version control and `bd` for local
  issue tracking

If a project uses different tools, add or replace an adapter instead of
rewriting every process document.
