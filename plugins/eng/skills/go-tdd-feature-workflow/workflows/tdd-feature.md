# Implement a Feature with TDD

## 0. Entry gate — hard stop

- [ ] Read the design source end to end.
- [ ] Quote the endpoint list (method + path) for every endpoint in scope.
- [ ] Quote the request and response shapes, with field names and types.
- [ ] Quote the tables and columns this feature reads or writes.
- [ ] Quote one sentence stating the feature's purpose.
- [ ] Check `internal/usecase/{feature}/` — is this already implemented?
- [ ] If anything is missing: stop, report per `references/entry-gate.md`, write nothing.
- [ ] If it is already implemented: switch to `workflows/add-tests-to-existing-feature.md`.

## 1. Derive acceptance criteria

- [ ] Success case per endpoint.
- [ ] Validation failures — which field, which rule.
- [ ] Not found, conflict, and permission cases.
- [ ] Dependency failure — repository unavailable, external service rejected or timed out.
- [ ] Idempotency or retry behaviour, if the design implies any.
- [ ] Write them into `templates/acceptance-criteria.md`.
- [ ] Confirm the list with the user before writing tests. These are the design decisions the
      overview did not make; they are not yours to assume silently.

## 2. Pick one criterion

- [ ] One. Not the whole list.
- [ ] Start with the simplest success path — it forces the Input/Output shape into existence.
- [ ] Choose the smallest useful level (`references/test-scope.md`).
- [ ] Choose the file and package (`references/test-file-placement.md`).

## 3. Red

- [ ] Write the test against the API you wish existed, not the one that exists.
- [ ] Arrange / Act / Assert, one behaviour.
- [ ] If it is a usecase test, build the fake by hand — the methods it needs are the port.
- [ ] Run the narrowest command: `go test -race ./internal/<pkg>/`.
- [ ] Confirm it fails for the expected reason. A compile error counts.
- [ ] If the failure is a fixture mistake or a pre-existing break, fix that first.
- [ ] If writing the test felt awkward, change the design before continuing.

## 4. Green

- [ ] Add only what this test needs: the port method, the entity behaviour, the usecase branch.
- [ ] Smallest implementation. No extra guard clauses, no generality nothing asked for.
- [ ] No unrelated tests, no unrelated abstractions.
- [ ] Run the same command until green.

## 5. Refactor

- [ ] Only while green.
- [ ] Remove duplication in production and test code both.
- [ ] Extract a fixture builder once three tests build the same thing.
- [ ] Improve names.
- [ ] Do not change behaviour. If a test needed editing, that was not a refactor — revert and redo.

## 6. Repeat

- [ ] Next criterion, back to step 2.
- [ ] Error-mapping criteria collapse into one table-driven test per usecase.
- [ ] Add the handler test once the usecase contract has stopped moving — usually after the success
      path and one error path are green.
- [ ] Do not restate a domain rule at the usecase or handler level.

## 7. Close the layers this loop does not cover

- [ ] Write the sqlc queries and the `internal/infra/postgres` implementation against the port the
      tests discovered — use `coke-eng:go-clean-architecture`.
- [ ] Confirm the compile-time interface check exists on the real implementation.
- [ ] Verify persistence against a real database with the project's golang-migrate migrations. This is not
      part of the TDD loop; it is the gap the loop leaves.

## 8. Final verification

- [ ] `go build ./...`
- [ ] `go vet ./...`
- [ ] `go test -race ./...`
- [ ] `golangci-lint run`, if the project has a config.
- [ ] Walk the architecture checklist in `coke-eng:go-clean-architecture` → `references/architecture.md`.
- [ ] Write the summary from `templates/test-summary.md`, including what is not covered.
