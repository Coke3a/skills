# Cache Components Optional

Source synthesized:

- https://skills.sh/vercel/next.js/cache-components

## Detection

Apply this guidance only when `cacheComponents: true` is present in `next.config.*`.

If the flag is absent, do not add `'use cache'`, `cacheTag()`, `cacheLife()`, or Cache
Components-specific structure.

## Decision Model

- Static content: no directive needed.
- Shared cacheable data: same output across users for the same arguments; may use `'use cache'`.
- Dynamic user-specific data: depends on `cookies()`, `headers()`, auth, request context, or private
  user data; keep dynamic and stream with Suspense when useful.

## Rules

- Cached functions/components must be async.
- Place `'use cache'` as the first statement in the cached function/component scope.
- Do not call `cookies()` or `headers()` inside cached scope.
- Use `cacheTag()` when mutations need targeted invalidation.
- Use `cacheLife()` only with a deliberate freshness policy.
- Keep cache keys in mind: arguments affect cached output.

## Mutations

- Invalidate relevant tags after successful Server Actions.
- Use immediate invalidation when users must see their own writes immediately.
- Use background revalidation when stale-while-revalidate behavior is acceptable.

## Suspense

- Use Suspense around dynamic request-time sections.
- Keep cached/static shells useful while dynamic content streams.

## Smells

- Caching user-specific profile, session, or permission data.
- Adding `'use cache'` to fix performance without checking data safety.
- Applying Cache Components patterns in a project that has not enabled the feature.
