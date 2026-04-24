# Issue Tracking

Use the tracker chosen by the project. Tracking can be local, hosted, or absent
for very small projects.

The default template adapter uses `bd` and is documented in
`../../workflows/jj-bd.md`.

## Why Track Work

- preserve intent across sessions
- split large work into reviewable parts
- make dependencies visible
- give agents a stable handoff surface

## Lifecycle

```
open -> in_progress -> closed
```

## Workflow Rules

1. Do not create tracking items before approval.
2. Small work gets one tracking item.
3. Large work gets one parent item plus subtasks.
4. Close tracking items before the commit that completes them when practical.

## Epics and Subtasks

For large work, create one parent item plus one item per approved phase.

## Best Practices

- keep titles short and clear
- put deep reasoning in specs or design docs, not tracker text
- prefer machine-readable output when the tool supports it
- commit local tracker files with related code or docs changes when the tracker
  is version-controlled
