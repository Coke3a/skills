# EVALS

## Purpose

These evals verify the skill triggers for focus on acceptance criteria and red/green/refactor and
avoids out-of-scope requests.

## Positive Trigger Prompts

- "Implement this feature with TDD."
- "Add a regression test for this bug first."
- "Choose the smallest useful test for this behavior."
- "Where should Rust tests go for this usecase?"
- "Add tests without creating every test level upfront."

## Negative Trigger Prompts

- "Create Docker deployment workflow."
- "Refactor architecture layers."
- "Review this code for concurrency bugs."
- "Optimize this DB query."
- "Set up production rollback."

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

- **Input prompt**: "Implement this feature with TDD."
- **Expected skill usage**: `coke-tdd-feature-workflow` is selected.
- **Expected output qualities**: Clear scope, concrete steps, boundary/safety rules, and
  verification guidance.
