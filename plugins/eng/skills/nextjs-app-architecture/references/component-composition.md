# Component Composition

Sources synthesized:

- https://github.com/vercel-labs/agent-skills/blob/main/skills/composition-patterns/SKILL.md
- https://github.com/vercel-labs/agent-skills/blob/main/skills/web-design-guidelines/SKILL.md

## API Design

- Avoid boolean prop proliferation for behavior and layout modes.
- Prefer explicit variants for simple mutually exclusive states.
- Prefer named components when variants represent different concepts.
- Prefer children composition for content and layout slots.
- Keep props small, intentional, and stable for humans and agents.

## Compound Components

Use compound components when:

- Multiple subcomponents coordinate through shared state.
- Consumers need flexible composition.
- A single monolithic component would need many mode props.

Do not use compound components for one-off feature UI or simple display components.

## Context And Providers

- Use context only for state truly shared across descendants.
- Keep the provider as the only place that knows the implementation details.
- Expose a small interface with state, actions, and metadata when dependency injection helps.
- Lift state only when siblings need coordinated access.

## Ownership

- Feature components own feature-specific behavior.
- Shared UI primitives own generic interaction and accessibility behavior.
- Pages own route composition, not component internals.
- Services own data mapping and business logic when that logic is not presentation-specific.

## Accessibility Preservation

Composition refactors must preserve labels, roles, focus behavior, keyboard interaction, and
field/error relationships.
