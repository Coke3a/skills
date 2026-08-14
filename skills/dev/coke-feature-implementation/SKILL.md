---
name: coke-feature-implementation
description: Use this skill whenever the user wants to implement a feature from a spec file. Trigger phrases include "implement this spec", "implement the feature from <path>", "ทำตาม spec", "build this feature end-to-end", or any request to drive a multi-batch implementation against a written design. Always writes a plan + implementation checklist first (for agent reference and user post-review), red-teams that plan with a reasoning-heavy subagent before any code is written, then implements all batches — tiering each batch to Opus or Sonnet by difficulty — with two-stage review (spec compliance + quality with scrutinize), and final verification (tests, dev servers, browser smoke, database data alignment). Applies surgical Karpathy YAGNI throughout and never commits — the user reviews the diff before deploy. Prefer this skill over manually orchestrating subagents whenever the user has a written spec or plan they want executed.
---

# Coke Feature Implementation

Drive a feature from spec → working code via subagent-driven development with two-stage review per batch and a thorough final verification. Optimized for surgical, scope-tight execution where the user reads the final diff.

## When to use

Invoke when the user wants to implement a feature from a spec. The trigger phrase usually contains "implement", or names a spec file. The user explicitly does not want commits — they review the final diff.

## Execution flow

Every invocation follows the same flow — no mode selection needed:

```
Phase 0 (project config) → Phase 1 (setup) → Phase 2 (write plan + checklist) → Phase 2.5 (red-team the plan) → Phase 3 (implement) → Phase 4 (final verification)
```

Phase 2 always writes a plan and implementation checklist. These serve as:
1. **Agent reference** — subagents read the plan to stay aligned during implementation.
2. **User post-review** — the user checks implementation details after the work is done.

Phase 2.5 red-teams the plan with one reasoning-heavy subagent *before* any code gets written. The reasoning: this skill already hard-reviews the *code* (two-stage review per batch), but a wrong plan means every batch faithfully implements the wrong thing — and the reviewers pass it, because they check against the plan. A plan defect surfaced at Phase 4 or in the user's diff review is far more expensive than one caught here. This is an **agent gate, not a user gate**: if nothing serious turns up, the controller revises nothing and moves on; the user is not asked to approve.

Neither Phase 2 nor Phase 2.5 stops for user approval — the flow runs straight through to Phase 3. The plan is a working document, not a checkpoint.

## Per-invocation questions (ask before Phase 1)

Use AskUserQuestion (one or more calls; group what fits) — do not assume answers.

1. **Spec path** — absolute path to the spec file (required).
2. **Final verification scope (this invocation only)** — Phase 4 scope. Default value comes from `verification_scope_default` in the project config (see Phase 0). Possible values: `full` (E2E + browser MCP + DB alignment), `tests-only` (typecheck/lint/unit tests only — no dev servers), or `ask`. If config default is `ask`, prompt here; if `full` or `tests-only`, only re-prompt if the user explicitly says they want a different scope this time.

Implementation skills and Review skills are **per-project, not per-invocation** — they live in the config file. See Phase 0.

### Always-on baseline (do NOT ask — non-negotiable)

Every dispatched subagent operates on top of:

- **`/karpathy-guidelines`** — surgical scope, no speculative features, no defense-in-depth for single callers, every changed line traces to spec.
- **`/scrutinize`** — outsider end-to-end trace, distinguish claim from verification, ask the simpler-alternative question. Mandatory in every quality reviewer dispatch.

State this baseline up front when echoing config back to the user so they understand the project config is layered on top of karpathy + scrutinize.

If you don't understand what the user wants to implement (no spec path, vague plan), **ask before doing anything**. Don't assume.

---

## Phase 0 — Project skill configuration (run BEFORE Phase 1)

Implementation and review skills are stable per-project (a Rust+Next.js monorepo answers the same way every time). Persist them once; re-read on subsequent invocations.

### Config file

**Path:** `<project-root>/.claude/coke-feature-implementation.json`

