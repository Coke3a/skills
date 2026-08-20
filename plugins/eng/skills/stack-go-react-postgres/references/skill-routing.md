# Which skill, which stage

> The tables below name skills from several sources. Only `coke-eng:` names ship in this
> plugin. `coke-product:` and `coke-productivity:` are sibling plugins, and unprefixed names
> (`scrutinize`, `impeccable`, `superpowers:*`, `debug-mantra`, `code-review`) come from
> elsewhere entirely. Route to whatever is installed; skip the row when it is not.

**The first question is not which skill. It is: does this project have a design yet?**

The answer splits the work in two, and the skills split with it. **Phase A produces the
artifacts Phase B consumes**, and Phase B is not merely inconvenient without them:
`coke-eng:go-tdd-feature-workflow` opens with a **hard entry gate** that stops dead unless the
endpoints, the request/response shapes and the schema are already written down somewhere
quotable.

Both phases have the **same three lanes — contract · schema · interface.** Only what runs in
each lane changes. Two skills appear in both phases, used for different halves of themselves;
that is noted where it happens.

`karpathy-guidelines` applies in both phases and at every stage: surgical scope, every
changed line traces to a stated criterion, nothing speculative.

## The design the project must bring

⚠ **The project has to bring one thing of its own: its product design.** This baseline is the
*how* and never the *what*. **Either form counts: a design document in the project, or a
design you can state as context.** What does not count is neither — and a project in that
state has not reached Phase B.

⚠ Two skills hold that to a stricter standard than *somebody knows*.
`coke-eng:go-tdd-feature-workflow` needs its four facts **quotable from a source**, which stated
context can be. `impeccable` needs an actual **file** — `PRODUCT.md` — and diverts to
`impeccable teach` when there is none, which is precisely the step that turns context into
that file.

## Phase A · Design — when there is no design yet

Three lanes, independent of each other, so run them in parallel. Each ends in a **written
artifact**, not in a conversation.

| Lane | Skill | What it must leave behind |
|---|---|---|
| **contract** | `api-and-interface-design` | every endpoint's method and path · request and response shapes · the single error-code table · the pagination rule and its ceiling |
| **schema** | `postgres-best-practices:supabase-postgres-best-practices` — its **`schema-` category** | tables, columns, keys, constraints, indexes — written out as the first `migration/` pair |
| **interface** | `impeccable shape` — ⚠ but `impeccable teach` **first**, if there is no `PRODUCT.md` | screens, states, and the design system, before a component exists |

Around them:

| | |
|---|---|
| Idea → spec | `superpowers:brainstorming` → `superpowers:writing-plans` |
| A spec already exists — check it before trusting it | `coke-eng:flow-spec-review` |
| Unfamiliar workspace | `coke-productivity:agent-workspace-orientation`, first — sibling plugin; if absent, `git rev-parse --show-toplevel` |

⚠ **`impeccable` will not start without two files** — `PRODUCT.md` (**required**) and
`DESIGN.md` (optional, wanted). Its loader looks at the **project root first**, then falls
back to `.agents/context/` and `docs/`, so either location works; pick one and stay there. A
missing `PRODUCT.md` is a hard block: the skill diverts to `impeccable teach` and returns.
Both files are **product content, so neither belongs in this baseline** — the repository
layout reserves the slot and nothing more. ⚠ **Nothing else in this baseline produces
`PRODUCT.md` either**, so on a greenfield project the interface lane's first action is
`impeccable teach`, not `impeccable shape`. The failure mode to watch for is quieter than a
hard block: the lane gets **skipped entirely** because it would not start, the other two
lanes finish, and the project reaches implementation with a contract, a schema, and no
interface design at all. ⚠ The skill documents its loader as a
**project-relative** path (`node .claude/skills/impeccable/scripts/load-context.mjs`); where
the skill is installed per-user rather than vendored into the repository, that command is
`~/.claude/skills/impeccable/scripts/…` instead.

⚠ **Phase A's exit condition is written by Phase B, not by Phase A.** It is the four facts
`coke-eng:go-tdd-feature-workflow` demands before it will write a line: **endpoints ·
request/response shapes · database schema · one quotable sentence on what the feature is
for.** If any of the four cannot be quoted from a document, design is not finished — and
starting Phase B anyway only moves the stop from a document into a stack trace.

## Phase B · Implement — when the design exists

Same three lanes, different skills. **`coke-eng:flow-feature-implementation` drives the whole feature
once** — not once per lane, and not once per batch.

| Lane | Stage | Skill |
|---|---|---|
| **backend** | test first — the loop that decides the signatures | `coke-eng:go-tdd-feature-workflow` |
| **backend** | structure, naming, error taxonomy, the sqlc/pgx pattern | `coke-eng:go-clean-architecture` |
| **db** | queries, indexes, RLS, locking, a slow plan | the postgres skill — its `query-` · `security-` · `lock-` · `data-` categories |
| **frontend** | correctness and performance | `react-best-practices` |
| **frontend** | the interface itself | `impeccable` |

Closing out, in order:

| | |
|---|---|
| Adversarial review of the change | `scrutinize` — mandatory |
| UI edge states a screenshot never reaches | `coke-product:design-scrutinize` |
| A gate went red | `debug-mantra` + `superpowers:systematic-debugging` |
| Before calling it done | `superpowers:verification-before-completion` → `code-review` |
| Multi-session or cross-repo work | `coke-productivity:agent-project-progress` |

