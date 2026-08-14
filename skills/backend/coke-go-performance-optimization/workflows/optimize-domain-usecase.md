# Optimize Domain / Usecase Level

Use when the profile shows cost inside domain logic or usecase orchestration (validation, mapping,
computation) — not IO.

## 1. Confirm ownership

- [ ] The cost is in domain/usecase frames, not repository wait or handler serialization
- [ ] The fix does not require importing infra, Fiber, or sqlc types into domain/usecase

## 2. Candidate optimizations (smallest first)

- [ ] Remove repeated work: cache derived values inside the call, hoist invariant computation out
      of loops
- [ ] Reduce allocations in mapping: preallocate output slices (`make([]T, 0, len(rows))`),
      `strings.Builder` for assembly, `strconv` over `fmt`
- [ ] Avoid intermediate collections; filter before materializing
- [ ] Replace regex/reflection-heavy validation in hot paths with explicit checks
- [ ] Keep interfaces at the boundary; concrete types inside inner loops
- [ ] Algorithmic fixes (O(n²) membership → map lookup) before micro-tuning

## 3. Keep the architecture

- [ ] Entities/value objects keep invariants — do not remove validation for speed without approval
- [ ] Copies at boundaries stay (getter copies, DTO ownership); optimize inside the layer instead
- [ ] If the real fix is a repository batch method (N+1), switch to
      `workflows/optimize-repository-db.md`

## 4. Verify

- [ ] `go test -race ./...`
- [ ] Benchmark before/after with benchstat (`workflows/benchmark-hot-path.md`)
- [ ] Report per `templates/before-after-report.md`
