# Repository layout

One repository. Top-level directories are the boundary; **what is inside each is that
stack's business.**

```text
<project>/
├── docker-compose.yml   ⚠ DEVELOPMENT ONLY — the disposable local stack
├── Makefile             the gate set — one command for CI and a laptop alike
├── backend/             the Go API
├── frontend/            the SPA
├── frontend-admin/      a second SPA, when there is one        (omit if not)
├── migration/           golang-migrate up/down pairs
├── local/               development-only bootstrap — postgres-init/ and nothing deployed
├── scripts/             the gate bodies (version-check.sh and friends)
├── deployment/          one compose file per deployable unit — production
├── observability/       Prometheus · Loki · Tempo · Alloy · Grafana + their compose
└── docs/
    ├── DECISIONS.md     ⚠ where "record it" means. See below
    └── …                the project's own design · the design phase's context files,
                         unless they sit at the root
```

⚠ **`docs/DECISIONS.md` is load-bearing, and it is the file most likely never to get
created.** This baseline repeatedly says to *record* something — a divergence from a
specialist skill, a gate whose tool was chosen, a pin deliberately held back, how far the
roles were split, which of the two RLS wiring shapes was taken. **A decision that was made in
a conversation and never written is a decision the next person will silently reverse**, and
the reversal will look like cleanup. One line each, with the reason, is enough; the reason is
the part that survives.

| Directory | Owns | Must never contain |
|---|---|---|
| `backend/` | the API · **the instrumentation contract** — metric names, log field set, the label allow-list | any deployment topology |
| `frontend/` | the SPA and its build | a hardcoded API host |
| `migration/` | every DDL statement that has ever run, in order, with its reverse | anything the application runs at request time |
| `deployment/` | compose files, `Caddyfile`s, nothing else | application source · **anything development-only** |
| `observability/` | collection, storage, dashboards, alert rules | **a metric or log label** — that is `backend/`'s |
| `docker-compose.yml` | the local stack: disposable Postgres, one-shot migrate, backend, frontend dev server | ⚠ **any credential, host or volume that a deployed environment also uses** |
| `local/` | the local stack's bootstrap — the role-creation SQL the compose file mounts | anything a deployed environment reads. It is the development twin of `deployment/` |

⚠ **The label allow-list lives under `backend/` and only there.** A label added under
`observability/` is the defect this boundary exists to prevent: the two directories are in
one repository, so nothing but this rule keeps them apart.

⚠ **Two of these directories are reserved and empty, and this skill does not fill them.**
`deployment/` and `observability/` are named and constrained here, but **no example of
either appears**, so a deployed environment cannot be produced from this skill alone. That
is deliberate for `deployment/`, whose contents follow the deploy target. It is **not**
deliberate for `observability/`: retention, exporters and the collection pipeline all have
to land in files that do not exist yet. ⚠ **Watch the boundary while writing them** — CI
path filtering requires that a change under `observability/` never redeploys the backend,
while the agent that collects container output has to run beside the application. One
directory, two placements; decide it once, in the project.

## `migration/` — golang-migrate

**Nothing changes a schema by hand.** Every DDL statement that has ever run is a file here,
in order, with the statement that reverses it beside it. That is what makes a schema the
same on four laptops and one server without anybody comparing them.

```text
migration/
├── 20260813142530_create_<table>.up.sql          ⚠ one version, two files —
├── 20260813142530_create_<table>.down.sql           a pair shares its timestamp
├── 20260814091205_add_<table>_<column>.up.sql
├── 20260814091205_add_<table>_<column>.down.sql
└── README.md          which role runs these, and the reset procedure
```

`templates/sqlc.yaml` is the whole configuration:

```yaml
# backend/sqlc.yaml — there is no ORM, so the generator reads the schema from here
schema:  "../migration"      # ← repository root. The only place a schema exists
queries: "db/queries"        # ← inside backend/. The boundary: the Go stack's business
```

sqlc supports golang-migrate natively and ignores every `.down.sql`. **The schema lives
outside `backend/` because it is not the backend's** — it outlives any one service, and a
second copy kept for the generator is a copy that goes stale in silence. Run `sqlc generate`
after either directory changes, and commit the diff.

### ⚠ The five things that bite

| | |
|---|---|
| ⚠ **Timestamps, never `-seq`** | two branches both create `000003` and the merge is a *"duplicate migration version"*, or a renumber silently changes what was reviewed. **And sqlc parses in *lexicographic* order** — fixed-width timestamps make lexicographic and numeric order identical by construction, where `10_` sorts before `9_` |
| ⚠ **The `dirty` flag blocks everything** | golang-migrate keeps one row of `schema_migrations (version, dirty)`. A migration that fails partway leaves it dirty and **every later run refuses**, which in CI means **all future deploys**. The only exit is `migrate force V`, which *sets the version without running anything* — an **assertion that the schema really is at V**, not a repair. Assert the wrong V and nothing errors until much later. The runbook for this has to exist before the first failure |
| ⚠ **One file is one transaction — with one exception** | statements sent in one `Exec` run inside a transaction, so a multi-statement file is all-or-nothing. Keep `x-multi-statement` off. Some statements cannot run in a transaction block — `CREATE INDEX CONCURRENTLY` is the one that comes up — so **put such a statement alone in its own file** |
| ⚠ **A `down` that drops a column destroys data, and the tool will not warn you** | rollback is safe for *reversible* DDL. For a destructive change the honest `down` either restores from backup or refuses — say which, in the file. And every `down` is proven by `make migrate-check`: one nobody has run is a file, not a rollback plan |
| ⚠ **A compose-started Postgres hands you a superuser, and a superuser bypasses RLS** | everything works locally and fails in production. `migration/README.md` is where to say so, because it is what somebody reads while holding exactly that connection |

**A migration that has run anywhere is history** — never edit, renumber or delete one; fix
forward with a new pair. **Migrations run as the schema owner and the application never
does**, from the `migrate/migrate` image in compose and CI alike, so the step is the same
command in both.

### Where to check the non-obvious claims

| Claim | Check it against |
|---|---|
| sqlc reads golang-migrate directly and **ignores every `.down.sql`**; it parses **in lexicographic order** | `docs.sqlc.dev/en/latest/howto/ddl.html` |
| `schema_migrations (version bigint primary key, dirty boolean)` · `ErrDatabaseDirty` · `force V` sets the version **without running anything** | `golang-migrate/migrate` — `MIGRATIONS.md`, `database/postgres/`, `cmd/migrate/README.md` |
| a multi-statement migration file executes **inside one transaction** on PostgreSQL | the same repository's `database/postgres/README.md` |
