# Evaluations

## Eval 1: Scaffold feature
**Input:** "Create a new Project feature with list and create endpoints."

**Expected:**
- Files placed in `src/usecases`, `src/handlers`, `src/domain`, `src/infra`.
- Handler uses usecase input/output structs; no business logic in handler.
- Repository trait in `src/domain`; ORM impl in `src/infra`.
- Usecase owns error enum and maps repo errors to user-facing variants.

**Pass/Fail checklist:**
- [ ] Route only parses request + calls usecase
- [ ] Usecase has explicit input/output structs
- [ ] Repo trait returns Option for not-found
- [ ] ORM implementation exists and is wired

## Eval 2: Refactor handler into clean layers
**Input:** "Refactor this messy axum handler into clean architecture layers."

**Expected:**
- New usecase struct extracted.
- Repo trait defined for data access.
- Route reduced to thin mapping.
- Errors mapped through usecase variants.

**Pass/Fail checklist:**
- [ ] Usecase introduced with Arc deps
- [ ] Repo trait extracted
- [ ] Route no longer contains business logic
- [ ] Error mapping is centralized in usecase

## Eval 3: Error handling for unique constraint
**Input:** "DB unique violation on project name."

**Expected:**
- Repo returns rich error with op context.
- Usecase maps to `UsecaseError::Conflict`.
- Route returns HTTP 409 with a stable error code.

**Pass/Fail checklist:**
- [ ] Repo error contains operation context
- [ ] Usecase maps to Conflict
- [ ] Route returns 409 with clear code/message

## Scoring rubric
- **Pass:** All checklist items are satisfied.
- **Fail:** Any checklist item is missing.
