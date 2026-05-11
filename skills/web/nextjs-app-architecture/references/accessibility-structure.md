# Accessibility Structure

Sources synthesized:
- https://github.com/vercel-labs/agent-skills/blob/main/skills/web-design-guidelines/SKILL.md
- https://raw.githubusercontent.com/vercel-labs/web-interface-guidelines/main/command.md

## Semantic HTML

- Use semantic landmarks and elements before ARIA.
- Use `button` for actions and links for navigation.
- Avoid `div` or `span` click handlers.
- Keep headings hierarchical.
- Include a skip link for substantial app shells when appropriate.

## Focus And Keyboard

- Provide visible `focus-visible` styles for interactive elements.
- Never remove outlines without an accessible replacement.
- Ensure menus, dialogs, drawers, tabs, and custom controls support keyboard interaction.
- Manage focus on dialog open/close and validation failure.

## Forms

- Every control needs a label or accessible name.
- Field errors should be near the field and programmatically associated when practical.
- Use meaningful `name`, `type`, `inputMode`, and `autoComplete`.
- Do not block paste.
- Keep checkbox/radio labels and controls in one hit target.

## Dynamic Updates

- Use `aria-live="polite"` for async validation, toast, or save status updates when screen reader users need the update.
- Do not overuse ARIA where semantic HTML communicates state.

## Images And Media

- Informative images need useful alt text.
- Decorative images use empty alt text.
- Icons inside icon-only buttons need an accessible button label and usually `aria-hidden` on the icon.
- Use explicit dimensions for media to avoid layout shift.

## Interaction And Layout

- Preserve touch targets.
- Prevent drawer/modal background interaction where appropriate.
- Respect reduced motion preferences for motion-heavy UI.
- Make responsive layouts work without JS measurement where possible.
