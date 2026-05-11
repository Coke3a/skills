# Add Client Component

Use this workflow when a component needs interactivity, browser APIs, local state, effects, refs, or
event handlers.

## Steps

1. Justify `"use client"`.
   - Confirm a Server Component cannot satisfy the requirement.
   - Keep the client boundary as low as possible.
2. Define server-to-client props.
   - Pass only serializable data.
   - Avoid large payloads, duplicated objects, and server-only types.
   - Prefer IDs or summarized view models when the client does not need full records.
3. Implement interactivity.
   - Keep event handlers close to the interactive UI.
   - Use effects only for synchronizing with external systems.
   - Derive render state during render instead of effect-driven state when possible.
4. Handle accessibility.
   - Use semantic controls.
   - Add labels, focus states, keyboard behavior, and `aria-live` for async updates where needed.
5. Protect bundle size.
   - Avoid broad imports and server-only dependencies.
   - Dynamically import heavy widgets, charts, editors, maps, or media tools when appropriate.
6. Optimize UI updates only when needed.
   - Use transitions or deferred values for non-urgent expensive updates.
   - Do not add memoization unless the component has a clear re-render cost.

## Checks

- `"use client"` is not applied to the whole route by convenience.
- Props are small, serializable, and stable.
- No server-only modules are imported.
- Focus, keyboard, and touch behavior remain usable.
