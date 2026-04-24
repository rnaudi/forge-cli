# References

This document captures a small set of readings that materially shaped `forge`.

The goal is not to preserve a reading log. The goal is to keep the strongest
inputs behind the repo's operating principles.

## Operating Doctrine

These references point in one direction:

- the repository should be the source of truth for product intent, design
  history, workflow, and contributor commands
- specs describe behavior and evolve with the product
- ADRs capture technical reasoning for a specific problem and preserve
  enough theory for future maintainers
- process should create execution, not replace it
- operational concerns are engineering concerns, not afterthoughts
- automation should reduce toil and noise, not create rituals or alert fatigue
- local-first, searchable, version-controlled context beats scattered
  management systems
- agents do not lower the bar for understanding, review, provenance, or
  maintainability

## Execution Over Theater

- [Ship Software That Does Nothing](https://kerrick.blog/articles/2025/ship-software-that-does-nothing/)
  Walking skeleton first. Deployment, health checks, and a path to production
  beat feature depth without an execution surface.
- [Things That Aren't Doing the Thing](https://strangestloop.io/essays/things-that-arent-doing-the-thing)
  Planning and tooling should support the work, not replace it.
- [The plan-execute pattern](https://mmapped.blog/posts/29-plan-execute)
  Separate planning from execution and keep the execution step explicit.

## Functional Specs and Technical Design

- [Oxide RFD 0001: Requests for Discussion](https://rfd.shared.oxide.computer/rfd/0001)
  Shaped the repository's RFD-inspired spec structure: discussion first,
  decision record after.
- [Rue ADRs](https://github.com/steveklabnik/rue/tree/trunk/docs/designs)
  Shaped the `docs/designs/` direction: compact decision records with context,
  decision, implementation phases, and consequences.
- [Effective Design Docs](https://mmapped.blog/posts/31-effective-design-docs)
  Drove the split between `docs/spec/` and `docs/designs/`.
- [A3 - Avoid Memos with Agenda](https://entropicthoughts.com/a3-avoid-memos-with-an-agenda)
  Problem-first writing is better than disguised solution advocacy.
- [Programming as Theory Building](https://pages.cs.wisc.edu/~remzi/Naur.pdf)
  Systems need preserved intent, not just source code.

## Operability and Observability

- [Bring Back Ops](https://charity.wtf/2026/01/19/bring-back-ops-pride-xpost/)
  Operations is a first-class engineering concern.
- [Queues Don't Fix Overload](https://ferd.ca/queues-don-t-fix-overload.html)
  Backpressure and overload are architecture concerns, not queue decorations.
- [A Practitioner’s Guide to Wide Events](https://jeremymorrell.dev/blog/a-practitioners-guide-to-wide-events/)
  Prefer rich structured observability over string-log archaeology.
- [Observability Engineering a book we write twice](https://charity.wtf/2026/02/18/observability-engineering-a-book-so-nice-we-wrote-it-twice-xpost/)
  Observability requires intentional design and governance.

## Stability, Compatibility, and Dependencies

- [Stability as a Deliverable](https://blog.rust-lang.org/2014/10/30/Stability/)
  Compatibility is a product promise.
- [One Version](https://abseil.io/resources/swe-book/html/ch16.html#one_version)
  Bias toward one-version simplicity.
- [Our Software Dependency Problem](https://research.swtch.com/deps)
  Dependency adoption has real maintenance and verification cost.
- [Hyrum’s Law](https://www.hyrumslaw.com/)
  Observable behavior becomes part of the contract whether documented or not.

## Simplicity, Locality, and Searchability

- [The Wrong Abstraction](https://sandimetz.com/blog/2016/1/20/the-wrong-abstraction)
  Delay abstractions until repetition is real.
- [Avoid Mini-frameworks](https://laike9m.com/blog/avoid-mini-frameworks,171/)
  Prefer directness over private local frameworks.
- [Locality of Behaviour](https://htmx.org/essays/locality-of-behaviour/)
  Behavior should be understandable from local context.
- [Greppability is an underrated code metric](https://morizbuesing.com/blog/greppability-code-metric/)
  Searchability is a design property, not an accident.
- [Index, Count, Offset, Size](https://tigerbeetle.com/blog/2026-02-16-index-count-offset-size/)
  Small naming rules can prevent bug classes cheaply.

## Testing and Review Culture

- [Arnau's Testing Links](https://arnau.bearblog.dev/links/)
  Seeded the local testing reading list and resource-boundary testing style.
- [Apollo GraphQL Rust Best Practices](https://github.com/apollographql/rust-best-practices)
  Informed the project Rust style guide.
- [Matklad: How to Test](https://matklad.github.io/2021/05/31/how-to-test.html)
  Reinforced feature-oriented and boundary-oriented tests.
- [Matklad: Unit and Integration Tests](https://matklad.github.io/2022/07/04/unit-and-integration-tests.html)
  Helped frame test boundaries around behavior and resources.
- [The argument against clearing the database between tests](https://calpaterson.com/against-database-teardown.html)
  Informed the dirty-resource testing guidance.
- [Why Good Developers Write Bad Unit Tests](https://mtlynch.io/good-developers-bad-tests/)
  Reinforced tests as maintainable change detectors.
- [SMURF: Beyond the Test Pyramid](https://testing.googleblog.com/2024/10/smurf-beyond-test-pyramid.html)
  Added speed, maintainability, utilization, reliability, and fidelity as test
  suite tradeoffs.
- [Risk-Driven Testing](https://testing.googleblog.com/2014/05/testing-on-toilet-risk-driven-testing.html)
  Informed the risk-driven coverage guidance.
- [Test Sizes](https://testing.googleblog.com/2010/12/test-sizes.html)
  Informed the resource-based test-size table.
- [Testing on error handling code](https://www.usenix.org/system/files/conference/osdi14/osdi14-paper-yuan.pdf)
  Reinforced explicit error-path testing.
- [TODOs aren't for doing](https://sophiebits.com/2025/07/21/todos-arent-for-doing)
  Informed the distinction between useful context comments and tracked work.
- [Go Commit Messages](https://go.dev/wiki/CommitMessage)
  Shaped the `area: lowercase summary` convention used for forge commits.
- [Write tests. Not too many. Mostly integration.](https://kentcdodds.com/blog/write-tests)
  Default testing philosophy should be explicit and pragmatic.
- [Code Comments](https://raw.githubusercontent.com/rnaudi/gh-log/main/docs/comments.md)
  Shaped the project documentation and comment taxonomy.
- [Look for What's True](https://dubroy.com/blog/look-for-whats-true/)
  Review should optimize for truth-seeking, not winning.
- [The cults of TDD and GenAI](https://drewdevault.com/2026/01/29/2026-01-29-Cult-of-TDD-and-LLMs.html)
  Avoid ritualized development theater.
- [What Should We Do With AI CL](https://groups.google.com/g/golang-dev/c/4Li4Ovd_ehE/m/8L9s_jq4BAAJ?pli=1)
  AI does not remove human accountability.

## Stewardship and Internal Surfaces

- [Beads](https://github.com/steveyegge/beads)
  Provides the repo-local issue tracker used for approved follow-up work.
- [How to be an Open source Gardener](https://steveklabnik.com/writing/how-to-be-an-open-source-gardener/)
  Triage, docs, and maintenance are real engineering work.
- [Record Every Meeting](https://rfd.shared.oxide.computer/rfd/0537)
  Durable searchable memory matters.
- [DevHub](https://matklad.github.io/2025/12/06/mechanical-habits.html#DevHub)
  Every serious project should have a contributor-facing internal surface.

## CI and Mechanical Discipline

- [Not Rocket Science Rule of Meetings](https://typesanitizer.com/blog/not-rocket-science.html)
  Coordination should leave decisions and actions behind, not just calendar time.
- [Turn Dependabot Off](https://words.filippo.io/dependabot/)
  Noisy automation degrades trust. Prefer targeted signal to automated churn.
