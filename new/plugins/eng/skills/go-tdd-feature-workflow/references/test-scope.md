# Test Scope

Choose the lowest level that proves the criterion without hiding the risk it exists to cover.

## Domain tests

`internal/domain/entity`, `internal/domain/valueobject`.

Use for value object validation, entity invariants, state transitions, and pure rules. No context,
no fakes, no IO — a domain test that needs a fake is a sign the rule belongs in the usecase.

Prove that `NewExampleEntityName("")` returns a `*domain.ValidationError`, that
`Publish()` on an already-published entity returns `*domain.InvariantError`, that
`ParseExampleEntityStatus("bogus")` fails. These are the cheapest and most durable tests in the
project; prefer them whenever the rule is genuinely pure.

## Usecase tests

`internal/usecase/{feature}`.

Use for orchestration, ownership and permission decisions, output shape, and — most importantly —
**error policy**. This is the only level that can prove the rules `coke-eng:go-clean-architecture`
states about error ownership:

- a `repository.ErrNotFound` the usecase has no opinion about becomes `usecase.ErrNotFound`
- a `repository.ErrUniqueViolation` on an idempotency key becomes a success, not a conflict
- a `service.ErrTimeout` becomes `usecase.ErrUnavailable`
- nothing raw leaves `Execute`

Without this level those rules are prose in a document with nothing enforcing them.

Use a hand-written fake for every port. Never a database, never HTTP.

## Handler tests

`internal/handler/router/{surface}`.

Use for route wiring, DTO binding, response body shape, and status codes. Build a `fiber.App` with
the project's real `ErrorHandler`, register the real handler, and drive it with `app.Test`.

Wire the **real usecase with a fake repository** rather than faking the usecase. Two reasons: the
handler holds a concrete `*Usecase`, not an interface, so there is nothing to substitute without
adding an interface that production code does not need; and the real usecase means one test proves
the entire chain — `repository.ErrNotFound` → `usecase.ErrNotFound` → `404` — which is the chain
most likely to be wrong.

Do not restate business rules here. A handler test asserting that an empty name is rejected is
asserting the domain rule for the third time; assert that a `usecase.ErrValidation` surfaces as
`400` with the expected body, once, and let the domain test own the rule itself.

## Repository tests — out of scope

This skill does not drive sqlc/pgx integration tests. Nothing about them is TDD-shaped: the queries
are generated, the mapping is mechanical, and the feedback loop needs a live database.

That leaves a real gap, and the summary must name it. What is *not* covered by anything in this
loop:

- whether the generated SQL matches the query file's intent
- row → entity mapping, including nullable columns and enum parsing
- unique and foreign-key constraints actually firing
- `mapPgError` translating real driver errors rather than hand-built ones
- transaction boundaries and rollback

Verify those separately — against a real Postgres with the project's goose migrations applied — and
record in `templates/test-summary.md` whether that happened.

## Cost and confidence

Domain and usecase tests run in milliseconds and prove nothing about infrastructure. Handler tests
cost a `fiber.App` per test and prove the contract. Pushing behaviour up into handler tests to get
"more realistic" coverage trades fast feedback for slow feedback and gains nothing — the fake
repository is the same fake either way.
