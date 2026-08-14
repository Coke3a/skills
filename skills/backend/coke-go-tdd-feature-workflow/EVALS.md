# EVALS

## Purpose

These evals verify the skill triggers for Go TDD work in the design phase, refuses outside that
phase, and never gets picked for Rust.

## Positive Trigger Prompts

- "The API design is approved — now drive the implementation with TDD."
- "Write the failing test first for this Go usecase, then make it pass."
- "Use TDD to work out what methods this feature's repository interface needs."
- "Add a regression test for this bug in the Fiber handler, then fix it."
- "I have the endpoints and schema; design the deeper flow through tests."
- "Characterise this Go usecase's current behaviour before I refactor it."
- "These Go tests are failing — work out whether the test or the code is wrong."

## Negative Trigger Prompts

- "Write TDD tests for this Rust usecase." → `coke-tdd-feature-workflow`
- "Scaffold the layers for a new Go feature." → `coke-go-clean-architecture`
- "Where should the repository interface live?" → `coke-go-clean-architecture`
- "Set up GitHub Actions for this Go project."
- "Review this PR."
- "Benchmark this endpoint." → `coke-go-performance-optimization`
- "Write integration tests for these sqlc queries." → out of scope, say so

## Gate Prompts

Each must produce a hard stop with a named missing precondition and no code written.

- "Use TDD to build the notifications feature." (no design source given)
- "Here's the ticket: 'users should be able to export reports'. Start with tests."
- "The endpoints are POST /v1/exports and GET /v1/exports/:id. Go." (no schema, no payloads)

And the other direction — must route to characterisation rather than the design loop:

- "Add TDD tests to `LoginUsecase`." (already implemented)

## Expected Behavior

- Run the entry gate before anything else and quote its sources.
- Stop on a failed gate, name the missing precondition, offer the three routes, write nothing.
- Confirm acceptance criteria with the user before the first test.
- One criterion at a time, red → green → refactor.
- Discover the repository interface from the usecase test rather than the schema.
- Hand off layer/naming/error-taxonomy questions to `coke-go-clean-architecture`.

## Must Not Do

- Must not write tests or code after a failed gate.
- Must not write every test up front and implement afterwards.
- Must not introduce gomock, mockery, or testify.
- Must not claim repository integration is covered.
- Must not apply Rust conventions (`tests/` directory, `#[cfg(test)]`, `cargo test`) to Go.
- Must not fabricate command output.

## Pass Criteria

- [ ] Correct trigger decision for positive and negative prompts.
- [ ] Gate prompts produce a hard stop naming the specific missing precondition.
- [ ] Already-implemented prompts route to `workflows/add-tests-to-existing-feature.md`.
- [ ] Tests use `package <pkg>_test`, hand-written fakes, `errors.Is`, `t.Parallel()`.
- [ ] Usecase error tests assert both the usecase sentinel and the absence of the leaked one.
- [ ] Verification ends with `go build ./...`, `go vet ./...`, `go test -race ./...`.
- [ ] The summary states repository integration is not covered.

## Example Evaluation

- **Input prompt**: "The endpoints are POST /v1/exports and GET /v1/exports/:id. Go."
- **Expected skill usage**: `coke-go-tdd-feature-workflow` triggers, then hard-stops.
- **Expected output qualities**: names endpoints as present; names request/response shapes and
  database schema as missing; offers to be pointed at a source, answered inline, or routed to
  `coke-spec-review`; states plainly that no tests or code were written.
