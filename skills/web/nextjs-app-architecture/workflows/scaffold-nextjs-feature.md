# Scaffold Next.js Feature

Use this workflow when adding a complete feature, screen, or product area.

## Steps

1. Inspect the project shape.
   - Detect standalone `src/app` or monorepo `apps/web/src/app`.
   - Inspect `package.json`, `next.config.*`, existing feature folders, shared UI, and API client conventions.
2. Identify the route/page.
   - Choose the App Router segment, route group, and whether `layout.tsx`, `loading.tsx`, `error.tsx`, or `not-found.tsx` are needed.
3. Identify the feature folder.
   - Reuse an existing feature when the domain already exists.
   - Create `features/<feature-name>/` only when the feature owns cohesive UI, actions, services, schemas, or hooks.
4. Decide Server/Client boundaries.
   - Keep route entries and static UI server-compatible by default.
   - Add Client Components only for interactivity, browser APIs, local state, effects, refs, or event handlers.
5. Decide data fetching.
   - Fetch in Server Components when possible.
   - Start independent async work early and parallelize independent requests.
   - Add Suspense boundaries for streaming or independent loading regions.
6. Define UI states.
   - Loading, error, empty, success, auth/permission, pending, and optimistic states where relevant.
7. Define component structure.
   - Keep pages thin.
   - Place feature-local UI under `features/<feature>/components`.
   - Promote repeated cross-feature UI to `shared/components` or `packages/ui`.
8. Define API client boundary.
   - Use an existing API client/service layer.
   - Do not duplicate `fetch` calls in components.
   - If the contract changes, use `app-api-contract-workflow`.
9. Implement the minimal structure.
   - Add only the route files, feature files, and shared components required by the use case.
   - Avoid generic abstractions until repeated use exists.
10. Run verification.
    - Inspect scripts first.
    - Run lint, typecheck, test, and build only when defined and appropriate.

## Output

Report the route, feature folder, Server/Client boundary, API boundary, UI states, accessibility notes, performance choices, Cache Components usage if enabled, and commands run.
