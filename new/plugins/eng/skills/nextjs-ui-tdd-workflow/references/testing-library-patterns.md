# Testing Library Patterns

Sources:

- Official: https://testing-library.com/docs/guiding-principles/
- Community inspiration only:
  https://agent-skills.md/skills/itechmeat/llm-code/react-testing-library
- Community inspiration only:
  https://agent-skills.md/skills/pluginagentmarketplace/custom-plugin-react/react-testing-library

## Query Priority

Prefer user-centered queries:

1. `screen.getByRole('button', { name: /save/i })`
2. `screen.getByLabelText(/email/i)`
3. `screen.getByPlaceholderText(/search/i)` when placeholder is the only user-facing label
4. `screen.getByText(/saved/i)` for visible copy
5. `screen.getByDisplayValue('Example')`
6. `screen.getByTestId('example')` only when accessible queries are not stable or expressive enough

Use:

- `getBy*` when the element must already be present.
- `findBy*` when the element appears asynchronously.
- `queryBy*` when asserting absence.

## user-event

Use `userEvent.setup()` and realistic interactions:

```tsx
const user = userEvent.setup();
await user.type(screen.getByLabelText(/email/i), "user@example.com");
await user.click(screen.getByRole("button", { name: /submit/i }));
```

Use `fireEvent` only for low-level events that user-event does not support well.

## Async UI

Prefer `findBy*` for appearance and `waitFor` for state changes that do not map cleanly to one
element query.

Avoid arbitrary sleeps. Wait for user-visible outcomes.

## Smells

- Querying by CSS class or DOM structure.
- Asserting React state or hook internals.
- Mocking the component under test.
- Testing a third-party component's behavior instead of your integration with it.
- Large tests that cover multiple unrelated behaviors.
