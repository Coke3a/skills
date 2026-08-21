---
name: stack-go-react-postgres
description: The big-picture engineering baseline for a Go + React (Vite) + PostgreSQL project — repository layout, version pins and the compatibility traps that decide them, the local Docker Compose stack, local/staging/production environments, same-origin proxying through Caddy, database roles, the gate set, CI shape, and observability. Its other half is routing — it decides whether the project is still in design or already in implementation, and names which specialist skill owns each lane. Use this whenever someone starts or restructures a full-stack project, or asks how one should be put together — "set up a new Go + React project", "start a new project", "what versions should I pin", "how do I run this locally", "why does my session cookie not work in dev", "what should CI check", "which skill do I use for this" — and read it before any deep work so the specialist skill is invoked with the right structure around it. Do NOT use it to do the deep work — a Go feature is coke-eng:go-clean-architecture, a React component is react-best-practices, a query or a policy is the postgres skill, and shipping to a server is coke-eng:ops-docker-vm-deploy.
---

# Full-Stack Baseline — Go + React (Vite)

The decisions that are the same on every project, taken once: stack, repository layout,
environments, gates, CI, observability — and, split by whether the project has a design
yet, which skill runs at which stage.

This skill is the **how** and never the **what**. It names no screen, no endpoint, no
table, and it cannot be used on its own.

## Positioning — what this owns, and what it hands off

This is the **overview** skill. It decides the shape of the project and who does the work;
it does not do the work.

| It owns | It hands off |
|---|---|
| Where directories go and what each may never contain | Go feature architecture → `coke-eng:go-clean-architecture` |
| Which versions, and the peer constraints that decide them | The feature test loop → `coke-eng:tdd-feature-workflow` |
| The local stack, ports, env vars, same-origin | React correctness and performance → `react-best-practices` |
| The proxy's four jobs | Screens, states, design system → `impeccable` |
| Which roles connect to the database, and that it must be more than one | Queries, indexes, policies, locking → `postgres-best-practices:supabase-postgres-best-practices` |
| The gate set and the CI shape up to the artifact | Shipping the artifact → `coke-eng:ops-docker-vm-deploy` (or whatever the target is) |
| That an observability store is not an audit log | Deep design of any of the above → the design lanes in `references/skill-routing.md` |

**When a question gets deeper than structure, stop and invoke the specialist.** This skill
carries the trap and the boundary, never the recipe. If you find yourself writing a policy,
a handler, a component or a query plan while inside this skill, you are in the wrong skill.

## Use this when

- Starting a new full-stack project, or restructuring one that grew without a plan.
- Deciding versions, or checking whether the pinned set still holds together.
- Setting up or debugging the local development stack (compose, ports, proxy, HMR, migrate).
- A cookie, a `404` on refresh, a stale white screen, or a `504` points at the serving path.
- Deciding what CI must check before an artifact exists, and what it must never do.
- Standing up logs / metrics / traces, or deciding what must not go into them.
- Somebody asks *"which skill do I use for this?"* — that question is **Routing**, below.
- Before any deep work, as the orientation pass: it takes one read and it decides which
  specialist gets invoked with what around it.

## Do not use this when

- The task is a single deep artifact — a handler, a component, a migration's SQL, a query
  plan, a policy. Route it (table above) and let the specialist own it.
- The stack is Next.js. This skill is specifically Vite SPA + Go API.
- The project has no product design and nobody can state one. That is not a structure
  problem; it is Phase A, below, and it comes first.
- The question is where to deploy. That is deliberately out of scope — see `references/ci.md`.

## The one idea everything follows from

**One image, one origin, one schema, one command.**

