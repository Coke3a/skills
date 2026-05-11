# Data Fetching

Sources synthesized:
- https://github.com/vercel-labs/agent-skills/blob/main/skills/react-best-practices/SKILL.md
- https://skills.sh/vercel/next.js/cache-components

## Placement

- Fetch server data in Server Components when possible.
- Keep direct API/database access behind server-safe modules or API client boundaries.
- Use client-side fetching only for data that must refresh from client state, browser APIs, realtime interactions, or post-hydration UX.
- Do not duplicate API fetching logic across pages, components, and actions.

## Waterfall Avoidance

- Start independent async work before the first `await`.
- Use `Promise.all` for independent fetches.
- Do not make a parent block a child unless the child depends on parent results.
- For dependent lists, fetch each item detail in a parallel batch when appropriate.
- Move cheap synchronous checks before async work.

## Suspense

- Use Suspense for independently loadable sections and dynamic streaming content.
- Place boundaries where fallback UI makes sense to users.
- Avoid wrapping the entire page in one generic fallback when smaller boundaries would reveal useful content earlier.

## Dedupe And Caching

- Use the project convention for request dedupe, generated clients, `fetch` caching, React cache, or service-level caching.
- Avoid duplicate serialization of the same data into multiple Client Components.
- Never store request-specific mutable state in module scope.

## Client Fetching

- Keep client fetchers behind hooks or client-safe API clients.
- Map loading, error, empty, and success states explicitly.
- Deduplicate repeated client requests with the project's chosen library or cache strategy.
