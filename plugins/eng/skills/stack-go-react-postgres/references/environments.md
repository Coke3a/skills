# Environments

**One image runs in every environment.** Configuration comes from the environment, never
from the image. A build that bakes in an environment is a build that must be repeated per
environment, and the artifact that was tested is then not the artifact that ships.

| | local | staging | production |
|---|---|---|---|
| Runs as | `docker compose up` from a clean checkout | compose on a host | compose on a host |
| Postgres | container | managed | managed, private networking |
| TLS | none — Vite's dev proxy | Caddy, ACME staging endpoint | Caddy, real cert + HSTS |
| Observability | optional | full | full |

⚠ **Staging points Caddy at Let's Encrypt's *staging* ACME endpoint** (`acme_ca`).
Production rate limits are per-account and per-name, and a staging environment that
re-deploys ten times a day will exhaust them against the names you actually need. Its
certificates will not be trusted by a browser, which is correct — that is what staging is.

## Ports

| | |
|---|---|
| backend | `:8080` |
| primary SPA (dev) | `:5173` |
| second SPA (dev) | `:5174` |
| Postgres | `:5432` |

⚠ **Two SPAs get two ports, always.** One dev server serving both reproduces a topology that
exists in no other environment, and the cookie behaviour it produces is not production's.

## Environment variables

| Variable | Where | Note |
|---|---|---|
| `DATABASE_URL_*` | backend | one per database role — never one shared superuser DSN |
| `PORT` | backend | |
| `ENV` | backend | `local` · `staging` · `production` — the `env` label in observability |
| `LOG_LEVEL` | backend | |
| `OTEL_EXPORTER_OTLP_ENDPOINT` | backend | absent = tracing off, which is a valid local state |

⚠ **No secret is ever committed, and no secret is ever in an image.** `make secret-scan` is
the gate; a committed `.env` holding live credentials is the single most common way this
baseline gets violated.

## ⚠ Same-origin is a rule, not a preference

`vite dev` on `:5173` calling a Go process on `:8080` is **two origins**. A session cookie
then either does not work, or is made to work by relaxing `SameSite`/`Domain` — **and that
relaxation ships**.

> **In development, Vite's dev-server proxy forwards `/api/v1` to the backend. In
> production, Caddy does. The browser sees one origin in both.**

**Consequence, and it is free:** the API base is the relative string `/api/v1`. There is
**no build-time API URL and no `VITE_API_URL`**. If one appears, same-origin has been broken
somewhere upstream.

⚠ **A second browser-facing SPA needs its own proxy**, not a shared API host. A host-only
cookie set on one hostname *cannot* be sent to an API on another — that is not a CORS
setting; the cookie never leaves the browser for that host at all.

⚠ **`DEV_PROXY_TARGET` is not the `VITE_API_URL` this forbids.** It is the opposite of it,
and the distinction is **where the value ends up**: `VITE_API_URL` is read by the **browser
bundle** at build time and ships baked into `dist/`, making the browser call a second origin.
`DEV_PROXY_TARGET` is read by the **dev-server process** at startup, never ships, and keeps
the browser on one origin. **Nothing named `VITE_*` may point at an API host** — the prefix
is what makes a value reach the bundle.

## Database roles

**The application never connects as the owner or as a superuser.** At minimum two roles: one
that owns the schema and runs migrations, one that the application uses and that holds only
the grants it needs. Split further where the data justifies it (one role per privilege
class), and **give each its own DSN and its own readiness check** — a readiness probe on one
role says nothing about the others.

⚠ **This must be true locally on day one.** A local superuser DSN is the trap: RLS is
bypassed, every query works, and the first time the real grants are exercised is in staging.
The mechanism locally is `templates/postgres-init.sql`, mounted into the Postgres container —
**roles are cluster-level, so they are not a migration**, and without that file the compose
stack hands out DSNs for roles that do not exist and nothing starts.

⚠ **"Not the superuser" is only half of it — the owner bypasses RLS too.** A role reading its
*own* tables is exempt from their policies unless the table is set to force otherwise. So the
role that runs migrations must not be the role the application uses, or the policies are
decoration. See the RLS section in `conventions.md`.

⚠ **One pool per role — not one pool.** `coke-eng:go-clean-architecture` says the
`*pgxpool.Pool` is *"a singleton owned by `main()`"*. **Keep the ownership; drop the
singleton.** `main()` builds one pool per role from that role's own DSN and injects the right
one into each repository. The repository constructor already takes a pool as a parameter, so
this is a **composition-root change and touches no layer boundary** — which is why it can
differ from the skill without arguing with it. A single shared pool makes the role split
decorative: the DSNs exist, the grants exist, and every query still runs as whoever won the
race to be first.

⚠ **Three consequences of that split, all structural rather than stylistic.**

- A transaction is bound to one connection as one role, so **it cannot span two pools** —
  while the idempotency record and the audit row both say *"in the same transaction"*, which
  puts a floor under how finely the roles can be divided.
- Every pool is the same Go type, so a mis-wire in `main()` compiles, passes `go vet`, and
  **works locally** — the local superuser bypasses RLS — then surfaces in staging.
- The pool size **divides** across the roles against the connection budget rather than being
  copied per role, or the connection alert fires at rest.

How far to split, and how to make the split safe, is a project decision. These are the three
facts that decide it.
