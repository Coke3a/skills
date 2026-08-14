# Coke Skills

Agent skills for Claude Code, split into three plugins. Install only the groups you need.

```
/plugin marketplace add Coke3a/skills

/plugin install coke-eng@coke-skills            # code and infrastructure
/plugin install coke-product@coke-skills        # design, positioning, CRO
/plugin install coke-productivity@coke-skills   # workspace, progress, writing
```

Skills trigger on their own from context. To invoke one explicitly, use its qualified name —
`/coke-eng:rust-clean-architecture`.

Within each plugin, skill names carry a topic prefix (`rust-`, `go-`, `nextjs-`, `flow-`,
`design-`, `writing-`, …) so related skills sort together.

---

## coke-eng

| Skill | Use when |
| --- | --- |
| `flow-feature-implementation` | Implementing a feature end-to-end from a written spec, across multiple batches with review gates |
| `flow-spec-review` | Validating a spec against the real current system before anyone writes code |
| `go-clean-architecture` | Go feature layout with Fiber v3, sqlc, pgx — handlers, usecases, domain, repositories |
| `go-performance-optimization` | A Go service has a latency, throughput, or allocation problem worth measuring |
| `go-tdd-feature-workflow` | Turning Go acceptance criteria into tests, then red/green/refactor |
| `nextjs-app-architecture` | Next.js App Router structure — routes, layouts, server/client boundaries, server actions |
| `nextjs-ui-tdd-workflow` | Test-driven UI work with Testing Library, hooks, forms, or Playwright E2E |
| `ops-docker-vm-deploy` | Shipping a Docker image from GitHub Actions to a plain Linux VM over SSH |
| `rust-ci-cd` | GitHub Actions for Rust — checks, caching, image builds, deploys, migrations, rollback |
| `rust-clean-architecture` | Rust feature layout with Axum and Diesel — handlers, usecases, domain, repository traits |
| `rust-code-review` | Reviewing Rust changes before finishing a feature or merging a PR |
| `rust-performance-optimization` | A Rust service has a measured performance goal or a suspected hot path |
| `rust-tdd-feature-workflow` | Turning Rust acceptance criteria into tests, then red/green/refactor |
| `stack-go-react-postgres` | Starting or restructuring a Go + React (Vite) + PostgreSQL project — layout, pins, Compose, CI, gates |

## coke-product

| Skill | Use when |
| --- | --- |
| `design-landing-page` | Creating, redesigning, or auditing a landing page for conversion |
| `design-scrutinize` | Getting an outsider review of a screen, flow, or prototype — including its edge states |
| `marketing-sell-the-outcome` | Checking whether an idea, feature, or page sells the result rather than the product |

## coke-productivity

| Skill | Use when |
| --- | --- |
| `agent-project-progress` | Carrying non-trivial work across coding sessions via `project/PROGRESS.md` |
| `agent-workspace-orientation` | Starting work in a multi-repository workspace where git roots must be discovered |
| `writing-community-reply` | Drafting an answer-first reply to a community or forum post |

---

## Adding a skill

1. Create `plugins/<group>/skills/<prefix>-<name>/SKILL.md`.
2. Set frontmatter `name:` to the folder name exactly.
3. Add one row to the table above.
4. Bump that plugin's `version` in `.claude-plugin/marketplace.json` and its `plugin.json`.
