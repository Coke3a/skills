# Setup Database Migration Pipeline

1. Inspect migration tooling

- [ ] Check for Diesel migrations.
- [ ] Check for sqlx migrations.
- [ ] Check for Supabase migrations.
- [ ] Check for custom scripts.
- [ ] Check Makefile, justfile, and package scripts.

2. Add migration step

- [ ] Use the project’s existing migration command.
- [ ] Run against environment-specific database.
- [ ] Use environment secrets.
- [ ] Stop deploy if migration fails.
- [ ] Capture logs.

3. Safety review

- [ ] Flag destructive migrations.
- [ ] Require manual approval for destructive production migrations.
- [ ] Prefer backward-compatible migration strategy.
- [ ] Document rollback or forward-fix risk.

4. Verify

- [ ] Migration command is not guessed.
- [ ] Secrets are environment scoped.
- [ ] Production migration is gated when risky.
