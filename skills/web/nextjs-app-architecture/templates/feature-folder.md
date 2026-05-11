# Feature Folder Template

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

Create only the folders the feature needs.

Recommended ownership:

- `components/`: feature-local UI.
- `actions/`: Server Actions.
- `services/`: API orchestration and view mapping.
- `hooks/`: client-only feature hooks.
- `schemas/`: validation shared by forms/actions.
- `types.ts`: feature-local types.
- `index.ts`: narrow public exports when the project uses public feature APIs.
