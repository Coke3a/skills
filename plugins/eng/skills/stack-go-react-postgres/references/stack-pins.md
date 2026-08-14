# Stack pins — and the couplings that decide them

⚠ **Re-verify this table before using it — it is the one part of this skill that goes stale
on its own.** Every pin below was a real, published release when it was written, and none is
a guess. But the ecosystem moves, and **a version table read as current when it is not is
worse than no table at all**, because it carries authority it no longer has.

`scripts/check-pins.sh` runs the whole check in one pass. The lookups it performs:

| | Read |
|---|---|
| npm packages | `npm view <pkg> version` — then `npm view <pkg> peerDependencies engines`, which is the half that matters |
| Go modules | `proxy.golang.org/<mod>/@latest`, and `/@v/<v>.mod` for that version's `go` directive |
| Go toolchain | `go.dev/dl/?mode=json&include=all` — **stable versus rc**, which the download page alone does not make obvious |
| Node | `nodejs.org/dist/index.json` — LTS codename and status per major, so *Current* is not mistaken for *LTS* |
| PostgreSQL | `endoflife.date/api/postgresql.json` — latest minor, release date, EOL |
| container images | the registry's own tag list, before any tag is written into a compose file |

⚠ **The compatibility section below is the part that actually needs re-reading, not the
table.** A newer version is only usable if its peers moved with it — and the two deliberate
holdbacks exist because they did not.

⚠ **"Latest" and "production-ready" are not the same thing, and two rows deliberately
differ.** The rule: take the newest release whose *whole dependency set* supports it, and let
something newer sit until its ecosystem catches up.

## Backend — Go

| | Pin | Why this one |
|---|---|---|
| **Go** | `1.26.6` | Newest stable at the time of writing; `1.27` was still a release candidate, and an rc is not a production floor. Go supports the last two majors |
| **Fiber** | `v3.5.0` | v3.0.0 GA'd 2026-02-02 — months of production soak. Requires Go ≥ 1.25.0 |
| **pgx** | `v5.10.0` | `pgxpool` directly — no `database/sql` wrapper. Requires Go ≥ 1.25.0 |
| **sqlc** | `v1.31.1` | Generated code stops at the repository layer and goes no further. **Run as a binary, not imported**, so its own `go` directive imposes no floor on `go.mod` — `check-pins.sh go` prints one anyway, and it is not a constraint |
| **golang-migrate** | `v4.19.1` · image `migrate/migrate` | The migration runner. Requires Go ≥ 1.24.0 as a library — **but it is run as a container**, so it imposes no floor on `go.mod` |
| **golangci-lint** | `v2.12.2` | |
| **testcontainers-go** | `v0.44.0` | `make test-db` runs against a real PostgreSQL, and this is what starts it |
| **OpenTelemetry Go** | `v1.45.0` | |

## Database

| | Pin | Why this one |
|---|---|---|
| **PostgreSQL** | `18.6` — image **`postgres:18.6-alpine`** | 18.0 GA'd 2025-09-25, EOL 2030-11-14. ⚠ **The pin follows the official image, not the upstream release.** `docker-library/postgres` lags upstream by weeks and skips minors, so the newest number on `endoflife.date` is regularly a tag that **does not exist yet**, and a compose file naming it fails to pull. Check the registry's tag list, never `endoflife.date`, before writing this number — `scripts/check-pins.sh images` prints both so the gap is visible |
| **Migrations** | **golang-migrate** — `{version}_{title}.up.sql` / `.down.sql` pairs | Nothing changes a schema by hand |

## Frontend

| | Pin | Why this one |
|---|---|---|
| **Node** | `24.19.0` — **Active LTS "Krypton"** | `26.x` is Current but does not become LTS until October 2026. `22` ("Jod") is already in maintenance |
| **Vite** | `8.2.1` | `engines: node ^20.19.0 \|\| >=22.12.0` — satisfied |
| **@vitejs/plugin-react** | `6.0.5` | peers `vite ^8.0.0` — **its only required peer.** The React Compiler peers are optional and deliberately not taken |
| **React · react-dom** | `19.2.8` | ⚠ React 18 is not an option — see below |
| **react-router** | `8.3.0` | |
| **TypeScript** | `6.0.3` | ⚠ Deliberately not `7.0.2` — see below |
| **typescript-eslint** | `8.67.0` | |
| **ESLint** | `10.8.1` | |
| **Tailwind** | `4.3.3` (+ `@tailwindcss/vite` `4.3.3`) | |
| **Vitest** | `4.1.10` | |

