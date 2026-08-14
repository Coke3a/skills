---
name: nextjs-app-architecture
description: Guides Next.js App Router architecture — pages, layouts, Server/Client component boundaries, data fetching placement, Server Actions, component composition, feature folders, API client boundaries, UI state patterns, and optional Cache Components/PPR. Use when creating or refactoring app structure, deciding server vs client, adding routes/server actions, or reviewing feature layout. Do not use for UI testing/TDD, CI/CD, backend implementation, design tokens, or final code review.
---

# Next.js App Architecture

## Use this when

- Creating a new Next.js feature, route, page, or layout.
- Adding `loading.tsx`, `error.tsx`, `not-found.tsx`, or route groups.
- Deciding Server Component vs Client Component boundaries.
- Adding Server Actions, forms, data fetching, or API client integration.
- Refactoring component composition or feature folder layout.
- Modelling loading, error, empty, success, auth, pending, or optimistic states.
- Reviewing architecture decisions in a Next.js codebase.
- Applying Cache Components/PPR guidance when `cacheComponents: true` is set in `next.config.*`.

## Do not use this when

- Writing UI tests or driving TDD → use `coke-eng:nextjs-ui-tdd-workflow`.
- Backend implementation, API contracts, or backend delivery → use backend skills.
- Mobile/Expo architecture.
- Defining design tokens or shared visual language → use the design-system skill if present.
- Final UI code review → use the app code-review skill if present.
- CI/CD, deployment, or full performance profiling.

## Core rules

- Prefer Server Components by default; add `"use client"` only at the smallest island that needs interactivity, browser APIs, state, effects, refs, or event handlers.
- Keep data fetching close to the server boundary; start independent async work early and parallelize.
- Use Suspense intentionally for streaming and independent loading regions.
- Keep `page.tsx` and `layout.tsx` thin and composition-focused.
- Keep business logic out of UI components when it belongs in a service, action, or usecase.
- Keep API client calls behind a clear boundary; do not duplicate API fetching across components.
- Model loading, error, empty, success, auth, pending, and optimistic states explicitly for data-driven UI.
- Prefer composition and explicit variants over boolean-prop proliferation.
- Do not over-abstract before repeated use exists.
- Preserve semantic HTML, keyboard behaviour, focus management, and accessible form structure.
- Avoid bundle bloat from unnecessary `"use client"`, broad/barrel imports, and heavy client widgets.

## Workflow

1. Pick the workflow file matching the task (see Workflows below).
2. Load only the reference files needed for the decision at hand.
3. Use templates as starting points; adapt to the existing project convention.
4. Verify with project scripts found in `package.json` (lint, typecheck, test, build).

## Project shape

Standalone apps use `src/app`, `src/features`, `src/components`, `src/shared`, `src/lib`. Monorepos add `apps/web/` plus `packages/api-client`, `packages/shared-types`, `packages/ui`, `packages/design-tokens`. Do not force a monorepo shape on a standalone project. For feature folder details, see `references/feature-structure.md`.

## Load more detail

| Decision                                  | Reference                                                  |
| ----------------------------------------- | ---------------------------------------------------------- |
| Server vs Client component boundary       | `references/server-client-boundaries.md`                   |
| Routes, layouts, loading/error/not-found  | `references/app-router-structure.md`                       |
| Feature folder layout and placement rules | `references/feature-structure.md`                          |
| Data fetching, waterfalls, Suspense       | `references/data-fetching.md`                              |
| Server Actions auth, validation, errors   | `references/server-actions.md`                             |
| Forms architecture and validation         | `references/form-architecture.md`                          |
| API client boundary                       | `references/api-client-boundary.md`                        |
| Component composition vs boolean props    | `references/component-composition.md`                      |
| UI state coverage (loading/error/empty…)  | `references/ui-state-patterns.md`                          |
| Accessibility structure                   | `references/accessibility-structure.md`                    |
| Performance-aware architecture            | `references/performance-aware-architecture.md`             |
| Cache Components / PPR (optional)         | `references/cache-components-optional.md`                  |
| Common architecture smells                | `references/architecture-smells.md`                        |

## Workflows

| Workflow                                       | Use for                                              |
| ---------------------------------------------- | ---------------------------------------------------- |
| `workflows/scaffold-nextjs-feature.md`         | New feature folder and entry points                  |
| `workflows/add-route-or-page.md`               | Adding a route, page, or layout                      |
| `workflows/add-client-component.md`            | Adding a client island                               |
| `workflows/add-server-action.md`               | Adding a Server Action                               |
| `workflows/add-api-client-integration.md`      | Wiring an API client behind a boundary               |
| `workflows/refactor-component-composition.md`  | Replacing boolean-prop sprawl with composition       |
| `workflows/apply-cache-components.md`          | Applying Cache Components/PPR when enabled           |

## Templates

Use the `templates/` folder as starting points. Common templates include `page-template.tsx`, `layout-template.tsx`, `loading-template.tsx`, `error-template.tsx`, `server-component-template.tsx`, `client-component-template.tsx`, `form-component-template.tsx`, `server-action-template.ts`, `api-client-template.ts`, and `component-composition-template.tsx`. Adapt names and imports to the project before committing.

## Related skills

- `coke-eng:nextjs-ui-tdd-workflow` — UI tests and TDD for the components designed here.
- Backend skills — backend feature implementation and API contracts.
- Design-system / API-contract / code-review skills — when present, own design tokens, contracts, and final review.

## Definition of done

- Routes, layouts, and special-state files exist where they earn their keep.
- Server/Client boundaries match the rules in `references/server-client-boundaries.md`.
- Data fetching avoids waterfalls and lives at the right boundary.
- Server Actions authenticate, validate, return stable shapes, and invalidate cache as needed.
- Loading, error, empty, success, auth, and pending states are handled for data-driven UI.
- Project scripts pass (e.g. `npm run lint`, `npm run typecheck`, `npm run test`, `npm run build` — only those defined in `package.json`).
