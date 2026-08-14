# Testing Reference Is Out Of Scope

This Go Clean Architecture skill does not define a full testing strategy, test pyramid, fake
repository workflow, integration test strategy, or TDD cycle. `coke-go-tdd-feature-workflow` owns
all of that — test level selection, `_test.go` placement, hand-written fakes, and the
red/green/refactor loop.

The only testing guidance owned by this skill:

- Downstream Go projects using the architecture pattern must pass:

```sh
go test -race ./...
```

- Handler tests use Fiber's in-process `app.Test(req)` with a fake repository behind the real
  usecase — the struct-based handler + constructor injection pattern exists partly to make this
  possible.
- Table-driven tests follow the uber-go conventions (`tests`, `tt`, `give`/`want`).
- Every repository implementation carries its compile-time interface check, so a port change breaks
  the build rather than a test assertion.

Use `coke-go-tdd-feature-workflow` for test design. Note that its loop deliberately excludes
sqlc/pgx integration tests: verify repository behavior against a real database separately.
