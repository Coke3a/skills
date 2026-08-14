# TDD Principles

## The tests are the design

The value of writing the test first is not the test. It is the twenty seconds spent deciding what to
type into it — the call site is written before the thing it calls exists, so the design is judged as
a *caller* experiences it rather than as its author imagines it. Write the code first and the test
can only agree with whatever was built.

Concretely, in this architecture the test decides:

- what `Execute` takes and returns, and whether the Input struct is a pleasant thing to fill in
- which methods the repository port actually needs
- which errors the usecase has an opinion about and which it forwards
- where a value object earns its existence versus where a plain `string` is fine
- whether a behaviour belongs to the entity, the usecase, or neither

## The repository interface is discovered

This is the rule that makes this skill worth running alongside `coke-go-clean-architecture`.

The tempting order is: design the table, write the interface, write the sqlc queries, then the
usecase. That produces a port shaped by the schema — `FindAll`, `FindByStatus`,
`FindByOwnerAndStatus` — and a usecase that bends around it.

The order here is inverted. Write the usecase test. It needs a fake. The fake needs exactly the
methods the usecase calls, with exactly the arguments the usecase has on hand. That set *is* the
port. Then `internal/infra/postgres` is written to satisfy it, and the SQL serves the domain instead
of the reverse.

A method appears in `repository.ExampleRepository` when a usecase calls it, and not before.

## One criterion at a time

Writing every test up front then implementing them all is test-first waterfall. It fails the same
way waterfall always does: thirty tests get written against an API nobody has used yet, the first
real implementation shows the API is wrong, and now thirty tests need rewriting. The feedback loop
that justifies the whole practice is gone.

So: one criterion, red, green, refactor, next. Two or three criteria in, the shape stops moving and
the rest becomes mechanical. That settling *is* the deliverable — the structure the design overview
was missing.

## Red means red for the right reason

A test that fails because a package does not compile is red. A test that fails because of a typo in
the fixture, or because something unrelated was already broken, is not — it proves nothing when it
later turns green.

Read the failure output before writing the implementation. If the message is not the one the test
was written to produce, fix the test first.

## Green means smallest

Write the least code that turns this test green. Not the general version, not the version that also
handles the next criterion, not the version with the extra guard clause for a caller that does not
exist yet. The next criterion will ask for what it needs, and if it never asks, the code was never
needed.

This matters more than usual here, because unneeded generality in a usecase becomes an unneeded
method on a port, which becomes an unneeded query, which becomes an index nobody uses.

## Refactor is not optional and not separate

Refactor while green, before the next criterion. Both production and test code. The duplication that
shows up across the second and third test is information about the design — usually that a fixture
builder is missing, or that two criteria are actually one.

Never change behaviour during a refactor. If the tests need editing to keep passing, that was not a
refactor.

## Behaviour, not implementation

Assert what a caller can observe: the returned value, the returned error, the recorded side effect
on the fake. Do not assert on private state, call counts that no behaviour depends on, or the order
of independent operations. Those assertions break during refactors that changed nothing a user
could see, and their only lesson is to stop refactoring.

The exception worth making: when the *point* of the criterion is that a side effect happened
(the session row was written, the email was dispatched once), asserting on the fake's record is
asserting on behaviour.

## Coverage is not the goal

Do not add a test level because the level exists. Each level earns its place by proving something
the level below cannot:

- the domain test proves the rule
- the usecase test proves the rule is applied and the error is translated
- the handler test proves the translation reaches the wire as the right status code

Proving the same validation rule at all three is triple maintenance for single confidence.
