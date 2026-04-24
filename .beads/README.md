# Beads

This repository uses Beads (`bd`) for local issue tracking.

The project issue prefix is `forge`; issue IDs look like `forge-<hash>`.

Use the project workflow in `../workflows/jj-bd.md` for normal work. Keep this
directory focused on Beads metadata and tracker-specific notes.

## Common Commands

```bash
bd status --json
bd ready --json
bd show <issue-id> --json
bd create "<title>" -t task -p 2 --json
bd update <issue-id> --claim --json
bd close <issue-id> --reason "Completed" --json
```

Avoid editor-opening commands in normal agent workflow. Prefer flags and JSON
output so sessions are reproducible.

## Tracked Files

- `metadata.json`: Beads backend metadata
- `config.yaml`: repository Beads configuration
- `issues.jsonl`: exported issue state when issues exist

Dolt runtime files, locks, credentials, and local sync state are ignored by
`.beads/.gitignore`.
