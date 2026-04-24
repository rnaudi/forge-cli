# Project Specification

This directory is for the full project specification.

Over time, this should become the stable written description of what the system
is supposed to do, independently of the current implementation.

This area should be written from the outside in. Focus on what should exist,
who it is for, and what behavior must remain true.

Use this area for:

- product and feature behavior
- user-visible rules
- domain constraints
- interface contracts
- non-functional requirements that should remain stable

Do not put implementation-specific design rationale here unless it is required
to understand the contract. Use `docs/designs/` for design decisions and phased
implementation plans.

## What Belongs Here

- overview and scope
- domain model
- feature behavior
- external interfaces
- invariants and rules
- error cases
- acceptance criteria
- user, customer, or operator impact

## Suggested Structure

- `_index.md` for the main specification landing page
- numbered chapters for major areas
- appendices if needed
- `0000-template.md` for new spec/RFD-style chapters

## Relationship to Other Docs

- `docs/architecture.md` explains how the system is shaped
- `docs/designs/` contains design docs for major changes
- `docs/development.md` explains how to build, test, and operate the project

## Functional Spec vs Design Doc

Use `docs/spec/` when the key questions are:

- what should we build?
- who is it for?
- what behavior is expected?
- what constraints or acceptance criteria matter?

Use `docs/designs/` when the key questions are:

- how should we implement it?
- which technical approach did we choose?
- what tradeoffs did we accept?
