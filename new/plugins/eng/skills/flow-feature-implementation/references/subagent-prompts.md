# Subagent dispatch templates

Four subagent roles: plan critic (Phase 2.5), implementer, spec reviewer, quality reviewer. Each prompt is self-contained — the subagent has zero conversation history. Brief like a smart colleague who just walked in.

**Model tier.** Dispatch each role on the model that fits the work: implementer and quality reviewer follow the batch's tier from the plan (`model: "sonnet"` for mechanical batches, `model: "opus"` for reasoning-heavy ones); the plan critic is always `model: "opus"`; the spec compliance reviewer can stay on Sonnet (it's a requirement-by-requirement compare). Assumption-evidence gathering in Phase 2.5 is mechanical lookup → Sonnet.

---

## 0. Plan critic (`subagent_type: general-purpose`, `model: "opus"`) — Phase 2.5

```
You are the **plan critic** for <feature name>. Your job is to BREAK this plan on
paper before anyone writes code. Attack it — do not admire it. A plan that survives
you saves a costly rewrite later; a manufactured finding just wastes a fix cycle, so
report only what's real.

## Apply these skills as your lens

- **`/scrutinize`** — you ARE the outsider red-team. Trace the plan, don't trust it.
- **`/karpathy-guidelines`** — flag speculative scope and over-engineered batches too,
  not just gaps. A plan that builds more than the spec asks is also a defect.
- **Implementation lens** (user-selected): <list — same as the implementer's, for
  judging whether the plan's approach is idiomatic for this stack>.

## Read these (paths)

- Spec: <path> — the source of truth for what should be built
- Design plan: <path>
- Implementation checklist: <path>

## Attack these failure modes specifically

1. **Batch ordering / dependencies** — does any batch depend on something a *later*
   batch produces? When batch N runs, is its stated "state of the world" actually
   reachable? Walk the sequence.
2. **Unverified assumptions** — for every assumption marked `UNVERIFIED` (and any
   load-bearing claim that SHOULD be listed but isn't), state exactly what breaks if
   it's false and the cheapest way to check it (a grep, a file:line, a query).
3. **Batch sizing** — any batch too big to review as one unit? Any two batches that
   should be merged?
4. **Missing scope** — a spec requirement no batch covers, or an acceptance
   criterion nothing verifies?
5. **Model tier sanity** — any batch tagged `Sonnet` that actually needs judgment,
   or `Opus` that's pure mechanics?

## Report format

A short list of concrete defects. For each:
- Severity: **blocker** / **worth-fixing** / **nit**
- One-line description citing the batch ID / assumption ID / spec section
- One-line suggested fix

If the plan is genuinely sound, say so plainly and list what you actually checked
(which batches you traced, which assumptions you probed) — do not invent findings to
look thorough.
```

---

## 1. Implementer (`subagent_type: general-purpose`, `model` per batch tier)

```
You are the implementer for **Batch <ID> — <name>** of <feature name>.

**Branch:** `<branch>` in `<subdir-abs-path>` (already checked out).
**Working directory:** `<subdir-abs-path>`

**Goal:** <one sentence — what changes after this batch>.

## State of the world

<List what previous batches changed and what's currently broken on purpose.
For example: "B1 added trait methods X+Y, removed Z. Currently src/foo.rs fails compile
because it still calls Z — that's expected; B3 rewrites foo.rs. cargo check --lib
errors are confined to <files>.">

## Reference materials (read first)

- Spec: <path> — sections "<X>", "<Y>", "<Z>"
- Implementation checklist: <path> — section ## <ID>
- Plan: <path> — <relevant rows + risks>

Then read these target files end-to-end before editing:
1. <file path>
2. <file path>

Also read these for cross-cutting context (do NOT modify — later batch):
- <file path> — <why you should understand it>

## What to do (exact)

### 1. <file path>

<step-by-step. Include code blocks where useful. The implementer SHOULD read the
file before editing, so don't paste 100 lines of the file — paste only the
ADDITIONS/REPLACEMENTS, and describe deletions structurally.>

### 2. <next file>

<...>

## Verify before reporting DONE

```bash
<command 1>
```
Expected: <result>

```bash
grep -rn "<pattern>" <subdir>/src
```
Expected: <zero matches | matches in X.rs only | etc>

## Apply these skills while implementing

- **Always:** `/karpathy-guidelines` — surgical scope, no speculative features,
  no defense-in-depth for single callers, every changed line must trace to spec.
- **Implementation lens** (user-selected for this project): <list — e.g.,
  `/coke-eng:rust-clean-architecture`, `/coke-eng:rust-performance-optimization`,
  `/react-best-practices`, `/coke-eng:nextjs-app-architecture`, etc.>
- Brief: read the skill briefs if you don't already know them; apply their
  patterns idiomatically, but don't over-engineer to satisfy a skill if the
  task is small.

## Constraints

- **Do NOT commit.** The user reviews the full diff at the end.
- **Do NOT touch** <files outside this batch>: <list>. Those are batch <ID+N>.
- **Reuse <existing type X>** rather than introducing <new type Y>. <One-sentence why.>
- **Match existing project conventions.** Read neighboring files before writing new ones.
- <any other guard rails — e.g., "Use `Arc<dyn Trait>` for repositories; no concrete types in usecase signatures.">

## Report format

- **Status:** DONE / DONE_WITH_CONCERNS / NEEDS_CONTEXT / BLOCKED
- **Files modified:** bullet list of paths
- **Files created:** bullet list
- **Files deleted:** bullet list
- **Verification command results:** quote actual output, don't paraphrase
- **Diff summary:** 5-10 lines describing the shape of the change
- **Concerns / surprises:** anything that surprised you or you're unsure about

Do not summarize the spec or quote large blocks back. Just confirm what you did.
```

