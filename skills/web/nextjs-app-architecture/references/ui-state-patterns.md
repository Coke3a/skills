# UI State Patterns

## Required States

Every data-driven screen should explicitly account for:

- Loading.
- Error.
- Empty.
- Success.
- Permission/auth state when relevant.
- Pending mutation state when relevant.
- Optimistic state when relevant.

## Loading

- Use route `loading.tsx` for route-level pending UI.
- Use Suspense fallbacks for independently loading regions.
- Match skeleton dimensions to final content when practical to reduce layout shift.

## Error

- Use `error.tsx` for recoverable route segment errors.
- Map known API/action errors to user-facing copy and recovery.
- Provide retry or navigation when useful.
- Do not expose raw internal error details.

## Empty

- Empty arrays and missing optional content should render intentional empty states.
- Empty states should explain what happened and what the user can do next when an action exists.

## Success

- Render the primary content path clearly.
- Avoid mixing success and error branches in deeply nested conditional JSX.

## Permission/Auth

- Distinguish unauthenticated, unauthorized, missing resource, and unexpected error when the product needs different UX.

## Optimistic/Pending

- Use pending state for form submit, transition, and background mutation feedback.
- Use optimistic state only when rollback behavior is clear.
