# CI Workflow

This workflow records the mechanical checks for `forge-cli`.

Keep CI thin, build-system-first, and low-noise.

## Green Main

The preferred invariant is:

```text
main only advances to commits whose required checks are known to pass
```

Use a merge queue, protected branch, or project-specific equivalent when the
team needs one. Do not make humans repeatedly reconcile stale CI by hand.

## GitHub Actions

CI is implemented in:

```text
.github/workflows/ci.yml
```

The workflow runs on pull requests and pushes to `main`.

## Required Checks

The current local quality gate is:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

Run `cargo fmt` before committing when formatting changed.

## Tidy Checks

Good future tidy checks for this project:

- no generated drift
- no forbidden large files
- no stale placeholders in curated docs
- no broken links in curated docs
- no unsupported dependency versions

## Dependency Automation

Avoid alert and PR noise.

Prefer targeted checks that identify real risk:

- vulnerability checks filtered to reachable or relevant dependencies when the
  ecosystem supports it
- scheduled latest-dependency test runs
- explicit review before dependency upgrades are committed
- project-specific validation before release
