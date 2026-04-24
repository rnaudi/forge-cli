# Specs

This directory is for product-level behavior, user experience, scope, and
external contracts that deserve durable RFD-style discussion.

Use it when:

- user-visible behavior should be defined independently of the current
  implementation
- tests should trace back to documented expectations
- future refactors should preserve a stable contract
- a decision changes the product direction or user workflow

Do not create a spec for every feature. Many command-level changes only need a
Beads issue, tests, README/development docs, or an ADR when the technical choice
is significant.

For the fuller RFD-inspired writing guidance, use [guide.md](guide.md).

Current spec chapters:

- [01-introduction.md](01-introduction.md): product introduction and first
  milestone scope
- [02-forge-doctor.md](02-forge-doctor.md): implemented readiness-check behavior
- [03-product-direction.md](03-product-direction.md): product direction and
  candidate future features
