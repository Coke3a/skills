# Mocking API and Server Actions

Sources:
- Official: https://nextjs.org/docs/app/guides/testing
- Official: https://nextjs.org/docs/app/guides/testing/jest
- Community inspiration only: https://agent-skills.md/skills/pluginagentmarketplace/custom-plugin-react/react-testing-library

## Component Tests

Mock at external boundaries:

- HTTP/network calls.
- Server Action modules when imported into Client Components.
- Router/navigation helpers when needed.
- Browser APIs not provided by the test environment.

Do not mock:

- React internals.
- The component under test.
- A child component solely to make assertions easier, unless the child is an expensive unrelated boundary and the behavior under test is still user-visible.

## API Mocks

Prefer MSW or the project-approved network mock strategy when available. Keep responses realistic and shaped like the API contract.

Cover only states relevant to the behavior under test:

- Loading.
- Success.
- Empty.
- Validation error.
- Server error.

If the API contract changes, use `app-api-contract-workflow`.

## Server Actions

Test isolated Server Action result mapping when practical:

- Valid input maps to success result.
- Validation failure maps to field/global error.
- Domain/API failure maps to user-visible error.

Use Playwright when the important behavior depends on browser submission, cookies, redirects, cache revalidation, auth state, or the full server flow.
