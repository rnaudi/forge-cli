# Implementing Work

This document describes how to implement approved work.

## Prerequisites

Before implementing:

1. a plan exists
2. large work has the required spec and/or design doc artifacts
3. the work is small enough for one session, or has been split

## Step 1: Load Context

1. Read the tracking item if the project uses a tracker. Use the selected
   workflow adapter in `../../workflows/`.
2. For large work, read the relevant spec in `docs/spec/` and design doc in
   `docs/designs/` when present.
3. Mark the tracking item as in progress when the project uses a tracker.

## Step 2: Scope Check

Proceed when the work is:

- clear
- bounded
- one logical unit

Split it when:

- the change spans too many files
- unrelated concerns are mixed together
- too much discovery is still needed

If needed, create subtasks using the selected workflow adapter.

## Step 3: Recommended Order

1. Update specs if this changes stable behavior, contracts, or product-facing
   scope.
2. Update architecture or development docs if the project-wide shape or
   contributor workflow changed.
3. Add or update tests first when practical.
4. Make code changes.
5. Run the fastest relevant verification.
6. For large work, update tracking and any affected specs or design docs.

## Step 4: Verify

Run the project-appropriate checks. Prefer the smallest command that gives real
confidence first, then broader checks before commit.

Examples:

- unit tests
- integration tests
- lint
- typecheck
- build

## Step 5: Update Progress

Update the relevant tracking item when the project uses a tracker.

Do not edit a design doc just to mark task progress. Edit it only when the
design itself changed; for major direction changes, write a follow-up design
doc that references the original.

## Step 6: Review and Commit

Before committing:

1. follow `code-review.md`
2. fix blocking issues
3. follow `committing.md`

## Important

- each commit should leave the repo in a coherent state
- split oversized work instead of forcing it through one session
- stable behavior changes should usually have spec documentation
- major technical direction changes should usually have design documentation
