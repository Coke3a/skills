# Design Skill Playbook

This README explains how to use the design-focused skills in this project.

The goal is to separate **design thinking**, **implementation architecture**, **behavior testing**,
and **final UX review** so the agent does not mix responsibilities.

## Design Skills

### 1. `ui-ux-pro-max-skill`

**Role:** design system generator / style intelligence

Use this skill when you need to define or improve the overall design direction before
implementation.

Good use cases:

- Generate a design system for a new product
- Choose visual style direction
- Choose color palette
- Choose typography pairing
- Define landing page structure
- Define dashboard style
- Define component visual variants
- Identify industry-specific UI patterns
- Identify anti-patterns to avoid
- Create a pre-delivery UI checklist

Use this skill before implementing UI when the product does not yet have a clear visual direction.

Example prompt:

```text
Use ui-ux-pro-max-skill to generate a design system for this product.

Product:
- <product name>

Target users:
- <target users>

Page or app type:
- <landing page | dashboard | mobile app | SaaS app | admin panel>

Brand tone:
- <minimal | premium | playful | enterprise | developer-focused | calm | bold>

Constraints:
- Must work well on web and mobile
- Must be accessible
- Must avoid generic AI-looking UI

Output:
- recommended UI pattern
- visual style
- color palette
- typography
- spacing/radius/shadow direction
- component list
- loading/error/empty state guidance
- anti-patterns to avoid
- pre-delivery checklist
```

Expected output:

- Design direction
- Design tokens
- UI style recommendation
- Component style rules
- UX guidelines
- Anti-patterns
- Delivery checklist

Do not use this skill for:

- Next.js folder structure
- Server/Client Component decisions
- UI testing
- CI/CD
- Backend API design
- Final code review

---

### 2. `frontend-design`

**Role:** creative polish / distinctive frontend

Use this skill when the UI already has a basic direction but needs to become more polished,
memorable, and production-grade.

Good use cases:

- Make a page look premium
- Redesign a boring page
- Improve a landing page hero section
- Improve dashboard visual hierarchy
- Add stronger typography and visual rhythm
- Add tasteful motion and micro-interactions
- Make the UI feel less generic
- Create a distinctive aesthetic for a feature or product

Use this skill when the question is not “what is the component architecture?” but “how should this
feel visually?”

Example prompt:

```text
Use frontend-design to improve the visual quality of this page.

Context:
- Product:
- Current page:
- Target users:
- Desired feeling:
- Existing design tokens:
- Existing components:
- Constraints:

Goal:
Make this UI distinctive, production-grade, and cohesive without breaking the existing design system.

Focus on:
- aesthetic direction
- typography
- color usage
- spacing
- visual hierarchy
- motion
- layout composition
- background/details
- premium polish

Do not change app architecture.
Do not invent new design tokens unless necessary.
Do not make the UI generic.
```

Expected output:

- Stronger aesthetic direction
- Improved layout
- Better typography usage
- Better visual hierarchy
- Motion or micro-interaction suggestions
- More polished component composition
- Concrete implementation guidance

Do not use this skill for:

- API contract
- Test workflow
- CI/CD
- Backend logic
- Store release
- Low-level architecture decisions

---

### 3. `web-design-guidelines`

**Role:** final UX/accessibility/design review

Use this skill after UI implementation to review whether the page follows web interface best
practices.

Good use cases:

- Review UI before finishing a feature
- Check accessibility
- Check form UX
- Check focus states
- Check responsive behavior
- Check navigation state
- Check dark mode readiness
- Check touch/click behavior
- Check UX issues in implemented code
- Audit a page against web UI best practices

Use this skill as a quality gate after implementation.

Example prompt:

```text
Use web-design-guidelines to review this implemented UI.

Review scope:
- <file or folder path>

Check:
- accessibility
- semantic HTML
- focus states
- keyboard navigation
- form UX
- loading/error/empty states
- responsive behavior
- touch targets
- visual hierarchy
- dark mode readiness
- motion and animation behavior

Output actionable findings only.
Use file/path references when possible.
Prioritize issues by severity.
```

Expected output:

- UX findings
- Accessibility findings
- Responsive issues
- Form/focus issues
- Visual consistency issues
- Actionable fixes

Do not use this skill for:

- Generating the whole design system
- Creative visual exploration
- Next.js architecture
- TDD test workflow
- Backend review

---

## Recommended Design Workflow

Use the skills in this order.

### New product or new major page

```text
1. ui-ux-pro-max-skill
   → define design system and style direction

2. frontend-design
   → make the design distinctive and polished

3. coke-nextjs-app-architecture
   → implement using maintainable Next.js structure

4. coke-nextjs-ui-tdd-workflow
   → test user-visible behavior

5. web-design-guidelines
   → final UX/accessibility review
```

### Existing page that looks too generic

```text
1. frontend-design
   → improve visual quality and distinctiveness

2. web-design-guidelines
   → review accessibility and UX quality

3. coke-nextjs-app-architecture
   → refactor implementation only if structure is messy
```

### Existing app with inconsistent UI

```text
1. ui-ux-pro-max-skill
   → generate or normalize design system

2. frontend-design
   → apply stronger visual language to key screens

3. web-design-guidelines
   → audit final UI consistency and accessibility
```

### Before shipping a UI feature

```text
1. coke-nextjs-ui-tdd-workflow
   → ensure behavior is tested

2. web-design-guidelines
   → ensure UX/accessibility quality

3. app-code-review
   → ensure app code quality
```

