# Test File Placement

Go has one placement rule and it is not negotiable: a test file sits beside the code it tests, named
`<file>_test.go`. There is no `tests/` directory in a Go project.

| Code under test                                | Test file                                                    |
| ---------------------------------------------- | ------------------------------------------------------------ |
| `internal/domain/entity/example_entity.go`     | `internal/domain/entity/example_entity_test.go`               |
| `internal/domain/valueobject/example_name.go`  | `internal/domain/valueobject/example_name_test.go`            |
| `internal/usecase/examplefeature/create.go`    | `internal/usecase/examplefeature/create_test.go`              |
| shared fake for that usecase package           | `internal/usecase/examplefeature/fake_test.go`                |
| `internal/handler/router/publicapi/example.go` | `internal/handler/router/publicapi/example_test.go`           |
| fixture files (golden JSON, SQL, payloads)     | `testdata/` beside the test, ignored by the toolchain         |

## External test package

Default to `package examplefeature_test`, not `package examplefeature`.

The external package can only reach exported identifiers, which is exactly the constraint that makes
the test useful as a design signal: it is written the way a real caller writes it. If a test can
only be expressed from inside the package, the thing it needs is either missing from the API or does
not belong to the package.

Both packages may live in the same directory — Go allows `foo` and `foo_test` side by side — so
adopting this costs nothing.

Use the internal package (`package examplefeature`) only for testing an unexported helper whose
behaviour is genuinely intricate on its own: a parser, a retry-backoff calculation, a mapping table.
Treat each occurrence as a question worth asking rather than a habit.

Domain packages are the common exception in practice. Entities use unexported fields with getters,
so `package entity_test` works fine for anything reachable through `New...`, `...FromExisting`, and
the getters — which is everything a caller can do. Stay external there too.

## Fakes

A fake used by one package lives in `fake_test.go` in that package, unexported. It compiles only
under test and costs the production binary nothing.

When a second package needs the same fake — usually when the handler test wants the repository fake
the usecase test already has — promote it to a real package rather than copying:

```
internal/domain/repository/repotest/example_repository.go
```

The `repotest` name follows the standard library's own convention for this
(`net/http/httptest`, `testing/fstest`, `testing/iotest`). Do not promote before the second caller
exists.

## Naming inside the file

- `TestCreateExampleEntity_Success`, `TestCreateExampleEntity_ErrorMapping` — the function names the
  unit and the aspect.
- Subtest names inside `t.Run` are lowercase sentences describing the case:
  `"unique violation becomes conflict"`, not `"case2"`.
- Helpers are `newTestUsecase`, `mustName`, `newTestApp` — `t.Helper()` in each.

## What must not appear

- a `tests/`, `test/`, or `test_process/` directory
- `_test.go` files that are not beside their subject
- test-only exported identifiers in production packages (`ExportedForTesting`, `SetClockForTest`) —
  inject the dependency through the constructor instead
- `init()` in a test file — use `TestMain` if setup is genuinely package-wide, and prefer `t.Cleanup`