| Question | Answer |
|---|---|
| Why is there no `VITE_API_URL`? | Vite's dev proxy and Caddy both make the browser see one origin, so the API base is the relative `/api/v1` everywhere. |
| Why is `migration/` at the repository root, not under `backend/`? | The schema outlives any one service, and a second copy kept for the generator goes stale in silence. |
| Why does configuration come only from the environment? | One image runs everywhere. A build that bakes in an environment must be repeated per environment, and the artifact that was tested is then not the artifact that ships. |
| Why are the gates `make` targets? | So CI and a laptop run the identical command. A gate that differs between them is not a gate. |

## Routing — the first question is not which skill

**It is: does this project have a design yet?** The answer splits the work in two, and the
skills split with it. Phase A produces the artifacts Phase B consumes. Phase B starts when
the intended behavior is clear enough to state acceptance criteria; endpoint, interface, and
schema artifacts are required only for the parts of the feature that actually use them.

**Either form of design counts: a document in the project, or a design you can state as
context. What does not count is neither.**

Both phases have the same three lanes — **contract · schema · interface**.

**Phase A · design** — run the lanes in parallel; each ends in a written artifact, not a conversation.

| Lane | Skill |
|---|---|
| contract | `api-and-interface-design` |
| schema | the postgres skill, its `schema-` category → the first `migration/` pair |
| interface | `impeccable shape` |

**Phase B · implement** — `coke-eng:flow-feature-implementation` drives the whole feature once.

| Lane | Skill |
|---|---|
| backend, test first | `coke-eng:tdd-feature-workflow` |
| backend, structure | `coke-eng:go-clean-architecture` |
| db | the postgres skill, its `query-` · `security-` · `lock-` · `data-` categories |
| frontend | `react-best-practices` · `impeccable` |

`karpathy-guidelines` applies in both phases and at every stage.

⚠ **Six skill lines are correct in their own context and wrong in this one** — go-clean-architecture's
sqlc.yaml path assumption, the singleton pool, the repository layer nobody tests, transactions under
RLS, Next.js-only React rules, and one platform's identity function. Read
`references/skill-routing.md` before following any of them, and record the divergence as a
decision rather than discovering it at `sqlc generate` time.

## Core rules

- **Nothing changes a schema by hand.** Every DDL statement that has run is a golang-migrate
  pair in `migration/`, at the repository root, and sqlc reads its schema from there.
- **A migration that has run anywhere is history.** Never edit, renumber or delete one — fix
  forward with a new pair. Timestamps, never `-seq`.
- **The application never connects as the owner or as a superuser**, and it must be true
  locally on day one. A local superuser DSN bypasses RLS, so every query works and the real
  grants are first exercised in staging.
- **Same-origin is a rule, not a preference.** Nothing named `VITE_*` may point at an API host.
- **`docker-compose.yml` at the root is development. `deployment/` is production.** They share
  no credential, no volume and no network, and neither is derived from the other.
- **No secret is committed and no secret is in an image.**
- **When a gate goes red, diagnose the failure — never weaken the check to pass it.** A gate
  that has been loosened once is not a gate.
- **The project brings its own product design.** This skill holds no product decision, no
  domain model, no schema, no endpoint, no screen. If one appears in it, that is a defect.

## Workflow

1. **Orient.** Is this a fresh project, an existing one, or a debug? For an unfamiliar
   workspace, run `coke-productivity:agent-workspace-orientation` first if that plugin is
   installed; otherwise establish the git root yourself with `git rev-parse --show-toplevel`
   and confirm which repository the change belongs to before touching anything.
2. **Answer the routing question.** Design or implement? **Phase A does not block steps 4–6**
   — nothing in the repository layout, the local stack or the gate set depends on a product
   decision, which is the whole reason they live here. What it blocks is **feature code**, and
   one structural thing: the first `migration/` pair is Phase A's schema lane output, not
   yours to invent. Phase A's exit condition is written by Phase B: endpoints ·
   request/response shapes · schema · one quotable sentence on what the feature is for.
3. **Before writing a version into any file, re-verify it** — `scripts/check-pins.sh` does the
   six upstream lookups in one pass. Skip this step entirely on a question that writes no
   versions. When it does apply it is not optional politeness: a version table read as current
   when it is not is worse than no table at all, and `templates/` names exact image tags that
   will eventually stop existing.
