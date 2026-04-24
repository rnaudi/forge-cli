# Committing Changes

This document describes how to create commits in this workflow.

## Prerequisites

Before committing:

1. relevant verification passed
2. review is complete
3. no blocking issues remain

## Version Control

Use the version-control tool chosen by the project.

The default project adapter is `../../workflows/jj-bd.md`, but the
commit process should remain the same with Git, Jujutsu, or another VCS:
inspect the diff, verify it is coherent, write a clear message, and leave the
main branch releasable.

## Commit Message Guidelines

The commit style rule for this project is Google's Go commit message style,
adapted to `forge-cli` areas.

Primary reference:

- [Go Commit Messages](https://go.dev/wiki/CommitMessage)

Format:

```text
<area>: <summary>

<optional body>
```

Treat the Go rule as normative unless this document says otherwise. The only
intentional adaptation is that `forge-cli` uses repo areas instead of Go package
names when no Rust module or CLI command is the best prefix.

Summary line:

- prefix the primary affected area before the colon, matching Go's package
  prefix convention
- write the text after the colon as a lowercase phrase
- make the phrase complete: "this change modifies forge-cli to ..."
- keep it short, preferably under 72 characters
- use no trailing period

Body:

- explain what changed and why
- wrap prose around 72 columns
- avoid file lists
- avoid restating the diff

Footer:

- reference tracking items when useful
- use `Fixes <id>` for completed tracked work
- use `Updates <id>` or `For <id>` for partial progress
- do not use alternate GitHub verbs such as `Closes` or `Resolves`

Examples:

```text
docs: add design template for large changes
```

```text
process: clarify implementation workflow

Clarify how approved plans become tracked work and how large changes
stay aligned with design docs.

Related to <tracking-id>
```

```text
cli: report missing required tools

Teach doctor to fail when a configured required tool is absent so a fresh
checkout can explain the next setup action before deeper build steps run.
```

Useful areas include `cli`, `config`, `doctor`, `docs`, `process`, `ci`, and
`deps`. Prefer the smallest accurate area over broad labels.

## Workflow

### 1. Close Related Tracking

If the commit completes tracked work, close or update the tracking item using
the selected workflow adapter.

### 2. Create the Commit

Use the selected VCS workflow to create a focused commit. Commands should be
noninteractive so they work reliably in agent sessions and scripts.

With Jujutsu, describe the current in-progress change when useful:

```bash
jj describe -m "docs: document commit message style"
```

Create the commit with `jj commit -m`, not editor-driven `jj commit`:

```bash
jj commit -m "docs: document commit message style"
```

For commits that need a body, pass additional `-m` arguments:

```bash
jj commit -m "cli: report missing required tools" \
  -m "Teach doctor to fail when a configured required tool is absent."
```

### 3. Verify

Inspect the resulting commit and working-copy status with the selected VCS.

## Commit Quality

Each commit should be:

- complete
- focused
- reviewable

Do not commit:

- debug leftovers
- partial work without an intentional reason
- unrelated edits bundled together

Do not use a commit command that opens an editor in normal agent workflow.
