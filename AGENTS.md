# AGENTS.md

This repository contains `forge-cli`, a Rust command-line tool installed as
`forge`.

`forge` helps humans and coding agents bootstrap software projects safely by
checking repository-owned setup requirements before deeper work begins.

## Purpose

This repo exists to hold:

- the Rust CLI implementation
- a start-of-session contract for agents
- reusable process docs
- workflow adapters for concrete tools
- project specs and technical design records

## Entry Point

Start here at the beginning of every new session.

Then read:

1. `docs/README.md`
2. `docs/process/README.md`
3. the single task-specific process doc you need
4. `workflows/README.md` when you need concrete tool commands

Task-specific docs:

- planning or design work: `docs/process/planning.md`
- implementation work: `docs/process/implementation.md`
- review work: `docs/process/code-review.md`
- commit work: `docs/process/committing.md`
- operability work: `docs/process/operability.md`
- stewardship or triage work: `docs/process/stewardship.md`
- tool or workflow setup: `workflows/README.md`
- bootstrap setup: `workflows/bootstrap.md`
- CI or automation setup: `workflows/ci.md`
- commit message style: `docs/process/committing.md`
- security, secrets, or protected data: `docs/security.md`

Project-specific context:

- development commands: `docs/development.md`
- architecture: `docs/architecture.md`
- bootstrap contract: `workflows/bootstrap.md`
- CI and quality gates: `workflows/ci.md`
- security and secret handling: `docs/security.md`
- first milestone behavior: `docs/spec/02-forge-doctor.md`
- initial config and secret-handling ADR:
  `docs/designs/0001-config-format-and-secret-handling.md`

Do not read the whole repo by default.

## Working Rules

1. Keep process docs in `docs/process/`.
2. Keep large design docs in `docs/designs/`.
3. Keep stable behavioral documentation in `docs/spec/`.
4. Put high-level project operating context in `docs/development.md`.
5. Put system shape in `docs/architecture.md`.
6. Put concrete tool commands in `workflows/`.
7. Do not print, persist, log, or commit secret values.
8. Put lightweight project status surfaces in `devhub/` when they become useful.
9. Create commits noninteractively with `jj commit -m "<message>"`.

## Document Taxonomy

- `docs/spec/` is for stable behavior, scope, and external contracts
- `docs/designs/` is for technical decisions, tradeoffs, and implementation
  plans
- `docs/architecture.md` is the project-wide structural overview
- `docs/development.md` is the concrete project's build, test, run, and
  environment guide
- `docs/process/` is for workflow rules used while shaping and changing the
  project
- `docs/notes/` is for low-friction notes that are not yet curated docs
- `workflows/` is for concrete adapters such as version control and issue
  tracking
- `.github/workflows/` is for GitHub Actions automation that enforces required
  repository checks
- `devhub/` is for project status pages generated or maintained from repo data
- `docs/security.md` is for secrets, protected data, and access-boundary rules

## Canonical Lifecycle

Use this lifecycle everywhere in the repo:

```text
understand -> plan/design -> approve -> implement -> review -> commit
```

Definitions:

- `understand`: gather enough context to avoid blind work
- `plan/design`: decide whether the work needs a brief plan, a spec, a design
  doc, or both
- `approve`: get approval before formal tracking and implementation
- `implement`: execute one coherent bounded slice
- `review`: check correctness, risk, and drift from the approved artifacts
- `commit`: create a clean commit and update tracking

Do not skip straight to implementation when the problem or scope is still
unclear.

## Default Workflow Adapter

Project workflow defaults live in `workflows/jj-bd.md`:

- version control: `jj`
- work tracking: `bd` when tracking is needed

The first MVP does not require `bd` to build or run. Start using `bd` for
follow-up work once tracking is needed. If the project chooses different tools,
use the closest project-specific equivalent and update `workflows/`.

## Portability

This repo is intended to be agent-interface neutral: Codex, Claude, OpenCode,
or another agent should be able to follow the same documentation structure.

Keep `docs/process/` tool-neutral. Put concrete commands, local defaults, and
tool substitutions in `workflows/`.
