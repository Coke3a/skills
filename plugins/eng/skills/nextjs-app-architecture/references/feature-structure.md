# Feature Structure

## Recommended Shape

```text
features/<feature-name>/
  components/
  actions/
  services/
  hooks/
  schemas/
  types.ts
  index.ts
```

Use only the folders the feature needs.

## Responsibilities

- `components/`: feature-local UI and feature-specific composition.
- `actions/`: Server Actions for feature mutations.
- `services/`: app-side orchestration, API client calls, and view mapping.
- `hooks/`: client-only feature hooks.
- `schemas/`: validation schemas shared by forms and actions.
- `types.ts`: feature-local types.
- `index.ts`: narrow public exports when the project uses feature public APIs.

## Placement Rules

- Keep route-specific UI near `app/` when it is not reusable.
- Keep product-domain UI in the owning feature.
- Promote repeated cross-feature UI to `shared/components` or `packages/ui`.
- Keep API client code out of presentational components.
- Avoid circular imports between features.
- Do not create empty folders solely to match the template.

## Monorepo Notes

- `apps/web/src/features` owns app-specific features.
- `packages/api-client` owns reusable API client code.
- `packages/shared-types` owns shared generated or contract types.
- `packages/ui` owns cross-app UI primitives.
- `packages/design-tokens` belongs to design-system ownership, not this skill.