4. **Lay out the repository** — `references/repository-layout.md`, then `templates/`.
5. **Stand up the local stack** — `templates/docker-compose.yml` plus the three files it
   requires. Once `migration/` holds one pair, `docker compose up` on a clean checkout gives a
   migrated database, a running API and a live frontend on a machine with no Go, Node or
   Postgres installed. ⚠ **Before that pair exists it cannot**: `migrate up` on an empty
   directory *fails* rather than no-ops, and `service_completed_successfully` then blocks the
   backend forever. Until Phase A delivers it, `make db` brings up Postgres alone.
6. **Wire the gates before the first feature**, not after — `templates/Makefile`. **Five of the
   nine exit non-zero as shipped** and say why: three because no tool is chosen, two because
   their body is project-specific. They are present and loud about it on purpose.
7. **Hand off.** Every line of feature code belongs to a specialist skill (Routing, above).

## Load more detail

Read the one the current question is about. They are independent.

- Version pins, the peer constraints that decide them, how to re-verify → `references/stack-pins.md`
- Directory boundaries, and `migration/` + sqlc → `references/repository-layout.md`
- Ports, env vars, same-origin, database roles → `references/environments.md`
- Caddy's four jobs and the four counter-intuitive lines → `references/proxy-caddy.md`
- The local compose stack and its eleven traps → `references/local-stack.md`
- API envelope, logging rules, the RLS silent-failure warning → `references/conventions.md`
- Which skill at which stage, and the six divergences → `references/skill-routing.md`
- The nine gates, and which five do not run as shipped → `references/gates.md`
- CI up to the artifact, and where CD stops being this skill's business → `references/ci.md`
- Three stores, retention, labels, health checks, alerts → `references/observability.md`

## Templates

Adapt, do not copy blindly. **Every pinned version in these files is a fact with a
shelf life** — run `scripts/check-pins.sh` first.

- `templates/docker-compose.yml` — the disposable local stack, annotated with its traps
- `templates/postgres-init.sql` — the two roles the compose file hands out DSNs for
- `templates/backend.Dockerfile` — multi-stage, distroless, static
- `templates/vite.config.ts` — the `server` block `vite build` never reads
- `templates/sqlc.yaml` — two lines, and both of them are the boundary
- `templates/Caddyfile` — the four jobs, in the order Caddy actually applies them
- `templates/Makefile` — the nine gates, with the unimplemented ones failing loudly

## Scripts

**One of these is copied into the project and one is not.** A gate has to live in the
repository it gates; a freshness check does not.

- `scripts/check-pins.sh` — **stays here.** The six upstream lookups from
  `references/stack-pins.md` in one pass: npm packages and their peers, Go modules, the Go
  toolchain's stable-vs-rc line, Node LTS status, PostgreSQL, and container tags. Run it
  before writing a version into a file; do not reference its path from project files.
- `scripts/version-check.sh` — **copy into the project's own `scripts/`.** It is the body of
  the `make version-check` gate: Node must agree across `.nvmrc`, any Dockerfile that pins it,
  the CI workflow and the compose image tag; Go across `go.mod`, the Dockerfile and the CI
  workflow. Floating tags fail it, which is the point. A `go 1.27` directive is treated as a
  legitimate minimum rather than a floating pin.

## Related skills

Names prefixed `coke-productivity:` or `coke-product:` ship in sibling plugins. Install them
separately, or skip that step — nothing here depends on them being present.

- `coke-productivity:agent-workspace-orientation` — run first in an unfamiliar or multi-repository workspace.
- `coke-eng:flow-feature-implementation` — drives a feature from spec to code once Phase B starts.
- `coke-eng:flow-spec-review` — check a design before trusting it.
- `coke-eng:ops-docker-vm-deploy` — the deploy half this skill deliberately does not carry.
