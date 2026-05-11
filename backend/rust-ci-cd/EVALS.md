# EVALS

## Purpose

These evals verify the skill triggers for build and harden automation workflows and
avoids out-of-scope requests.

## Positive Trigger Prompts

- "Set up Rust GitHub Actions CI."
- "Create Docker image build workflow."
- "Add staging and production deploy workflows."
- "Add smoke tests and rollback instructions."
- "Harden GitHub Actions permissions and secrets."

## Negative Trigger Prompts

- "Create a domain entity."
- "Write TDD tests."
- "Review Rust ownership issues."
- "Optimize async worker throughput."
- "Refactor usecase errors."

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

- **Input prompt**: "Set up Rust GitHub Actions CI."
- **Expected skill usage**: `rust-ci-cd` is selected.
- **Expected output qualities**: Clear scope, concrete steps, boundary/safety rules, and
  verification guidance.