## ⚠ The six places a skill has to be read against this baseline

Each is a line that is correct in its own context and wrong in this one. They are listed so
that the divergence is a decision on the record rather than a surprise at `sqlc generate`
time.

| | |
|---|---|
| ⚠ **`coke-eng:go-clean-architecture`'s `templates/sqlc.yaml` assumes a standalone repo** | Both skills use golang-migrate and a `migration/` directory now — the tool matches. What still differs: that skill's sqlc.yaml is written for a Go-only repo, so it reads `schema: "migration"` from beside itself. Here sqlc.yaml lives in `backend/`, one level below the repository root where `migration/` sits — copying that file unedited reads the wrong path and finds no schema. The two-line `templates/sqlc.yaml` here (`schema: "../migration"`) is the fix |
| ⚠ **The same skill says the pgx pool is a singleton** | one pool per role — see `environments.md`. Composition root only; no layer boundary moves |
| ⚠ **`coke-eng:go-tdd-feature-workflow` puts the repository layer out of scope, deliberately** | its handler tests wire a real usecase to a **fake** repository, so grants, RLS, constraints and pgx mapping are proven by **nothing in the skill set at all.** `make test-db` is where that closes — and is the reason that gate exists rather than being a nice-to-have |
| ⚠ **`coke-eng:go-clean-architecture` says to use a transaction only for multi-write atomicity** | *"Use a transaction only when one usecase requires multiple writes to commit atomically."* ⚠ **Under RLS that is false, and its canonical `gen.New(pool)` constructor is the shape that fails silently** — see `conventions.md` |
| ⚠ **`react-best-practices` is written for Next.js in more categories than one** | below |
| ⚠ **The postgres skill's RLS examples assume one platform's identity function — and call it bare in the predicate** | below, for the platform half. The *performance* half is the `(select …)` wrapper |

## ⚠ `react-best-practices` — what a Vite SPA actually gets

**Category 3 (Server-Side Performance) is Next.js-only** and does not apply here. That much
is easy. What is easy to miss: **five more rules outside category 3 prescribe a Next.js
API**, and one of them describes a situation a Vite SPA is never in.

| Rule | Prescribes | Here |
|---|---|---|
| `bundle-dynamic-imports` · `bundle-defer-third-party` | `next/dynamic` | `React.lazy` + `<Suspense>`, or a bare `import()` |
| `rendering-script-defer-async` | `next/script` with `strategy` | a plain `<script defer>` |
| `bundle-barrel-imports` | `optimizePackageImports` | import the deep path, or rely on the bundler's tree-shaking |
| ⚠ `rendering-hydration-suppress-warning` | `suppressHydrationWarning` for SSR mismatches | ⚠ **inert — a Vite SPA does not hydrate.** There is no server render to mismatch against |

The **principle** behind all five holds. The **API** in four of them does not, and the fifth
has no subject here. Categories 1, 2, 4–8 otherwise apply unchanged, and **category 1 is the
one that bites**.

## ⚠ The postgres skill, and the one thing this baseline will not decide for you

**Whether the database is a managed platform or a plain PostgreSQL is a per-project fact, and
this baseline does not fix it.** Both are in scope; the pin is a version, not a vendor.

That open decision is exactly what the skill's `security-` category depends on. Its policies
are written against **one platform's identity function and role names** — read them as a
*shape*, and substitute whatever this project's session identity actually is before running a
line of it.

⚠ **One rule in it is portable either way, and it is the expensive one.** A volatile function
called bare in a policy predicate is evaluated **once per row** — a million rows, a million
calls. Wrapping it as `(select …)` turns it into a one-time initplan. That is PostgreSQL's
own behaviour rather than any platform's, so it holds whichever way the decision above goes,
and it is the highest-value thing the skill contains.

The same caution covers `conn-pooling`, which recommends an external pooler. This stack
already pools in-process. **An external pooler is a second pool, an amendment to the
connection budget, and a change in how prepared statements behave** — verify it against pgx's
exec mode before adopting one, rather than adding it because a rule file said so. ⚠ **And if
the project relies on RLS, the pooling *mode* is a correctness question, not a performance
one**: statement-mode pooling cannot hold a transaction open, so `SET LOCAL` never reaches
the query it was meant to scope.

## Deliberately in neither phase

| | |
|---|---|
| ⚠ **`coke-eng:go-performance-optimization`** | measurement-first by its own first rule. Invoke it when a **stated budget has been measured and missed** — never speculatively, and never while building a feature |
| ⚠ **CD, and any skill that names a deploy target** | where a deploy lands is a project decision, so no skill is named for it here |

⚠ **When a gate goes red, diagnose the failure. Never weaken the check to pass it.** A gate
that has been loosened once is not a gate.

---

**Every divergence above is quoted from the skill files themselves** — each skill's
`SKILL.md` plus the `references/`, `templates/` and `workflows/` files beneath it, never the
one-line description, which is where they disagree.

⚠ **Skills are edited by their authors, so this file is the second thing to re-check after
the version pins.** A divergence that has since been fixed upstream is worse than a missing
one: it is a line telling you to work around a problem that no longer exists, and it will be
believed.
