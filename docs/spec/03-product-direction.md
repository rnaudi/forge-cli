+++
title = "Product Direction"
weight = 3
status = "draft"
authors = ["arnau.diaz"]
labels = ["product", "cli", "bootstrap", "agents"]
discussion = ""
+++

# Spec: Product Direction

## Summary

`forge-cli` is a repository bootstrap assistant. It helps humans and coding
agents answer one question before they start changing code:

```text
Is this checkout ready for useful, safe work?
```

The product should turn hidden setup lore into a repository-owned bootstrap
contract, check that contract locally, and keep onboarding documentation honest
without becoming a secret manager, package manager, task runner, or hosted
workflow system.

## Status and Discussion

Draft. The first useful slice, `forge doctor`, is implemented and documented in
`docs/spec/02-forge-doctor.md`.

This document defines product direction and candidate features. Convert accepted
slices into Beads issues before implementation.

- Authors: arnau.diaz
- Discussion: none yet
- Labels: product, cli, bootstrap, agents

## Determination

`forge-cli` should become a small, local-first bootstrap readiness tool. Its
core job is to evaluate a repository-owned setup contract and explain whether a
checkout is ready for work.

The product should expand through narrow command families that preserve that
contract:

- check readiness
- validate and explain bootstrap config
- generate or verify safe onboarding documentation
- provide non-secret context for coding agents

It should not grow into a secret manager, package manager, general task runner,
or hosted onboarding platform.

## Context

Fresh checkouts fail for avoidable reasons:

- required tools are missing or at the wrong entry point
- secret names are known only by maintainers or private docs
- private registries require credentials that are not discoverable locally
- build and test commands are scattered across CI, chat, and memory
- coding agents start work before understanding local setup constraints

`forge-cli` should make those constraints explicit, safe to inspect, and easy to
validate.

## Standard and Current Condition

Standard condition:

- a fresh human or agent can run one local command and understand checkout
  readiness
- the repository names required setup inputs without storing secret values
- onboarding docs, bootstrap config, and CI expectations stay aligned
- follow-up setup work is tracked in Beads when it becomes implementation work

Current condition:

- `forge doctor`, `forge secrets check`, and `forge registries check` exist
- `forge.bootstrap.toml` supports tools, environment variables, and generic HTTP
  registry checks
- text output and binary success/failure exits are implemented
- broader product behavior, future commands, and feature sequencing are not yet
  specified

The gap matters because the first MVP proves the local readiness model, but the
larger product needs clear boundaries before features accumulate.

## Product Principles

- Repository-owned: setup requirements live in version-controlled files.
- Safe by default: never print, persist, log, or commit secret values.
- Local-first: checks run before a user or agent depends on hosted systems.
- Explicit over inferred: prefer reviewable config over hidden conventions.
- Bounded probes: network and tool checks should be predictable and timely.
- Human-readable first, machine-readable where useful.
- No surprise mutation: commands that change files or local state are explicit.

## Goals

- Make bootstrap requirements discoverable from the checkout.
- Validate the local environment before build, test, or agent work begins.
- Provide safe remediation hints for missing tools, secrets, and access.
- Give coding agents a compact, reliable readiness contract.
- Help maintainers keep onboarding docs and executable checks aligned.
- Support project-specific requirements without hardcoding one company or
  registry ecosystem.

## Non-goals

- Managing, fetching, generating, storing, or rotating secret values.
- Writing shell profiles or mutating the user's environment implicitly.
- Replacing Cargo, npm, Maven, Make, CI, or project-specific build tools.
- Becoming a general task runner.
- Uploading local environment state to a hosted service.
- Guessing private requirements from Slack, memory, or private documents.

## Users and Stakeholders

- Developers starting work in a new or stale checkout.
- Coding agents preparing to inspect, build, test, or modify a repository.
- Maintainers who own onboarding correctness.
- Platform engineers who own registries, credentials, and local tooling.
- CI maintainers who want the same readiness contract locally and in automation.

## Core Domain Model

- Bootstrap config: `forge.bootstrap.toml`, the repository-owned requirement
  manifest.
- Requirement: a named tool, secret, registry, command, document, or policy that
  affects readiness.
