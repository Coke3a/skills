# EVALS

## Purpose

These evals verify the skill triggers for measurement-first Go performance work and avoids
out-of-scope requests.

## Positive Trigger Prompts

- "This Fiber endpoint's p95 latency is too high — optimize it."
- "Reduce the allocations in this Go hot path."
- "Our Go service's memory keeps growing in production — find out why."
- "The goroutine count keeps climbing; I think we have a leak."
- "This repository query is slow, speed it up."
- "Improve the throughput of this worker/queue processor."
- "Profile this Go service and find the bottleneck."
- "Benchmark this function before and after my change."
- "GC is eating 15% CPU — tune it."
- "Is this mutex causing contention under load?"

## Negative Trigger Prompts

- "Create a new Go backend feature following clean architecture."
- "Set up GitHub Actions CI."
- "Review this PR."
- "Write TDD tests for this bug."
- "Make this code cleaner / more idiomatic."
- "Optimize this Rust service's hot path."

## Expected Behavior

- Trigger only for in-scope requests.
- Always establish goal + baseline before changing code; never claim wins without before/after.
- Preserve clean architecture boundaries; propose (not perform) layer changes.
- Coordinate with `coke-go-clean-architecture` for structure questions.

## Must Not Do

- Must not fabricate benchmark/profile results or claim improvement without data.
- Must not optimize speculatively or rewrite broadly without an identified bottleneck.
- Must not weaken correctness (race safety, buffer copies, validation) for speed.
- Must not tune GC knobs before allocation evidence.

## Pass Criteria

- [ ] Correct trigger decision for positive prompts.
- [ ] Correct non-trigger decision for negative prompts.
- [ ] Output follows the optimization loop (goal → baseline → profile → smallest change → verify).
- [ ] Output reports before/after with variance honesty (benchstat `~` handling).

## Example Evaluation

- **Input prompt**: "This Fiber endpoint's p95 latency is too high — optimize it."
- **Expected skill usage**: `coke-go-performance-optimization` is selected.
- **Expected output qualities**: Defines the goal and workload, measures a baseline, profiles
  before changing code, optimizes in the owning layer, verifies with `go test -race` and
  benchstat/load-test comparison, reports tradeoffs.
