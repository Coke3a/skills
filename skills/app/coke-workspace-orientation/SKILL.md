---
name: coke-workspace-orientation
description: Orient Codex inside multi-repository workspaces where the root may contain several independent repositories plus a stable project/ directory. Use when working in a workspace with project/AGENTS.md, cross-repository code, workspace-level docs/tests/scripts, or when repository names and git roots must be discovered rather than assumed.
---

# Coke Workspace Orientation

## Purpose

Use this skill when working inside a workspace that may contain multiple repositories or project areas. The goal is to discover the actual structure, choose the relevant repositories, run commands from the correct directories, and report work by repository.

## Workspace Pattern

The workspace root may contain several repositories or project areas plus a stable `project/` directory:

```text
workspace/
  api/
  web/
  worker/
  project/
    docs/
    tests/
    scripts/
    AGENTS.md
```

Repository names vary by project. Examples may include `api/`, `web/`, `mobile/`, `worker/`, `admin/`, `docs-site/`, or other names. Do not assume `backend/` or `frontend/` exist.

Treat only `project/` as a stable convention. It contains project-level documentation, workspace-level tests, shared scripts, and agent instructions when present.

## First Step

Every time this skill is invoked, first read `project/AGENTS.md` if it exists.

If `project/AGENTS.md` exists, treat it as the source of truth for:

- workspace structure
- repository list
- project-specific commands
- documentation locations
- testing strategy
- development rules
- verification requirements

If it does not exist, inspect the workspace carefully and state that no project-level agent instructions were found.

## Repository Discovery

Identify repositories by combining:

- `project/AGENTS.md`
- top-level directory inspection
- `.git` directories
- package, build, or dependency files
- source, test, config, and docs directories
- task-specific imports, paths, or references

Do not hard-code repository names. Discover the relevant repositories for the task before editing.

## Docs Rules

Use `project/docs/` for project-level specs, architecture notes, decisions, and cross-repository behavior. Read relevant docs before changing behavior or architecture. Update `project/docs/` when a change affects documented behavior, architecture, shared workflows, or cross-repository contracts.

## Before Editing

Before modifying files:

1. Read `project/AGENTS.md` if it exists.
2. Identify the repositories affected by the task.
3. Read relevant files in `project/docs/` and the affected repositories.
4. Confirm where repo-specific and workspace-level commands should run.
5. Decide the smallest relevant verification set based on `project/AGENTS.md`, the task, and the changed files.

## Working Directory Rules

Run commands from the directory that owns the command:

- Run repo-specific commands inside the relevant repository.
- Run workspace-level scripts from `project/scripts/` or exactly as documented in `project/AGENTS.md`.
- Do not assume commands work from the workspace root.

Examples:

```bash
cd api && npm test
cd web && npm run lint
project/scripts/test-e2e.sh
```

Adjust the repository names and commands to the discovered workspace.

## Git Rules

The workspace root may not be a git repository. Each subdirectory may be its own independent git repository.

Always run git commands inside the relevant repository:

```bash
cd {$repo} && git status
cd {$repo} && git diff
cd {$repo} && git diff --stat
cd {$repo} && git log --oneline -n 5
```

If multiple repositories are affected, check git status and diffs in each affected repository separately.

Do not run `git status`, `git diff`, `git diff --stat`, or `git log` from the workspace root unless `project/AGENTS.md` explicitly says the root is a git repository.

## Test Rules

Use repo-level tests for changes isolated to one repository. Use `project/tests/` for workspace-level or cross-repository end-to-end tests when the change crosses repository boundaries.

Do not run every possible test by default. Choose relevant commands from `project/AGENTS.md`, repo docs, package files, and the risk of the change. Do not add workspace-level end-to-end tests for trivial isolated changes.

## Final Report

Group the final response by repository when changes span repositories. Include:

- repositories inspected
- repositories changed
- docs consulted
- files changed grouped by repository
- commands run and the directory each ran from
- git status or diff checked per affected repository
- tests or verification performed
- anything not verified

## Examples

Correct:

```bash
cd api && git diff
cd web && npm test
project/scripts/test-e2e.sh
```

Incorrect:

```bash
git diff
cd backend && npm test
cd frontend && npm test
```

The incorrect examples assume the workspace root is a git repository or assume repositories named `backend/` and `frontend/` exist. Also avoid editing code before reading `project/AGENTS.md` when that file is present.
