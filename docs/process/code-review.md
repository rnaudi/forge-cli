# Code and PR Review

This document describes how to review changes before committing and how review
should operate.

## When to Review

Review before every non-trivial commit.

Agents should also perform this review before handing work back as ready for
human approval.

## Review Ownership

- the author picks an explicit reviewer when the project uses human review
- the reviewer owns helping the change reach a mergeable state
- if feedback is given, the author responds with changes or a clear reply
- if another pass is needed, re-engage the reviewer explicitly

For a solo agent or single-developer workflow, use the same mindset as a
self-review checklist.

## What to Review

Review the diff first, then select the relevant lenses. Not every lens applies
to every change, but correctness, scope, and tests should always be considered.

### 1. Correctness and Bugs

- does the change do what it claims?
- are edge cases handled?
- are assumptions stated or enforced?
- could the change introduce data loss, security issues, race conditions,
  incorrect exits, or misleading output?

### 2. Scope and Design Fit

- does the implementation match the approved plan, spec, and ADR?
- did the change quietly expand scope?
- is there a simpler way to get the same result?

### 3. Tests and Verification

- are the right tests present?
- was the right validation run?
- are important failure modes covered?
- do tests follow `docs/testing.md`?
- do tests protect behavior rather than implementation chatter?
- what specific regression would a missing test catch?

### 4. Error Handling and Silent Failures

- are failures surfaced clearly?
- are logs, messages, or exceptions actionable?
- are partial failure cases safe?
- are errors propagated instead of swallowed?
- are fallback behaviors explicit and justified?
- can a user or agent tell what went wrong and what to do next?

For `forge-cli`, a required check must not silently degrade into OK or WARN.
Secret-related errors must remain safe to print.

### 5. Type and API Design

- do types express useful invariants?
- can invalid states be constructed too easily?
- are constructors and validation boundaries clear?
- are public APIs minimal and aligned with the domain?
- are trait boundaries located at real resources such as environment,
  filesystem, process, network, or time?

### 6. Maintainability

- naming and structure are clear
- comments explain intent, not trivia
- docs and comments follow `docs/documentation.md`
- complexity is justified
- Rust code follows `docs/rust.md`
- behavior is discoverable from local context where practical
- names are stable and searchable across boundaries
- new abstractions earn their complexity
- no private mini-framework is introduced without an explicit design reason

### 7. Comment and Documentation Accuracy

- do comments match the current code?
- do examples compile or at least clearly mark pseudo-code?
- do TODOs link to Beads issues?
- did user-visible behavior changes update specs or docs?
- did technical direction changes update or add an ADR?

### 8. Contract Fidelity

- if there is a spec, does the implementation actually satisfy it?
- did technical choices drift away from the ADR without being documented?
- should a new ADR be written instead of silently changing direction?

### 9. Performance and Operational Risk

- obvious regressions?
- surprising cost or latency?
- rollout or migration risk?
- health check, rollback, or recovery path affected?
- backpressure, retries, idempotency, or load shedding affected?
- structured telemetry still answers "what changed?" and "who is affected?"

### 10. Dependencies and Automation

- any new dependency justified by real value?
- transitive dependency, license, security, or maintenance risk considered?
- dependency upgrade verified against project-specific behavior?
- automation adds signal instead of alert or PR noise?

## Review Norms

- optimize for discovering what is true, not for winning arguments
- distinguish blocking issues from preferences
- use concrete references instead of broad objections
- AI-assisted changes do not lower the review bar
- authors remain accountable for correctness and provenance

## Review Output

Provide:

- scope reviewed
- guide gates applied
- blocking issues with file or code references
- important non-blocking improvements
- residual risks or testing gaps

Prioritize findings that can prevent real bugs. Avoid padding the review with
style preferences or generic observations.

## Rule of Thumb

The review should answer:

- should this commit exist?
- is it safe to keep?
- what still worries us?
