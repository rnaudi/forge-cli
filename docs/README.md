# Docs

This directory is the documentation home for `forge-cli`, the Rust command-line
tool installed as `forge`.

Use it for:

- stable project behavior and architecture
- technical design records
- concrete development commands
- process and workflow guidance for humans and agents

## Core Docs

- [architecture.md](architecture.md): stable project architecture overview
- [development.md](development.md): contributor operating manual for the concrete project
- [references.md](references.md): curated readings that shaped the project

## Structured Project Docs

- [spec/README.md](spec/README.md): stable behavior and contract docs
- [designs/README.md](designs/README.md): technical design docs
- [process/README.md](process/README.md): workflow guidance
- [notes/README.md](notes/README.md): append-only topical notes, discoveries,
  and operational breadcrumbs
- [../workflows/README.md](../workflows/README.md): concrete tool adapters for
  version control, tracking, and local workflow commands

## Rule of Thumb

- use `spec/` for what should be built and why
- use `designs/` for technical choices and tradeoffs
- use `architecture.md` for the system shape as a whole
- use `development.md` for concrete contributor commands and environment rules
- use `process/` for the workflow around planning, implementation, review, and commit
- use `workflows/` for tool-specific commands that implement the process
- use `notes/` when writing something quickly is better than not writing it at
  all
