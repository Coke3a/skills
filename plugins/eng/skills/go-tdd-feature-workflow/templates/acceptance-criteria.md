# Acceptance Criteria — <feature name>

Confirm this list with the user before writing the first test. These are the design decisions the
overview did not make; they are not assumptions to make silently.

## Entry gate

| # | Precondition     | Source                            |
| - | ---------------- | --------------------------------- |
| 1 | Endpoints        | `<file:line>` — `<quoted value>`  |
| 2 | Request/response | `<file:line>` — `<quoted value>`  |
| 3 | Database schema  | `<file:line>` — `<quoted value>`  |
| 4 | Purpose          | `<file:line>` — `<quoted value>`  |

Overridden by the user despite a gap: `<none / which precondition and what was assumed>`

## Criteria

One row per behaviour. `AC-1` is the simplest success path — it is what forces the Input and Output
structs into existence.

| ID   | Given                            | When                     | Then                                       | Level    |
| ---- | -------------------------------- | ------------------------ | ------------------------------------------ | -------- |
| AC-1 | a valid request from an owner    | POST /v1/…               | 201 with the created resource              | usecase  |
| AC-2 | an empty name                    | POST /v1/…               | `usecase.ErrValidation` → 400              | domain   |
| AC-3 | a name already used by the owner | POST /v1/…               | `usecase.ErrConflict` → 409                | usecase  |
| AC-4 | the repository is unreachable    | POST /v1/…               | `usecase.ErrInternal` → 500, chain not leaked | usecase |
| AC-5 | a caller who does not own it     | PATCH /v1/…              | `usecase.ErrForbidden` → 403               | usecase  |

Cover each category or state explicitly that it does not apply:

- Success, per endpoint
- Validation — which field, which rule
- Not found
- Conflict / uniqueness
- Permission and ownership
- Dependency failure — repository unavailable, external service rejected or timed out
- Idempotency or retry, if the design implies any
- Edge cases — empty list, boundary lengths, timezone, pagination limits

## First test

- Criterion: `<AC-n>`
- Level and why: `<domain | usecase | handler>` — `<the lowest level that proves it>`
- File: `<path>`
- Expected failure before implementation: `<compile error | wrong sentinel | wrong status>`

## Ports expected to emerge

Filled in as the tests discover them, not decided up front. A method lands here when a usecase calls
it.

| Port                             | Method | Discovered by |
| -------------------------------- | ------ | ------------- |
| `repository.ExampleRepository`   |        |               |
| `service.<X>`                    |        |               |

## Not covered

- Repository integration (sqlc/pgx mapping, constraints, transactions) — out of scope for this loop,
  verify separately against a real database.
- `<anything else deliberately excluded, and why>`
