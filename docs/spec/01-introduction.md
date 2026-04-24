+++
title = "Introduction"
weight = 1
+++

# Introduction

`forge-cli` helps humans and coding agents prepare a repository for safe local
development. It makes hidden setup requirements explicit, checks them locally,
and reports readiness without exposing secret values.

## Purpose

Starting work in a repository often depends on requirements that are not obvious
from the checkout itself: runtime versions, local tools, private registry
access, secret environment-variable names, and build/test commands. `forge-cli`
turns the most important bootstrap requirements into a repository-owned config
and a small set of local readiness checks.

## Scope

This specification covers the first milestone command behavior:

- reading `forge.bootstrap.toml`
- checking local tool availability
- checking environment-variable presence without printing values
- checking generic HTTP/private registry reachability
- reporting OK/WARN/ERR statuses
- returning non-zero when required checks fail

## Out of Scope

- managing, fetching, generating, or storing secret values
- writing shell profiles or mutating the user's environment
- supporting every package registry ecosystem
- replacing project-specific build or test tools

## Definitions

- Bootstrap config: the `forge.bootstrap.toml` file checked into a repository.
- Required check: a check whose failure makes `forge` exit non-zero.
- Optional check: a check whose failure is reported as WARN but does not fail
  the command by itself.
- Secret: sensitive local value supplied through an environment variable.
- Registry: a private or authenticated HTTP endpoint that must be reachable for
  development work.

## Conformance

A conforming first-milestone implementation provides the documented command
surface, preserves the secret-handling constraints, and returns the documented
exit status for required check failures.

## Document Status

Early draft. The initial command behavior is defined in
`docs/spec/02-forge-doctor.md`.

## Relationship to Technical Design Docs

This specification describes expected behavior and product intent.

Technical implementation decisions should be captured separately in
`docs/designs/`.
