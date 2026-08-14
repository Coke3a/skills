---
name: go-tdd-feature-workflow
description: Drives test-driven development for Go backend features on Fiber v3 + sqlc/pgx — turns an approved design overview into working code one acceptance criterion at a time through red/green/refactor, letting the tests decide method signatures, repository interfaces, input/output shapes, and control flow. Use after the design overview is settled (endpoint list, request/response shapes, database schema) but before the deep behaviour is written. Hard-stops when that overview is incomplete. Do not use for Rust (use coke-eng:rust-tdd-feature-workflow), for layer structure and naming (use coke-eng:go-clean-architecture), or for CI/CD, performance benchmarking, or code review.
---

# Go TDD Feature Workflow

TDD is a design activity here, not a testing activity. The tests are how the method set, the
repository interface, and the flow get decided — writing them down afterwards proves nothing that
was not already assumed.

## Use this when

- The design overview is approved and the deep behaviour still has to be designed.
- Adding a new behaviour to an existing Go feature.
- Changing usecase, handler, or error-mapping behaviour.
- Adding or changing validation rules or invariants.
- Fixing a bug, starting from a regression test.
- Characterising existing behaviour before refactoring it.

## Do not use this when

- **The entry gate fails** → stop. See `references/entry-gate.md`. This is a hard stop, not a warning.
- The stack is Rust → use `coke-eng:rust-tdd-feature-workflow`.
- Deciding layer structure, package layout, naming, or error taxonomy → use `coke-eng:go-clean-architecture`.
- Proving sqlc queries, pgx mapping, constraints, or transactions → out of scope, see **Scope** below.
- Setting up CI or deployment → no Go CI skill exists yet; write it from the verification commands.
- Producing a review report → no Go review skill exists yet; use the architecture checklist.
- Profiling or benchmarking → use `coke-eng:go-performance-optimization`.

## Entry gate (hard stop)

Before writing a single test, confirm all four, quoting the source:

1. **Endpoints** — method and path for every endpoint in scope.
2. **Request/response shapes** — field names and types for each.
3. **Database schema** — tables and columns this feature reads or writes.
4. **Purpose** — one quotable sentence on what the feature is for.

If any is missing, **stop and report exactly which one and where to get it**. Do not write tests,
do not write code, do not guess. Full procedure and the wording to use: `references/entry-gate.md`.

The gate also fails in the other direction: if the behaviour is already implemented, this is not a
design loop — switch to `workflows/add-tests-to-existing-feature.md`.

## Core rules

- **One acceptance criterion at a time.** Red → green → refactor, then the next. Writing every test
  up front and implementing afterwards is test-first waterfall, not TDD: it commits the whole API to
  an imagined shape before anything has pushed back on it.
- **The repository interface is discovered, not declared.** Write the usecase test first; whatever
  its fake needs is what the port needs. A method nothing calls does not get written.
- **Test through the exported API.** Use `package <pkg>_test`. If a test can only be written from
  inside the package, the design is telling you something.
- **Hand-written fakes only.** No gomock, mockery, or testify mocks. See `references/fakes.md`.
- **Standard library assertions.** `errors.Is`, `errors.As`, plain comparisons, `go-cmp` for structs.
  No testify — unless the project already depends on it, in which case follow the project.
- **Table-driven with `give`/`want`**, `t.Run` subtests, `t.Parallel()` by default, `t.Helper()` in
  helpers, `t.Cleanup` over `defer` for fixtures.
- **Error tests assert both directions:** the right usecase sentinel comes out, *and* the
  lower-layer sentinel does not leak. `ConvertError` joins with `%s`, not `%w`, so a leak is a real
  detectable bug, not a style preference.
- **Never let a test push logic into a handler.** If a handler test is hard to write, the usecase is
  wrong — fix the usecase.
- **Do not prove the same rule at every layer.** Each level earns its place by adding confidence the
  level below cannot give.
- `go test -race ./...` is the standard, never `go test ./...`.

## Scope

