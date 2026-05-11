---
name: nextjs-app-architecture
description:
  Use when creating or refactoring Next.js App Router application structure, pages, layouts,
  Server/Client Component boundaries, data fetching placement, Server Actions, component
  composition, feature folders, API client boundaries, UI state patterns, and optional Cache
  Components/PPR guidance. Pair with nextjs-ui-tdd-workflow for tests.
---

# Next.js App Architecture

## Purpose

Build maintainable, scalable Next.js apps with App Router, React composition, clear Server/Client
Component boundaries, explicit UI states, and performance-aware structure.

Use this skill as architecture guidance. Load the relevant workflow first, then load only the
reference files needed for the task.

## When to Use

- Creating a new Next.js feature.
- Adding a route, page, layout, loading state, error boundary, or not-found state.
- Deciding Server Component vs Client Component boundaries.
- Adding data fetching, Server Actions, forms, or API client integration.
- Refactoring component structure or reusable component APIs.
- Organizing feature folders in standalone apps or monorepos.
- Handling loading, error, empty, success, auth, pending, or optimistic states.
- Reviewing Next.js architecture.
- Applying Cache Components/PPR guidance when `cacheComponents: true` is enabled.

## When Not to Use

- Writing tests or defining the TDD workflow.
- Setting up CI/CD, deployment, or full performance profiling.
- Implementing backend features, mobile/Expo architecture, or backend delivery.
- Designing backend/app API contracts beyond consuming existing API clients.
- Defining visual design tokens or shared UI language.

## Companion Skills

- `app-api-contract-workflow` owns backend to app API contracts.
- `app-design-system` owns design tokens and shared UI language.
- `app-code-review` owns final app quality review.
- `nextjs-ui-tdd-workflow` owns UI testing and TDD workflow.
- Backend skills own backend architecture and delivery.

## Core Rules

- Prefer Server Components by default.
- Use Client Components only for interactivity, browser APIs, state, effects, refs, or event
  handlers.
- Keep data fetching close to the server boundary.
- Avoid async waterfalls; start independent async work early and parallelize it.
- Use Suspense intentionally for streaming and independent loading regions.
- Keep pages and layouts thin, compositional, and route-focused.
- Keep business logic out of UI components when it belongs in an API, domain, service, or usecase
  layer.
- Keep API client calls behind a clear boundary.
- Model loading, error, empty, success, auth, pending, and optimistic states explicitly.
- Avoid component APIs with many boolean props; prefer composition and explicit variants.
- Do not over-abstract before repeated use exists.
- Preserve semantic HTML, keyboard behavior, focus states, and accessible form structure.
- Avoid bundle bloat from unnecessary Client Components, broad imports, and heavy client widgets.

## Recommended Project Shapes

Standalone apps:

```text
src/
  app/
  features/
  components/
  shared/
  lib/
```

Monorepos:

```text
apps/
  web/
    src/
      app/
      features/
      components/
      shared/
      lib/

packages/
  api-client/
  shared-types/
  ui/
  design-tokens/
```

Do not force a monorepo shape when the project is standalone.

## Feature Structure

Use feature folders for cohesive product areas:

```text
features/<feature-name>/
  components/
  actions/
  services/
  hooks/
  schemas/
  types.ts
  index.ts
```

- Keep feature-local components inside the feature.
- Move cross-feature reusable UI to `shared/components` or `packages/ui`.
- Do not duplicate API client code inside components.
- Share schemas between form and action where practical.
- Avoid circular imports between features.

## App Router Structure

- Use `app/<route>/page.tsx` for the route entry.
- Add `layout.tsx`, `loading.tsx`, `error.tsx`, and `not-found.tsx` only where they own useful
  behavior.
- Use route groups to organize without changing URLs.
- Use nested layouts for persistent route UI, not one-off wrappers.
- Colocate route-specific components near the route or feature.
- Define metadata at the nearest stable route boundary.

See `references/app-router-structure.md`.

## Server / Client Boundary

| Need                                  | Prefer                                                                 |
| ------------------------------------- | ---------------------------------------------------------------------- |
| Static render / data from server      | Server Component                                                       |
| DB/server-only access                 | Server Component or Server Action                                      |
| Form mutation                         | Server Action + client form shell when needed                          |
| Browser API                           | Client Component                                                       |
| `useState`/`useEffect`/event handlers | Client Component                                                       |
| Heavy interactive widget              | Client Component, dynamically imported if appropriate                  |
| User-specific dynamic data            | Server Component with dynamic boundary or client fetch depending on UX |
| Shared UI with no interactivity       | Server Component-compatible component                                  |

