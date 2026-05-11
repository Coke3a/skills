# Add UI Regression Test

Use this workflow when fixing a user-visible UI bug.

1. Reproduce the bug manually or with the existing failing test.
2. State the bug as expected user behavior and actual user behavior.
3. Choose the smallest test level that would have caught the bug.
4. Place the regression test near the behavior it protects.
5. Write the test so it fails before the fix.
6. Confirm the failure reason matches the bug.
7. Fix the behavior with the smallest code change.
8. Run the same test until green.
9. Check whether related tests should run.
10. Avoid broad duplication: do not add E2E coverage if a component/integration test proves the regression.

Prefer a regression test that protects the observed user outcome, not the implementation detail that caused the bug.
