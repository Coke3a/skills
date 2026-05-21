---
name: coke-workspace-orientation
description: Orients a coding agent inside multi-repository workspaces whose root may contain several independent repositories plus a stable `project/` directory. Use when starting work in a workspace with `project/AGENTS.md`, cross-repository code, workspace-level docs/tests/scripts, or when repository names and git roots must be discovered rather than assumed. Run before editing code so commands, git, and tests run in the right directory.
---

# Coke Workspace Orientation

## Use this when

- Starting work inside a workspace that may contain multiple repositories or project areas.
- `project/AGENTS.md`, `project/docs/`, `project/tests/`, or `project/scripts/` exists.
- The task spans more than one repository or area.
- Repository names and git roots have not been verified for the current task.

## Do not use this when

- The repo is a single-package monorepo or standalone project that you have already worked in this session.
- The task is a trivial edit confined to a known file and tool.

## Core rules

- The `project/` directory is the only stable convention. Repository names like `api/`, `web/`, `worker/`, `mobile/`, `admin/`, `docs-site/` vary by project — discover them; never assume.
- Read `project/AGENTS.md` first when it exists. Treat it as the source of truth for structure, commands, docs locations, testing strategy, and verification.
- Discover repositories from `project/AGENTS.md`, top-level inspection, `.git` directories, package/build/dependency files, and task-specific imports.
- The workspace root may not be a git repository. Each subdirectory may be its own independent git repo.
- Always run `git`, build, lint, and test commands from the directory that owns them, not the workspace root.
- Use `project/docs/` for project-level specs, architecture notes, decisions, and cross-repository behaviour. Update it when changing documented behaviour, architecture, or cross-repo contracts.
- Use repo-level tests for changes isolated to one repository. Use `project/tests/` for workspace-level or cross-repository end-to-end tests.
- Choose the smallest relevant verification set; do not run every possible test by default.

## Workflow

1. Read `project/AGENTS.md` if it exists. If not, state that no project-level agent instructions were found.
2. Inspect the workspace root to identify the repositories or project areas relevant to the task.
3. Read relevant files in `project/docs/` and inside the affected repositories.
4. Confirm where repo-specific vs workspace-level commands should run.
5. Decide the smallest relevant verification set based on `project/AGENTS.md`, the task, and the changed files.
6. Run commands from the correct directory, group git status and diffs per affected repository.
7. Report changes grouped by repository.

## Working directory and git examples

Correct:

```bash
cd api && git status
cd api && git diff
cd web && npm test
cd web && npm run lint
project/scripts/test-e2e.sh
```

Incorrect (assumes the root is a git repo or assumes fixed repo names):

```bash
git diff
npm test
npm run dev
```

Adjust the repository names and commands to the discovered workspace.

## Related skills

- `coke-project-progress` — read and update `project/PROGRESS.md` after orienting.
- Verification skills (if present) — running the right end-to-end checks before finishing.

Recommended order: workspace orientation → project progress (read) → implementation → verification → project progress (update) → final report.

## Definition of done

The final response should group results by repository when changes span repositories and include:

- Repositories inspected.
- Repositories changed.
- Docs consulted.
- Files changed, grouped by repository.
- Commands run and the directory each ran from.
- Git status or diff checked per affected repository.
- Tests or verification performed.
- Anything not verified.
