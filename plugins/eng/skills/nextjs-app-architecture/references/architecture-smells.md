# Architecture Smells

Use this reference when reviewing or refactoring Next.js feature structure.

## App Router

- `page.tsx` contains business logic, large forms, and low-level API calls.
- `layout.tsx` includes route-specific logic that does not persist across children.
- `error.tsx` is missing for a route with known recoverable failures.
- Route groups obscure ownership instead of clarifying it.

## Server/Client Boundary

- High-level `"use client"` appears without a browser-only requirement.
- Client Components import server-only modules.
- Large records are serialized to client islands that need only a few fields.
- Shared UI primitives are client-only by default.

## Data Fetching

- Parent and child components fetch independent data serially.
- The same API call is copied across pages.
- Raw transport errors leak into UI copy.
- Request-specific mutable data is stored in module scope.

## Components

- Component API has many boolean props.
- One component owns unrelated variants, layout modes, and data fetching.
- Context exists for state used by only one component.
- Generic components are created before repeated use exists.

## UI States And Accessibility

- Empty arrays render broken or blank UI.
- Loading and error states are generic and do not match the final layout.
- Forms lack labels, associated errors, or pending states.
- Interactive elements are not keyboard accessible.

## Performance

- Heavy charts, editors, or maps are in the initial route bundle.
- Effects derive render state.
- Large lists render without virtualization or containment.
- Third-party scripts load before they are needed.
