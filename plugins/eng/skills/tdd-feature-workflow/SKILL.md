---
name: tdd-feature-workflow
description: Implement or change observable software behavior through a language-neutral red-green-refactor loop. Use when acceptance criteria can drive a feature, bug fix, or behavior-preserving refactor. Adapt test levels, placement, doubles, and commands to the repository; do not use merely to add tests after implementation or for CI configuration.
---

# TDD Feature Workflow

Use tests to discover the smallest useful interface and implementation one behavior at a time. The repository's architecture and conventions own file placement and tooling; this skill owns the feedback loop.

## Entry condition

Start when the intended observable behavior is clear enough to state one acceptance criterion and distinguish success from failure. Derive criteria from the user's request, an approved spec, a regression, or characterized existing behavior.

Do not require HTTP endpoints, database schemas, or a particular layer unless the feature actually uses them. If a materially different product decision is missing, stop and ask for that decision. Otherwise choose the smallest reasonable behavior and proceed.

If the behavior is already implemented, first characterize it with a focused test; do not pretend the implementation was test-driven.

## Loop

For one acceptance criterion:

1. Select the lowest-cost test level that observes the required behavior with useful confidence.
2. Write one focused test against the intended public or stable interface.
3. Run the narrowest relevant command and confirm the test fails for the expected reason.
4. Write the smallest implementation that makes it pass.
5. Run the focused test until green, then run the relevant broader checks.
6. Refactor production and test code while green.
7. Repeat for the next criterion.

Never write the entire imagined test suite before receiving feedback from the first implementation slice. A compile error may be a valid red when it demonstrates the missing interface, but unrelated environment or dependency failures are not.

## Test design

Prefer observable outcomes over implementation details. Fake only boundaries the test must control, such as time, randomness, storage, networks, or external services. Follow existing project conventions for doubles and assertions; do not introduce a mocking framework solely for one test.

Use unit/component tests for isolated rules and orchestration, integration tests for real boundary contracts, and end-to-end tests for a small number of critical journeys. Do not repeat the same rule at every level unless each test catches a distinct failure mode.

Read [references/test-scope.md](references/test-scope.md) when the correct test level or use of a test double is unclear.

## Repository adaptation

Before adding files or commands, inspect the repository's manifests, test configuration, nearby tests, task runner, and CI. Use its canonical narrow and full-suite commands. Preserve its public API, architectural boundaries, naming, file placement, fixtures, and database strategy unless changing them is part of the request.

## Definition of done

Each acceptance criterion has the smallest useful test, every new test was observed failing for the intended reason, the relevant suite passes, and refactoring happened only while green. Summarize behavior covered, commands run, boundaries faked or exercised for real, and intentionally deferred cases.
