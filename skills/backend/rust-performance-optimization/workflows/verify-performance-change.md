# Verify Performance Change

## 1. Correctness

- [ ] Run relevant tests.
- [ ] Run cargo test --all-features.
- [ ] Confirm behavior unchanged unless intentionally changed.

## 2. Architecture

- [ ] Handler remains thin.
- [ ] Usecase does not depend on infra/HTTP.
- [ ] Domain remains pure.
- [ ] Infra owns DB-specific optimization.
- [ ] No DTO/row leakage.
- [ ] No bypassed repository boundary.

## 3. Performance

- [ ] Re-run benchmark/profile.
- [ ] Compare baseline and after.
- [ ] Identify variance/uncertainty.
- [ ] If impact is inconclusive, report that honestly.

## 4. Quality

- [ ] Run fmt/clippy if code changed.
- [ ] Request rust-code-review for non-trivial optimization.
- [ ] Document tradeoffs.
