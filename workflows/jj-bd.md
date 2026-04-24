# Jujutsu + bd Workflow

This is the default workflow adapter for `forge-cli`. `jj` is active now; `bd`
is intended for follow-up tracking when the project needs it.

## Tools

- version control: Jujutsu (`jj`)
- issue tracking: `bd` / beads

## Inspect Work

```bash
jj status
jj diff
bd ready --json
```

If `bd` is not installed or tracking is not needed yet, use `jj status` and
`jj diff` only.

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

## Work an Item

```bash
bd show <bd-id> --json
bd update <bd-id> --status in_progress --json
```

Create a subtask when the approved work needs splitting:

```bash
bd create "Subtask: <description>" --parent <bd-id> -t task --json
```

## Close and Commit

```bash
bd close <bd-id> --reason "Completed" --json
jj commit -m "<message>"
jj log -r @-
jj status
```

Commit `.beads/issues.jsonl` with related code or docs changes when `bd`
tracking changed.

Always pass `-m` to `jj commit`. Do not use editor-driven `jj commit` in the
normal workflow.
