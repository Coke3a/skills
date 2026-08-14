---
name: coke-docker-vm-deploy
description: Build a Docker image in GitHub Actions and deploy it to a plain Linux VM over SSH — DigitalOcean droplet, EC2, Hetzner, Contabo, any VPS — for an app in any language. Covers image tagging and registry push, the remote deploy script and its rollback, GitHub Environment secret scoping, host preparation, reverse proxy, port exposure, and testing the deploy script without a server. Use this whenever the user wants to set up, review, harden, or debug shipping a container to a server they own, including phrasings like "deploy to my droplet", "GitHub Actions docker deploy", "ssh deploy script", "CI/CD for my VPS", "deploy container to EC2", "why did my deploy not roll back", "ทำ CI/CD deploy ขึ้น server", or "deploy docker ขึ้น droplet" — even when they do not say "CI/CD" explicitly. Do not use for managed platforms that deploy themselves (Vercel, Netlify, Fly.io, Render, Railway, App Runner), for Kubernetes/ECS/Nomad orchestration, or for language-specific CI checks (Rust → coke-rust-ci-cd).
---

# Docker → VM Deploy

Ship a container from GitHub Actions to a Linux box you own. Language-agnostic:
the app can be Node, Go, Python, Rust, PHP, Java — the pipeline shape is the same
because the image is the interface.

## Use this when

- Setting up build → push → deploy for a server the user administers themselves.
- Writing or reviewing the remote deploy script (the part that runs over SSH).
- Deciding which files CI ships to the host and which live only on the host.
- Wiring registry auth, image tags, and rollback.
- Preparing a fresh VM to receive containers.
- Debugging a deploy that failed, half-succeeded, or "succeeded" while the app is down.
- Reviewing an existing pipeline for the failure modes in `references/deploy-smells.md`.

## Do not use this when

- The platform deploys itself (Vercel, Netlify, Fly, Render, Railway, Heroku, App Runner).
- The target is Kubernetes, ECS, Nomad, or another orchestrator with its own rollout model.
- The task is language-specific CI (test/lint/build matrices) — that belongs to the
  language's own skill; this skill starts once there is an image to ship.
- The user wants application code written. Deploy plumbing only.

## The one idea everything follows from

**Build on the runner, run on the host.** The VM never compiles anything, never holds
source, never holds git credentials. It pulls an image by digest-stable tag and runs it.

Everything else in this skill is a consequence:

| Question | Answer |
|---|---|
| Why is rollback just a tag swap? | The image is immutable, so the previous tag is a known-good state. |
| Why must CI ship the compose file? | It is in git and the host needs it; anything else silently diverges. |
| Why is `app.env` never in git? | Secrets are the one input the image cannot carry. |
| Why does the host need no registry PAT? | CI passes a short-lived token over SSH and logs out on exit. |

## Core rules

- Deploy an immutable tag (`sha-<short>`), never `latest`. The tag *is* the rollback.
- One writer per file. If it lives in git, CI ships it every deploy; if it holds secrets
  or machine-specific values, it lives only on the host and CI never touches it.
- Bind published ports to `127.0.0.1` unless the container is meant to face the internet.
  Docker's iptables rules are traversed before `ufw`, so a firewall will not save you.
- Secrets reach the container through a host-side env file at runtime — never through
  build args, `ENV`, or the compose file, all of which are readable from the image.
- Under `set -euo pipefail`, any command that can fail *after* you have mutated host state
  must be captured (`cmd || ok=0`), or `set -e` will skip the rollback you wrote.
- Every mutation gets a paired restore, and the restore gets verified. A rollback that
  prints success without re-checking health is a log that lies.
- Fail with the actual cause. "compose file is invalid" when the real problem is a missing
  env file sends the next person hunting the wrong bug.
- Never sync a *directory* to the host. One file at a time; `rsync --delete` eats secrets.

## Workflow

1. Establish the ownership boundary first — read `references/ownership-boundary.md` and
   write down which files CI owns and which the host owns. Every later decision falls out
   of this table, and getting it wrong is the most common source of silent breakage.
2. Pick the workflow file matching the task (below).
3. Read the target host before writing anything for it: OS, existing web server and which
   ports it holds, whether Docker is present, firewall state, free RAM/disk, and what else
   runs there. A shared box changes almost every recommendation.
4. Verify tool versions on the host rather than assuming — Compose v2 and v5 are both in
   the wild and behave differently. `docker compose config -q` runs client-side and needs
   no daemon, so it is a cheap compatibility probe.
5. Use `templates/` as starting points; adapt names and paths to the repo.
6. Test the deploy script before it ever touches the server — see
   `workflows/test-deploy-script.md`. This is the step people skip and then regret.

## Workflows

| Workflow | Use for |
|---|---|
| `workflows/setup-pipeline.md` | Greenfield: build, push, deploy, rollback end to end |
| `workflows/write-deploy-script.md` | The remote script, its failure paths, and rollback |
| `workflows/test-deploy-script.md` | Proving the script's failure paths without a server |
| `workflows/prepare-host.md` | First-time VM preparation, in a safe order |
| `workflows/harden-secrets.md` | Environments, secret scoping, job permissions |
| `workflows/fix-broken-deploy.md` | Diagnosing a failed, half-done, or lying deploy |

## Load more detail

- Who owns which file, and why → `references/ownership-boundary.md`
- Tags, registries, digests, rollback window → `references/image-and-tagging.md`
- Multi-stage image shape for any language → `references/dockerfile-patterns.md`
- Triggers, concurrency, gating, permissions → `references/workflow-patterns.md`
- Environment vs repo secrets, and the fallback trap → `references/github-secrets-scoping.md`
- `set -e` traps, paired restore, verified rollback → `references/rollback-safety.md`
- Port binding, firewalls, pruning, logs, memory → `references/host-exposure.md`
- What a healthcheck does and does not prove → `references/healthchecks.md`
- Review checklist of known failure patterns → `references/deploy-smells.md`

## Templates

Adapt, do not copy blindly — every one has a placeholder that must change.

- `templates/github-actions-deploy.yml` — the two-job workflow
- `templates/deploy.sh` — the remote script, with rollback and preflight
- `templates/docker-compose.yml` — the host-side service definition
- `templates/Dockerfile` — multi-stage shape with per-language notes
- `templates/app.env.example` — the secret template that ships; the real file does not
- `templates/nginx-reverse-proxy.conf` — two-stage `:80` then `:443`
- `templates/SETUP.md` — the host runbook handed to whoever operates the box

## Scripts

- `scripts/test-deploy-script.sh` — runs a deploy script against a stubbed `docker`
  and asserts host state for every failure path. Use it instead of writing a new harness.
- `scripts/docker-stub.sh` — the fake `docker` the harness puts on `PATH`.

## Related skills

- `coke-rust-ci-cd` — Rust-specific CI checks feeding this pipeline.

## Definition of done

Summarise:

- Workflows created or changed, their triggers, and what gates the deploy.
- The ownership table: which files CI ships, which live only on the host.
- Image tag strategy and how far back rollback reaches.
- Secrets: names, where they are scoped, and what happens if one is missing.
- Which ports are published and on which interface.
- Failure paths tested, how they were tested, and what was *not* tested.
- Anything that must be done by hand on the host before the first deploy.
