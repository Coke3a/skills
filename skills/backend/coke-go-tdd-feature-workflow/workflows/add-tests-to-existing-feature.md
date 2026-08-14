# Characterise an Existing Feature

Use when the behaviour already works and is about to change. The goal is a safety net that describes
what the code does **today**, so the diff afterwards shows what actually changed.

This is not the design loop. The entry gate does not apply — there is nothing left to design — and
the tests are expected to pass on their first run.

## 1. Establish what "today" is

- [ ] Read the usecase end to end, then the entity and the handler it sits between.
- [ ] List the observable behaviours: what it returns on success, which errors it produces, which
      repository calls it makes, what it writes.
- [ ] Note behaviour that looks like a bug. Do not fix it yet — write the test that captures it as it
      is, mark it, and raise it separately. Fixing during characterisation destroys the baseline.
- [ ] Confirm the current test state: `go test -race ./...`. A suite that is already red cannot serve
      as a baseline.

## 2. Cover the change surface first

- [ ] Start with the behaviours the upcoming change will touch. Full coverage of untouched code is
      not the goal and costs more than it returns.
- [ ] Usecase level first — it is where behaviour is densest and cheapest to pin.
- [ ] Add the handler test only if the HTTP contract is changing.
- [ ] Add a domain test only if the rule itself is changing.

## 3. Write the tests

- [ ] Same rules as the design loop: `package <pkg>_test`, hand-written fakes, `errors.Is`,
      table-driven, `t.Parallel()`.
- [ ] Assert what the code does, including the parts you disagree with.
- [ ] Expect green on the first run — that is correct here.

## 4. Prove they mean something

- [ ] Break the implementation deliberately, one behaviour at a time: invert a condition, return the
      wrong sentinel, drop a repository call.
- [ ] Confirm the matching test fails.
- [ ] Revert.

A characterisation test that never failed has proven nothing. This step is the substitute for the
red phase and is not optional.

## 5. Record and hand off

- [ ] `go test -race ./...` green.
- [ ] List the behaviours now protected, and the ones deliberately left uncovered.
- [ ] List the suspected bugs found and captured as-is, with file:line.
- [ ] Now make the change — from here, `workflows/tdd-feature.md` step 2 onward applies to each new
      criterion.
