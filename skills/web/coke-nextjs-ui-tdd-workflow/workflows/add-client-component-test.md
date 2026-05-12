# Add Client Component Test

Use this workflow for interactive Client Components.

1. Identify the user interaction: click, type, select, focus, submit, dismiss, or navigate.
2. Render the component with realistic props and providers.
3. Query through `screen`, preferring role and accessible name.
4. Use `userEvent.setup()` and user-event interactions.
5. Assert user-visible output, enabled/disabled state, error messages, callbacks at public
   boundaries, or accessible state.
6. Use `findBy*` or `waitFor` only for async behavior.
7. Avoid component instances, private state, CSS class assertions, and implementation hook details.
8. Mock only external boundaries such as network, router, analytics, or browser APIs.

If the component is mostly composition around an async Server Component, prefer Playwright E2E for
the page behavior.
