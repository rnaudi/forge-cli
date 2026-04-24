# Code Review

This document describes how to review changes before committing and how review
should operate.

## When to Review

Review before every non-trivial commit.

## Review Ownership

- the author picks an explicit reviewer when the project uses human review
- the reviewer owns helping the change reach a mergeable state
- if feedback is given, the author responds with changes or a clear reply
- if another pass is needed, re-engage the reviewer explicitly

For a solo agent or single-developer workflow, use the same mindset as a
self-review checklist.

## What to Review

### 1. Correctness

- does the change do what it claims?
- are edge cases handled?
- are assumptions stated or enforced?

### 2. Scope and Design Fit

- does the implementation match the approved plan, spec, and design doc?
- did the change quietly expand scope?
- is there a simpler way to get the same result?

### 3. Tests and Verification

- are the right tests present?
- was the right validation run?
- are important failure modes covered?

### 4. Error Handling

- are failures surfaced clearly?
- are logs, messages, or exceptions actionable?
- are partial failure cases safe?

### 5. Maintainability

- naming and structure are clear
- comments explain intent, not trivia
- complexity is justified

### 6. Contract Fidelity

- if there is a spec, does the implementation actually satisfy it?
- did technical choices drift away from the design doc without being documented?
- should a new design doc be written instead of silently changing direction?

### 7. Performance and Operational Risk

- obvious regressions?
- surprising cost or latency?
- rollout or migration risk?

## Review Norms

- optimize for discovering what is true, not for winning arguments
- distinguish blocking issues from preferences
- use concrete references instead of broad objections
- AI-assisted changes do not lower the review bar
- authors remain accountable for correctness and provenance

## Review Output

Provide:

- blocking issues with file or code references
- non-blocking follow-ups
- residual risks or testing gaps

## Rule of Thumb

The review should answer:

- should this commit exist?
- is it safe to keep?
- what still worries us?
