# EVALS

## Purpose

Verify that `nextjs-app-architecture` triggers for Next.js App Router architecture work and defers out-of-scope testing, backend, mobile, CI/CD, and design-system concerns to companion skills.

## Positive Trigger Prompts

- "Create a Next.js feature page with App Router."
- "Decide which components should be server or client."
- "Refactor this component API to avoid too many boolean props."
- "Add a Server Action for this form."
- "Review this Next.js feature structure."

## Negative Trigger Prompts

- "Write UI tests for this page."
- "Set up CI/CD."
- "Create Expo mobile navigation."
- "Implement backend repository trait."
- "Run Playwright E2E tests."

## Expected Behavior

- Trigger for App Router structure, Server/Client boundaries, data fetching placement, Server Actions, component composition, feature folders, API client boundaries, UI states, accessibility-aware structure, and performance-aware architecture.
- Inspect the project shape before choosing standalone or monorepo paths.
- Apply Cache Components guidance only when `cacheComponents: true` is detected in `next.config.*`.
- Coordinate with companion skills when the request crosses API contract, design system, code review, TDD, backend, or mobile boundaries.

## Must Not Do

- Must not own test-writing workflow or TDD.
- Must not set up CI/CD or deployment.
- Must not implement backend architecture.
- Must not design API contracts beyond consuming existing API clients.
- Must not force monorepo layout on standalone apps.
- Must not apply Cache Components patterns when the project has not enabled them.
- Must not copy source reference skills verbatim.

## Pass Criteria

- [ ] Correct trigger decision for positive prompts.
- [ ] Correct non-trigger or companion-skill routing for negative prompts.
- [ ] Output includes Server/Client boundary reasoning.
- [ ] Output includes route/page/layout/loading/error/not-found guidance when relevant.
- [ ] Output includes explicit loading/error/empty/success state handling.
- [ ] Output preserves API client boundaries.
- [ ] Output includes accessibility and bundle/performance architecture considerations.
- [ ] Cache Components guidance is conditional on detected config.

## Example Evaluation

- **Input prompt**: "Add a Server Action for this Next.js settings form and decide which pieces should be client components."
- **Expected skill usage**: `nextjs-app-architecture` is selected; `nextjs-ui-tdd-workflow` is not selected unless tests are requested.
- **Expected output qualities**: The agent validates/authenticates/authorizes the action, returns stable form state, keeps the client form shell small, keeps server logic out of Client Components, invalidates changed data, and reports verification commands from `package.json`.
