# Testing Style

This document defines how `forge-cli` tests should be written.

The short version:

```text
tests are change detectors
```

A good test fails when behavior that matters changes. Formatting, linting, and
CI checks are part of the same change-detection system.

The testing bias is feature-oriented and boundary-oriented: test behavior users
or callers depend on, then keep IO behind small boundaries so the fast suite can
exercise most logic without real external resources.

## Test Entry Point

Use one normal test command locally and in CI:

```bash
cargo test
```

Do not create environment-specific test suites for normal development. Tests
should not require real private registries, real credentials, or machine-local
state beyond temporary files and localhost fixtures.

## Test Shape

Each test should:

1. arrange visible input
2. call the system under test through a public or stable module boundary
3. compare the result to an expected value or observable output

Prefer descriptive names:

```rust
missing_required_secret_should_fail_without_leaking_value
registry_rejected_auth_should_be_error_without_secret_value
```

Use one logical assertion per test when practical. Multiple concrete assertions
are fine when they verify one behavior, such as exit code plus redaction.

## Black-Box Bias

Tests should behave like callers.

- Prefer testing crate/module behavior over private helper implementation.
- Prefer feature behavior over implementation steps.
- Test private helpers only when they are pure, risky, and awkward to observe
  through a better boundary.
- Avoid tests that lock in incidental implementation details.
- If a private helper needs extensive direct testing, consider extracting a
  clearer module boundary.

## Whole-Value Assertions

When a function returns a value type, compare the whole expected value:

```rust
assert_eq!(actual, expected);
```

Prefer deriving `PartialEq` and `Eq` for testable value types when that matches
the domain. Field-by-field assertions can miss changes when new fields are
added.

Rendered terminal output is an exception: assert on stable required fragments,
not every byte of layout, unless a snapshot-style contract is explicitly added.

## Visible Test Data

Keep important input values in the test body.

- Inline small TOML configs.
- Inline fake environment-variable names.
- Use local helper functions for repeated setup only when they make the test
  easier to read.
- Avoid shared fixtures that force readers to jump across files to understand
  behavior.

## Resource Boundaries

Use resource boundaries to decide test shape:

| Resource | Small test | Larger test |
| --- | --- | --- |
| CPU and memory only | normal unit test | property/table test if needed |
| Filesystem | temp directory | integration-style test |
| Network | fake client or localhost server | no external network |
| Environment | fake environment trait | controlled process env only when needed |
| Time | injected clock or fixed timeout | no sleeps |

For `forge-cli`, pure config parsing, report aggregation, and status mapping
should stay fast unit tests. Filesystem config loading can use temporary
directories. Registry checks should use fake clients or localhost HTTP
fixtures, never private endpoints.

Treat this table as a design signal, not a bureaucracy. If a test needs
external network, real credentials, wall-clock sleeps, or global machine state,
the product boundary probably needs another seam.

## Fakes and Contracts

Prefer fakes over mocks.

A fake implements the same boundary as the real resource with a small in-memory
or deterministic implementation. A mock that asserts call order usually tests
wiring rather than behavior.

Current useful boundaries:

- `Environment` for environment-variable and PATH control
- `ToolResolver` for executable discovery
- `RegistryClient` for HTTP readiness outcomes

When a fake represents a real boundary with subtle behavior, write contract
tests that exercise both the fake and the real implementation where practical.

## Secret-Safe Tests

Tests must never depend on real secret values.

- Use fake values such as `secret-value` only inside test-controlled
  environments.
- Always assert fake secret values are absent from rendered reports when the
  test exercises secret handling.
- Do not commit private registry URLs, customer data, production logs, or real
  credentials as fixtures.
- Redaction behavior is part of the security contract and should have focused
  tests.

## Failure Modes

For every command behavior change, consider tests for:

- success
- required failure
- optional warning
- invalid config
- missing config
- secret redaction
- bounded network failure when relevant

Do not over-test every combination if a smaller table proves the rule.

Prefer risk-driven coverage over coverage theater. A small number of tests that
exercise the public behavior, error boundary, and secret-safety invariant is
better than many brittle tests that mirror internal branches.

## Anti-Patterns

- Hidden setup that changes what the test means.
- Tests that assume global environment state.
- Tests that reach external networks.
- `sleep`-based timing tests.
- Field-by-field assertions for returned value objects.
- Mocks that verify implementation chatter instead of observable behavior.
- TODOs without Beads issue IDs.

## Review Checklist

- Would this test fail for the behavior change it claims to protect?
- Can a reader understand the input without chasing fixtures?
- Does the test avoid real secrets and private infrastructure?
- Does it use a fake at the right resource boundary?
- Is the test stable under parallel runs?
- Is the failure message useful enough to debug quickly?

## References

- [Arnau's Testing Links](https://arnau.bearblog.dev/links/)
- [How to Test](https://matklad.github.io/2021/05/31/how-to-test.html)
- [Unit and Integration Tests](https://matklad.github.io/2022/07/04/unit-and-integration-tests.html)
- [The argument against clearing the database between tests](https://calpaterson.com/against-database-teardown.html)
- [Why Good Developers Write Bad Unit Tests](https://mtlynch.io/good-developers-bad-tests/)
- [SMURF: Beyond the Test Pyramid](https://testing.googleblog.com/2024/10/smurf-beyond-test-pyramid.html)
- [Risk-Driven Testing](https://testing.googleblog.com/2014/05/testing-on-toilet-risk-driven-testing.html)
- [Test Sizes](https://testing.googleblog.com/2010/12/test-sizes.html)
- [Testing on error handling code](https://www.usenix.org/system/files/conference/osdi14/osdi14-paper-yuan.pdf)
- [TODOs aren't for doing](https://sophiebits.com/2025/07/21/todos-arent-for-doing)
