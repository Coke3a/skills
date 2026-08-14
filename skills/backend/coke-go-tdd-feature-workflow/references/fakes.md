# Fakes

## Hand-written, not generated

No gomock, mockery, or testify mocks in this stack.

The generated-mock argument is that writing fakes is tedious. In this architecture it is not:
repository ports have four or five methods, and the fake is thirty lines that get written once per
feature. What generation costs in exchange is worth more:

- **The fake is the design feedback.** Writing it by hand is the moment the port's shape is felt. A
  fake that is awkward to write is a port that is awkward to implement, and that signal is the point
  of the exercise. A generated mock hides it.
- **Expectation DSLs test the wrong thing.** `EXPECT().Create(gomock.Any()).Times(1)` asserts that a
  call happened, not that the behaviour is right. Tests written that way break on refactors that
  changed nothing observable.
- **A generated mock tracks the interface automatically**, which sounds good and means a port can
  grow a method nobody noticed. The compile-time check below makes that failure loud instead.
- **One less dependency and one less code-generation step** in a project that already runs `sqlc
  generate`.

If the project already standardises on a mocking library, follow the project. Do not introduce one
here.

## Shape

```go
var _ repository.ExampleRepository = (*fakeExampleRepository)(nil)

type fakeExampleRepository struct {
	// Stubbed results — what the fake returns.
	createErr  error
	findResult *entity.ExampleEntity
	findErr    error

	// Recorded calls — what the fake was asked to do.
	created []*entity.ExampleEntity
}
```

Two halves, always. **Stubs** drive the branch under test; **records** let the test assert the side
effect happened. Full version in `templates/fake_repository.go`.

The compile-time check is not decoration. It is what makes a port change break the test file at
build time rather than at some later assertion, which is the same reason
`coke-go-clean-architecture` requires it on the real implementation.

## Rules

- **Return the errors the real repository returns**, wrapped the same way:
  `fmt.Errorf("example_entity.create: %w", repository.ErrUniqueViolation)`. A fake returning a bare
  sentinel, or `errors.New("boom")`, tests a chain that will never exist in production.
- **Zero value is the happy path.** `&fakeExampleRepository{}` succeeds at everything. Every test
  then sets only the one field its case is about, and reads as a description of that case.
- **No logic.** A fake with an `if` on its input is a second implementation of the thing under test,
  and it will drift. Stub the result; do not compute it.
- **In-memory storage only when the test needs read-after-write** — a usecase that writes then reads
  back. Otherwise a `[]*entity.ExampleEntity` record slice is enough.
- **Add a mutex only if a test actually runs the fake concurrently.** `t.Parallel()` on sibling
  subtests does not: each builds its own fake. A usecase spawning goroutines does.
- **Never fake the usecase.** Handler tests wire the real usecase with a fake repository — see
  `references/test-scope.md`.

## Service ports

Ports in `internal/domain/service` (token services, payment clients, notification dispatchers) get
the same treatment. Their sentinels matter more than repositories' do, because the usecase usually
*does* have an opinion about them:

```go
type fakeTokenService struct {
	issued   string
	issueErr error
	claims   service.AccessClaims
	verifyErr error
}
```

A test that sets `verifyErr: fmt.Errorf("jwt.verify: %w", service.ErrTokenExpired)` and asserts
`usecase.ErrUnauthenticated` comes out is proving the policy the design decided on. That test is the
reason the port exists.

## Fixture builders

Once three tests build the same entity, extract a builder rather than repeating the constructor:

```go
func newTestEntity(t *testing.T, opts ...func(*entity.ExampleEntity)) *entity.ExampleEntity {
	t.Helper()
	// ...
}
```

Keep them in the same `_test.go` file as the fake. A builder that grows more than two or three
options is describing a construction problem in the entity itself.
