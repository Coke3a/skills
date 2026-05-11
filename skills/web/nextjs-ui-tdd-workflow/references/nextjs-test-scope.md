# Next.js Test Scope

Sources:
- Official: https://nextjs.org/docs/app/guides/testing
- Official: https://nextjs.org/docs/app/guides/testing/jest

## Scope Map

| Need | Use | Avoid |
| --- | --- | --- |
| Pure function result | Unit test | Rendering a page for a pure calculation |
| Hook state transition | Hook/unit test | E2E for pure state |
| Client interaction | Testing Library component test | Testing private component state |
| Multiple UI units together | Integration test | Mocking the component under test |
| Async Server Component page | Playwright E2E | Jest unit test for unsupported async component behavior |
| Routing/auth/browser behavior | Playwright E2E | CSS selector assertions |
| Snapshot drift check | Snapshot as secondary signal | Snapshot-only behavior coverage |

## Next.js Guidance

Next.js recognizes unit, component, integration, E2E, and snapshot tests. Use each for its purpose:

- Unit tests isolate a function, hook, or simple component.
- Component tests focus on rendering, props, and user events.
- Integration tests prove multiple units working together.
- E2E tests prove user flows in a browser-like production scenario.
- Snapshot tests can catch unexpected output changes but should not replace behavior tests.

## Async Server Components

Async Server Components have limited unit test support in common React tooling. Prefer Playwright E2E for user-visible behavior that depends on async Server Component rendering, route behavior, server data, redirects, or browser integration.

## Server Actions

Use unit/integration tests for isolated result mapping when the action can be imported and called safely in the project setup. Use Playwright when the important confidence comes from form submission, routing, cookies, auth, revalidation, or the full browser/server flow.
