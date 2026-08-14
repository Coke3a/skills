# UI Test Smells

## Test Smells

- The test knows private state variable names.
- The test reaches into component instances.
- The test asserts CSS classes that are not the user contract.
- The test uses `container.querySelector` without a strong reason.
- The test mocks the component under test.
- The test duplicates the same behavior at unit, component, integration, and E2E levels.
- The test checks a third-party library instead of the app's integration with it.
- The test depends on test order or shared mutable data.
- The test uses arbitrary sleeps.
- The test has broad setup but one small assertion.
- The test name describes implementation instead of user behavior.

## Better Patterns

- Query like a user.
- Assert visible output and accessible state.
- Mock only boundaries.
- Isolate data/session/storage.
- Use async utilities and web-first assertions.
- Keep one behavior per test.

## Test Naming

Prefer:

- `shows a validation error when email is missing`
- `disables submit while saving`
- `redirects signed-out users to login`

Avoid:

- `sets error state`
- `calls handleSubmit`
- `renders correctly`
