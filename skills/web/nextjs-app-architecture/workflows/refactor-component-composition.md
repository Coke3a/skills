# Refactor Component Composition

Use this workflow when a component API is becoming hard to use, especially from boolean props or unclear ownership.

## Steps

1. Find the pressure point.
   - Look for multiple booleans, mode flags, render branches, repeated override props, or unclear slot ownership.
2. Classify the component.
   - One-off feature UI: keep it local and simple.
   - Repeated reusable UI: improve the public API.
   - Complex reusable primitive: consider compound components.
3. Replace boolean prop proliferation.
   - Prefer explicit variants for simple mutually exclusive modes.
   - Prefer named components for meaningfully different use cases.
   - Prefer children composition for layout/content slots.
4. Use compound components only when useful.
   - Use when multiple subcomponents must share state or context.
   - Keep the provider as the only implementation-aware layer.
   - Expose a small human-readable API.
5. Keep state ownership clear.
   - Keep state local until siblings need it.
   - Lift state only when it clarifies ownership or enables coordination.
   - Use context only for truly shared state.
6. Preserve accessibility.
   - Keep semantic elements, labels, focus handling, keyboard behavior, and ARIA relationships intact.
7. Avoid premature abstraction.
   - Do not create generic components without repeated use cases.

## Checks

- Component API is easier for humans and agents to use.
- Accessibility behavior did not regress.
- The new abstraction removes real duplication or complexity.