- Check: a bounded local probe that evaluates one requirement.
- Report: the safe user-facing result of one or more checks.
- Remediation hint: safe guidance that tells a user what to do next without
  exposing secret values.
- Profile: a named subset of requirements for a context such as local
  development, CI, release, or a specific service.

## Command Direction

### Implemented

- `forge doctor`: run all configured readiness checks.
- `forge secrets check`: check configured environment-variable requirements.
- `forge registries check`: check configured registry reachability and auth
  readiness.
- `forge config validate`: validate `forge.bootstrap.toml` without live probes.

### Near-Term Candidates

- `forge tools check`: run only configured local tool checks.
- `forge config explain`: print the safe meaning of configured requirements.
- `forge doctor --config <path>`: run against an explicit config file.
- `forge doctor --format json`: emit a stable machine-readable report for
  agents and CI.
- `forge init`: create a minimal bootstrap config without secret values.

### Later Candidates

- `forge docs check`: detect drift between onboarding docs and bootstrap config.
- `forge docs render`: generate a safe setup fragment from
  `forge.bootstrap.toml`.
- `forge env example`: print required environment-variable names and safe
  descriptions without values.
- `forge profiles list`: show configured requirement profiles.
- `forge doctor --profile <name>`: check a named profile such as `ci`, `dev`, or
  `release`.
- `forge registries explain`: show safe registry/auth configuration and likely
  remediation paths.
- `forge agent context`: emit compact non-secret context for coding agents.

## Options Considered

| Option | Benefits | Costs / Risks | Determination |
| --- | --- | --- | --- |
| Local readiness CLI | Keeps setup close to code, works for humans and agents, preserves secret boundaries | Requires each repo to maintain config | Chosen direction |
| Generated onboarding docs only | Easy for humans to read, low runtime complexity | Docs can drift and do not prove local readiness | Useful later, not enough by itself |
| General task runner | Could centralize setup, build, test, and docs commands | Competes with native project tools and increases scope quickly | Rejected |
| Secret manager integration first | Could give richer credential diagnostics | Pulls `forge` toward managing secret values and increases security risk | Rejected for product core |
| CI-only validation | Keeps local machine simpler | Fails too late for fresh checkouts and agents | Rejected as primary workflow |

The chosen path is deliberately conservative: start with local validation, add
documentation and machine-readable output where they strengthen that contract,
and avoid owning workflows that already belong to project-native tools.

## Candidate Feature Slices

These are feature ideas, not approved implementation work. Turn accepted slices
into Beads issues before coding.

### 1. Config Validation Command

Add `forge config validate` so maintainers and CI can verify config syntax,
unknown fields, invalid URLs, and secret-value mistakes without running network
checks.

Status: implemented.

Value:

- fast feedback for config edits
- safer CI integration
- clearer separation between static validation and live readiness

### 2. Tools-Only Check

Add `forge tools check` to mirror `secrets check` and `registries check`.

Value:

- completes the command family
- lets agents cheaply verify local executables before deeper work
- gives docs and tests a cleaner command contract

### 3. JSON Report Output

Add `--format text|json` for `doctor` and supporting check commands.

Value:

- stable input for agents
- easier CI annotations
- fewer brittle terminal-output tests for downstream users

### 4. Config Discovery

Support `--config <path>` and optional parent-directory discovery from nested
working directories.

Value:

- better monorepo ergonomics
- safer scripting
- easier tests and examples

### 5. Safe Remediation Hints

Extend requirements with safe hints such as install commands, doc links, and
credential-owner contact names. Hints must not include secret values.

Value:

- missing setup becomes actionable
- maintainers can encode known next steps once
- agents can report useful blockers instead of guessing

### 6. Documentation Drift Checks

Add `forge docs check` to verify that key configured requirements are mentioned
in curated onboarding docs.

Value:

- keeps executable config and human docs aligned
- catches forgotten secret names or tool additions
- reinforces the repository as source of truth

### 7. Generated Setup Fragment

Add `forge docs render` to output a safe markdown fragment describing tools,
secret names, registries, and commands.

Value:

- reduces duplicated onboarding prose
- lets maintainers review generated docs in commits
- keeps secret handling explicit

### 8. Requirement Profiles

Allow config to group checks by profile, such as `dev`, `ci`, `release`, or
`agent`.

