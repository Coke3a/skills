# The local stack — `docker-compose.yml`

⚠ **`docker-compose.yml` at the repository root is development. `deployment/` is production.
They are two different files and neither is derived from the other.** The local database is a
**disposable container** that shares no network, no volume and no credential with any deployed
environment. A local file that can reach production is the single worst outcome this
separation prevents.

The file itself is `templates/docker-compose.yml`, with `templates/backend.Dockerfile`,
`templates/vite.config.ts` and `templates/postgres-init.sql` — **the compose file does not
work without all three.**

⚠ **The roles are the piece people leave out, and the stack simply does not start without
them.** The compose file hands the backend an `app_user` DSN and the migrate step an
`app_owner` DSN; nothing creates either until `postgres-init.sql` is mounted at
`/docker-entrypoint-initdb.d/`. **Roles are cluster-level, so they are not a migration** — a
migration runs inside one database and a role outlives it. This is the one piece of SQL in
the project that is infrastructure rather than schema, and it is why it sits beside the
compose file instead of in `migration/`.

A second SPA is a second service, on `5174`, with its own `DEV_PROXY_TARGET`. **Caddy is not
in here** — locally Vite's proxy does Caddy's job.

## ⚠ The traps — every one of these looks like it works

| | |
|---|---|
| ⚠ **The postgres volume is `/var/lib/postgresql`, not `…/data`** | **PostgreSQL 18 moved it** — the image sets `PGDATA=/var/lib/postgresql/<major>/docker` and declares `VOLUME /var/lib/postgresql`. Mount the old path and the named volume lands on a directory nothing writes to; the cluster goes to an *anonymous* volume, `docker compose down && up` returns an **empty database**, and an orphan is left behind each time |
| ⚠ **`pg_isready` needs `-h`** | without a host it uses the **unix socket**, and the entrypoint's `initdb` server listens on the socket with TCP closed. The healthcheck goes green while the database is still initialising, `migrate` gets connection-refused, and `backend` never starts. Worst on a cold first `up` |
| ⚠ **`migration/` must contain a pair before any of this runs** | `migrate up` on an empty directory fails rather than no-ops, and `service_completed_successfully` then blocks `backend` forever. The first migration is the design phase's output and a **prerequisite** of `docker compose up` |
| ⚠ **`node_modules` must not be shadowed** | `./frontend:/app` mounts your source over the image's `/app`, hiding what the build installed. The `/app/node_modules` anonymous volume masks that path back out — without it you get *"vite: not found"*, or a macOS/arm module inside a linux container |
| ⚠ **File watching does not cross a bind mount reliably** | inotify from a macOS or Windows host often never reaches the container, so **edits save and nothing reloads**. Polling fixes it, but Vite's docs are blunt that it *"leads to high CPU utilization"* — wire it to an env var, off by default |
| ⚠ **`strictPort: true` is not optional here** | Vite otherwise *"automatically tries the next available port"*, and inside a `5173:5173` mapping the published port then points at nothing — connection-refused with no error anywhere |
| ⚠ **Vite 8 moved the HMR socket options** | `server.hmr.*` is deprecated in favour of `server.ws`. Auto-synced, so old configs work — but any guide written for Vite 6 points at the deprecated shape. Needed only when the published port differs from the container port |
| ⚠ **`npm ci`, never `npm install`, inside the container** | `npm install` rewrites the lockfile with linux-specific optional dependencies (rollup, esbuild), and **committing that breaks CI**. Add dependencies on the host |
| ⚠ **`CGO_ENABLED=0` in the Dockerfile, cgo *on* for `make test`** | the static binary is what makes `distroless/static` possible — but `-race` **requires cgo and a C toolchain**. A CI image built only for `CGO_ENABLED=0` cannot run the test gate |
| ⚠ **The backend is not hot-reloaded** | it is a built image and `docker compose up` reuses it; a Go change needs `--build`. The frontend gets HMR and the backend gets nothing — an asymmetry worth knowing rather than rediscovering |
| ⚠ **The whole `server` block in `vite.config.ts` is invisible to `vite build`** | it looks like dev cruft; removing it breaks the container while the build keeps passing |

## What it buys, and what it costs

`docker compose up` on a clean checkout gives a migrated database, a running API and a live
frontend, on a machine with **no Go, Node or Postgres installed**. Reset is
`docker compose down -v && docker compose up` — the named volume is the only state.
`depends_on` conditions, not `sleep`, mean the backend cannot start against an unmigrated
database.

⚠ **Keep the host-side escape hatch and say so out loud.** Containerised HMR is slower than
native, polling burns CPU, and `npm ci` runs on every `up`. For the fastest frontend loop:
`docker compose stop frontend && cd frontend && npm run dev` — same port,
`DEV_PROXY_TARGET` falls back to `localhost:8080`, nothing else changes. That indirection is
what buys it.

⚠ **And the gap it opens: nobody exercises the production serving path locally any more.**
See the closing section of `proxy-caddy.md`. The compensating control is the post-deploy
smoke test.

⚠ **Two things this file must never become.** `POSTGRES_USER` is a superuser, and a superuser
bypasses RLS — **so nothing but the init script uses it.** Migrations run as `app_owner` and
the application as `app_user`; if either DSN ever points at `POSTGRES_USER`, every policy in
the database goes quietly inert and the local stack stops resembling production in the one
way that matters. And **never restore a production dump into this container**: that is
personal data on a laptop with no access control and no retention clock. Generate what you
need instead.

## Where to check the non-obvious claims

| Claim | Check it against |
|---|---|
| `server.host` · `strictPort` · `watch.usePolling` and its CPU warning · ⚠ the **Vite 8 move** of the HMR socket options from `server.hmr` to `server.ws` | `vite.dev/config/server-options` |
| PostgreSQL 18's `PGDATA` and `VOLUME` paths | the `docker-library/postgres` Dockerfile for the `18/alpine` variant |
