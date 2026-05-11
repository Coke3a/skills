# Add Route Or Page

Use this workflow when adding or changing App Router route files.

## Steps

1. Identify the route segment and URL.
   - Use route groups for organization without URL changes.
   - Use dynamic segments only when the URL needs them.
2. Create or update `page.tsx`.
   - Keep it server-compatible unless interaction is required.
   - Compose feature components and server data loaders.
   - Keep page-level branching readable.
3. Add route support files only when useful.
   - `layout.tsx` for persistent route shell or shared metadata.
   - `loading.tsx` for immediate route-level fallback.
   - `error.tsx` for recoverable route errors; it must be a Client Component.
   - `not-found.tsx` for missing entity or route-specific 404 UI.
4. Define metadata.
   - Put static metadata near the route.
   - Use dynamic metadata only when it depends on route params or fetched data.
5. Place server data fetching.
   - Start independent fetches before rendering child components.
   - Use `Promise.all` for independent work.
   - Avoid making children wait on parent data they do not depend on.
6. Add Suspense intentionally.
   - Use Suspense around independently loadable route sections.
   - Provide fallbacks that match final layout dimensions when practical.
7. Add client islands only when required.
   - Keep `"use client"` below the route boundary.
   - Pass small, serializable props.

## Checks

- No unnecessary high-level `"use client"`.
- Loading, error, empty, and not-found states are explicit where relevant.
- Links preserve normal browser navigation behavior.
- Route-specific code is colocated without creating cross-feature cycles.
