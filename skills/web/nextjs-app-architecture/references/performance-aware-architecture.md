# Performance-Aware Architecture

Sources synthesized:
- https://github.com/vercel-labs/agent-skills/blob/main/skills/react-best-practices/SKILL.md
- https://github.com/vercel-labs/agent-skills/blob/main/skills/web-design-guidelines/SKILL.md

## Server Work

- Avoid async waterfalls.
- Start independent work early and await late.
- Use `Promise.all` for independent requests.
- Use Suspense to stream independent sections.
- Avoid mutable module-level request state in RSC/SSR.
- Avoid duplicate serialization of the same data into multiple Client Components.
- Minimize data passed from Server Components to Client Components.

## Bundle Size

- Avoid unnecessary `"use client"`.
- Avoid broad barrel imports when they pull large modules into the bundle.
- Import directly from package subpaths when the package supports it and the project convention allows it.
- Dynamically import heavy client-only widgets such as charts, editors, maps, and media tools.
- Defer non-critical third-party scripts and analytics.

## Client Rendering

- Avoid defining components inside components.
- Avoid unnecessary effects.
- Derive state during render when possible instead of effect-driven derived state.
- Move interaction logic into event handlers when it does not need an effect.
- Use transitions and deferred values for non-urgent expensive UI updates when appropriate.
- Memoize only for real repeated expensive renders or stable public APIs.

## Layout And Media

- Use `next/image` or the project media abstraction for optimized images.
- Provide dimensions for images/media.
- Consider virtualization or `content-visibility` for large lists.
- Avoid layout reads during render.

## Smells

- A mostly static page is fully client-rendered.
- A chart/editor is bundled into the initial route without being immediately needed.
- Effects compute data that could be derived during render.
- Parent data fetching serializes independent child fetches.
