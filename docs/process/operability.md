# Operability

Use this document when a change affects how the project is built, deployed,
observed, operated, scaled, recovered, or paid for.

## Rule of Thumb

If the project can fail in production, the repo should say how humans and agents
notice, reason about, and recover from that failure.

## What to Define

For operationally meaningful work, capture:

- owner or responsible team
- build and deploy path
- health check or smoke check
- rollback or recovery path
- relevant logs, events, metrics, traces, or dashboards
- latency, throughput, storage, and cost expectations
- data safety and migration concerns
- on-call or incident handoff path when applicable

## Overload and Backpressure

For queues, background jobs, APIs, or batch systems, define:

- bottleneck or constrained resource
- maximum backlog or concurrency
- retry and idempotency rules
- timeout and cancellation behavior
- backpressure or load-shedding behavior
- telemetry needed to see saturation before users report it

Queues are not capacity plans. If work can arrive faster than it can be
processed, decide what gives.

## Observability

Prefer structured, queryable events over string-log archaeology.

For each unit of work, decide which context should be visible:

- service, environment, version, and build identifier
- route, command, job type, or operation name
- user, tenant, account, or domain identifiers when safe
- feature flag, experiment, or configuration state
- queue, retry, rate-limit, or saturation state
- error class and recovery outcome

## Review Checklist

- Can a fresh operator tell whether the system is healthy?
- Can a fresh agent tell which command verifies the operational path?
- Can failures be correlated to a deploy or config change?
- Is the rollback path documented and realistic?
- Are costs, limits, and overload behavior explicit?
