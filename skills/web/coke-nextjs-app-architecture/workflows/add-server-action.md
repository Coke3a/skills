# Add Server Action

Use this workflow when adding a form mutation or server-side user action.

## Steps

1. Place the action.
   - Put feature-specific actions in `features/<feature>/actions`.
   - Keep route-only actions near the route only if they will not be reused.
2. Validate input.
   - Parse `FormData` or typed input through a schema.
   - Share validation with the form when practical.
3. Authenticate and authorize.
   - Treat Server Actions like API routes.
   - Check both identity and permission for the target resource.
4. Keep the action thin.
   - Delegate business logic to services/usecases when present.
   - Avoid large business logic directly inside components.
5. Return a stable result shape.
   - Use predictable success/error objects.
   - Map field errors, form errors, and permission errors to user-facing states.
6. Protect secrets and internals.
   - Do not return stack traces, raw database errors, tokens, or secret values.
7. Invalidate changed data.
   - Use the project cache strategy.
   - If Cache Components are enabled, update or revalidate relevant tags after mutation.
8. Integrate with the form.
   - Provide pending, error, success, and optimistic states where relevant.
   - Focus first invalid field when possible.

## Checks

- Action has validation, authentication, authorization, and safe error mapping.
- Client form receives stable state.
- Cache invalidation matches the data changed by the mutation.
