# Planning Work

This document describes how to plan and design new work.

The key document split is:

- `docs/spec/` for stable behavior, scope, and contracts
- `docs/designs/` for technical decisions, tradeoffs, and phased implementation

## When to Plan

Plan before implementing when:

- adding new features
- making significant architectural or workflow changes
- requirements or scope are still fuzzy
- the work could span multiple sessions

Skip formal planning for:

- trivial fixes with obvious solutions
- typo-only documentation changes
- mechanical edits with no design choice

## Step 1: Understand the Work

Before planning, make sure you understand:

1. What problem is being solved?
2. What outcome is desired?
3. What already exists in the repo or docs?
4. Is there related ready work in the project's tracker?
5. Is this primarily a product/behavior question, a technical implementation
   question, or both?

Ask clarifying questions if the scope is ambiguous.

## Step 2: Assess Size

### Small Work

Characteristics:

- touches a small number of files
- one concept or bounded change
- can fit in one focused session
- no design doc required

Output:

- short implementation plan
- one tracking item after approval, when the project uses tracking

### Large Work

Characteristics:

- multiple files or systems
- multiple implementation phases
- meaningful design tradeoffs
- likely to span sessions

Output:

- spec in `docs/spec/` when product behavior, scope, or user
  impact must be made explicit
- design doc in `docs/designs/NNNN-<title>.md` when technical approach or
  implementation tradeoffs must be chosen
- tracking parent plus subtasks after approval, when the project uses tracking

## Step 3: Create the Plan

### For Small Work

Create a brief plan that states:

- what will change
- likely files or areas affected
- tests or validation needed
- risks or unknowns

After approval, create one tracking item if the project uses a tracker. Use the
selected adapter in `../../workflows/`.

### For Large Work

1. Decide which artifacts are required:

- **Spec only** when the core problem is product behavior, user-facing flow,
  workflow definition, or contract definition
- **Design doc only** when the problem is mainly technical and the product
  behavior is already clear
- **Spec + design doc** when product intent and technical approach both need
  explicit treatment

2. If a spec is needed, start from the templates in `docs/spec/`.

The spec should cover:

- problem and scope
- goals and non-goals
- user or operator impact
- constraints
- open questions
- acceptance criteria or stable behavioral expectations

3. If a design doc is needed, copy the design template:

```bash
cp docs/designs/0000-template.md docs/designs/NNNN-<title>.md
```

4. Fill in the key design sections:

- Summary
- Context
- Goals / Non-goals
- Decision
- Alternatives Considered
- Implementation Plan
- Consequences

5. Present the draft and wait for approval.

6. After approval, create tracking with one parent and one item per approved
   phase if the project uses a tracker.

7. Link the tracking items from the spec or design doc only when useful.

## Step 4: Get Approval

Do not create tracking until the user approves the plan, spec, or design doc.

That keeps exploration cheap and avoids cluttering the tracker with abandoned
ideas.

## Output Summary

| Work Size | Artifacts |
|-----------|-----------|
| Small | brief plan + optional tracking item |
| Large | spec and/or design doc + optional tracking parent and subtasks |

## Next Step

Once approved:

- small work: implement the single approved item
- large work: implement the first approved phase or subtask
