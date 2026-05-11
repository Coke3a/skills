# Add Playwright Critical Journey

Use this workflow for a small number of high-value journeys.

1. Name the critical journey and why E2E is needed.
2. Keep the path short: start from the first meaningful route and finish at the user-visible success state.
3. Isolate state with unique test data, controlled fixtures, or per-test setup.
4. Set up auth/session using the project's existing strategy.
5. Use locators based on role, label, text, or explicit test ids when user-facing locators are not enough.
6. Use web-first assertions and avoid manual polling.
7. Avoid test order dependencies.
8. Avoid testing third-party dependencies directly.
9. Clean up according to project convention.
10. Run the narrowest E2E command for the spec, then related E2E tests if needed.

Keep E2E tests few and meaningful; do not mirror every component test in Playwright.
