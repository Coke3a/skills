# Accessibility Testing

Sources:

- Official: https://testing-library.com/docs/guiding-principles/
- Official: https://playwright.dev/docs/best-practices

## Why It Matters

Accessible queries are usually better tests because they match how users and assistive technologies
discover UI.

## Query Guidance

Prefer:

- Buttons and links by role and accessible name.
- Form controls by label.
- Headings by role and name.
- Alerts/status messages by role when the app exposes them.

Examples:

```tsx
screen.getByRole("button", { name: /save/i });
screen.getByLabelText(/email/i);
screen.getByRole("heading", { name: /settings/i });
```

```ts
page.getByRole("button", { name: /save/i });
page.getByLabel(/email/i);
```

## What Not to Do

- Do not add test ids to avoid fixing inaccessible markup.
- Do not query by placeholder if the field should have a label.
- Do not assert CSS class names for errors when visible text or accessible state is available.

If accessible queries are impossible, consider whether the UI needs accessible markup changes.
