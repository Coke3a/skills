# Add API Client Integration

Use this workflow when a Next.js feature consumes a backend or external API.

## Steps

1. Check whether the API contract changes.
   - If yes, use `app-api-contract-workflow`.
   - This skill only owns consuming an existing contract from the app side.
2. Find the existing API boundary.
   - Look for `packages/api-client`, `src/lib/api`, `src/shared/api`, feature `services`, or
     generated clients.
   - Follow the project convention instead of creating a competing client.
3. Place integration code.
   - Use shared/lib/package clients for cross-feature API calls.
   - Use feature services for feature-specific orchestration and view mapping.
   - Keep direct API calls out of React components unless the project already uses that boundary.
4. Map API results to UI states.
   - Success, empty, validation error, permission error, not found, and unexpected error.
   - Do not leak raw transport errors into user-facing copy.
5. Avoid duplicate fetching logic.
   - Centralize repeated URL construction, headers, parsing, and error mapping.
   - Keep route pages and components focused on composition.
6. Respect runtime boundaries.
   - Server-only credentials stay in server modules.
   - Browser clients use public configuration only.

## Checks

- API contract ownership is not mixed into this task.
- Components consume a stable client/service interface.
- Errors and empty states are explicitly mapped.
