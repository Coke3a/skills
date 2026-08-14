# Test Summary — <feature name>

## Criteria

| ID   | Behaviour | Level   | Test                                     | Status |
| ---- | --------- | ------- | ---------------------------------------- | ------ |
| AC-1 |           | usecase | `TestCreateExampleEntity_Success`        | green  |
| AC-2 |           | domain  | `TestNewExampleEntityName/rejects_empty` | green  |

Every row must have failed for its expected reason before its implementation existed. If any did
not, say which and why.

## Design decided by the tests

The point of the loop — what the overview did not specify and the tests settled:

- **Ports discovered:** methods now on `repository.ExampleRepository` / `service.<X>`, and which
  criterion asked for each.
- **Signatures settled:** `Execute` input and output shape, and anything that changed after the
  first call site was written.
- **Rules placed:** which behaviour ended up on the entity, which on the usecase, and why.
- **Error policy:** which errors the usecase has an opinion about, and which fall through to
  `ConvertError`.
- **Rejected along the way:** designs tried and abandoned because the test was awkward to write.
  This is the most useful section at review time.

## Files

- Domain: `<paths>`
- Usecase: `<paths>`
- Handler: `<paths>`
- Fakes: `<paths>` — promoted to `repotest`? yes/no

## Commands

```sh
go build ./...
go vet ./...
go test -race ./...
golangci-lint run
```

Paste the real result. Do not report a command that was not run.

## Not covered

Required section. An unstated gap reads as coverage.

- **Repository integration** — sqlc query correctness, row → entity mapping, nullable columns, enum
  parsing, unique and foreign-key constraints, `mapPgError` against real driver errors, transaction
  boundaries. Verified separately against a real database: yes/no, and how.
- **Behaviour left untested** — what, and why it was an acceptable call.
- **Criteria deferred** — what was cut from scope and who agreed.

## Risks and follow-up

- Entry-gate overrides carried through, and the assumptions still unverified.
- Suspected bugs found and captured as-is rather than fixed, with `file:line`.
- Anything the next change should test first.
