# Diagnose a Failing Test

## 1. Read the failure before touching anything

- [ ] Read the actual output. Not the test name — the message, the file:line, the diff.
- [ ] Compile error, assertion failure, panic, race, or timeout? Each has a different cause.
- [ ] Isolate it: `go test -race -run 'TestName/subtest_name' ./internal/<pkg>/ -count=1`.
- [ ] `-count=1` matters — a cached result is not a run.

## 2. Decide which side is wrong

The failure means the test and the code disagree. Establish which one is right *before* editing
either. Changing the test to match the code is the single most common way a suite stops being worth
running.

| Signal                                                     | Likely wrong |
| ---------------------------------------------------------- | ------------ |
| The test states a criterion the user confirmed              | the code     |
| The test was just written and has never passed              | either — reread the criterion |
| The test passed before this change and the change was a refactor | the code |
| The test asserts an implementation detail, not behaviour    | the test     |
| The test asserts an error string, a call count, or ordering | the test     |
| The criterion itself turned out to be wrong                 | the criterion — go back to the user |

## 3. Common causes in this stack

- **Wrong sentinel out of `Execute`.** The usecase forgot `ConvertError`, or matched with `errors.Is`
  in the wrong order. Check that the specific opinions come before the fallback.
- **The leak assertion fired.** A raw `repository.Err*` or `service.Err*` reached the caller. The
  usecase returned it without converting.
- **A `500` where a `404` was expected.** The error reached `apierror.Handler`'s default branch,
  which means it was not a `usecase.Err*`. Same cause as above, seen from the edge.
- **The fake returns a bare sentinel.** Production wraps with `%w` and context; a fake returning
  `repository.ErrNotFound` unwrapped tests a chain that does not exist. Wrap it the same way.
- **Race detector fires.** Two subtests share a fake, or a worker goroutine outlives the test. Give
  each subtest its own fake; make the worker's `Stop()` wait.
- **Handler test gets the wrong status.** The test app was built without the project's real
  `ErrorHandler`.
- **`app.Test` times out.** Default is one second. Either the handler is doing real IO it should not,
  or a fake blocks. Raise the timeout only after ruling both out.
- **Passes alone, fails in the package.** Shared state. Look for package-level variables, `TestMain`
  setup, `t.Setenv`, or a fake built outside the subtest.
- **Flaky.** `time.Sleep`, real clock, map iteration order, or a missing `t.Cleanup`. Run
  `go test -race -count=20 -shuffle=on` to confirm before hunting.

## 4. Fix one thing

- [ ] One cause at a time. A batch of speculative edits makes the next failure unreadable.
- [ ] Re-run the isolated command.
- [ ] Then the package.
- [ ] Then `go test -race ./...`.

## 5. If the test was wrong

- [ ] Fix it, then **prove it still catches the bug**: break the implementation, confirm red, revert.
- [ ] A test relaxed until it passes is worse than no test — it reports safety that is not there.

## 6. If a criterion was wrong

- [ ] Stop. Take it back to the user with what the code does and what the criterion said.
- [ ] Do not silently pick whichever is easier to implement.
