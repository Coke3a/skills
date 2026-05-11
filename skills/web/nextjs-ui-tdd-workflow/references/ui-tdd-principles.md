# UI TDD Principles

Sources:
- Official: https://nextjs.org/docs/app/guides/testing
- Official: https://testing-library.com/docs/guiding-principles/
- Official: https://playwright.dev/docs/best-practices
- Community inspiration only: https://agent-skills.md/skills/itechmeat/llm-code/react-testing-library
- Community inspiration only: https://agent-skills.md/skills/pluginagentmarketplace/custom-plugin-react/react-testing-library
- Community inspiration only: https://skillsauth.com/skills/absolutelyskilled/playwright-testing

## Principle

UI TDD starts from a user-visible behavior, not a component tree. The stronger the resemblance between the test and real use, the more confidence the test provides.

## Workflow

1. Convert the request into UI acceptance criteria.
2. Pick one criterion.
3. Choose the smallest useful test level.
4. Write the failing test.
5. Confirm the failure.
6. Implement the smallest change.
7. Refactor while green.
8. Run related tests.

## Good UI Acceptance Criteria

- Name what the user can see or do.
- Include relevant states: loading, empty, error, validation, pending, success.
- Avoid implementation details such as state variable names, component internals, CSS classes, or hook names.
- Keep one behavior per criterion when possible.

## Behavior Test Shape

Prefer:

- "A user can submit the login form and sees the dashboard link."
- "A user sees a validation message when email is missing."
- "A signed-out user is redirected to login."

Avoid:

- "The component sets `isSubmitting` to true."
- "The component calls `setState` with the expected array."
- "The element has class `text-red-500`."