### Implementer status handling

- **DONE** → proceed to quick diff verify, then dispatch reviewers.
- **DONE_WITH_CONCERNS** → read concerns. If observation → proceed. If real doubt → address inline first.
- **NEEDS_CONTEXT** → provide missing info, re-dispatch (use SendMessage to the same agent ID if context already grew).
- **BLOCKED** → diagnose:
  1. Missing context? Provide and re-dispatch.
  2. Needs more reasoning? Re-dispatch with `model: "opus"` for next call.
  3. Task too big? Split into 2 smaller dispatches.
  4. Plan defect? Escalate to user.

---

## 2. Spec compliance reviewer (`subagent_type: feature-dev:code-reviewer`)

```
You are the **spec compliance reviewer** for Batch <ID> of <feature>.

**Working directory:** `<subdir-abs-path>`
**Branch:** `<branch>` — **changes are unstaged**. Use `git diff main` not `git diff main..HEAD`.

## Files changed

- <list>

Use `git diff main --stat` then `git diff main -- <file>` per file.

## Canonical spec

`<path>` — sections "<X>", "<Y>".

## Plan acceptance criteria

`<path>` — row for batch <ID>.

## What to verify (numbered)

1. <exact spec requirement> — cite where to look in the file
2. <next>
3. <next>
4. <DTO field exactly: X, Y, Z; KEEP: A, B; DROPPED: C, D>
5. <method signature matches: `pub fn foo(...) -> ...`>
6. <important constraints — e.g., "No new enum X; reuse Y">

## Acceptable transitional state (do NOT flag these)

- `<command>` fails ONLY in <later-batch files>: <list>. <reason>
- <other expected transitional things>

## Report

**PASS** if every requirement above is met.
**FAIL** with numbered findings citing `file:line` per gap.

Under 250 words. Do NOT flag style, alternative phrasings, or refactor suggestions —
that's the quality reviewer's job. Confidence-score load-bearing claims.
```

---

## 3. Quality reviewer (`subagent_type: feature-dev:code-reviewer`)

