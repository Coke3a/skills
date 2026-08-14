# Performance Principles

- Measure first; do not optimize by guessing.
- Optimize hot paths only — the uber-go guide's own performance rules are explicitly scoped to the
  hot path, and this skill keeps that scoping.
- Prefer clarity until measurement proves a need for complexity.
- Make small, safe changes that can be verified; one change per measurement cycle.
- Re-measure after each change with the same workload.
- Optimize within the correct architecture layer.
- Keep correctness before speed; `go test -race ./...` stays green throughout.
- Do not introduce `unsafe` or reflection tricks by default.
- Do not perform speculative broad rewrites.
- Keep tradeoffs visible in the final report.