## Infrastructure

| | Pin | Why this one |
|---|---|---|
| **Caddy** | `2.11.4` — `caddy:2.11.4-alpine` | Automatic HTTPS, a config file that fits on a screen, one binary. ⚠ Its certificate storage is a **stateful volume** — see below |
| **Docker Compose** | `v5.4.0` | |
| **Prometheus** | `v3.13.2` | |
| **Loki** | `v3.7.6` | |
| **Tempo** | `v3.0.2` | |
| **Alloy** | `v1.18.1` | |
| **Grafana** | `v13.1.3` | |

## ⚠ Compatibility constraints — the couplings that decide the pins

These are not preferences. Each is a peer-dependency or toolchain floor that makes an
otherwise obvious choice wrong, and each is the kind of thing discovered at `npm install` on
day one otherwise.

| Constraint | Consequence |
|---|---|
| ⚠ **`typescript-eslint@8.67.0` peers `typescript >=4.8.4 <6.1.0`** — and so does its `canary` tag | **TypeScript 7 breaks typed linting.** TS 7.0.2 is the Go-native compiler — 8–12× faster and semantically identical — but it ships **without a stable programmatic API** (due in 7.1), which is exactly what typescript-eslint consumes. **So the pin is `6.0.3`, and `make lint` is the reason.** ⏱ Revisit when typescript-eslint declares support for TS 7 — that is the single gate |
| ⚠ **`react-router@8.3.0` peers `react >=19.2.7`** | **React 18 cannot be used with react-router 8.** Either React is 19 or the router stays on 7.x. Take React 19 — a router major is a smaller migration than a React major, and doing them apart means doing them twice |
| **The Go floor is `1.25.0`, set by Fiber and pgx** | golang-migrate runs as a container, so it adds no floor |
| **`@vitejs/plugin-react@6.0.5` peers `vite ^8.0.0`** | Vite 8 and plugin-react 6 move together. Neither can be held back alone |
| ⚠ **The plugin's other two peers are `optional: true`, and taking them pulls in a release candidate** | `peerDependenciesMeta` marks **both** `@rolldown/plugin-babel` and `babel-plugin-react-compiler` optional — they are the **React Compiler**, not the plugin. Pin them and two undeclared peers arrive with them: `@rolldown/plugin-babel` peers `@babel/core: ^7.29.0 \|\| ^8.0.0-rc.1` **non-optional**, and npm resolves it to a **release candidate** even when a stable release satisfies the range — the same rule that rejects a Go rc above. It also peers `rolldown` non-optionally, satisfied only because Vite happens to hoist its own copy. **So the React Compiler is off.** ⏱ To turn it on: pin all four — the two plugins, `@babel/core` explicitly at a stable version, and `rolldown` — and record it |
| **`vite@8.2.1` engines `node ^20.19.0 \|\| >=22.12.0`** | Node 24 LTS satisfies it. Node 22 also would — but 22 is in maintenance, so the choice is made on support horizon, not on this constraint |
| ⚠ **Caddy's `/data` volume is not a cache — it is the certificate store** | It holds the issued certificates **and the ACME account key**. A compose file without a named volume on `/data` re-issues every certificate on every recreate, and Let's Encrypt rate-limits duplicate certificates (**5 per week per identical name set**) — so the failure is not "slow first request", it is **the site cannot get a certificate for days**. Mount `/data` *and* `/config`, and back up `/data` |
| ⚠ **Automatic HTTPS needs inbound `:80`, or DNS-01** | Caddy's default challenge path needs port 80 reachable from the internet. Behind a firewall, on an internal host, or for a wildcard, that is not available, and the answer is a **DNS-01 provider module — which is not in the stock image** and needs a custom build (`caddy:<version>-builder-alpine`). Decide this before the first deploy, not during it |

**Bumping a version is a deliberate act.** Renovate/Dependabot opens the PR; a human reads
the changelog and merges it. Nothing floats on `^` in a lockfile-less install.

⚠ **The Go and Node versions are pinned in several places each** — `go.mod`/`.nvmrc`, the
Dockerfile, the CI workflow, and for Node the compose image tag. They drift silently and the
symptom is *"works on my machine"*. `scripts/version-check.sh` asserts they agree.
