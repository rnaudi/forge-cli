# Jujutsu + bd Workflow

This is the default workflow adapter for the template. It is optional: replace
it when a project chooses different tools.

## Tools

- version control: Jujutsu (`jj`)
- issue tracking: `bd` / beads

## Inspect Work

```bash
jj status
jj diff
bd ready --json
```

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
