# Form Architecture

## Placement

- Keep the visual form shell in a Client Component when it needs pending state, client validation, local state, or interactive controls.
- Keep mutation logic in Server Actions when the mutation can run on the server.
- Share validation schemas between the form and action where practical.

## Structure

- Use semantic `form`, `label`, `input`, `select`, `textarea`, `fieldset`, and `button` elements.
- Keep labels clickable through `htmlFor` or wrapped controls.
- Use meaningful `name`, `type`, `inputMode`, and `autoComplete`.
- Keep submit buttons enabled until a request starts, then show pending state.
- Associate field errors with fields.
- Focus the first invalid field on failed submit when practical.

## States

- Initial.
- Dirty/edited when relevant.
- Pending.
- Field validation error.
- Form-level error.
- Permission/auth error.
- Success.
- Optimistic state when it improves UX and can be reconciled safely.

## Server Action Integration

- Validate untrusted `FormData`.
- Return stable action state.
- Do not expose internals in action errors.
- Invalidate changed data after successful mutation.

## Accessibility

- Do not block paste.
- Do not rely on placeholder text as a label.
- Use `aria-live="polite"` for async validation or submit results when useful.
- Preserve keyboard submit and tab order.
