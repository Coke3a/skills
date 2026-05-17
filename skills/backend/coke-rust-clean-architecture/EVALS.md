# EVALS

## Purpose

These evals verify the skill triggers for focus on clean architecture layers/repo pattern and avoids
out-of-scope requests.

## Positive Trigger Prompts

- "Create a new Rust backend feature following clean architecture."
- "Refactor this handler/usecase/domain code into proper layers."
- "Organize this Rust handlers directory into app, shared, and router modules."
- "Add a new endpoint surface while keeping handler boundaries clean."
- "Add a Diesel repository implementation for this entity."
- "Define repository trait and usecase structure."
- "Organize usecases, domain services, repositories, entities, and value objects for this Rust backend."
- "Refactor this backend into feature/action usecases and domain service ports."
- "Check whether this module follows handler -> usecase -> domain."

## Negative Trigger Prompts

- "Set up GitHub Actions CI."
- "Write TDD tests for this bug."
- "Review this PR."
- "Benchmark this hot path."
- "Deploy this app."

## Expected Behavior

- Trigger only for in-scope requests.
- Follow the skill workflow and produce structured, actionable guidance.
- Coordinate with companion skills when needed.

## Must Not Do

- Must not take ownership of other skill domains.
- Must not fabricate command outcomes or measurements.
- Must not skip required safety and boundary guidance.

## Pass Criteria

- [ ] Correct trigger decision for positive prompts.
- [ ] Correct non-trigger decision for negative prompts.
- [ ] Output includes skill-specific workflow and constraints.
- [ ] Output preserves clean architecture boundaries where applicable.

## Example Evaluation

- **Input prompt**: "Create a new Rust backend feature following clean architecture."
- **Expected skill usage**: `coke-rust-clean-architecture` is selected.
- **Expected output qualities**: Clear scope, concrete steps, boundary/safety rules, and
  verification guidance.
