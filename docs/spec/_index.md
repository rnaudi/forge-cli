+++
title = "Project Specification"
sort_by = "weight"
+++

# Project Specification

This specification describes `forge-cli`, a Rust command-line tool for making
repository bootstrap requirements explicit and locally checkable.

The spec is written from the outside in: it defines user-visible behavior,
contracts, constraints, and acceptance criteria that implementations should
preserve.

Current command surface:

- `forge doctor`
- `forge secrets check`
- `forge registries check`
- `forge config validate`
- `forge.bootstrap.toml`
- environment-variable secret presence checks
- generic HTTP/private registry reachability checks

Out of scope for the first milestone:

- managing or fetching secret values
- writing shell profiles
- supporting every registry ecosystem
- replacing project-specific build or test commands

Technical implementation choices live in `docs/designs/`.

Product direction, user outcomes, and candidate future command families are
tracked in `03-product-direction.md`.
