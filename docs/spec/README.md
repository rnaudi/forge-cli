# Specs

This directory is for stable behavior, scope, and external contracts that
deserve durable documentation.

Use it when:

- behavior should be defined independently of the current implementation
- tests should trace back to documented expectations
- future refactors should preserve a stable contract

Do not force every change into a formal spec. Many changes only need a plan or
design doc.

For the fuller RFD-inspired writing guidance, use [guide.md](guide.md).

Current spec chapters:

- [01-introduction.md](01-introduction.md): product introduction and first
  milestone scope
- [02-forge-doctor.md](02-forge-doctor.md): implemented readiness-check behavior
- [03-product-direction.md](03-product-direction.md): product direction and
  candidate future features
