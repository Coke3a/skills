# Final Verification (Phase 4 — V1)

Phase 4 is the user-facing proof that the spec works end-to-end. It is heavier than test runs — it boots real servers, drives a real browser, and queries the real database (where authorized). Skip only if the user explicitly opted out.

## Order matters

Run in this order — each step builds confidence the next step is meaningful:

1. **Parallel test commands** — typecheck, lint, unit tests, build
2. **Start dev servers** (only if browser smoke is in scope)
3. **Run automated E2E** (Playwright, Cypress, etc.)
4. **Browser MCP smoke** at multiple viewports
5. **Database data alignment** spot-check
6. **Stop dev servers**
7. **Write final report**

Don't reorder. If tests fail at step 1, there's no point starting servers.

## Step 1 — Parallel test commands

Use `run_in_background: true` per command, then wait for notifications. Example (adjust per project):

```bash
# Backend
cd <backend-subdir> && <env vars> cargo test --workspace
# Frontend typecheck
cd <frontend-subdir> && pnpm typecheck
# Frontend lint
cd <frontend-subdir> && pnpm lint
# Frontend unit tests
cd <frontend-subdir> && pnpm exec vitest run
# Frontend prod build
cd <frontend-subdir> && pnpm build
```

After all notifications arrive, read tails to confirm exit 0 + count of passing tests. Don't skip the read — exit 0 with warnings can hide real issues.

## Step 2 — Start dev servers

You start the servers yourself — no permission ask. The only rule: **every server you start, you stop** (Step 6). Track each: port + background task ID, so kill-by-port works deterministically at the end.

Background each (`run_in_background: true`):

```bash
cd <backend-subdir> && <env vars> <backend dev cmd>
cd <frontend-subdir> && <frontend dev cmd>
```

Detect readiness with an `until` loop, NOT a sleep:

```bash
until grep -qE "<ready marker pattern>" <log-file> 2>/dev/null; do sleep 1; done; echo ready
```

Examples:
- Next.js: grep `"Ready in"`
- Rust Axum: server may log nothing — fall back to `until curl -sSf http://localhost:<port>/healthz >/dev/null; do sleep 1; done`
- Vite: grep `"Local:"`

Run this until-loop with `run_in_background: true` so you get one notification when ready.

If a server starts silently and the until-loop times out, just `curl <port>/healthz` manually — it's faster than debugging the log filter.

## Step 3 — Automated E2E

If the project has E2E tests:

```bash
<env vars from memory> pnpm test:e2e --project=chromium
```

Run in background. Wait for notification (E2E can take minutes).

### When E2E fails — diagnose before reporting

Playwright auto-generates `test-results/<run-name>/error-context.md` for each failure. Read it — the page snapshot at failure point usually tells you the root cause in seconds.

For each failure, classify:
- **Regression from this work** — the failure stems from changes this PR made. Fix immediately.
- **Pre-existing flake** — test isolation issue (e.g., test account has 3 seeded fixtures + free plan cap = create-test fails), environment-dependent, etc. Document in the final report; don't fix in this PR.
- **Infrastructure issue** — dev server not actually up, DB unavailable, etc. Fix the infra, re-run.

Spot the difference: if every failed test has the same root cause unrelated to your change, it's almost certainly pre-existing. If the failures all touch code you changed, it's a regression.

## Step 4 — Browser MCP smoke

Use Playwright MCP (`mcp__plugin_playwright_playwright__*`). Pattern:

```
1. browser_resize to <width>x<height>
2. browser_navigate to <url>
3. browser_fill_form / browser_click for auth flow
4. browser_take_screenshot with filename="<v1-XX>.png"
5. Read the screenshot file (it's saved at the workspace root usually)
6. Verify the new UI elements + absence of old UI
```

Cover multiple viewports — at least 2-4 of: 1440 (xl), 1024 (lg), 768 (md), 375 (sm). The narrow-viewport behavior often diverges from the wide-viewport behavior.

What to verify:
- New UI elements present (new components, new copy, new aria roles)
- Removed UI absent (deleted components, deleted query params, deleted controls)
- Responsive breakpoints behave (status panels show/hide, layout reflows)
- Data-bound elements render with real DB data (not skeletons / not error states)

Save screenshots — reference them in the final report so the user can spot-check.

### Browser MCP quirks

- Screenshots save to the workspace root usually, not the cwd you'd guess. Use `find . -name "<filename>" -newer /tmp` to locate if surprised.
- `browser_snapshot` returns an accessibility tree which is often MORE useful than screenshots for asserting on copy / structure.
- `browser_close` cleans up the browser session.

