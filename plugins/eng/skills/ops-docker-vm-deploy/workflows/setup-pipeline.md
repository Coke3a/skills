# Set up the pipeline

End to end, greenfield. Roughly in order, though steps 1–3 are worth doing together before
writing anything.

## 1. Read the target host first

Recommendations change completely depending on what is already there. Read-only:

```bash
cat /etc/os-release
docker --version 2>&1; docker compose version 2>&1
ss -tlnp                        # who holds 80/443, and what else is listening
systemctl list-units --type=service --state=running | head -30
free -h; swapon --show; df -h /
ufw status verbose 2>&1
ls /etc/nginx/sites-enabled/ 2>/dev/null
```

What you are looking for, and what each answer changes:

- **A web server already on 80/443** → you add one site file, you do not install a second
  proxy, and you must not disturb the existing ones.
- **Other people's services** → pruning must be label-scoped, memory must be limited,
  firewall changes need approval.
- **Docker already present** → merge into the existing daemon config rather than
  overwriting it; check for existing containers before creating networks.
- **No swap on a small box** → see `references/host-exposure.md`.
- **Compose major version** → v2 and v5 differ; validate the compose file against the
  version actually on the host before relying on it.

## 2. Write the ownership table

Before any YAML. `references/ownership-boundary.md` has the rule; the output is a concrete
list for this app: which files CI ships, which the host owns, and where each lives. Every
later decision refers back to it.

## 3. Confirm which config is build-time and which is runtime

Anything inlined into a shipped artefact must be a build argument, and the app's build will
not tell you which those are — you have to look. Grep the source for the framework's
build-time env prefix, or read the build config. Getting this wrong produces a silent
runtime break, not a build failure. See `references/dockerfile-patterns.md`.

## 4. Dockerfile

Multi-stage, prod-only dependencies in the runtime stage, non-root user, healthcheck,
exec-form command. Start from `templates/Dockerfile`; the per-language table in
`references/dockerfile-patterns.md` gives the install commands.

Build it locally and run it before wiring any CI. A pipeline debugging session is a slow
way to discover a Dockerfile problem.

## 5. Compose file

`templates/docker-compose.yml`. The three lines that matter most:

- `image: <registry>/<org-lowercased>/<app>:${TAG}`
- `ports: - "127.0.0.1:<port>:<port>"`
- `env_file: - ./app.env`

## 6. Workflow

`templates/github-actions-deploy.yml`. Two jobs, tag passed through job outputs, deploy
gated on event and flag. See `references/workflow-patterns.md`.

## 7. Deploy script

`workflows/write-deploy-script.md`. Keep it in a file and reference it with `script_path:`.

## 8. Test it before it touches the host

`workflows/test-deploy-script.md`. Do not skip this; it is fast and it finds real bugs.

## 9. Secrets and environment

`workflows/harden-secrets.md`. Set the deploy flag to `false` initially.

## 10. Prepare the host

`workflows/prepare-host.md`.

## 11. First deploy

Flip the flag to `true`. Watch the run. Then, on the host:

```bash
docker compose ps                          # healthy
ss -tlnp | grep <port>                     # 127.0.0.1, not 0.0.0.0
docker compose logs --tail 100
```

And from outside the machine:

```bash
nc -z -w5 <public-ip> <port>               # must fail
```

## 12. Rehearse a rollback on the real host

Deploy twice, then force a failure and confirm the script recovers the service without
help. This is the only evidence that the safety net works against real Docker rather than a
stub. Do it **before** the service carries traffic — the alternative is finding out during
an incident.

## 13. Write the runbook

`templates/SETUP.md`. Whoever operates the box next has none of your context: where secrets
live, how to roll back by hand, how far back that reaches, what must never be edited on the
host because CI overwrites it.
