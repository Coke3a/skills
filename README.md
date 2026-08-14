# Skills Repository

This repository is separated into two kinds of content:

- `skills/`: agent skills that contain `SKILL.md` files and bundled resources.
- `text/`: playbooks and reference text that explain which official or external
  skills to use.

## Agent Skills

### Backend

Rust:

- `skills/backend/coke-rust-clean-architecture`
- `skills/backend/coke-tdd-feature-workflow`
- `skills/backend/coke-rust-code-review`
- `skills/backend/coke-rust-performance-optimization`
- `skills/backend/coke-rust-ci-cd`

Go:

- `skills/backend/coke-go-clean-architecture`
- `skills/backend/coke-go-tdd-feature-workflow`
- `skills/backend/coke-go-performance-optimization`

### Web

- `skills/web/coke-nextjs-app-architecture`
- `skills/web/coke-nextjs-ui-tdd-workflow`

### Ops

- `skills/ops/coke-docker-vm-deploy` — build a Docker image in GitHub Actions and ship it to
  a plain Linux VM (droplet / EC2 / VPS) over SSH. Language-agnostic; starts where the
  stack-specific CI skills stop. Bundles a harness that exercises the deploy script's
  failure paths with no server.

### dev

- `skills/dev/coke-fullstack-baseline` — the big-picture engineering baseline for a
  Go + React (Vite) + PostgreSQL project: repository layout, version pins and the peer
  constraints that decide them, the local Compose stack, same-origin proxying, database
  roles, gates, CI up to the artifact, observability. Its other half is routing — design
  phase vs implement phase, and which specialist skill owns each lane. Deliberately stops
  where the deep work starts. Bundles `check-pins.sh` (upstream freshness) and
  `version-check.sh` (the Go/Node agreement gate).
- `skills/dev/coke-workspace-orientation`

### Writing

- `skills/writing/coke-community-reply` — draft answer-first replies to community / forum posts (n8n community, Reddit, Make/Zapier, IndieHackers). Future siblings: `coke-email-reply`, `coke-marketing-post`.

## Text Playbooks

- `text/mobile-expo`: explains when to use official Expo skills.
- `text/design`: explains when to use design-focused skills.
