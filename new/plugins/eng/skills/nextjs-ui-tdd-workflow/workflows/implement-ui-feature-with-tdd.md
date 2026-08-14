# Implement UI Feature With TDD

Use this workflow for new or changed Next.js UI behavior.

1. Clarify the user behavior in plain language.
2. Write UI acceptance criteria in observable terms: what the user sees, enters, clicks, waits for,
   and receives.
3. Pick one acceptance criterion.
4. Choose the smallest useful test level:
   - Unit/hook for pure state or data shaping.
   - Testing Library component test for client interaction.
   - Integration test for composed UI behavior.
   - Playwright E2E only for routing, auth, async Server Components, browser/server integration, or
     critical journeys.
5. Choose the test location by following the existing project convention.
6. Write the failing test first when practical.
7. Run the narrowest existing test command and confirm the failure is for the expected reason.
8. Implement the smallest UI/code change.
9. Re-run the same test until green.
10. Refactor while green.
11. Add E2E only when lower-level coverage cannot prove the behavior.
12. Run related tests and summarize coverage, commands, and intentional gaps.

Do not scaffold unit, component, integration, and E2E tests for the same behavior upfront.