- Do not add `"use client"` at a high-level route unless needed.
- Keep client islands small.
- Avoid passing large serialized props to client components.
- Do not import server-only modules into client components.
- Do not pass functions across the server/client boundary except supported Server Actions.

See `references/server-client-boundaries.md`.

## Data Fetching

- Fetch server data in Server Components when possible.
- Start independent promises early and use `Promise.all` for independent work.
- Avoid parent-child waterfalls.
- Use Suspense for streaming where useful.
- Dedupe repeated server work where appropriate.
- Use client-side fetching only when data must update on the client or depends on client-only state.
- Do not duplicate API fetching logic across pages.

See `references/data-fetching.md`.

## Server Actions

- Authenticate and authorize Server Actions like API routes.
- Validate input before mutation.
- Return stable action result shapes.
- Map errors into user-facing form/action states.
- Invalidate cache when mutation changes cached data.
- Keep Server Actions thin; call services or usecases when present.
- Do not leak secrets, stack traces, or internal errors to the client.

See `references/server-actions.md`.

## Component Composition

- Avoid boolean prop proliferation.
- Prefer explicit variants or named components.
- Use compound components for complex reusable UI.
- Use children composition for layout and content.
- Use context/provider only when state is truly shared.
- Define ownership clearly.
- Do not create generic components before repeated use cases exist.
- Keep props small and intentional.

See `references/component-composition.md`.

## UI State Patterns

Every data-driven screen accounts for:

- Loading.
- Error.
- Empty.
- Success.
- Permission/auth state when relevant.
- Optimistic/pending state when relevant.

See `references/ui-state-patterns.md`.

## Accessibility Structure

- Prefer semantic HTML before ARIA.
- Provide labels and associated errors for forms.
- Preserve keyboard navigation and visible focus states.
- Manage focus for dialogs, drawers, sheets, and validation errors.
- Avoid `div`/`span` click handlers when `button` or `a` is correct.
- Provide useful image alt text, or empty alt for decorative images.

See `references/accessibility-structure.md`.

## Performance-Aware Architecture

- Avoid unnecessary `"use client"`.
- Avoid broad or barrel imports when they increase bundles.
- Dynamically import heavy client-only widgets.
- Avoid passing large data into Client Components.
- Avoid unnecessary effects and derived state in effects.
- Memoize only when there is a measured or obvious reason.
- Split heavy interactive parts from mostly-static UI.
- Defer non-critical third-party scripts.
- Use `next/image` and appropriate media handling.

See `references/performance-aware-architecture.md`.

## Optional Cache Components

Only apply Cache Components guidance if `cacheComponents: true` is detected in `next.config.*`.

- Keep user-specific data dynamic.
- Use `'use cache'` only for cacheable data that is the same across users.
- Place `'use cache'` as the first statement in cached functions/components.
- Use Suspense for dynamic streaming.
- Use cache tags and cache life intentionally.
- Invalidate relevant tags after mutations through Server Actions.
- Do not put `cookies()` or `headers()` inside cached scope.
- Do not apply Cache Components patterns when the project does not use them.

See `references/cache-components-optional.md` and `workflows/apply-cache-components.md`.

## Workflows

- New feature: `workflows/scaffold-nextjs-feature.md`.
- Route/page: `workflows/add-route-or-page.md`.
- Client island: `workflows/add-client-component.md`.
- Server Action: `workflows/add-server-action.md`.
- Composition refactor: `workflows/refactor-component-composition.md`.
- API client integration: `workflows/add-api-client-integration.md`.
- Cache Components: `workflows/apply-cache-components.md`.

## Final Verification

When changing a downstream Next.js project, inspect `package.json` first and run or request
project-appropriate scripts, for example:

```sh
npm run lint
npm run typecheck
npm run test
npm run build
```

Do not invent commands if the project does not define them.

## Final Response Format

Summarize:

- Routes/pages added or changed.
- Server/Client boundary decisions.
- Feature structure used.
- API client boundary.
- UI states handled.
- Composition choices.
- Accessibility notes.
- Performance considerations.
- Cache Components usage if applicable.
- Commands run.
