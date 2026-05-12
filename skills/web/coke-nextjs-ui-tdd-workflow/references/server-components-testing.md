# Server Components Testing

Sources:

- Official: https://nextjs.org/docs/app/guides/testing
- Official: https://nextjs.org/docs/app/guides/testing/jest

## Rule

Prefer E2E tests for async Server Component behavior. Common unit test tools do not fully support
async Server Components, and Next.js recommends E2E for async components.

## What to Test Lower

Extract and unit test pure logic used by Server Components:

- Data formatting.
- Permission predicates.
- Empty-state selection.
- Error mapping.
- URL/search param parsing.

## What to Test With E2E

Use Playwright when behavior depends on:

- Async Server Component rendering.
- Route params or search params.
- Redirects and not-found behavior.
- Auth cookies/session.
- Server data fetching.
- Browser-visible loading/error/success output.

## Avoid

- Rewriting Server Component architecture to make tests easier; use `coke-nextjs-app-architecture` for
  structural decisions.
- Snapshot-only coverage for async page behavior.
- Mocking so much server behavior that the E2E no longer proves the route behavior.