**Schema:**
```json
{
  "implementation_skills": [
    "/coke-rust-clean-architecture",
    "/coke-rust-performance-optimization",
    "/coke-nextjs-app-architecture",
    "/react-best-practices"
  ],
  "review_skills": [
    "/coke-rust-code-review",
    "/react-best-practices"
  ],
  "verification_scope_default": "full",
  "_comment": "Edited manually OK. /karpathy-guidelines + /scrutinize always apply on top.",
  "_updated_at": "YYYY-MM-DD"
}
```

- **Flat lists, not partitioned by component.** The controller picks the subset relevant to each dispatched batch (e.g., for a Rust file, the implementer dispatch references only the `/coke-rust-*` entries from `implementation_skills`).
- **`verification_scope_default`:** `"full"` | `"tests-only"` | `"ask"`. If `"ask"`, the controller prompts per-invocation question 3 every time.
- **`_comment` and `_updated_at`** are advisory; not used programmatically. The user may edit the file by hand at any time.

### Resolve project root

1. Walk up from the spec path looking for the first directory that contains any of: `.claude/`, `.git/`, `package.json` at workspace root, `pnpm-workspace.yaml`, `Cargo.toml`.
2. If still unclear, ask the user for the absolute path to project root.
3. The `.claude/` directory may not exist yet — create it if you'll write the config file.

### Decide which case applies

| Trigger / state | Behavior |
|---|---|
| **Config file does NOT exist** | Run interview (below), save to JSON, proceed. |
| **Config file exists, normal invocation** | Read JSON. Echo to user briefly: "Using this project config — implementation skills: `<list>`; review skills: `<list>`; verification default: `<value>`. Reply 'edit' to change, otherwise I'll proceed." Default to proceeding if no objection. |
| **User said "reconfigure" / "update config" / "reconfigure skills" / "เปลี่ยน config"** | Force re-interview. Overwrite. Do NOT read the old values; ask fresh. |

### Interview (when config is missing or being rewritten)

Ask via AskUserQuestion. State `/karpathy-guidelines` + `/scrutinize` are baseline. Then:

- **Implementation skills** — for plan writer + implementer subagents. Free-form list (skill names with leading `/`). Examples per stack:
  - Rust backend: `/coke-rust-clean-architecture`, `/coke-rust-performance-optimization`
  - React / Next.js frontend: `/coke-nextjs-app-architecture`, `/react-best-practices`
  - TDD-driven repos: `/coke-tdd-feature-workflow`, `/coke-nextjs-ui-tdd-workflow`
  - Other: whatever the user names; empty list is OK
- **Review skills** — for quality reviewer subagent. May overlap with implementation skills. Examples:
  - Rust backend: `/coke-rust-code-review`
  - React / Next.js frontend: `/react-best-practices`
  - Other: whatever the user names
- **Verification scope default** — `full` | `tests-only` | `ask`. Default suggestion: `full` for monorepos with UI, `tests-only` for pure libraries.

### Saving

Use `Write` (file may be new):
- `mkdir -p <project-root>/.claude` if needed
- Write JSON pretty-printed (2-space indent) so a human can hand-edit
- Always populate `_updated_at` with today's date
- Confirm the path back to the user so they know where to find/edit it later

### What flows downstream

- Phase 2 (plan writing) reads `implementation_skills` from config and applies them as lens
- Phase 3 implementer dispatches embed `implementation_skills` (subset matching the affected component) in the prompt
- Phase 3 quality reviewer dispatches embed `review_skills` (subset matching the affected component) in the prompt
- Phase 4 uses `verification_scope_default` (or the per-invocation override from question 3) to decide what to run

If you ever can't write the config file (permission error, etc.), continue with the interviewed values in memory and report the persistence failure to the user — don't block on it.

## Memory + workspace awareness

