---
name: nextjs-ui-tdd-workflow
description:
  Use when adding or changing Next.js UI behavior with TDD. Guides UI acceptance criteria, smallest
  useful test level, Testing Library component tests, hook/unit tests, form tests, Server Action
  tests, Playwright E2E critical journeys, regression tests, test placement, and behavior-focused
  test summaries. Pair with nextjs-app-architecture for structure.
---

# Next.js UI TDD Workflow

## Purpose

Guide UI feature implementation through user behavior tests, one behavior at a time. Use this skill
to turn UI acceptance criteria into the smallest useful test, run a red/green/refactor loop, and
summarize what user-visible behavior is protected.

## When to Use

- Adding a UI feature
- Changing page/component behavior
- Fixing a UI bug with a regression test
- Adding form validation behavior
- Adding loading/error/empty/success states
- Changing navigation behavior
- Changing Server Action user-visible behavior
- Adding E2E coverage for a critical journey
- Testing async Server Component behavior through E2E
- Reviewing where a UI test should live

## When Not to Use

- Choosing Next.js architecture structure
- Adding CI/CD
- Backend testing
- Mobile testing
- Pure visual design review
- Snapshot-only testing
- Performance profiling

## Core Rules

- Test user behavior, not implementation details.
- Pick one acceptance criterion.
- Choose the smallest useful test level.
- Do not create every test level upfront.
- Write the failing test first when practical.
- Make it pass with the smallest UI/code change.
- Refactor only when green.
- Use accessible queries.
- Prefer user-event over fireEvent when available.
- Use Playwright only for critical journeys or behavior requiring browser/server integration.
- Keep E2E tests isolated.
- Do not assert CSS classes unless the visual class is the actual contract.
- Snapshot tests are not a substitute for behavior tests.
- Avoid testing third-party libraries directly.

## TDD Loop

1. Pick one UI acceptance criterion.
2. Choose the smallest useful test level.
3. Choose the test file location.
4. Write the failing test.
5. Run the narrowest test command.
6. Confirm it fails for the expected reason.
7. Implement the smallest UI/code change.
8. Run the same test until green.
9. Refactor component/test code while green.
10. Run related tests.
11. Repeat.

## Do Not Create All Test Levels Upfront

- Do not create unit, component, integration, and E2E tests all at once.
- Start with the smallest test that proves the next user behavior.
- Add Playwright E2E only when lower-level tests cannot provide enough confidence.
- Add component tests for interactive client components.
- Add hook/unit tests for pure state logic.
- Add E2E for async Server Components, routing, auth, browser integration, or critical journeys.
- Do not duplicate the same behavior at every level unless each level gives unique confidence.

## Test Scope Selection

| Behavior type                          | Preferred test level                                                  | Preferred location                                      |
| -------------------------------------- | --------------------------------------------------------------------- | ------------------------------------------------------- |
| Pure function                          | Unit test                                                             | `src/**/__tests__/*.test.ts` or colocated `*.test.ts`   |
| Custom hook                            | Hook/unit test                                                        | `src/**/__tests__/*.test.ts` or colocated `*.test.ts`   |
| Client component interaction           | Testing Library component test                                        | `src/**/__tests__/*.test.tsx` or colocated `*.test.tsx` |
| Form validation                        | Component/integration test                                            | feature/component test                                  |
| Server Action result mapping           | Unit/integration test if isolated, E2E if full server behavior needed | depends on setup                                        |
| Route/page with async Server Component | Playwright E2E                                                        | `tests/e2e/*.spec.ts`                                   |
| Navigation flow                        | Playwright E2E or integration test                                    | `tests/e2e/*.spec.ts`                                   |
| Auth flow                              | Playwright E2E critical journey                                       | `tests/e2e/*.spec.ts`                                   |
| Loading/error/empty UI state           | Component or integration test                                         | feature/component test                                  |
| Bug fix                                | Smallest regression test that would have caught the bug               | depends on behavior                                     |
| Visual appearance                      | Visual test only if project has tooling                               | project-specific                                        |

