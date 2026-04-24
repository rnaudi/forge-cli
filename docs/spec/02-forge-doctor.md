+++
title = "Forge Doctor"
weight = 2
status = "implemented"
+++

# Spec: Forge Doctor

## Summary

`forge doctor` reads `forge.bootstrap.toml`, checks whether the local checkout is
ready for development, and prints a readable terminal report with OK/WARN/ERR
statuses. Required check failures make the command exit non-zero.

## Status

Implemented on 2026-04-24. This spec defines the first milestone behavior.

## Context

Repository setup requirements are often scattered across memory, chat, CI, or
private docs. That makes a fresh checkout risky for both humans and coding
agents: missing tools, private registry access, or secret environment variables
may only surface after a build or test fails.

`forge doctor` makes those requirements explicit and locally checkable without
managing secrets or exposing sensitive values.

## Standard and Current Condition

Standard condition:

- a fresh human or agent can discover required local tools, secret names, and
  private registry readiness checks from the repository
- the readiness command explains missing setup without exposing secret values
- required failures produce a non-zero exit status

Current condition:

- this repository has an MVP `forge.bootstrap.toml` and `forge doctor` command
- the MVP checks local tools, environment-variable presence, and generic HTTP
  registry readiness
- follow-up work still needs broader fixtures, config discovery options, and
  machine-readable output

The gap matters because `forge-cli` is intended to remove bootstrap ambiguity
without becoming a secret manager or a private setup script.

## Goals

- Read bootstrap requirements from `forge.bootstrap.toml`.
- Validate required local tools are available.
- Validate required environment variables are present without printing values.
- Check configured private registry URLs for reachability/auth readiness.
- Print a clear readiness report using OK/WARN/ERR statuses.
- Exit non-zero when required checks fail.

## Non-goals

- Manage, fetch, generate, or store secret values.
- Write shell profiles or mutate the user's environment.
- Support every package registry ecosystem in the first milestone.
- Replace project-specific build or test commands.
- Infer hidden requirements from CI or private documentation.

## Users and Stakeholders

- Developers bootstrapping a new checkout.
- Coding agents preparing to work safely in a repository.
- Maintainers documenting required local setup.

## Cause Analysis

Bootstrap requirements drift because they are often maintained in places that do
not execute locally: chat, CI configuration, private docs, and individual
memory. `forge doctor` addresses the executable part of that gap by making
checks local, reviewable, and safe to run.

## Proposed Behavior

### Config Discovery

By default, `forge doctor` reads `forge.bootstrap.toml` from the current working
directory. If the file is missing or invalid, `forge doctor` reports an ERR and
exits non-zero.

### Tool Checks

For each configured tool requirement, `forge doctor` checks whether the command
is available locally. A missing required tool is ERR. A missing optional tool is
WARN.

The report may print the configured tool name and safe install guidance when
present. It must not execute arbitrary project commands as part of a tool
presence check.

### Secret Checks

For each configured secret requirement, `forge doctor` checks whether the named
environment variable exists and is non-empty.

The report may print the environment-variable name. It must never print the
environment-variable value.

Missing required secrets are ERR. Missing optional secrets are WARN.

### Registry Checks

For each configured registry requirement, `forge doctor` checks whether the URL
is reachable with a generic HTTP request suitable for private registries such as
Artifactory.

If the registry requires authentication, config names the environment variables
that provide auth material. `forge doctor` may use those values for the request
but must never print or persist them.

Expected status handling:

- HTTP success responses are OK.
- HTTP unauthorized or forbidden responses are ERR when auth is required because
  credentials are absent or not accepted.
- Connection failures, DNS failures, and timeouts are ERR for required
  registries and WARN for optional registries.

### Supporting Commands

`forge secrets check` runs only configured secret checks.

`forge registries check` runs only configured registry checks.

Both commands use the same config file, reporting rules, secret-handling rules,
and exit-code rules as `forge doctor`.

## Output Contract

Output is optimized for human-readable terminals. The exact layout may evolve,
but it must preserve:

- one visible status per check: OK, WARN, or ERR
- the safe requirement identity, such as a tool name, environment-variable name,
  or registry URL
- enough safe detail to guide remediation
- a final readiness summary

## Exit Codes

The first milestone only requires two exit classes:

- `0`: all required checks pass
- non-zero: one or more required checks fail, config is missing, or config is
  invalid

Warnings alone do not require a non-zero exit.

## Constraints

- Secret values must never be printed, persisted, logged, or committed.
- Config files may name secrets but must not contain secret values.
- Checks should be deterministic and avoid surprising local mutation.
- Network checks should use bounded timeouts.

## Acceptance Criteria

- Given a valid config with available tools, present secrets, and reachable
  registries, `forge doctor` prints OK statuses and exits `0`.
- Given a missing required tool, `forge doctor` prints an ERR for that tool and
  exits non-zero.
- Given a missing required environment variable, `forge doctor` prints the
  variable name, does not print any value, and exits non-zero.
- Given an unreachable required registry, `forge doctor` prints an ERR for that
  registry and exits non-zero.
- Given only optional missing requirements, `forge doctor` prints WARN statuses
  and exits `0`.
- `forge secrets check` and `forge registries check` run only their respective
  check categories.

## Follow-up Measurement

- `cargo run -- doctor` should pass in this repository without private secrets.
- Future repositories using `forge` should be able to add bootstrap
  requirements without leaking secret values in reports, tests, or docs.
- Follow-up issues should track recurring false positives, unclear remediation
  messages, and registry-check gaps.

## Open Questions

- Should a later milestone add an explicit `--config` path?
- Should registry checks support custom headers beyond environment-backed basic
  or bearer auth?
- Should the report support machine-readable output such as JSON?

## References

- `docs/architecture.md`
- `docs/development.md`
- `docs/designs/0001-config-format-and-secret-handling.md`
