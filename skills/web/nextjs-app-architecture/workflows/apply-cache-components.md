# Apply Cache Components

Use this workflow only when a downstream Next.js project has Cache Components enabled.

## Steps

1. Detect enablement.
   - Inspect `next.config.ts`, `next.config.js`, `next.config.mjs`, or `next.config.cjs`.
   - Continue only when `cacheComponents: true` is present.
2. Classify each data source.
   - Static: no request or data dependency.
   - Cacheable: same output across users for the same arguments.
   - Dynamic: depends on cookies, headers, auth, request context, live user state, or private data.
3. Add `'use cache'` only when safe.
   - Place it as the first statement in the cached async function or component.
   - Do not use cached scope for `cookies()`, `headers()`, or user-specific data.
4. Add cache policy intentionally.
   - Use `cacheTag()` for data that mutations must invalidate.
   - Use `cacheLife()` only with a clear freshness expectation.
5. Add Suspense for dynamic streaming.
   - Wrap dynamic request-time sections in Suspense.
   - Use stable fallbacks that preserve layout.
6. Invalidate after mutations.
   - Update or revalidate relevant tags in Server Actions after successful writes.
   - Keep invalidation specific enough to avoid broad stale UI.
7. Do not retrofit blindly.
   - If `cacheComponents: true` is not enabled, do not add Cache Components patterns.

## Checks

- Cached data is shared across users.
- Dynamic user-specific data remains outside cached scope.
- Mutations invalidate the same tags that reads use.
