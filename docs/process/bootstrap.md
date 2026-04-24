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
- smallest executable slice: build, test, run, deploy, or smoke-check path

## Golden Path

1. Define the first executable slice: the smallest command or path that proves
   the project can build, run, test, deploy, or smoke-check something real.
2. Update `docs/development.md` with real build, test, run, bootstrap, and
   environment commands.
3. Update `docs/architecture.md` with the actual system shape and boundaries.
4. If the project has stable external behavior or scope worth preserving, start
   the first spec in `docs/spec/`.
5. If the project needs a meaningful technical decision record, create the
   first design doc in `docs/designs/`.
6. Choose or update the concrete workflow adapter in `workflows/`.
7. Fill in `docs/security.md` if the project needs secrets, protected data, or
   internal access.
8. Start `devhub/` only when there is useful project status to expose.
9. Keep process docs generic unless the project has clearly changed the process
   itself.
10. Delete or rewrite template placeholders once real project content exists.

## First Executable Slice

Documentation should create execution, not replace it.

Before writing deep specs or design docs, identify the smallest useful
validation path. Examples:

- a build command
- a unit or smoke test
- a local run command
- a health check
- a deploy-to-nowhere or blank-page deployment
- a script that verifies required tools and configuration

Record the real command in `docs/development.md` or the relevant workflow
adapter.

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
- write more process than the project can execute

## Expected Outcome

After specialization, a fresh contributor or agent should be able to answer:

- what this project is
- how to build, test, and run it
- how to bootstrap a fresh checkout
- where behavior is documented
- where major technical decisions are recorded
- what workflow to follow for the next change
