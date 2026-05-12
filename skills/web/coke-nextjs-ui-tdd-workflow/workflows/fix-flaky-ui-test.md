# Fix Flaky UI Test

Use this workflow when a UI test passes and fails intermittently.

1. Classify the flake:
   - Async timing or missing await.
   - Shared state or session leakage.
   - Test order dependency.
   - Brittle selector.
   - Network dependency.
   - Animation, clock, or browser timing.
   - Uncontrolled test data.
2. Re-run the narrowest failing test enough to observe the failure mode.
3. Inspect the failure output, screenshot, trace, DOM, or Testing Library debug output.
4. Fix with the smallest deterministic change:
   - Use `findBy*`, `waitFor`, or web-first Playwright assertions.
   - Replace brittle selectors with accessible queries or locators.
   - Isolate storage/session/data.
   - Mock network at the boundary.
   - Register network waits before the action that triggers them.
5. Re-run the flaky test repeatedly if practical.
6. Run related tests.
7. Do not hide flakes by increasing timeouts unless the timeout reflects a real user-visible SLA.