---

## Skill Responsibilities

| Skill                     | Main Responsibility                               | Use When                                                                      |
| ------------------------- | ------------------------------------------------- | ----------------------------------------------------------------------------- |
| `ui-ux-pro-max-skill`     | Design system generator / style intelligence      | You need design direction, tokens, style, palette, typography, layout pattern |
| `frontend-design`         | Creative polish / distinctive frontend            | You want the UI to look premium, memorable, and less generic                  |
| `web-design-guidelines`   | Final UX/accessibility/design review              | You already implemented UI and need review findings                           |
| `coke-nextjs-app-architecture` | Next.js structure and implementation architecture | You need App Router, Server/Client boundaries, component structure            |
| `coke-nextjs-ui-tdd-workflow`  | UI behavior testing                               | You need tests for forms, loading, error, navigation, critical journeys       |

---

## When to Use Which Skill

### Use `ui-ux-pro-max-skill` when asking:

```text
What should this product look like?
What design system should this app use?
What colors, typography, and layout pattern fit this product?
What UI style should this dashboard use?
What should we avoid visually?
```

### Use `frontend-design` when asking:

```text
Make this page look better.
Make this UI feel premium.
Make this landing page less generic.
Improve visual hierarchy.
Add polish and personality.
```

### Use `web-design-guidelines` when asking:

```text
Review this UI.
Check accessibility.
Check UX issues.
Audit this page before release.
Find form/focus/responsive problems.
```

---

## Rules

### Do

- Use design skills before implementation when the visual direction is unclear.
- Use design tokens instead of one-off colors and spacing.
- Keep visual decisions consistent across pages.
- Review accessibility before shipping.
- Keep UI behavior tests separate from visual design review.
- Use `coke-nextjs-app-architecture` after design direction is clear.
- Use `coke-nextjs-ui-tdd-workflow` for behavior, not pixel-perfect styling.

### Do Not

- Do not let architecture skills invent random visual styles.
- Do not let TDD skills test exact colors or spacing unless visual regression tooling exists.
- Do not use creative design skills as a replacement for accessibility review.
- Do not use design review as a replacement for behavior tests.
- Do not create one-off components when design system components already exist.
- Do not change design tokens casually in feature work.

---

## Example: Landing Page Workflow

```text
Step 1: Design system

Use ui-ux-pro-max-skill to generate a design system for a SaaS landing page.

Product:
- AI workflow monitoring tool

Target users:
- solo developers
- small teams
- automation-heavy teams

Brand tone:
- calm
- reliable
- technical but friendly

Output:
- design direction
- color palette
- typography
- landing page structure
- component list
- anti-patterns
```

```text
Step 2: Creative polish

Use frontend-design to turn this design system into a distinctive landing page concept.

Focus on:
- hero section
- trust signals
- visual hierarchy
- subtle motion
- memorable product identity

Do not make it generic AI SaaS purple-gradient UI.
```

```text
Step 3: Implementation

Use coke-nextjs-app-architecture to implement this landing page in Next.js App Router.

Rules:
- use design tokens
- keep components composable
- preserve accessibility
- add loading/error states if data-driven
- do not invent new styles outside the design system
```

```text
Step 4: Review

Use web-design-guidelines to review the implemented landing page.

Check:
- accessibility
- responsive layout
- focus states
- navigation
- visual hierarchy
- form UX
- contrast
```

---

## Example: Dashboard Workflow

```text
Step 1: Style intelligence

Use ui-ux-pro-max-skill to define dashboard design direction.

Context:
- Real-time monitoring dashboard
- Shows workflows, failures, latency, logs, and alerts

Output:
- dashboard pattern
- chart/card style
- data density guidance
- color rules for status/error/warning
- typography and spacing
```

```text
Step 2: Architecture

Use coke-nextjs-app-architecture to structure the dashboard.

Rules:
- Server Components for initial data where possible
- Client Components for filters, charts, interactions
- Suspense for slow sections
- API client boundary
- loading/error/empty states for each data panel
```

```text
Step 3: Behavior testing

Use coke-nextjs-ui-tdd-workflow to test:
- empty state
- loading state
- failed API state
- filter interaction
- alert details interaction
```

```text
Step 4: Final design review

Use web-design-guidelines to audit:
- data readability
- accessibility
- responsive layout
- keyboard navigation
- contrast
- focus states
```

---

## Minimal Prompt Template

Use this when you want a clean design-to-implementation flow:

```text
I want to design and implement this UI feature.

Feature:
- <feature>

Product context:
- <context>

Target users:
- <users>

First, use ui-ux-pro-max-skill to define the design direction.
Then use frontend-design to polish the visual concept.
Then use coke-nextjs-app-architecture to implement it.
Then use coke-nextjs-ui-tdd-workflow to test user-visible behavior.
Finally use web-design-guidelines to review UX/accessibility quality.

Do not mix responsibilities:
- design skills define visual direction
- architecture skill implements structure
- TDD skill tests behavior
- guidelines skill reviews final UX/accessibility
```

---

## Source References

- UI UX Pro Max Skill: https://github.com/nextlevelbuilder/ui-ux-pro-max-skill
- Anthropic Frontend Design Skill:
  https://github.com/anthropics/skills/blob/main/skills/frontend-design/SKILL.md
- Vercel Web Design Guidelines Skill:
  https://github.com/vercel-labs/agent-skills/blob/main/skills/web-design-guidelines/SKILL.md