Honor whatever the project's memory and CLAUDE.md say. Common patterns:
- **Workspace root may not be a git repo** — multi-component monorepos have git inside each subdir. Run `git` from the correct subdir; check with `git -C <subdir> status` if unsure.
- **Dev servers** — you (the agent) may start dev servers whenever you need them — for Phase 4 smoke, for debugging during an implementation batch, or to verify a hunch. **No prior user authorization needed.** Hard rule: **every server you start, you also stop.** Track them — port + background task ID — and `kill` before ending the turn or moving on. Leaving an orphan dev server pollutes the user's port space and is the one thing that turns "agent that helps" into "agent that messes things up."
- **Env vars** — never hard-code secrets; pull from env or `.env.example`.
- **Test creds for E2E** — if the project's CLAUDE.md or memory documents them, use them. Otherwise ask.

---

## Phase 1 — Setup

1. **Read the spec end-to-end.** Don't skim. The spec is the source of truth for what should be built. Note its TODO section if any.
2. **Detect affected components.** Look at file paths in the spec/plan; identify each `<subdir>/` that has its own git repo.
3. **Check git state per subdir:** `git -C <subdir> status && git -C <subdir> branch --show-current && git -C <subdir> log --oneline -5`. Working tree should be clean before starting. If dirty, ask the user.
4. **Choose a branch name:** default to `<topic-slug>-<YYYYMMDD>` (e.g., `event-driven-automations-20260523`). Confirm with the user only if the slug isn't obvious.
5. **Create the branch in every affected subdir** (e.g., backend + frontend) — workspace root is usually not a git repo, so don't try to branch there.
6. **Create TaskCreate entries** — one per logical unit you expect to batch (refined after Phase 2 writes the checklist). Include a `V1 — Final verification` task at the end.

---

## Phase 2 — Write plan + implementation checklist

The output is two markdown files at paths the user specified (default to the same directory as the spec, but follow user preference if they give one).

Apply the `implementation_skills` from the Phase 0 config + always-on `/karpathy-guidelines` + `/scrutinize` while drafting. This means:
- Skill knowledge informs which patterns the plan recommends (e.g., `/coke-rust-clean-architecture` shapes the task batching by layer; `/react-best-practices` shapes the frontend batches around component boundaries).
- Karpathy lens prunes speculative scope from the task list.
- Scrutinize lens populates the "Risk callouts" section.

### `<date>-<topic>-design.md` — engineering plan

Structure:
- **Goal** (1 sentence)
- **Companion files** — spec path, implementation checklist path, branches
- **Header for agentic execution** — "REQUIRED SUB-SKILL: superpowers:subagent-driven-development. Backend `B*` batches block frontend `F*` batches. Do NOT commit — user reviews diff."
- **Architecture overview** — minimal ASCII or text diagram of data/control flow
- **Engineering principles applied (Karpathy + scrutinize)** — numbered list of the load-bearing judgment calls and the rationale
- **Load-bearing assumptions + evidence** — numbered (A1, A2, ...). Each row is one assumption the plan *depends on* being true (e.g., "this function has a single caller", "this DTO field is unused by the frontend", "migration N runs before N+1") paired with the evidence that confirms it — a `grep` result, a file:line, a query. An assumption with no evidence yet is flagged `UNVERIFIED`; those get checked in Phase 2.5 before implementation relies on them. The point: a wrong assumption makes a batch fail silently, and no code reviewer catches it because the code faithfully matches the (wrong) plan.
- **Task batches table** — one row per batch (B1, B2, ..., F1, F2, ..., V1). Columns: batch ID, primary files, acceptance criteria, **model**. The `model` column is `Sonnet` or `Opus` (see tiering rule below).
- **Risk callouts (from scrutinize pass)** — numbered (R1, R2, ...) with what could go wrong and what to verify
- **What NOT to do (locked by spec)** — bullet list
- **Definition of done**

**Batch model tiering (Sonnet vs Opus).** Each batch is dispatched to whichever tier fits its difficulty — no reason to burn Opus on a rename, or hand a concurrency rewrite to Sonnet:
- **Sonnet** — mechanical or small: renames, moving code, wiring an already-designed call site, boilerplate DTOs, straightforward CRUD, following an explicit step list with no judgment calls.
- **Opus** — reasoning-heavy or risky: concurrency / async ordering, migration sequencing, auth / permissions / money paths, anything touching a `UNVERIFIED` assumption or a scrutinize risk callout, or a batch where the "exact steps" still require the implementer to make design decisions.

