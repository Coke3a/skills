# Coke Skills

Agent skills I use when I work with coding agents — Go and Rust backends, shipping to a plain
VPS, and the product thinking around it. They install into Claude Code, and the repo ships a
Codex marketplace manifest too.

I'm [Coke](https://github.com/Coke3a). I keep these skills in one repo so every project starts
from the same habits instead of re-explaining them to the agent each time.

```
/plugin marketplace add Coke3a/skills

/plugin install coke-eng@coke-skills            # code and infrastructure
/plugin install coke-product@coke-skills        # design, positioning, CRO
/plugin install coke-productivity@coke-skills   # workspace, progress, writing
```

Install only the group you need. Skills trigger on their own from context. To invoke one
explicitly, use its qualified name — `/coke-eng:rust-clean-architecture`.

Codex: `.agents/plugins/marketplace.json` lists the same three plugins, each with a
`.codex-plugin/plugin.json`.

---

## What I care about

**Verify, don't assume.** A skill that reviews a spec should check the spec against the running
system first. `flow-spec-review` runs the existing tests, compares what the spec calls "current
behavior" with what the system does, and stops if they disagree. It won't review a spec built on
a false premise.

**A skill is only useful if it fires.** A description is a trigger, and triggers fail
quietly. So the eval harness in `evals/` doesn't test a stand-in copy of a skill. It runs
`claude -p` with every plugin you have installed competing, and records which skill won the
prompt. When a skill loses, you see who beat it, because "another skill outranked mine" and "my
description doesn't match" need different fixes. Eight skills have prompt sets
(`should_trigger` true and false) so far.

**Keep the workflow separate from the language.** Process skills (`tdd-feature-workflow`,
`ci-cd-workflow`, `performance-optimization`) are language-neutral and defer to the repository's
own commands and conventions. Only the guidance that is really language-specific carries a
language prefix, like `go-clean-architecture` and `rust-clean-architecture`.

**Stop at the decision that is yours.** Several skills hard-stop and ask rather than guess:
when the test scope can't be met, when a product decision is missing, when a rollback would
be unsafe. I'd rather the agent ask once than build on a guess.

---

## What it looks like

Abridged and illustrative. This is the shape of a `flow-spec-review` run, not a verbatim log.

```
You:    review specs/refund-flow.md before we implement it

Agent:  [flow-spec-review] The spec changes refunds from "full refund only" to
        partial refunds. Is that right? What can I test, and where?
You:    API tests against staging, nothing destructive.

Agent:  Ran the existing refund tests and called the staging API.

        GATE 2 — current state doesn't match the spec
        Spec says:     "Today POST /refunds rejects any amount below the order total."
        Reality shows: Staging accepts partial amounts and returns 201.

        I'm stopping here. Is the spec outdated, or is staging ahead of production?
```

The skill goes on to a big-picture before/after for each change, and every change needs your
approval before it moves forward. It never implements anything.

---

## How the engineering skills fit together

When I build a feature, I use three skills in order. Each one answers a different question.

```
 spec ──► flow-spec-review ──► tdd-feature-workflow ──► rust-code-review
          Is the spec true?     Is each behavior         Did the result
                                proven by a test?        hold up?
                                      │
                          go- / rust-clean-architecture
                          Does the code sit in the right layer?
```

1. **`flow-spec-review` — is the spec right?** Before any code, it checks the spec against the
   running system and walks me through every change as a before/after that I approve. It
   never implements anything. What comes out is a spec I've signed off on. It's the heaviest
   skill here (it needs a test environment and my approval on every change), so I use it for
   non-trivial specs and skip it for small changes.
2. **`tdd-feature-workflow` — build it one behavior at a time.** I take the approved spec's
   acceptance criteria and implement them one by one, red then green then refactor. Each
   criterion gets a test that failed for the intended reason, then the smallest code that
   passes. That keeps the agent tied to the criteria I approved, and a missing product
   decision makes it stop and ask rather than invent one.