See `references/nextjs-test-scope.md` for more test-level detail.

## Test File Placement

Support both common structures:

```text
features/
  auth/
    components/
      LoginForm.tsx
      LoginForm.test.tsx
```

```text
features/
  auth/
    components/
      LoginForm.tsx
    __tests__/
      LoginForm.test.tsx
```

E2E tests:

```text
tests/
  e2e/
    login.spec.ts
```

Rules:

- Follow the existing project convention if present.
- Do not introduce a new convention if the project already has one.
- Component/hook tests stay near feature code.
- E2E tests live under `tests/e2e` or `e2e` depending on existing convention.
- Shared test utilities live in `test-utils` or `tests/utils`.
- API mocks can live in `tests/mocks` or `src/mocks` depending on existing convention.

## Testing Library Rules

- Use `screen` queries.
- Prefer `getByRole` with accessible name.
- Use `getByLabelText` for form fields.
- Use `findBy*` for async appearance.
- Use `queryBy*` for absence.
- Use `userEvent` for realistic interactions.
- Avoid `container.querySelector` unless no better query exists.
- Avoid testing component state directly.
- Avoid testing implementation hooks directly when behavior can be tested through UI.
- Avoid relying on CSS classes.

See `references/testing-library-patterns.md` for examples.

## Playwright Rules

- Test user-visible behavior.
- Use locators.
- Prefer role/text/label locators.
- Use web-first assertions.
- Isolate tests.
- Avoid shared state between tests.
- Avoid depending on test order.
- Use storage state carefully.
- Avoid testing third-party dependencies.
- Keep E2E tests few and meaningful.
- Use E2E for critical journeys, routing, auth, async Server Components, and browser integration.

See `references/playwright-patterns.md` for examples.

## Mocking API and Server Actions

- Mock network at the boundary for component tests.
- Use MSW or the project-approved mock strategy when available.
- Do not mock React internals.
- Do not mock the component under test.
- Mock API responses to cover success/error/loading states.
- For Server Actions, test result mapping when isolated.
- Use E2E when server behavior and browser flow must be validated together.
- Keep mocks realistic and contract-shaped.
- If API contract changes, use `app-api-contract-workflow`.

## UI State Coverage

For data-driven UI, consider:

- Initial state
- Loading
- Success
- Empty
- Validation error
- Server error
- Permission/auth state
- Pending/disabled state
- Retry behavior when relevant

Do not test every state at every level. Choose the smallest level that gives confidence.

## Final Verification

Inspect `package.json` first. Use existing project scripts.

Possible commands:

- `npm run test`
- `npm run test:watch`
- `npm run test:e2e`
- `npm run lint`
- `npm run typecheck`
- `npm run build`

Do not invent commands. Do not fabricate test results.

## Final Response Format

Summarize:

- Acceptance criteria covered
- First failing test
- Test levels used
- Test file locations
- Tests added/changed
- Implementation changed
- Commands run
- Behavior intentionally not covered
- Risks/follow-up

## Bundled Resources

- Use `workflows/implement-ui-feature-with-tdd.md` for new UI behavior.
- Use `workflows/add-ui-regression-test.md` for UI bug fixes.
- Use `workflows/add-client-component-test.md` for interactive client components.
- Use `workflows/add-server-route-e2e-test.md` for async Server Component/page behavior.
- Use `workflows/add-form-test.md` for form validation and submission behavior.
- Use `workflows/add-playwright-critical-journey.md` for critical user journeys.
- Use `workflows/fix-flaky-ui-test.md` for flaky UI tests.
- Use `references/` only as needed for detailed patterns.
- Use `templates/` as starting points, then adapt to the project convention.

## Companion Skills

- `nextjs-app-architecture` owns Next.js structure, App Router, Server/Client boundaries, data
  fetching, Server Actions, and component architecture.
- `app-api-contract-workflow` owns API contracts shared with backend/mobile.
- `app-design-system` owns design tokens and design language.
- `app-code-review` owns final UI code review.
- Backend TDD skills own backend test workflow.
