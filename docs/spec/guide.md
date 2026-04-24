# Project Specification

This directory is for the full project specification.

Over time, this should become the stable written description of what the system
is supposed to do, independently of the current implementation.

This area should be written from the outside in. Focus on what should exist,
who it is for, and what behavior must remain true.

## RFD-Inspired Shape

Specs in this repository are inspired by Oxide's RFD model: write a concrete
request for discussion, circulate it while the decision is still shapeable, and
preserve the final reasoning for future readers.

Use RFD-style content when a spec is more than a small behavior note:

- metadata near the top: title, status, authors, labels, and discussion link
  when one exists
- summary: one short synthesis of the problem and proposed direction
- context: what is happening now and why it matters
- determination: the current proposed or accepted decision
- goals and non-goals: boundaries that reviewers can challenge
- options considered: real alternatives and what is lost by rejecting them
- proposal or behavior: the contract being discussed
- consequences: expected benefits, costs, and risks
- open questions: unresolved points for review
- follow-up: what work should become Beads issues after approval

Do not copy external RFD mechanics blindly. Keep specs local, concise, and
useful for `forge-cli`.

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
- decisions that affect product direction
- alternatives and tradeoffs when the path is not obvious

## Suggested Structure

- `_index.md` for the main specification landing page
- numbered chapters for major areas
- appendices if needed
- `0000-template.md` for new spec/RFD-style chapters

## Relationship to Other Docs

- `docs/architecture.md` explains how the system is shaped
- `docs/designs/` contains ADRs for major technical decisions
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

## References

- [Oxide RFD 0001: Requests for Discussion](https://rfd.shared.oxide.computer/rfd/0001)
- [Oxide public RFD index](https://rfd.shared.oxide.computer/)