| Layer                        | In this loop | Proves                                               |
| ---------------------------- | ------------ | ---------------------------------------------------- |
| Domain entity / value object | Yes          | invariants, validation, state transitions            |
| Usecase                      | Yes          | orchestration, error policy, ports the feature needs |
| Handler / API                | Yes          | HTTP contract, DTO mapping, status codes             |
| Repository (sqlc/pgx)        | **No**       | —                                                    |

Repository integration is deliberately out. Handler tests wire the **real usecase with a fake
repository**, so the whole error chain is proven without a database, and sqlc/pgx mapping is left to
verification outside this loop. The final summary must say so — see `templates/test-summary.md`.

## TDD loop

1. Pass the entry gate.
2. Derive acceptance criteria from the design overview and confirm them with the user
   (`templates/acceptance-criteria.md`). Success, validation, not-found/conflict, permission, and
   dependency-failure cases.
3. Pick **one** criterion.
4. Choose the smallest useful level (`references/test-scope.md`).
5. Place the file (`references/test-file-placement.md`).
6. Write the failing test against the API you *wish* existed.
7. `go test -race ./internal/...` — narrowest package. Confirm it fails for the expected reason.
8. Write the smallest code that passes. Compile errors count as red.
9. Same command until green.
10. Refactor while green — production and test code both.
11. Next criterion. Stop when every criterion is green.

Steps 6–8 are where the design happens. When step 6 is awkward to write, that is the signal to
change the design, not the test.

## Load more detail

| Decision                                        | Reference                          |
| ----------------------------------------------- | ---------------------------------- |
| Whether this skill may run at all               | `references/entry-gate.md`         |
| Why the loop is shaped this way                 | `references/tdd-principles.md`     |
| Choosing the test level                         | `references/test-scope.md`         |
| `_test.go` placement and package choice         | `references/test-file-placement.md`|
| Writing and sharing fakes                       | `references/fakes.md`              |
| Idiomatic Go test patterns for this stack       | `references/go-test-patterns.md`   |
| Test smells to avoid                            | `references/test-smells.md`        |

## Workflows

| Workflow                                     | Use for                                              |
| -------------------------------------------- | ---------------------------------------------------- |
| `workflows/tdd-feature.md`                   | New feature or behaviour from an approved overview    |
| `workflows/add-tests-to-existing-feature.md` | Characterising code that already works, before change |
| `workflows/fix-failing-tests.md`             | Diagnosing a red suite                                |

## Templates

| Template                        | Use for                                                     |
| ------------------------------- | ----------------------------------------------------------- |
| `templates/acceptance-criteria.md` | Criteria list confirmed with the user before step 3      |
| `templates/entity_test.go`      | Domain entity and value object tests                        |
| `templates/fake_repository.go`  | Hand-written fake with stubbed results and recorded calls   |
| `templates/usecase_test.go`     | Orchestration and error-policy tests with a fake repository |
| `templates/handler_fiber_test.go` | HTTP contract tests via `app.Test`, real usecase + fake repo |
| `templates/test-summary.md`     | Final report, including what is deliberately not covered    |

## Related skills

- `coke-eng:go-clean-architecture` — owns layers, naming, error taxonomy, repository/sqlc pattern.
  This skill owns test order, level, placement, and the red/green/refactor loop.
- `coke-eng:flow-spec-review` — validates the design overview this skill's gate depends on.
- `coke-eng:go-performance-optimization` — after correctness, when there is a measured problem.

## Definition of done

- Every acceptance criterion is green at the smallest level that proves it.
- Every test failed for the expected reason before its implementation existed.
- Test files use `package <pkg>_test`; fakes are hand-written; no mocking library was added.
- The repository interface contains only methods some usecase calls.
- Error tests assert the usecase sentinel is present and the repository/service sentinel is not.
- The summary lists what was intentionally left uncovered — repository integration at minimum.

```sh
go build ./...
go vet ./...
go test -race ./...
```

- If the project has `golangci-lint` configured: `golangci-lint run` passes.
