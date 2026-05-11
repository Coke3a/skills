# Server Actions

Sources synthesized:
- https://github.com/vercel-labs/agent-skills/blob/main/skills/react-best-practices/SKILL.md
- https://skills.sh/vercel/next.js/cache-components

## Placement

- Put feature-specific actions in `features/<feature>/actions`.
- Put route-only actions near the route only when they will not be shared.
- Keep actions thin and delegate business logic to services/usecases when present.

## Safety

- Authenticate every action that requires a user.
- Authorize against the target resource, not only the user session.
- Validate all input with the project schema convention.
- Treat `FormData` as untrusted.
- Do not leak secrets, raw database errors, stack traces, or internal identifiers that users should not see.

## Result Shape

Return stable state that forms can consume:

```ts
type ActionResult<T = unknown> =
  | { ok: true; data?: T; message?: string }
  | { ok: false; fieldErrors?: Record<string, string>; formError?: string };
```

Adapt this shape to the project convention if one exists.

## Cache Invalidation

- Invalidate affected data after successful mutations.
- Keep invalidation close to the mutation.
- If Cache Components are enabled, align `cacheTag()` reads with action invalidation through `updateTag()` or `revalidateTag()`.

## Form Integration

- Surface pending, validation, permission, unexpected error, and success states.
- Keep field errors associated with their controls.
- Focus the first invalid field when practical.
