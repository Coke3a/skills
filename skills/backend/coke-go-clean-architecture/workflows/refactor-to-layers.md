# Refactor mixed code into clean layers

Use this when handler/business/IO code is mixed (fat handlers, SQL in handlers, business rules in
repositories) and needs to move into clean layers without changing behavior.

## 1. Map the current code

- [ ] List what the mixed code does: HTTP parsing, validation, business decisions, SQL, external IO
- [ ] Identify the entities, value objects, and invariants hiding in the code
- [ ] Identify the persistence operations hiding in inline SQL or query-builder calls
- [ ] Note existing tests; they define the behavior that must not change

## 2. Extract downward, one layer at a time

Work domain-first so each extraction has a stable target:

- [ ] Create value objects for validated fields currently checked inline
- [ ] Create the entity with unexported fields and move invariant checks into
      constructors/transitions
- [ ] Define the repository interface in `internal/domain/repository` from the persistence
      operations actually used
- [ ] Move SQL into `db/queries/{entity}.sql`, run `sqlc generate`, and implement the interface in
      `internal/infra/postgres`
- [ ] Create the usecase; move orchestration and user-facing error decisions into `Execute`
- [ ] Shrink the handler to bind → map → call → map → return

## 3. Fix the error flow

- [ ] Replace inline status-code decisions with usecase sentinels + the central `apierror.Handler`
- [ ] Replace `(nil, nil)` "not found" returns with wrapped `repository.ErrNotFound`
- [ ] Remove any `panic`/`log.Fatal` outside `main()`
- [ ] Ensure each error is handled once (no log-and-return)
- [ ] Move logging out of repositories and infra; keep it on the usecase paths that handle an error
      and at the edge
- [ ] Move retry, fallback, and default-value decisions out of repositories into the usecase
- [ ] Ensure no raw domain, repository, or service error escapes a usecase `Execute`

## 4. Fix lifecycle and context

- [ ] Thread `context.Context` from the handler through the usecase into the repository
- [ ] Give any bare `go func()` an owner with stop signal and exit wait, or remove it
- [ ] Copy any fasthttp-backed value (`c.Params`, `c.Query`, `c.Body`) that escapes the handler

## 5. Verify

- [ ] Architecture checks in `references/architecture.md` pass
- [ ] Behavior unchanged: existing tests pass without modification (or changes are called out)
- [ ] `go build ./...`, `go vet ./...`, `go test -race ./...` pass
