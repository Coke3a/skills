# Form Testing

## Behaviors to Consider

- Labels and accessible names.
- Required fields.
- Field-level validation.
- Form-level validation.
- Pending/disabled state.
- Successful submit.
- Server validation error.
- Server failure.
- Reset or retry behavior.

## Testing Library Pattern

```tsx
const user = userEvent.setup()

render(<LoginForm />)

await user.type(screen.getByLabelText(/email/i), 'user@example.com')
await user.type(screen.getByLabelText(/password/i), 'correct horse battery staple')
await user.click(screen.getByRole('button', { name: /sign in/i }))

expect(await screen.findByText(/welcome/i)).toBeVisible()
```

## Assertions

Prefer visible and accessible outcomes:

- Error text is visible.
- Submit button is disabled while pending.
- Success message appears.
- Focus moves only if that is part of the user contract.

Avoid testing form library internals or private state.

## Server Actions

For Server Action-backed forms, component tests can mock the action result for validation and error mapping. Use Playwright for the full form submission flow when server integration is the behavior being protected.