```
You are the **code quality reviewer** for Batch <ID> of <feature>.

## Apply these skills

- **`/scrutinize`** — mandatory and most important. Trace end-to-end.
  Distinguish claim from verification. Ask the simpler-alternative question.
- **`/karpathy-guidelines`** — lens for filtering findings. Reject defense-in-depth
  for single callers, reject out-of-scope refactor suggestions, reject pre-existing
  pattern complaints. Surface real correctness bugs and real spec gaps.
- **Review lens** (user-selected): <list — e.g., `/coke-eng:rust-code-review`,
  `/react-best-practices`, etc.>

**Working directory:** `<subdir-abs-path>`
**Branch:** `<branch>` — changes still unstaged.

## Files to review

- <list>
Use `git diff main -- <file>` per file.

## Scrutinize lens (do this FIRST — trace each explicitly)

1. **<input that might break it>** — what happens for empty / single / max / boundary?
   Trace through the code. State observed behavior.
2. **<call path question>** — does this query/function actually return what its
   docstring claims? Walk an example.
3. **<ordering / off-by-one>** — verify ASC vs DESC, oldest-vs-newest, index-vs-key,
   inclusive-vs-exclusive. State which one this code is.
4. **<bound parameter / injection safety>** — for SQL: is the parameter bound or
   string-interpolated? Cite the exact bind type.
5. **<mock vs real semantic equivalence>** — if there's both a Postgres and a Mock
   impl of the same trait, trace a 5-row example through both. Same shape?
6. **<simpler alternative>** — is there a smaller change that achieves the same goal?
   Even if no, the question itself surfaces issues.

## Technical lens checks

<bullet list specific to chosen skill — read the skill brief before writing this prompt>

For coke-eng:rust-code-review:
- `Arc<dyn Trait>` for repositories; no concrete types leaking
- No `unwrap()` / `expect()` in production code
- Errors flow `DomainError → UsecaseError → ApiError`
- `mod.rs` declaration-only
- DTOs stay in handler layer; domain doesn't import infra
- Imports tidy (no orphans from removed code)

For react-best-practices:
- No barrel imports
- `"use client"` only at the smallest island
- No inline component definitions inside components
- Suspense / loading / error states present
- No defer-able event listeners on the hot path

## What to report

Use **blocker / major / nit** priorities. Cite `file:line` per finding.

For each finding (1-3 sentences):
- One-sentence finding citing `file:line`
- One-sentence WHY it matters (user-facing or correctness consequence)
- One-sentence FIX suggestion

If you find nothing, list:
(a) which files you read end-to-end
(b) which trace examples you actually walked through the code (not just claimed)
(c) which technical-lens checks you ran

Cap **500 words**. Lead with blockers. Skip "the change looks good" framing.

## Pre-acknowledged scope notes (do NOT flag)

<list any scope expansions you've already accepted, e.g., "Implementer also tightened
DTO field types from String to typed (Uuid, DateTime, RunState). Wire format unchanged.
This is acceptable scope creep matching the spec's code blocks.">
```

---

## Tips for writing good dispatch prompts

1. **Concrete file paths beat abstract descriptions.** "`src/foo/bar.rs`" beats "the bar module".
2. **Quote code snippets where they prevent ambiguity** — especially function signatures and field lists.
3. **State the constraint AND the reason** — "Do NOT touch X (it's batch Y's scope)" lets the implementer judge edge cases.
4. **Acknowledge transitional state** — without this, both implementer and reviewer waste cycles flagging things you already know are temporary.
5. **Request a specific report format** — saves you parsing free-form output.
6. **Don't over-prescribe** — leave room for the implementer to apply judgment. Brief like a smart colleague, not a junior with a checklist.

## Skill propagation summary

Three slots, two sourced from the project config (Phase 0), one always-on baseline:

| Dispatch | Always-on | User-selected slot | Sourced from |
|---|---|---|---|
| Plan writer (controller, Phase 2) | `/karpathy-guidelines` + `/scrutinize` | Implementation skills | `<project>/.claude/coke-feature-implementation.json` → `implementation_skills` |
| Plan critic (Phase 2.5) | `/karpathy-guidelines` + `/scrutinize` | Implementation skills | same — as a lens for judging the plan's approach |
| Implementer | `/karpathy-guidelines` | Implementation skills | same — subset matching the affected component |
| Spec reviewer | `/karpathy-guidelines` | (none — pure spec compare) | — |
| Quality reviewer | `/karpathy-guidelines` + `/scrutinize` | Review skills | `<project>/.claude/coke-feature-implementation.json` → `review_skills` |

Implementation skills and Review skills may overlap (e.g., `/react-best-practices` is useful in both slots). Both lists live in the project config so they don't get re-asked every invocation. The user can edit the JSON by hand or trigger a re-interview with "reconfigure".

When dispatching, **filter the relevant subset** for the affected component — don't dump every skill into every prompt:
- Implementer dispatch for a Rust file → only `/coke-eng:rust-*` entries from `implementation_skills`
- Implementer dispatch for a `.tsx` file → only `/coke-eng:nextjs-*`, `/react-*` entries
- Quality reviewer same logic against `review_skills`
