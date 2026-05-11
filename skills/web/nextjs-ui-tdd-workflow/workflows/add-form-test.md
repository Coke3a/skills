# Add Form Test

Use this workflow for form validation, submission, pending, success, and error behavior.

1. List user-visible form requirements: labels, required fields, validation messages, submission
   result, pending/disabled state, and server errors.
2. Prefer a Testing Library component/integration test for client form behavior.
3. Query fields by label or role.
4. Use `userEvent.type`, `userEvent.click`, `userEvent.tab`, and realistic selection helpers.
5. Submit like a user: click the submit button or press Enter when relevant.
6. Assert field errors with visible text and accessible relationships when present.
7. Assert pending/disabled state when it is part of the contract.
8. Mock API/Server Action outcomes at the boundary for component tests.
9. Use Playwright only when the full route, browser, auth, or server behavior must be validated
   together.

Do not assert internal form library state unless the state is exposed through user-visible behavior.
