---
name: coke-project-progress
description: Read and maintain project progress across coding agent sessions in multi-repository workspaces with a stable project/ directory. Use when implementing non-trivial features, fixing complex bugs, refactoring across files or repositories, changing API, database, auth, payment, files, background jobs, or cross-repo behavior, or continuing work from a previous session; ensures project/AGENTS.md and project/PROGRESS.md are read, project state is summarized, affected areas are identified, and project/PROGRESS.md is updated after meaningful milestones and before final response.
---

# Coke Project Progress

## Purpose

Keep project progress understandable across coding agent sessions.

Use this skill to preserve useful project-level state so future agents can quickly understand what is being worked on, what changed, what remains incomplete, what verification has already been done, and what risks or blockers still exist.

Do not use this skill to create noisy logs. Record useful project state, not every action.

## Responsibilities

1. Read current project progress before starting non-trivial work.
2. Update progress after meaningful milestones and before finishing non-trivial tasks.

## Expected Location

Progress lives at:

```text
project/PROGRESS.md
```

Project-level agent instructions may live at:

```text
project/AGENTS.md
```

The workspace may contain multiple repositories or project areas. Repository names are not fixed. Do not assume names such as `backend/`, `frontend/`, `api/`, `web/`, or `worker/` exist.

The only stable convention is the `project/` directory.

## Phase 1: Read Progress

Before editing code for a non-trivial task:

1. Read `project/AGENTS.md` if it exists.
2. Read `project/PROGRESS.md` if it exists.
3. Summarize the current project/task state before editing.
4. Identify affected repositories or project areas through workspace inspection.
5. Use the existing progress state to continue work instead of starting blindly.

Extract from `project/PROGRESS.md`:

- current task
- current status
- completed work
- incomplete work
- next steps
- blockers
- risks
- open questions
- changed areas
- verification already done

If `project/PROGRESS.md` does not exist, create it when the task is non-trivial and there is meaningful state to preserve.

## Phase 2: Update Progress

Update `project/PROGRESS.md` at meaningful milestones.

Update progress:

- at the start of a non-trivial task
- when continuing work from a previous session
- after understanding the workspace and affected areas
- after meaningful implementation milestones
- after tests or verification
- when blockers, risks, decisions, or open questions appear
- before the final response for non-trivial tasks

Do not update progress for every tiny action.

## When To Use

Use this skill for:

- implementing features
- fixing complex bugs
- refactoring across multiple files
- changing API, database, auth, payment, files, background jobs, or cross-repo behavior
- continuing work from a previous session
- tasks where future agents need reliable handoff context

For trivial one-line changes, a final report may be enough.

## Recommended PROGRESS.md Format

Use this structure unless the existing file already has a clear project-specific format. Preserve useful existing content and update it in place.

```markdown
# Project Progress

## Current Snapshot

Status:
Current task:
Current objective:
Last updated:

## What Was Done

## Current State

## Verification Done

## Next Steps

## Open Questions

## Risks / Blockers

## Changed Areas
```

In `Changed Areas`, group work by repository or project area, for example:

```markdown
## Changed Areas

- `api/`: Updated authentication handler and request validation.
- `web/`: Added login form error state.
- `project/tests/`: Added cross-repo smoke test notes.
```

Use the actual discovered repository or area names. Do not invent fixed names.

## Update Rules

Record:

- meaningful implementation progress
- changed repositories or project areas
- important decisions
- incomplete work
- verification status
- commands that matter
- next steps
- blockers
- risks
- open questions

Do not record:

- every file opened
- every minor edit
- every command attempt unless it matters
- speculative thoughts that do not affect the project
- private reasoning
- noisy chronological logs

Prefer concise project-level summaries over detailed transcripts.

## Multi-Repository Rules

Repository names are not fixed.

Use `project/AGENTS.md`, workspace inspection, and the task context to identify repositories and project areas.

Group progress by repository or project area. If a change spans multiple repositories, make the relationship clear.

Examples of project areas include:

- application repository
- service repository
- worker repository
- shared package
- `project/docs/`
- `project/tests/`
- `project/scripts/`
- database or migration area
- deployment or CI area

Do not assume any repository exists until inspected.

## Final Response Requirement

The final response must include:

- whether `project/PROGRESS.md` was read
- whether `project/PROGRESS.md` was updated
- current task status
- what was completed
- what should happen next
- verification performed
- anything not verified

Keep the final response concise. Mention progress updates directly, for example:

```text
Read `project/PROGRESS.md` before starting and updated it with the implementation summary, verification results, and next steps.
```

If the task was trivial and progress was not updated, say so.

## Relationship With Other Skills

This skill works well with:

- `coke-workspace-orientation`
- `coke-end-to-end-verification`

Recommended order:

1. `coke-workspace-orientation`
2. `coke-project-progress`
3. implementation
4. `coke-end-to-end-verification`
5. `coke-project-progress` final update
6. final response
