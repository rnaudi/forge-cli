# DevHub

DevHub is the lightweight project status surface for `forge-cli`.

Use it when the project needs a repo-native place to expose status that
contributors and agents should see quickly, such as:

- release readiness
- known broken checks
- triage queues
- benchmark or quality signals
- links to active specs, design docs, and work items

## Current Status

- MVP command surface is implemented: `forge doctor`, `forge secrets check`,
  and `forge registries check`.
- Required local quality gate lives in `workflows/ci.md` and is enforced by
  `.github/workflows/ci.yml`.
- Bootstrap path lives in `workflows/bootstrap.md`.
- `bd` tracking is planned for follow-up work, but not required for the MVP.

Avoid turning DevHub into a second documentation hierarchy. Link to the source
docs instead.
