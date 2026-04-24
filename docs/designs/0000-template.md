---
id: 0000
title: ADR Title
status: proposal
tags: []
created: YYYY-MM-DD
accepted:
implemented:
spec-sections: []
superseded-by:
---

# ADR-0000: Title

## Status

Proposal

## Summary

One paragraph explaining the decision and why it matters.

## Context

Why is this needed? What problem does it solve? What constraints matter?

## Mental Model

Explain the theory of the design in plain language.

Include:

- the main metaphor or model, if useful
- key invariants
- why the major components exist
- what future maintainers should preserve

## Goals / Non-goals

### Goals

- ...

### Non-goals

- ...

## Decision

What we are going to do.

Include:

- user-visible behavior affected by the decision
- internal boundaries or data structures affected by the decision
- compatibility or migration rules
- security constraints

## Rationale

Why this decision is preferable to the alternatives.

Name the evaluation criteria explicitly:

- simplicity
- correctness
- compatibility
- security
- operational cost
- delivery risk

## Alternatives Considered

### Option 1

- benefits
- drawbacks
- why rejected or deferred

### Option 2

- benefits
- drawbacks
- why rejected or deferred

## Implementation Phases

- [ ] **Phase 1: Name** - Beads issue when available
- [ ] **Phase 2: Name** - Beads issue when available

## Operability and Observability

Describe how this design behaves in production or under operational stress.

Cover what applies:

- health checks or smoke checks
- deploy and rollback path
- overload, backpressure, retry, timeout, and idempotency behavior
- structured events, logs, metrics, traces, or dashboards
- cost, latency, throughput, storage, and scaling limits

## Stability and Compatibility

Describe the compatibility promise and migration path.

Include:

- external contracts affected
- observable behavior that users may depend on
- versioning or deprecation plan
- data migration or rollback concerns
- dependency adoption or upgrade risk

## Consequences

### Positive

- ...

### Negative

- ...

## Open Questions

- Question 1?

## Spec Impact

List user-visible behavior that should move into `docs/spec/` when this ADR is
implemented.

## Design Drift

How should a future maintainer recognize that this design no longer fits?

## Future Work

Things explicitly out of scope for this design.

## References

- Related docs, tracking items, or external references