3. **`go-clean-architecture` / `rust-clean-architecture` — keep my design.** TDD decides
   *when* code gets written. These decide *where it lives*: dependency direction
   (handlers → usecases → domain), who owns which error, what a repository looks like. The
   agent keeps to my layering instead of inventing a new one for each feature.

Afterwards, `rust-code-review` checks the result against the same boundaries, and
`stack-go-react-postgres` is the entry point when the project doesn't exist yet. It decides
which of these skills runs first.

Two things to know before you adopt this:

- **The chain is manual.** The skills don't call each other. I carry the approved spec from
  step 1 into step 2 myself, and each skill's "do not use for" list keeps them from
  overlapping. This is a working order, not an automated pipeline.
- **The architecture skills are for two specific stacks.** Go uses Fiber v3, sqlc, and pgx,
  and Rust uses Axum and Diesel. If your stack differs, take the layering rules and rewrite
  the templates.

---

## Skills

Grouped by what you're trying to do. Each plugin is installed separately (see above).

### Start a project

| Skill | Use when |
| --- | --- |
| `coke-eng:stack-go-react-postgres` | Starting or restructuring a Go + React (Vite) + PostgreSQL project — layout, pins, Compose, CI, gates |
| `coke-productivity:agent-workspace-orientation` | Starting work in a multi-repository workspace where git roots must be discovered |

### Build a feature

| Skill | Use when |
| --- | --- |
| `coke-eng:flow-spec-review` | Validating a spec against the real current system before anyone writes code |
| `coke-eng:go-clean-architecture` | Go feature layout with Fiber v3, sqlc, pgx — handlers, usecases, domain, repositories |
| `coke-eng:rust-clean-architecture` | Rust feature layout with Axum and Diesel — handlers, usecases, domain, repository traits |
| `coke-eng:tdd-feature-workflow` | Implementing observable behavior one acceptance criterion at a time with red/green/refactor |
| `coke-productivity:agent-project-progress` | Carrying non-trivial work across coding sessions via `project/PROGRESS.md` |

### Review and improve

| Skill | Use when |
| --- | --- |
| `coke-eng:rust-code-review` | Reviewing Rust changes before finishing a feature or merging a PR |
| `coke-eng:performance-optimization` | Investigating a measured latency, throughput, CPU, memory, I/O, query, or bundle-size problem |
| `coke-eng:write-code-comments` | Deciding what earns a code comment and what to delete — any language, writing or auditing |

### Ship

| Skill | Use when |
| --- | --- |
| `coke-eng:ci-cd-workflow` | Designing or repairing validation, artifact promotion, deployment verification, and rollback across stacks |
| `coke-eng:ops-docker-vm-deploy` | Shipping a Docker image from GitHub Actions to a plain Linux VM over SSH |

### Product and writing

| Skill | Use when |
| --- | --- |
| `coke-product:marketing-sell-the-outcome` | Checking whether an idea, feature, or page sells the result rather than the product |
| `coke-product:design-landing-page` | Creating, redesigning, or auditing a landing page for conversion |
| `coke-product:design-scrutinize` | Getting an outsider review of a screen, flow, or prototype — including its edge states |
| `coke-productivity:writing-community-reply` | Drafting an answer-first reply to a community or forum post |

---

## Which plugin do I want?

- **`coke-eng`** — you write backend code, especially Go or Rust, and deploy it yourself.
- **`coke-product`** — you're shaping positioning, a landing page, or a UI and want a harsh
  second opinion.
- **`coke-productivity`** — you run agents across several repositories and need them oriented
  and tracked.

## Take these as a starting point

These skills reflect how I work, and a few are personal on purpose. `agent-project-progress`
and `agent-workspace-orientation` assume a workspace with a stable `project/` directory, and
`writing-community-reply` is written around my own voice and rules. Fork the repo and rewrite
them for your habits. Issues and ideas are welcome at
[github.com/Coke3a/skills/issues](https://github.com/Coke3a/skills/issues).

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
