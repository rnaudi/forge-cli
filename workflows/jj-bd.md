# Jujutsu + Beads Workflow

This is the default workflow adapter for `forge-cli`.

## Tools

- version control: Jujutsu (`jj`)
- issue tracking: Beads (`bd`)

Beads is initialized in `.beads/` with issue prefix `forge`.

## Tool Setup

```bash
brew install beads
bd version
bd status --json
```

## Inspect Work

```bash
jj status
jj diff
bd ready --json
bd status --json
```

Use `bd` for approved follow-up work and handoff-worthy tasks. Do not create
tracking items before the work is understood and approved.

## Create Tracking

Small approved work:

```bash
bd create "<title>" -t feature -p 2 --json
```

Large approved work:

```bash
bd create "<title>" -t epic -p 2 --json
bd create "Phase 1: <desc>" -t task --parent <epic-id> --json
bd create "Phase 2: <desc>" -t task --parent <epic-id> --json
```

Use `task` for implementation chores, `feature` for user-visible product work,
`bug` for defects, `decision` for ADR-level choices, and `epic` for parent
items.

## Work an Item

```bash
bd show <bd-id> --json
bd update <bd-id> --claim --json
```

Create a subtask when the approved work needs splitting:

```bash
bd create "Subtask: <description>" --parent <bd-id> -t task --json
```

Avoid `bd edit`, `bd create-form`, or any command that opens `$EDITOR` in
normal agent workflow.

## Close and Commit

```bash
bd close <bd-id> --reason "Completed" --json
bd status --json
jj commit -m "<message>"
jj log -r @-
jj status
```

Commit `.beads/` metadata and exported issue state with related code or docs
changes when tracking changed.

Always pass `-m` to `jj commit`. Do not use editor-driven `jj commit` in the
normal workflow.

## Sync Notes

`bd init` configured the embedded Dolt backend and a Dolt remote for:

```text
git+ssh://git@github.com/rnaudi/forge-cli.git
```

Use explicit Beads/Dolt sync commands only when the team decides the remote is
ready for shared tracker state. Until then, commit the repo-visible `.beads/`
files with normal jj changes.
