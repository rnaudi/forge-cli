# Architecture Decision Records

This directory contains Architecture Decision Records (ADRs) for `forge-cli`.

ADRs capture significant engineering decisions after the problem and product
intent are clear enough to choose an implementation direction.

Use `docs/spec/` for RFD-style product and behavior discussion. Use
`docs/designs/` for the historical record of why a technical choice was made.

## What an ADR Captures

- context: why a decision was needed
- decision: what we chose
- rationale: why this option won
- alternatives: what else was considered
- consequences: expected tradeoffs and implications
- implementation phases: how the decision becomes code or docs
- spec impact: behavior that should move into `docs/spec/`

## When to Write One

Write an ADR for work that:

- spans multiple sessions
- has meaningful design choices
- needs phased delivery
- benefits from a durable historical record
- changes core config, command behavior, security posture, or compatibility
- introduces a dependency or integration with lasting maintenance cost

Do not write an ADR for small, obvious changes. Those can be tracked directly
with Beads or handled in the commit.

ADRs may describe intended phases, but they are not task trackers. Keep live
status, ownership, and issue state in Beads.

## Lifecycle

```
proposal -> accepted -> implemented
```

| Status | Meaning |
| --- | --- |
| `proposal` | Under discussion, not approved |
| `accepted` | Decision approved, implementation in progress or ready |
| `implemented` | Decision has landed; durable behavior belongs in `docs/spec/` when user-visible |
| `superseded` | Replaced by a newer ADR |

## Creating an ADR

1. Copy the template:

```bash
cp docs/designs/0000-template.md docs/designs/NNNN-<title>.md
```

2. Use the next available 4-digit number.

3. Fill in:

- Summary
- Context
- Decision
- Rationale
- Alternatives Considered
- Implementation Phases
- Consequences
- Spec Impact

4. After approval, create Beads tracking for the implementation phases when the
   work is large enough to need handoff or sequencing.

## Relationship to Specs

Use ADRs for technical decisions. Use `docs/spec/` for functional-spec /
RFD-style behavior and scope documents that should evolve with the product.

When an ADR is implemented and changes user-visible behavior, promote the stable
contract into `docs/spec/` and leave the ADR as historical context.

## References

- [Rue ADRs](https://github.com/steveklabnik/rue/tree/trunk/docs/designs)
