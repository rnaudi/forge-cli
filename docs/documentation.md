# Documentation Style

This document defines writing style for `forge-cli` docs and code comments.

Use it with:

- `docs/spec/` for RFD-style product and behavior docs
- `docs/designs/` for ADR-style technical decisions
- `docs/rust.md` for Rust code style
- `docs/testing.md` for testing style

## Writing Rules

- Write for the next maintainer or agent who lacks your current context.
- Lead with the point, then give the necessary detail.
- Prefer concrete nouns and commands over abstract process language.
- Keep examples safe: no real secrets, private registry URLs, customer data, or
  production logs.
- Link to specs, ADRs, Beads issues, or source files when they carry the
  durable context.
- Remove stale text when behavior changes.

## Repo Docs

Use the existing taxonomy:

- `docs/spec/`: RFD-style product behavior and external contracts
- `docs/designs/`: ADR-style technical decisions
- `docs/process/`: workflow rules
- `workflows/`: concrete tool commands
- `docs/notes/`: rough notes that are not yet curated

Specs and ADRs should state their status clearly. Drafts should make unresolved
questions obvious. Implemented docs should preserve the reasoning, not just the
outcome.

## Reviewing Specs and ADRs

Review docs before they become the basis for implementation.

For RFD-style specs, focus on problem clarity, scope, user-visible behavior,
options, consequences, and open questions.

For ADRs, focus on context, decision quality, rationale, alternatives,
implementation phases, consequences, and spec impact.

Good review findings name what is missing or unclear and suggest the smallest
specific improvement. Avoid rewriting the document for the author.

## Rust Documentation Comments

Public Rust items should have doc comments when they are part of a meaningful
crate or module contract.

Use `//!` for module-level docs and `///` for item docs.

Module docs may use headings when they help navigation:

- `# Overview`: intent and scope
- `# Design`: tradeoffs, invariants, and resource boundaries
- `# Why`: non-obvious ordering or constraints
- `# Examples`: runnable snippets or short sketches
- `# Limitations`: known boundaries or follow-up work

Item docs should usually start with a concise summary and continue in normal
prose. Reserve `# Examples` for snippets that clarify real use.

## Comment Taxonomy

Use comments for information the code cannot make obvious.

- Function comments: explain what a function promises and when to call it.
- Design comments: record the big idea, invariants, and tradeoffs for a module
  or subsystem.
- Why comments: explain hidden reasons for ordering, thresholds, guards, or API
  quirks.
- Teacher comments: explain background math, protocols, or data structures.
- Checklist comments: remind maintainers about coordinated updates that tooling
  cannot enforce.
- Debt comments: mark shortcuts with exit criteria and a Beads issue.
- Guide comments: lightly section long routines when extraction would make the
  code harder to read.

Delete trivial comments that restate the code.

Do not commit old code commented out. Version control already preserves history.

## TODO and Debt

Every committed TODO must point to a Beads issue:

```rust
// TODO(forge-abc): replace fake registry fixture with localhost contract test.
```

Use `FIXME` only for defects that should block or strongly influence follow-up
planning. Prefer a Beads issue over a long explanatory comment.

## Examples

Good module comment:

```rust
//! Registry readiness checks.
//!
//! # Design
//! Registry checks use bounded HTTP probes and environment-backed auth. Reports
//! may name the registry and auth environment variables, but never include
//! secret values.
```

Good function comment:

```rust
/// Resolve registry authentication from environment variables without exposing
/// the resulting secret values in debug output.
```

Good why comment:

```rust
// HEAD keeps the probe cheap; some registries reject it, so callers fall back
// to GET when the server returns 405.
```

Bad comment:

```rust
// Increment index by one.
index += 1;
```

## Review Checklist

- Does the doc or comment explain intent, constraint, or contract?
- Is the status of a spec or ADR clear?
- Are examples safe to commit?
- Are TODOs linked to Beads issues?
- Were stale comments and old commented-out code removed?
- Would an agent have enough context to avoid unsafe assumptions?
