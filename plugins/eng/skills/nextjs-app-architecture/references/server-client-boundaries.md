# Server And Client Boundaries

Sources synthesized:

- https://github.com/vercel-labs/agent-skills/blob/main/skills/react-best-practices/SKILL.md
- https://github.com/vercel-labs/agent-skills/blob/main/skills/composition-patterns/SKILL.md

## Default

Use Server Components by default. Add `"use client"` only at the smallest component that needs
browser-only behavior.

## Prefer Server Components For

- Static UI.
- Server data fetching.
- Database or server-only SDK access.
- Auth/session checks that require secrets.
- Components that only render props and children.
- Shared UI primitives with no interactivity.

## Prefer Client Components For

- Event handlers.
- `useState`, `useEffect`, `useReducer`, refs, or browser APIs.
- Interactive controls, dialogs, drawers, menus, drag/drop, charts, editors, maps, and media
  widgets.
- Client-only data that depends on viewport, storage, live browser state, or realtime updates.

## Boundary Rules

- Do not mark a route, layout, or whole feature `"use client"` by convenience.
- Keep client islands small and nested inside server-rendered shells.
- Pass small serializable props from server to client.
- Avoid passing large collections, duplicated records, class instances, dates without serialization
  decisions, or server-only types.
- Do not import `fs`, database clients, server SDKs, secret config, `cookies()`, or `headers()` into
  Client Components.
- Do not pass functions across the boundary except supported Server Actions.
- Keep shared UI server-compatible unless interactivity is inherent.

## Decision Table

| Need                             | Prefer                                              |
| -------------------------------- | --------------------------------------------------- |
| Static render / data from server | Server Component                                    |
| DB/server-only access            | Server Component or Server Action                   |
| Form mutation                    | Server Action + client form shell when needed       |
| Browser API                      | Client Component                                    |
| State/effects/event handlers     | Client Component                                    |
| Heavy interactive widget         | Client Component, dynamic import when appropriate   |
| User-specific dynamic data       | Server dynamic boundary or client fetch based on UX |
| Shared non-interactive UI        | Server-compatible component                         |

## Smells

- `"use client"` appears in `page.tsx` without a strong reason.
- A Client Component imports a feature service that uses server credentials.
- A server component builds a huge object only to pass it unchanged to a client child.
- Client and server versions of the same fetch logic drift apart.
