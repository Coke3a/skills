# API Client Boundary

## Scope

This skill owns consuming an existing backend or external API from the Next.js app. It does not own backend implementation or API contract design.

Use `app-api-contract-workflow` when endpoints, schemas, payloads, or contract behavior must change.

## Boundaries

- Cross-feature API client: `packages/api-client`, `src/lib/api`, or `src/shared/api`.
- Feature orchestration: `features/<feature>/services`.
- Server-only API code: modules that may access secrets, cookies, headers, or backend credentials.
- Client-safe API code: browser-safe clients using public configuration only.

## Rules

- Do not duplicate raw `fetch` calls across components.
- Centralize base URLs, headers, parsing, and transport error mapping.
- Map API responses to feature view models before passing data deep into UI.
- Keep server credentials out of Client Components.
- Keep retry, auth refresh, and transport concerns in the client boundary, not UI components.
- Surface errors as UI states: validation, permission, not found, empty, and unexpected.

## Smells

- Multiple components build the same endpoint URL.
- A Client Component imports server config.
- UI components inspect raw HTTP status codes.
- Backend contract changes are made ad hoc in the frontend without companion workflow.
