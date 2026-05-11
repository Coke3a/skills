# Database Migrations

- Keep migrations version controlled.
- Inspect existing tooling before adding commands.
- Do not guess migration commands.
- Common placeholders include `diesel migration run`, `sqlx migrate run`, Supabase
  migration commands, Makefile targets, justfile recipes, or custom scripts.
- Prefer backward-compatible migrations.
- Add nullable columns before code depends on them.
- Backfill large data separately.
- Avoid dropping or renaming columns in the same deploy where old code may still use
  them.
- Require approval for destructive production migrations.
- Stop deployment if required migration fails.
- Capture migration logs.
- Document rollback or forward-fix risk, especially when migrations are not reversible.
