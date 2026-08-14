# EVALS

## Purpose

These evals verify the skill triggers for Go clean architecture layers/repository pattern work and
avoids out-of-scope requests.

## Positive Trigger Prompts

- "Create a new Go backend feature following clean architecture."
- "Scaffold a Fiber handler, usecase, and repository for this entity."
- "Refactor this fat Fiber handler into proper layers."
- "Organize this Go project's directory structure for a Fiber + PostgreSQL service."
- "Add a sqlc repository implementation for this entity."
- "Define the repository interface and usecase structure for this feature."
- "Add a new admin API route group while keeping handler boundaries clean."
- "Design the error flow from domain errors to HTTP responses in this Go service."
- "Check whether this package follows handler -> usecase -> domain."
- "Add a background worker that processes pending jobs in this Go backend."

## Negative Trigger Prompts

- "Set up GitHub Actions CI for this Go project."
- "Write TDD tests for this bug."
- "Review this PR."
- "Profile this endpoint and reduce its allocations."
- "Deploy this app."
- "Create a Rust backend feature with Axum and Diesel."

## Expected Behavior

- Trigger only for in-scope requests.
- Follow the skill workflow and produce structured, actionable guidance.
- Coordinate with companion skills when needed (performance work → `coke-go-performance-optimization`).

## Must Not Do

- Must not take ownership of other skill domains.
- Must not fabricate command outcomes or measurements.
- Must not skip required safety and boundary guidance (buffer-reuse copies, goroutine lifecycle,
  sentinel error contracts).

## Pass Criteria

- [ ] Correct trigger decision for positive prompts.
- [ ] Correct non-trigger decision for negative prompts.
- [ ] Output includes skill-specific workflow and constraints.
- [ ] Output preserves clean architecture boundaries where applicable.

## Example Evaluation

- **Input prompt**: "Create a new Go backend feature following clean architecture."
- **Expected skill usage**: `coke-go-clean-architecture` is selected.
- **Expected output qualities**: Clear scope, concrete steps, boundary/safety rules, and
  verification guidance ending with `go build ./...`, `go vet ./...`, `go test -race ./...`.
