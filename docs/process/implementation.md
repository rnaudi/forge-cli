# Implementing Work

This document describes how to implement approved work.

## Prerequisites

Before implementing:

1. a plan exists
2. large work has the required spec and/or ADR artifacts
3. the work is small enough for one session, or has been split

## Step 1: Load Context

1. Read the tracking item if the project uses a tracker. Use the selected
   workflow adapter in `../../workflows/`.
2. For large work, read the relevant spec in `docs/spec/` and ADR in
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
3. Update operability docs if the change affects deploy, rollback,
   observability, cost, capacity, or incident response.
4. Add or update tests first when practical.
5. Make code changes.
6. Run the fastest relevant verification.
7. For large work, update tracking and any affected specs or ADRs.

## Step 4: Verify Mechanically

Run the project-appropriate checks. Prefer the smallest command that gives real
confidence first, then broader checks before commit.

Examples:

- unit tests
- integration tests
- lint
- typecheck
- build

For `forge-cli`, the full mechanical gate is documented in
`docs/development.md` and `../../workflows/ci.md`.

## Step 5: Self-Review Against Guides

Before returning to a human for approval, review, or commit, apply the relevant
repo guides:

- `docs/rust.md` for Rust changes
- `docs/testing.md` for tests and test coverage
- `docs/documentation.md` for prose and code comments
- `docs/process/code-review.md` for implementation changes
- `docs/process/planning.md#reviewing-specs-and-adrs` for specs and ADRs
- `docs/security.md` for secrets, credentials, registries, or protected data

This guide review is part of the quality gate. Treat findings the same way as
test failures: fix blocking issues before handoff.

## Step 6: Update Progress

Update the relevant tracking item when the project uses a tracker.

Do not edit an ADR just to mark task progress. Edit it only when the decision
itself changed; for major direction changes, write a follow-up ADR that
references the original.

## Step 7: Review and Commit

Before committing:

1. complete the guide self-review above
2. fix blocking issues
3. follow `committing.md`

## Important

- each commit should leave the repo in a coherent state
- split oversized work instead of forcing it through one session
- stable behavior changes should usually have spec documentation
- major technical direction changes should usually have an ADR