When genuinely unsure, prefer Opus — a wrong mechanical call is cheap to redo, a wrong reasoning call is not. Record the chosen tier in the batches table so Phase 3 dispatches read it straight from the plan.

### `<date>-<topic>-implementation.md` — todo checklist

Structure:
- Header: pointers to plan + spec + branch + execution method (subagent-driven, no commits)
- One `## Bn / Fn / V1` header per batch — each is one subagent dispatch boundary
- Under each header, `- [ ]` bullets with **exact file paths and the exact action** (e.g., `- [ ] backend/src/foo.rs — replace bucket_counts_in_window call with tokio::try_join!(...)`)
- **Per-batch verification commands** (grep, cargo check, pnpm typecheck, etc.) with expected output
- **Per-batch review protocol** at the bottom — what spec sections + which technical lens + scrutinize

Keep both files focused and non-duplicative of the spec. The spec already documents what should be built; the plan documents how, in what order, with what risk mitigations.

After writing both files, **proceed to Phase 2.5** — do not stop for user approval. The plan exists for agent reference and user post-review, not as a gate.

---

## Phase 2.5 — Red-team the plan (agent gate, before any code)

Before the implementation loop spends real effort, one reasoning-heavy subagent tries to *break the plan on paper*. Catching a batching mistake or an unverified assumption here costs one dispatch; catching it after five batches are built costs a rewrite.

### Step 1 — Dispatch the plan critic

Use `Agent` with `subagent_type: general-purpose` and **`model: "opus"`** (this is exactly the reasoning-heavy work Opus is for). The prompt is self-contained — see `references/subagent-prompts.md` for the full template. It must:

