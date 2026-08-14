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
4. Release it with the steps below.

## Releasing a change

**A version bump is mandatory.** The update check compares version *strings*, not commit
SHAs. Push a skill edit without bumping and `claude plugin update` fetches the new commit,
reports `already at the latest version`, and leaves the installed copy untouched — the edit
never reaches the agent.

1. Edit the skill.
2. Bump `version` in `plugins/<group>/.claude-plugin/plugin.json`. **This is the one that
   decides** — the CLI's own words are "plugin.json wins at install time".
3. Set the matching entry in `.claude-plugin/marketplace.json` to the same value. It does not
   trigger the update, but `claude plugin tag` refuses to tag while the two disagree.
4. `claude plugin validate .` — stricter than eyeballing; it catches malformed frontmatter
   that grep-based checks miss.
5. Commit and push to `main`.
6. `claude plugin update coke-eng@coke-skills` — the qualified name is required; a bare
   `coke-eng` fails with `Plugin "coke-eng" not found`.
7. Restart Claude Code. The CLI prints `Restart to apply changes`; a session already running
   keeps the copy it started with.

Plugins version independently — releasing `coke-eng` does not touch the other two.

Optional, for a release marker: `claude plugin tag --push plugins/eng` creates
`coke-eng--v<version>`. It refuses to run on a dirty working tree or on mismatched manifests,
so it doubles as a pre-push check (`--dry-run` to look without tagging).

### Where things live

| Path | What |
| --- | --- |
| `~/.claude/plugins/marketplaces/coke-skills/` | Git clone of this repo. `plugin update` fetches it — no separate `marketplace update` needed. |
| `~/.claude/plugins/cache/coke-skills/<plugin>/<version>/` | The copy the agent actually reads. A new directory per version; the previous one is stamped `.orphaned_at` and swept later. |
| `~/.claude/plugins/installed_plugins.json` | Records the installed `version` and `gitCommitSha`. The SHA is stored but not used to decide whether to update. |

Verified 2026-08-14 by pushing a marker without a bump (not picked up), then with one (picked
up), and confirming the cache contents by hash in both directions.
