# Test File Placement

## Rule

Follow the existing project convention. Do not introduce a second convention because the template
prefers one.

## Supported Component/Hook Patterns

Colocated:

```text
features/
  auth/
    components/
      LoginForm.tsx
      LoginForm.test.tsx
```

Feature `__tests__`:

```text
features/
  auth/
    components/
      LoginForm.tsx
    __tests__/
      LoginForm.test.tsx
```

## E2E Patterns

Use the existing convention:

```text
tests/
  e2e/
    login.spec.ts
```

or:

```text
e2e/
  login.spec.ts
```

## Monorepos

For `apps/web`, place UI tests inside the web app unless the repo already centralizes tests:

```text
apps/
  web/
    features/
      auth/
        components/
          LoginForm.test.tsx
    tests/
      e2e/
        login.spec.ts
```

Shared test utilities should follow the project convention, commonly `test-utils`, `tests/utils`, or
`apps/web/tests/utils`.

## Mock Placement

Use `tests/mocks` or `src/mocks` depending on existing convention. Keep mock data contract-shaped
and close to the tests that need it unless it is reused broadly.
