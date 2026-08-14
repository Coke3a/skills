# Playwright Patterns

Sources:

- Official: https://playwright.dev/docs/best-practices
- Community inspiration only: https://skillsauth.com/skills/absolutelyskilled/playwright-testing

## Locator Priority

Prefer locators that match user-visible contracts:

```ts
await page.getByRole("button", { name: /sign in/i }).click();
await expect(page.getByRole("heading", { name: /dashboard/i })).toBeVisible();
```

Use CSS/XPath only when there is no stable user-facing contract.

## Web-First Assertions

Prefer Playwright assertions that wait automatically:

```ts
await expect(page.getByText(/saved/i)).toBeVisible();
await expect(page.getByRole("button", { name: /submit/i })).toBeEnabled();
await expect(page).toHaveURL(/dashboard/);
```

Avoid manual assertions like `expect(await locator.isVisible()).toBe(true)` when a web-first
assertion exists.

## Isolation

- Each test owns its data setup.
- Each test owns its session/storage setup.
- Do not depend on test order.
- Do not reuse mutable records unless the project provides reset/transaction isolation.
- Use storage state carefully and understand what it covers in the project.

## Network and Third Parties

Test what the project controls. Mock, route, or stage external dependencies instead of testing
third-party sites or services directly.

## When E2E Is Worth It

Use Playwright for:

- Auth journeys.
- Critical conversion or checkout-like flows.
- Route redirects and navigation behavior.
- Browser APIs.
- Async Server Component page output.
- Full form submission through server behavior.

Do not use Playwright to duplicate every component state covered by Testing Library.