Value:

- avoids one huge all-or-nothing readiness check
- supports repos with multiple services or roles
- keeps optional requirements understandable

### 9. Ecosystem Registry Adapters

Add focused adapters after the generic HTTP check is proven. Candidate adapters:

- Cargo registry
- npm registry
- Maven/Artifactory repository
- Docker registry

Value:

- more precise diagnostics
- ecosystem-specific auth readiness
- fewer false positives from generic HTTP endpoints

### 10. Agent Context Output

Add `forge agent context` or a JSON mode that summarizes safe bootstrap facts:
required commands, missing checks, doc links, and non-secret remediation hints.

Value:

- gives agents a constrained setup contract
- reduces accidental secret exposure
- improves handoff between human and agent sessions

## Behavior Invariants

- Secret values are never printed, persisted, logged, or committed.
- Config may name secret environment variables but must not contain secret
  values.
- Required check failures exit non-zero.
- Optional check failures are visible but do not fail by themselves.
- Commands that perform network probes use bounded timeouts.
- Commands that mutate files or local state are explicit and documented.
- Unknown config fields fail validation unless a later spec changes that rule.
- Human output remains readable without depending on colors.

## Consequences

Positive consequences:

- fresh checkouts get a single readiness entry point
- setup requirements become reviewable in normal code review
- agents have a safer contract before attempting build or test work
- teams can improve onboarding incrementally without adopting a hosted system

Negative consequences:

- each repository must keep `forge.bootstrap.toml` accurate
- generic checks can be less precise than ecosystem-specific diagnostics
- docs drift checks may become noisy if implemented too strictly
- adding profiles and generated docs will increase config complexity

Security consequences:

- the product remains viable only if secret values stay out of config, output,
  logs, tests, and commits
- future features must preserve the rule that `forge` names secrets but does not
  manage secret values

## Configuration Direction

The initial config model stays intentionally small:

- `[[tools]]`
- `[[secrets]]`
- `[[registries]]`

Likely future additions:

- `[[commands]]` for documenting build/test commands without replacing them
- `[[docs]]` for onboarding documents that should stay aligned
- `[profiles.<name>]` for named requirement subsets
- structured `hint` fields for safe remediation

Any future config addition should answer:

- Is this a bootstrap requirement?
- Can it be validated locally?
- Can it be described without secret values?
- Does it avoid replacing the project’s native build or package tools?

## Beads Planning

Use Beads for accepted follow-up implementation work.

Suggested conversion rule:

- one Beads epic for a product milestone
- one Beads task or feature per command or config contract
- link tasks to the relevant spec section in the issue description
- close the issue in the same commit as the implementation or spec update when
  practical

Do not create Beads items for speculative ideas until they are accepted as work.

Near-term Beads epics that would be reasonable after this spec is accepted:

- `forge config`: static config validation and explanation
- `forge reports`: JSON output and stable report schema
- `forge docs`: documentation drift checks and generated setup fragments
- `forge profiles`: named requirement subsets

## Acceptance Criteria

This product direction is useful when:

- maintainers can point to one spec for what `forge-cli` is trying to become
- feature proposals can be accepted, rejected, or deferred against clear
  product boundaries
- new commands can be mapped to a documented problem and user
- Beads issues can be created from accepted slices without re-litigating the
  product purpose

## Follow-up Measurement

- Future Beads issues should reference this spec when implementing new command
  families.
- Future specs or ADRs should explain deviations from the product principles.
- Revisit this document after the next two implemented feature slices to remove
  stale candidates and promote accepted behavior into stable command specs.

## Open Questions

- Should `forge init` write only config, or also create docs fragments?
- Should JSON output be part of `doctor` or a separate report command?
- Should profiles be a first-class config concept before docs drift checks?
- How strict should documentation drift checks be to avoid noisy failures?
- Should ecosystem registry adapters live behind feature flags or config-only
  selection?

## References

- `docs/spec/02-forge-doctor.md`
- `docs/designs/0001-config-format-and-secret-handling.md`
- `docs/architecture.md`
- `docs/security.md`
- `workflows/jj-bd.md`
- [Oxide RFD 0001: Requests for Discussion](https://rfd.shared.oxide.computer/rfd/0001)
