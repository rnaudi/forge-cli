# Committing Changes

This document describes how to create commits in this workflow.

## Prerequisites

Before committing:

1. relevant verification passed
2. review is complete
3. no blocking issues remain

## Version Control

Use the version-control tool chosen by the project.

The default adapter for this template is `../../workflows/jj-bd.md`, but the
commit process should remain the same with Git, Jujutsu, or another VCS:
inspect the diff, verify it is coherent, write a clear message, and leave the
main branch releasable.

## Commit Message Guidelines

Format:

```text
<summary line>

<optional body>
```

Summary line:

- imperative mood
- concise
- no period at the end

Body:

- explain what and why
- avoid file lists
- avoid restating the diff

Footer:

- reference tracking items when useful

Examples:

```text
Add design template for large changes
```

```text
Refine implementation workflow

Clarify how approved plans become tracked work and how large changes
stay aligned with design docs.

Related to <tracking-id>
```

## Workflow

### 1. Close Related Tracking

If the commit completes tracked work, close or update the tracking item using
the selected workflow adapter.

### 2. Create the Commit

Use the selected VCS workflow to create a focused commit.

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
