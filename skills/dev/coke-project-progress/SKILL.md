---
name: coke-project-progress
description: Maintains `project/PROGRESS.md` across coding-agent sessions in multi-repository workspaces with a stable `project/` directory. Use when implementing non-trivial features, fixing complex bugs, refactoring across files or repositories, changing API/database/auth/payment/files/background-job/cross-repo behaviour, or continuing work from a previous session. Skip for one-line or trivial changes.
---

# Coke Project Progress

## Use this when

- Starting a non-trivial task in a workspace that uses `project/PROGRESS.md`.
- Continuing work from a previous coding-agent session.
- Implementing a feature, fixing a complex bug, or refactoring across files or repositories.
- Changing API, database, auth, payment, files, background-job, or cross-repo behaviour.
- Reaching a meaningful milestone that future sessions need to pick up from.

## Do not use this when

- The task is a trivial one-line or formatting change.
- Pure exploration with no code change.
- The user explicitly asks for a quiet, log-free response.

## Core rules

- Progress lives at `project/PROGRESS.md`. Project-level agent instructions may live at `project/AGENTS.md`.
- The `project/` directory is the only stable convention. Repository names (e.g. `api/`, `web/`, `worker/`) vary — discover them; never assume.
- Read `project/AGENTS.md` and `project/PROGRESS.md` (when they exist) before editing code for a non-trivial task.
- Update `project/PROGRESS.md` at meaningful milestones and before the final response for non-trivial tasks.
- Record useful project-level state, not every action. See `references/recording-rules.md`.
- If `project/PROGRESS.md` does not exist and the task is non-trivial with meaningful state to preserve, create it using `templates/progress-template.md`.
- Group `Changed Areas` by the actual discovered repository or project area names.

## Workflow

### Phase 1 — Read

1. Read `project/AGENTS.md` if it exists.
2. Read `project/PROGRESS.md` if it exists.
3. Summarise current task, completed work, incomplete work, next steps, blockers, risks, open questions, and verification already done.
4. Identify affected repositories or project areas through workspace inspection.
5. Continue from the existing state instead of starting blind.

### Phase 2 — Update

Update `project/PROGRESS.md`:

- After understanding the workspace and affected areas.
- After meaningful implementation milestones.
- After tests or verification.
- When blockers, risks, decisions, or open questions appear.
- Before the final response for non-trivial tasks.

Do not update for every tiny action — see `references/recording-rules.md`.

## Load more detail

- PROGRESS.md format and `Changed Areas` grouping → `templates/progress-template.md`
- What to record / not record, when to update, style → `references/recording-rules.md`

## Related skills

- `coke-workspace-orientation` — discovering the workspace and repository structure before editing. Run this first.
- Verification skills (if present) — running the right checks before the final progress update.

Recommended order: workspace orientation → read progress → implementation → verification → update progress → final response.

## Definition of done

The final response must state:

- Whether `project/PROGRESS.md` was read.
- Whether `project/PROGRESS.md` was updated.
- Current task status.
- What was completed and what should happen next.
- Verification performed and anything left unverified.

Keep the final response concise. Example:

```text
Read `project/PROGRESS.md` before starting and updated it with the implementation summary, verification results, and next steps.
```

If the task was trivial and progress was not updated, say so.
