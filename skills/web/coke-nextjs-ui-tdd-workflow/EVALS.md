# EVALS

## Purpose

These evals verify the skill triggers for Next.js UI TDD work, chooses the smallest useful UI test
level, and stays out of architecture, backend, mobile, CI, and deployment ownership.

## Positive Trigger Prompts

- "Implement this Next.js form with TDD."
- "Add a regression test for this UI bug."
- "Choose the smallest useful test for this page behavior."
- "Add a Playwright test for the login journey."
- "Test this client component with Testing Library."

## Negative Trigger Prompts

- "Create Next.js App Router folder structure."
- "Set up GitHub Actions CI."
- "Design backend API contract."
- "Create Expo mobile tests."
- "Optimize bundle size."

## Expected Behavior

- Trigger for Next.js UI behavior changes, UI regression tests, form behavior, client component
  tests, Server Action user-visible behavior, and critical Playwright journeys.
- Start from UI acceptance criteria.
- Choose the smallest useful test level.
- Use Testing Library for user-centric component behavior.
- Use Playwright only for critical journeys or behavior requiring browser/server integration.
- Follow existing test placement and package scripts.
- Summarize tests, commands, coverage, gaps, and risks.

## Must Not Do

- Must not own Next.js architecture structure.
- Must not own CI/CD setup.
- Must not own backend tests.
- Must not own mobile/Expo tests.
- Must not create a design system.
- Must not use snapshot tests as the main behavior test.
- Must not fabricate test results.
- Must not copy community skills verbatim.

## Pass Criteria

- [ ] Correctly triggers for positive prompts.
- [ ] Correctly avoids negative prompts.
- [ ] Uses official docs as source of truth.
- [ ] Community skill links are labeled inspiration only.
- [ ] Selects the smallest useful test level.
- [ ] Avoids creating all test levels upfront.
- [ ] Gives Testing Library and Playwright guidance consistent with user-visible behavior.
- [ ] Keeps companion skill boundaries clear.

## Example Evaluation

- **Input prompt**: "Add a Playwright test for the login journey."
- **Expected skill usage**: `coke-nextjs-ui-tdd-workflow` is selected.
- **Expected output qualities**: The agent defines the critical journey, checks existing E2E
  placement and scripts, uses role/label/text locators, isolates session/data, uses web-first
  assertions, and explains why E2E is appropriate.
