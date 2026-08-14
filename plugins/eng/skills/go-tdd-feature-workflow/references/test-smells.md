# Test Smells

Each entry is a signal that the design, not the test, needs changing.

## Design smells

**The test needs the internal package.** Something the caller needs is unexported, or the behaviour
belongs somewhere else. Move it or export it — do not switch the test to `package foo`.

**Setting up the usecase takes twenty lines.** Too many dependencies. Either the usecase is doing
two jobs, or several ports should be one.

**The fake needs an `if`.** The test is asking the fake to make a decision, which means the decision
is in the wrong place — it belongs in the usecase under test.

**A handler test is hard to write.** The handler is holding logic. Push it into the usecase; a
handler test should only bind, call, and map.

**A domain test needs a fake.** The rule depends on IO, so it is not a domain rule. It is a usecase
rule.

**Only reachable through the HTTP layer.** A rule that can only be exercised via `app.Test` lives in
the handler. Move it down.

**A port method exists that no usecase calls.** It was written from the schema instead of discovered
from a test. Delete it.

## Assertion smells

**Comparing error strings.** `err.Error() == "not found"` breaks the moment a wrapper adds context,
which is exactly what `%w` wrapping is for. Use `errors.Is` / `errors.As`.

**Asserting the raw sentinel that escaped.** A test asserting `errors.Is(err, repository.ErrNotFound)`
on a usecase result is asserting the bug, not the contract. The usecase must return
`usecase.ErrNotFound`.

**No leak assertion on error tests.** Half the contract is unproven. See `references/go-test-patterns.md`.

**Asserting call counts nothing depends on.** `if repo.calls != 1` fails when an equivalent
implementation reorders or memoises. Assert the observable effect instead.

**Asserting on unexported fields via reflection.** The test has given up on the API.

**A test with no assertion.** It proves the code does not panic. Say that explicitly or delete it.

**Asserting `err != nil` and nothing else.** Any failure passes, including the wrong one.

## Structure smells

**The same rule asserted at three layers.** Triple maintenance, single confidence. The rule belongs
to one level; the others assert their own translation of it.

**A table row with a `setup func()` column.** It stopped being a table. Split into functions.

**Shared mutable state between tests.** Package-level variables, a fake built once in `TestMain`,
`t.Setenv` in a parallel test. All three forbid `t.Parallel()`, and the reason they were introduced
is usually expensive setup that should have been made cheap.

**Tests that must run in order.** State leaked between them. Nothing about `go test` guarantees the
order, and `-shuffle` will find it.

**`time.Sleep` in a test.** Either flaky or slow, usually both. Use channels, `t.Cleanup`, or an
injected clock.

**Golden files nobody reads.** A `testdata/*.golden` regenerated whenever it fails asserts that the
output equals itself. Golden files are for output a human reviews on change.

## Process smells

**The test passed the first time it ran.** It was written after the code, so it proves the code
matches itself. Break the implementation deliberately and confirm the test catches it.

**Red for the wrong reason, then green.** The failure was a fixture typo or a pre-existing break; the
criterion under design was never actually red.

**The implementation grew past the criterion.** Generality nothing asked for. In this architecture
it propagates: unneeded usecase branch → unneeded port method → unneeded query.

**Tests edited to keep a refactor green.** That was not a refactor, it was a behaviour change.

**All tests written, then all code.** Test-first waterfall. See `references/tdd-principles.md`.
