# Add Server Route E2E Test

Use this workflow for route/page behavior that depends on async Server Components, routing, server
rendering, auth redirects, or browser/server integration.

1. Define the route and user-visible outcome.
2. Confirm lower-level tests cannot provide enough confidence.
3. Use Playwright and the existing E2E test convention.
4. Isolate data and session for the test.
5. Navigate through user-visible routes and controls.
6. Use role/text/label locators.
7. Use web-first assertions such as `toBeVisible`, `toHaveURL`, and `toBeEnabled`.
8. Avoid CSS implementation assertions.
9. Avoid testing third-party services directly; mock or control external responses.
10. Clean up only if the project does not already isolate test data.

Keep the test focused on one route behavior or journey step.
