---
name: coke-nextjs-ui-tdd-workflow
description: Drives test-driven development for Next.js UI — turns UI acceptance criteria into the smallest useful Testing Library, hook, form, Server Action, or Playwright E2E test, then runs red/green/refactor. Use when adding or changing UI behaviour, fixing a UI bug with a regression test, adding form/state coverage, or adding an E2E for a critical journey. Do not use for app architecture, design tokens, backend tests, CI/CD, or pure visual review.
---

# Next.js UI TDD Workflow

## Use this when

- Adding a UI feature or changing page/component behaviour.
- Fixing a UI bug with a regression test.
- Adding form validation, loading, error, empty, or success state coverage.
- Changing navigation or Server Action user-visible behaviour.
- Adding Playwright coverage for a critical journey or async Server Component flow.
- Deciding where a UI test should live.

## Do not use this when

- Designing Next.js architecture or boundaries → use `coke-nextjs-app-architecture`.
- Backend or mobile testing.
- CI/CD or deployment.
- Pure visual design or snapshot-only review.
- Performance profiling.

## Core rules

- Test user behaviour, not implementation details.
- Pick one acceptance criterion per loop.
- Choose the smallest useful test level; do not create unit + component + integration + E2E upfront.
- Write the failing test first when practical, then make it pass with the smallest change.
- Refactor only when green.
- Use accessible queries (`getByRole`, `getByLabelText`); prefer `userEvent` over `fireEvent`.
- Reserve Playwright for critical journeys, async Server Components, routing, auth, and browser integration.
- Keep E2E tests isolated; do not depend on test order or shared mutable data.
- Do not assert CSS classes unless that class is the actual contract.
- Snapshots are not a substitute for behaviour tests.
- Mock at boundaries (HTTP, Server Action modules in client tests). Never mock the component under test.

## TDD loop

1. Pick one UI acceptance criterion.
2. Choose the smallest useful test level (see test scope reference).
3. Choose the test file location matching the project convention.
4. Write the failing test.
5. Run the narrowest test command.
6. Confirm it fails for the expected reason.
7. Implement the smallest UI/code change.
8. Run the same test until green.
9. Refactor component and test code while green.
10. Run related tests.
11. Repeat for the next criterion.

## Load more detail

| Decision                                       | Reference                                            |
| ---------------------------------------------- | ---------------------------------------------------- |
| Picking the right test level                   | `references/nextjs-test-scope.md`                    |
| Where the test file lives                      | `references/test-file-placement.md`                  |
| Testing Library queries and patterns           | `references/testing-library-patterns.md`             |
| Playwright locators, assertions, isolation     | `references/playwright-patterns.md`                  |
| Mocking HTTP, Server Actions, router           | `references/mocking-api-and-server-actions.md`       |
| Async Server Component testing                 | `references/server-components-testing.md`            |
| Form validation + submission tests             | `references/form-testing.md`                         |
| Accessibility-focused testing                  | `references/accessibility-testing.md`                |
| Core UI TDD principles                         | `references/ui-tdd-principles.md`                    |
| Common UI test smells                          | `references/ui-test-smells.md`                       |

## Workflows

| Workflow                                            | Use for                                              |
| --------------------------------------------------- | ---------------------------------------------------- |
| `workflows/implement-ui-feature-with-tdd.md`        | New UI behaviour from acceptance criteria            |
| `workflows/add-ui-regression-test.md`               | UI bug fix with a regression test                    |
| `workflows/add-client-component-test.md`            | Interactive client component                         |
| `workflows/add-form-test.md`                        | Form validation and submission                       |
| `workflows/add-server-route-e2e-test.md`            | Async Server Component / page behaviour              |
| `workflows/add-playwright-critical-journey.md`      | Critical user journey                                |
| `workflows/fix-flaky-ui-test.md`                    | Diagnosing flaky UI tests                            |

## Templates

`templates/` contains starting points for `client-component-test`, `form-component-test`, `hook-test`, `server-action-test`, `playwright-critical-journey`, and the `msw-handler-template`. Use `templates/ui-acceptance-criteria.md` for planning and `templates/ui-test-summary.md` for the final report. Adapt to project convention before committing.

## Related skills

- `coke-nextjs-app-architecture` — structure, App Router, Server/Client boundaries that this workflow tests.
- Backend TDD skills — backend test workflows.
- Design-system / API-contract / code-review skills — when present, own design tokens, contracts, and final review.

## Definition of done

- Each acceptance criterion is covered by the smallest useful test.
- Tests assert user-visible behaviour through accessible queries.
- The narrowest test command runs green.
- Project test scripts pass (e.g. `npm run test`, `npm run test:e2e` — only those defined in `package.json`).
- Risks or behaviour intentionally not covered are listed in the final summary.
