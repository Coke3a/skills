# CI — and where CD stops being this skill's business

⚠ **CD is out of scope, and deliberately so.** Where a deploy lands is a project decision — a
droplet, an EC2 instance, Fly.io, a managed platform — and each target carries a different
mechanism, a different credential model and a different rollback. Fixing one here would be
exactly the project-specific fact this skill does not carry. **What stays is the half that
does not change with the target:** the gate order before the artifact exists, the properties
any deploy must have whatever executes it, and the four things CI must never do.

⚠ **No skill is named for the deploy step.** One exists for a VM target
(`coke-eng:ops-docker-vm-deploy`) and others exist for other targets; picking among them is the
project's call, made once, in the project's own documents.

## The pipeline — invariant up to the artifact

```text
push ─▶ path filter ─▶ lint ─▶ test ─▶ test-db ─▶ contract-check ─▶ secret-scan
                                                                        │
                                                  build ─▶ frontend `dist/` + backend image
                                                            (both tagged with the git sha)
                                                                        │
                          ╌╌╌╌╌╌╌╌╌╌╌╌╌ target-specific from here ╌╌╌╌╌╌╌╌╌╌╌╌╌
                                                                        │
                                                     push ─▶ migrate ─▶ deploy ─▶ smoke
```

**Above the dashed line is the same on every project** — it is the gate set in order, run by
whatever CI the project uses. Below it the mechanism differs per target; **these properties
do not.**

- **`test-db` runs against a real PostgreSQL.** Not a fake, not sqlite. The rules that fail
  silently — grants, RLS, constraints — exist only in a real database. ⚠ **Say which
  mechanism starts it**: a CI service container and testcontainers are different things, and
  configuring both gives you two databases with the tests silently using whichever the code
  points at.
- ⚠ **There are two artifacts, and it is easy to build only one.** The backend is an image;
  **the frontend is `dist/`, produced by a build step that has to be in this pipeline** and
  delivered to whatever serves it. Nothing else in this baseline builds it — and the smoke
  test's last two assertions are entirely about the built frontend, so a pipeline that skips
  the build fails them without ever explaining why.
- **Every image is tagged with the git sha.** `latest` is not a deploy target. A deploy names
  a sha, which is what makes a rollback `deploy <previous sha>` rather than a rebuild.
- **Migrations are their own step, before the app rolls**, run as the owner role from the
  `migrate/migrate` image — the same command as the local compose file. If they fail, nothing
  deploys.
- ⚠ **A failed migration step leaves the database `dirty` and blocks every later deploy**
  until a human inspects the schema and runs `migrate force V`. **This is correct** — the
  alternative is deploying application code against a half-applied schema — but it means the
  runbook for it must exist *before* the first failure, not be written during one. It is also
  why `make migrate-check` runs earlier in the pipeline: the cheap place to discover a broken
  migration is CI, not the deploy step.
- ⚠ **The smoke test carries the checks development can no longer make.** Because the local
  stack runs Vite's dev server rather than Caddy, the SPA serving path is first exercised
  here. Four assertions, and the last two are the ones nothing else covers:

  | | Assert |
  |---|---|
  | health | the trio of health checks |
  | API | one real authenticated request through the public URL |
  | ⚠ **deep link** | `GET /<some/nested/route>` returns **`200` and `index.html`**, not `404` |
  | ⚠ **cache headers** | `index.html` is `no-cache`; an `/assets/*` file is `immutable` — the trap a redeploy cannot fix |

  **A deploy that is not smoke-tested is a deploy nobody has verified**, and here that is
  literally true of the frontend: no earlier stage in the pipeline serves `dist/` through
  Caddy at all.

## ⚠ Path-filtered, per top-level directory — this is a requirement

One repository defaults to one CI trigger. The failure is not hypothetical, it is the default
behaviour of every CI system given a monorepo and no filters:

> a commit that edits only a Grafana dashboard triggers a backend rebuild and a production
> deploy of the API.

**A change under `observability/` must not rebuild or redeploy the backend. A change under
`backend/` must not touch the monitoring host.** One repository does not mean one deployment.

## What CI must not do

| | |
|---|---|
| **Never hold a long-lived production credential** | short-lived, scoped tokens; deploy through a key that can be rotated in one place |
| **Never build a different image per environment** | one image, configuration from the environment |
| **Never auto-merge on green** | a green pipeline says the rules held, not that the change is right |
| **Never skip a gate to unblock a release** | if that is ever the correct call, it is a human decision recorded in the PR, not a flag in the workflow |
