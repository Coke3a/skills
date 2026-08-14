# Optimize Domain or Usecase

## 1. Confirm scope

- [ ] Domain optimization is pure and has no infra/HTTP dependency.
- [ ] Usecase optimization does not import Axum/Diesel/schema.
- [ ] Repository trait boundary remains intact.

## 2. Optimize

- [ ] Remove unnecessary clone/allocation only if measured/hot.
- [ ] Avoid repeated parsing/formatting.
- [ ] Batch repository operations through trait if N+1 is found.
- [ ] Bound concurrency if usecase calls many async operations.
- [ ] Keep error semantics unchanged.

## 3. Verify

- [ ] Run correctness tests.
- [ ] Run benchmark/profile again.
- [ ] Compare before/after.
