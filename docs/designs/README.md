# Technical Design Docs

This directory contains numbered design docs for large technical changes.

Design docs capture the technical approach after the problem and product intent
are clear enough to choose an implementation direction.

## What a Design Doc Captures

- context
- the decision
- goals and non-goals
- alternatives considered
- implementation plan or phases
- consequences and tradeoffs

## When to Write One

Write a design doc for work that:

- spans multiple sessions
- has meaningful design choices
- needs phased delivery
- benefits from a durable historical record

Do not write a design doc for small, obvious changes.

Design docs may describe intended phases, but they are not task trackers. Keep
live status, checkboxes, ownership, and issue IDs in the project tracker.

## Lifecycle

```
proposal -> accepted -> implemented -> superseded
```

## Creating a Design Doc

1. Copy the template:

```bash
cp docs/designs/0000-template.md docs/designs/NNNN-<title>.md
```

2. Use the next available 4-digit number.

3. Fill in:

- Summary
- Context
- Goals / Non-goals
- Decision
- Alternatives Considered
- Implementation Plan
- Consequences

4. After approval, create tracking with the selected workflow adapter when the
   project uses tracking.

## Relationship to Specs

Use design docs for technical decisions. Use `docs/spec/` for functional-spec /
RFD-style behavior and scope documents that should evolve with the product.
