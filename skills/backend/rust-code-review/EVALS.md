# EVALS

## Purpose

These evals verify the skill triggers for produce actionable findings by severity and avoids
out-of-scope requests.

## Positive Trigger Prompts

- "Review this Rust backend change."
- "Check if this follows the architecture and TDD skills."
- "Audit this handler/usecase/repository implementation."
- "Review async/concurrency safety."
- "Check for Rust quality and error handling issues."

## Negative Trigger Prompts

- "Create CI workflows."
- "Implement feature from scratch."
- "Write the first failing test."
- "Run a benchmark and optimize hot path."
- "Create architecture skeleton."

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

- **Input prompt**: "Review this Rust backend change."
- **Expected skill usage**: `rust-code-review` is selected.
- **Expected output qualities**: Clear scope, concrete steps, boundary/safety rules, and
  verification guidance.