- Point the critic at the spec, the design plan, and the implementation checklist (paths).
- Ask it to *attack*, not admire — its job is to find what's wrong, not confirm the plan is fine.
- Focus on these failure modes specifically:
  1. **Batch ordering / dependencies** — does any batch depend on something a later batch produces? Is a batch's "state of the world" actually reachable when it runs?
  2. **Unverified assumptions** — for every `A*` marked `UNVERIFIED` (and any load-bearing claim that *should* have been listed but wasn't), state what breaks if it's false and how to check it cheaply.
  3. **Batch sizing** — any batch doing too much to review as one unit, or two batches that should be one?
  4. **Missing scope** — a spec requirement no batch covers, or an acceptance criterion nothing verifies.
  5. **Model tier sanity** — any batch tagged `Sonnet` that actually needs judgment, or `Opus` that's pure mechanics?
- Report format: a short list of concrete defects, each with severity (blocker / worth-fixing / nit) and a one-line suggested fix. If the plan is sound, say so plainly and list what it actually checked — no manufactured findings.

### Step 2 — Verify UNVERIFIED assumptions with evidence

For each assumption the critic flags as load-bearing and still unproven, get evidence *before* trusting it — don't carry a guess into implementation. Cheap checks (a `grep`, reading a file:line) the controller can just run. For anything that needs a real trace across files, dispatch a **Sonnet** agent to gather the evidence — this is mechanical lookup, not reasoning. Update the plan's assumptions section from `UNVERIFIED` to the evidence found (or correct the plan if the assumption was wrong).

### Step 3 — Triage and revise

- **Blocker / worth-fixing** defects that are real → edit the design plan and/or implementation checklist to fix them (reorder batches, split a batch, add a missing batch, retag a model tier, correct a wrong assumption).
- **Nits or things the critic misread** → leave as-is; keep a one-line record so the Final Report can note "plan-critique findings (considered, not actioned)".
- If a defect reveals a genuine spec ambiguity only the user can resolve → escalate to the user (this is one of the sanctioned stop points).

Do **not** loop this indefinitely — one critic pass plus the resulting fixes is the intent. If the revision was substantial (batches reordered or added), a second quick critic pass on just the changed parts is reasonable; otherwise proceed to Phase 3.

---

## Phase 3 — Implementation loop (HEART OF THIS SKILL)

For each batch in order:

### Step 1 — Mark and prepare

- TaskUpdate the batch to `in_progress`.
- Read the batch's section in the implementation checklist + the spec sections it references.
- Read 2-3 of the affected source files briefly so your dispatch prompt can include accurate state-of-the-world context.

### Step 2 — Dispatch the implementer subagent

Use `Agent` with `subagent_type: general-purpose`, and set **`model`** from this batch's tier in the plan's batches table — `model: "sonnet"` for a Sonnet batch, `model: "opus"` for an Opus batch. The prompt must be self-contained — the subagent has no conversation history. See `references/subagent-prompts.md` for the full template; the gist is:

```
You are the implementer for Batch <ID> — <name> of <feature name>.

Branch: <branch> in <subdir-abs-path> (already checked out).
Working directory: <subdir-abs-path>

Goal: <one sentence>

State of the world: <what previous batches changed; what's currently broken on purpose>

Reference materials (read first):
- Spec: <path> — sections "<X>", "<Y>"
- Implementation checklist: <path> — section ## <ID>
- Plan: <path> — <relevant rows + risks>

Then read these target files end-to-end before editing:
- <file paths>

What to do (exact):
1. <step with file path + action>
2. <step>
...

Verify before reporting DONE:
- <command> → expected <result>
- grep -rn "<pattern>" → expected zero matches
- cargo check / pnpm typecheck

Constraints:
- Do NOT commit.
- Do NOT touch <files outside this batch> — those are batch <ID+N>.
- Reuse <existing type X> rather than introducing <new type Y> [if applicable].
- Match existing project conventions; read neighboring files first.

Report format:
- Status: DONE / DONE_WITH_CONCERNS / NEEDS_CONTEXT / BLOCKED
- Files modified / created / deleted: bullets
- Verification command results
- Diff summary (5-10 lines)
- Surprises
```

The implementer status determines next step:
- **DONE** → proceed to Step 3 (review).
- **DONE_WITH_CONCERNS** → read concerns; if they're observations, proceed; if they're real doubts about correctness, address inline or in a fix sub-dispatch first.
- **NEEDS_CONTEXT** → provide the missing info, re-dispatch.
- **BLOCKED** → diagnose: more context? more capable model? smaller task slice? plan defect? Re-dispatch or escalate to user.

### Step 3 — Quick diff verify (controller responsibility)

Before paying for two reviewer subagents, do a 30-second sanity check:
- `git -C <subdir> diff --stat` — file count + line counts plausible
- `git -C <subdir> diff -- <one key file>` — quickly read the actual shape of the change

If the implementer's report contradicts the diff (e.g., claims they kept field X but the diff shows it deleted), confront the contradiction now — don't waste reviewer time.

### Step 4 — Dispatch BOTH reviewers in PARALLEL

Send both in the same Agent call message (parallel tool use). They're independent.

Tier the reviewers to match the batch: for an **Opus batch**, dispatch the **quality reviewer with `model: "opus"`** — reasoning-heavy code deserves a reasoning-heavy reviewer. The spec compliance reviewer is a requirement-by-requirement compare and can stay on Sonnet regardless of tier. For a Sonnet batch, both reviewers on Sonnet is fine. (If a Sonnet batch's diff turns out to hide something subtle, nothing stops you re-running the quality reviewer on Opus.)

**Spec reviewer** (`subagent_type: feature-dev:code-reviewer`):
```
You are the spec compliance reviewer for Batch <ID>.
Branch: <branch> (changes still unstaged — use `git diff main` not `git diff main..HEAD`).

Files changed: <list>

Canonical spec: <path> — sections "<X>", "<Y>"
Plan acceptance criteria: <path> — Bn row.

What to verify:
1-N — bullet each spec requirement
- explicit numbered list

Acceptable transitional state:
- cargo check fails ONLY in <later-batch files>: <list>
- <other transitional things>

Report: PASS or FAIL with numbered findings citing file:line. <250 words. Do NOT flag style.
```

**Quality reviewer** (`subagent_type: feature-dev:code-reviewer`):
```
You are the code quality reviewer for Batch <ID>.
Apply <technical lens skill> + /scrutinize. Scrutinize is mandatory and most important.

Files: <list>
Use `git diff main -- <file>` per file.

Scrutinize lens (trace these explicitly):
1. <input that might break it>
2. <call path question>
3. <ordering / off-by-one>
4. <bound-parameter / injection>
5. <mock vs real semantic equivalence>

Technical lens checks:
- <bullet list per chosen skill>

Report: blocker / major / nit. Each finding has file:line + one-line finding + one-line why + one-line fix. <500 words. Lead with blockers. If no findings, list which files you read end-to-end + which traces you actually walked + which checks you ran.
```

### Step 5 — Process reviewer findings through the Karpathy filter

This is where most controllers fail. See `references/karpathy-filter.md` for the full decision matrix; the short version:

| Finding | Action |
|---|---|
| Real correctness bug in this batch's code | **FIX** — dispatch implementer (same agent ID via SendMessage, or new fix dispatch) |
| Code doesn't match spec for this batch's scope | **FIX** |
| Pre-existing pattern not introduced by this batch | **REJECT** — out of scope |
| Expected transitional compile error (later batch fixes) | **REJECT** — documented in plan |
| Refactor outside batch's surgical scope | **REJECT** — Karpathy: don't refactor what isn't broken |
| Defense-in-depth for a single caller | **REJECT** — YAGNI |
| Spec typo or table example wrong | **REPORT to user, leave code as-is** |
| Style nit with 1-line fix | Maybe fix; otherwise reject |
| Other style nit | **REJECT** — final review will catch |

When rejecting, state the reason — keep a brief record so the Final Report can list "Audit findings (reported, not fixed)".

### Step 6 — Re-review after fix (if any)

If you fixed something, re-dispatch the relevant reviewer. Don't trust that "small fix" stays correct without verification.

### Step 7 — Verify + mark complete

- Run the batch's verification commands one more time (the implementer ran them; you re-verify).
- TaskUpdate → `completed`.
- Move to next batch.

### Loop until all batches complete

Do not pause to check in between batches unless:
- A subagent reports BLOCKED you can't unstick
- The spec is genuinely ambiguous and only the user can resolve
- The user explicitly asked to confirm at a checkpoint

Otherwise execute straight through. "Should I continue?" messages waste user time when they already asked you to execute the plan.

---

## Phase 4 — Final verification (V1)

The scope comes from per-invocation question 3 (which defaults to Phase 0's `verification_scope_default`):
- `full` → all steps below
- `tests-only` → Steps 1 + 7 only (skip dev servers, E2E, browser MCP, DB alignment)
- `ask` → re-prompt at top of phase if not already answered

Otherwise stop after the last functional batch and write the final report.

### Step 1 — Parallel test commands

Run these in parallel (use `run_in_background: true` for each Bash):
- Backend: `cargo test --workspace` (with whatever LIBRARY_PATH the project needs)
- Frontend: `pnpm typecheck`, `pnpm lint`, `pnpm test -- --run` (or `pnpm exec vitest run`)
- Frontend production build: `pnpm build`
- Any other project-specific check the plan listed

Wait for all notifications. Read tail outputs to confirm green.

### Step 2 — Start dev servers (only if browser smoke is in scope)

Background each:
- Backend dev server (project-specific command)
- Frontend dev server (project-specific command)

Use `until grep -q "<ready marker>" <log>; do sleep 1; done` patterns to detect ready signals. If a server logs nothing on startup (some Rust binaries are silent), poll `curl <port>/healthz` instead.

### Step 3 — Run E2E

If the project has E2E (Playwright, etc.):
```
<env vars from memory or .env.example> pnpm test:e2e --project=chromium
```
Run in background, wait for notification. If failures occur, **diagnose before reporting**:
- Read each failure's `test-results/<name>/error-context.md` (Playwright auto-generates these)
- Decide per failure: regression-from-this-work vs pre-existing flake (test fixtures, state pollution, etc.)
- Use Supabase MCP or other DB tools to investigate when needed

### Step 4 — Browser MCP smoke (multiple viewports)

Use Playwright MCP tools:
- `browser_resize` to xl/lg/md/sm widths (e.g., 1440, 1024, 768, 375)
- `browser_navigate` to the affected pages
- `browser_take_screenshot` (with `filename` so you can `Read` the resulting PNG)
- Visually verify: new UI elements present, removed UI absent, responsive behavior correct
- Save screenshots; reference them in the final report

### Step 5 — Database data alignment (if applicable)

Use Supabase MCP `execute_sql` (or whatever DB tool the project uses):
- Pick one entity visible in the browser test (e.g., one automation's UUID)
- Run a query that mirrors the new derivation logic on the DB
- Compute the expected UI value from the raw rows
- Compare to what the browser actually rendered
- A match here is the strongest signal the new logic works end-to-end

### Step 6 — Stop dev servers

- Kill by port: `lsof -ti:<port> | xargs -r kill -TERM; sleep 1; lsof -ti:<port> | xargs -r kill -KILL`
- Confirm with `lsof -i:<port>` (should be empty)
- Do this **always** at the end of Phase 4 — leaving dev servers running violates the "dev servers on demand" memory rule.

### Step 7 — Write the final report

Use this structure:
```
# Final Report — <feature>

Branch (subdirs): <branch>
Status: changes uncommitted (user reviews before deploy)

## Plan + spec files
<paths>

## Implementation summary (<N> batches)
**<Component A>** (<file count>, +<lines>/−<lines>):
- B1 — one-line description
- B2 — ...
**<Component B>** (...):
- F1 — ...

## Review findings reported (audit, NOT blockers)
1. <finding> — file:line — reason kept open
2. <finding> — ...

## Verification
| Check | Result |
|---|---|
| <tests + counts> |
| Chrome smoke @ widths | ... |
| Database spot-check | ... |
| Dev servers stopped | ✓ |

## Screenshots
<list>

## Next steps (yours)
1. Review diff in <subdir> on branch <branch>
2. Decide on the audit findings
3. Commit + deploy
```

---

## Always-on constraints (regardless of phase)

- **Never `git commit` or `git push`** unless the user explicitly says to. The user reviews the diff.
- **Never run destructive git ops** (`reset --hard`, `clean -f`, `branch -D`) without explicit user approval.
- **Reuse types and helpers** instead of inventing new ones — Karpathy: don't expand surface area unless required.
- **Surgical scope** — every changed line should trace to the spec. Reject "while we're here" refactors.
- **Read before editing** — the implementer subagent should read the file before editing it; the controller should read the diff before trusting the implementer's report.
- **Evidence over assumption** — any load-bearing assumption (single caller, unused field, ordering guarantee) gets a `grep`/file:line/query behind it before code depends on it. If you catch yourself writing "this is probably only called here", verify it instead of guessing — a wrong assumption fails a batch silently and no code reviewer will catch it, because the code matches the plan.
- **Memory and CLAUDE.md override defaults** — when in doubt, defer to what the project documents.

## When to escalate to user

- BLOCKED implementer that more context can't unstick.
- Spec contradiction or ambiguity that affects implementation choice.
- Authorization needed for destructive action (delete a file that's checked in, modify shared infra, etc.).
- A reviewer finding you genuinely can't classify (real bug? out of scope?).

Otherwise execute through.

---

## Reference files

- `references/subagent-prompts.md` — full subagent dispatch templates (implementer, spec reviewer, quality reviewer)
- `references/karpathy-filter.md` — extended decision matrix for reviewer findings + worked examples
- `references/final-verification.md` — Phase 4 detail including Chrome MCP + Supabase MCP patterns
