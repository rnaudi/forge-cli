# Bootstrap a Real Project

Use this document when `forge` is still a generic template and the user has
started providing concrete project constraints.

## Goal

Convert the template into a project-specific operating surface without filling
the repo with speculative detail.

## Inputs to Collect

Before specializing, confirm the smallest useful set of constraints:

- language and framework
- build and test entrypoints
- deployment environment
- main product or feature scope
- architectural constraints that are already known
- chosen version-control and work-tracking tools, or whether the default adapter
  in `workflows/jj-bd.md` should remain

## Golden Path

1. Update `docs/development.md` with real build, test, run, and environment commands.
2. Update `docs/architecture.md` with the actual system shape and boundaries.
3. If the project has stable external behavior or scope worth preserving, start
   the first spec in `docs/spec/`.
4. If the project needs a meaningful technical decision record, create the
   first design doc in `docs/designs/`.
5. Choose or update the concrete workflow adapter in `workflows/`.
6. Keep process docs generic unless the project has clearly changed the process
   itself.
7. Delete or rewrite template placeholders once real project content exists.

## Decision Rules

- If the main uncertainty is user-visible behavior, write a spec first.
- If the main uncertainty is technical approach, write a design doc first.
- If the change is small and obvious, a brief plan may be enough.
- If a template section is not true for the real project, replace it rather
  than preserving it as doctrine.

## Anti-Goals

Do not:

- fill every placeholder immediately
- create fake commands to make the repo look complete
- write design docs for obvious or single-session changes
- keep claiming portability where the project has chosen concrete tools
- put tool-specific commands in `docs/process/`

## Expected Outcome

After specialization, a fresh contributor or agent should be able to answer:

- what this project is
- how to build, test, and run it
- where behavior is documented
- where major technical decisions are recorded
- what workflow to follow for the next change
