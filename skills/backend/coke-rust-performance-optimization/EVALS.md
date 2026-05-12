# EVALS

## Purpose

These evals verify the skill triggers for measure-first optimization preserving architecture and
avoids out-of-scope requests.

## Positive Trigger Prompts

- "Optimize this slow Rust endpoint."
- "Benchmark this hot path."
- "Profile this worker throughput issue."
- "Reduce allocations in this loop."
- "Improve DB repository performance without breaking architecture."

## Negative Trigger Prompts

- "Set up CI."
- "Review PR quality."
- "Create a clean architecture skeleton."
- "Write TDD acceptance tests."
- "Deploy to production."

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

- **Input prompt**: "Optimize this slow Rust endpoint."
- **Expected skill usage**: `coke-rust-performance-optimization` is selected.
- **Expected output qualities**: Clear scope, concrete steps, boundary/safety rules, and
  verification guidance.
