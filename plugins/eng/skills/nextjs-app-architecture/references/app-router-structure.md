# App Router Structure

Sources synthesized:

- https://github.com/vercel-labs/agent-skills/blob/main/skills/react-best-practices/SKILL.md
- https://github.com/vercel-labs/agent-skills/blob/main/skills/web-design-guidelines/SKILL.md

## Route Files

- `page.tsx` is the route entry and should stay thin: fetch or compose, then delegate feature UI.
- `layout.tsx` owns persistent route chrome, shared providers at that route depth, and stable
  metadata.
- `loading.tsx` provides an immediate fallback for the route segment; keep it layout-compatible with
  final content.
- `error.tsx` catches recoverable errors for a segment and must be a Client Component.
- `not-found.tsx` handles missing route/entity states and should be user-facing.
- `route.ts` is for route handlers, not page UI.

## Organization

- Use route groups such as `(marketing)` or `(dashboard)` for organization without changing URLs.
- Use nested layouts only when the UI persists across multiple child routes.
- Colocate route-specific components under the route when they are not feature-reusable.
- Put product-domain UI in `features/<feature>/components` when it is reused across routes.
- Keep global reusable UI in `shared/components` or `packages/ui`.

## Metadata

- Prefer static `metadata` when values do not depend on request data.
- Use `generateMetadata` only when route params or fetched data are needed.
- Avoid duplicate data fetching between `generateMetadata` and the page when the project has a
  dedupe/cache convention.

## Architecture Checks

- Route entries do not become large business-logic files.
- Loading, error, empty, and not-found states match user-facing outcomes.
- Navigation uses links for navigation and buttons for actions.
- Route grouping improves maintainability without hiding ownership.