## Step 5 — Database data alignment

The strongest verification: pick one entity visible in the browser and confirm the DB state matches what the UI rendered.

Pattern:

```
1. From browser MCP / network DevTools, get one entity's UUID.
2. Use Supabase MCP execute_sql (or psql / pg cli) to query that entity's raw rows.
3. Compute the expected UI value from the raw rows using the new derivation logic.
4. Compare to what the browser actually rendered.
```

Example:

```sql
SELECT state, started_at
FROM runs
WHERE automation_id = '<uuid>'
  AND state IN ('success', 'failed', 'timed_out')
ORDER BY started_at DESC
LIMIT 30;
```

If the UI shows "7 runs · 86% success · Unstable" and the DB returns 6 success + 1 failed:
- Count check: 7 ✓
- Success rate: 6/7 = 85.7%, rounded = 86% ✓
- Status threshold: 0.80 ≤ 0.857 < 0.95 → Unstable ✓

This is the smoking-gun proof the new derivation logic works end-to-end.

For aggregate stats (status panel, narrow description), sum across all visible entities and verify against the aggregated UI numbers.

## Step 6 — Stop dev servers

**Always.** Every server you started in Step 2 (or any earlier batch where you started one for debugging) gets stopped here. This is non-negotiable — orphan servers pollute the user's port space and the trust contract for "agent may start servers freely."

```bash
lsof -ti:<backend-port> -ti:<frontend-port> | xargs -r kill -TERM
sleep 1
lsof -ti:<backend-port> -ti:<frontend-port> | xargs -r kill -KILL 2>/dev/null
lsof -i:<backend-port> -i:<frontend-port>  # confirm empty
```

Also `browser_close` the Playwright MCP browser if you opened one.

## Step 7 — Final report

Use this structure (adapt to project):

```markdown
# Final Report — <feature name>

**Branch (both subdirs):** `<branch>` — **status: changes uncommitted** (user reviews diff before deploy)

## Plan + spec
- Plan: `<path>`
- Implementation checklist: `<path>`
- Spec: `<path>`

## Implementation summary (<N> batches)

**<Component A>** (<N> files, +<X>/−<Y> lines):
- B1 — one-line description of what changed
- B2 — ...

**<Component B>** (<N> files, +<X>/−<Y> lines):
- F1 — ...
- F2 — ...

## Review findings reported (audit, NOT blockers)

Scrutinize lens flagged these — not fixed per Karpathy filter:

1. **<finding>** — `<file:line>` — <reason kept open>
2. **<finding>** — ...

## Verification

| Check | Result |
|---|---|
| `cargo test --workspace` | <N passed / 0 failed / N ignored> |
| `pnpm typecheck` | clean |
| `pnpm lint` | clean |
| `pnpm test` (vitest) | <N / N passed> |
| `pnpm build` | exit 0 |
| Playwright E2E | <N passed / N failed (M did not run)> — <classification: regression / pre-existing flake / mixed> |
| Chrome MCP smoke @ <widths> | all render correctly: <key things verified> |
| Database data alignment | ✓ for `<uuid>` — UI <X>; DB <Y>; matches |
| Dev servers stopped | ✓ |

## Screenshots

Workspace root: `<v1-...>.png` × N

## Next steps (yours)

1. Review diff in `<subdir>` on branch `<branch>`
2. Decide on the audit findings (each can be fixed in a separate PR or left)
3. Commit + deploy
```

## When Phase 4 is skipped

If the user opted out of dev-server smoke (e.g., "backend only, no UI"), do steps 1 and the report. Skip 2-6. Be explicit in the report that browser smoke wasn't run.

If the project doesn't have E2E, do steps 1-2 + 4-7 (skip automated E2E). The browser MCP smoke is the proof.

If the project doesn't have a database to spot-check (e.g., pure stateless service), skip step 5.

## Common failure modes

- **Forgot to stop dev servers** — leaves orphan processes that block the user's next session.
- **Forgot to verify UI didn't render skeleton/error state** — screenshots show layout but not actual data.
- **Verified only one viewport** — responsive bugs hide at the breakpoints you didn't test.
- **Reported E2E pass/fail without diagnosing failures** — the user can't tell if it's your bug or pre-existing.
- **Compared UI to spec text instead of UI to live DB state** — only DB alignment proves the new logic actually executes correctly with real data.
