# Optimize Allocation and Clone Hot Path

## 1. Confirm hot allocation

- [ ] Profile or benchmark indicates allocation/clone cost.
- [ ] Identify exact hot call site.

## 2. Optimize safely

- [ ] Remove unnecessary clone.
- [ ] Borrow instead of owning if lifetimes remain simple.
- [ ] Move values instead of cloning where ownership allows.
- [ ] Pre-allocate Vec/String when size is known.
- [ ] Avoid format!/to_string in loops.
- [ ] Use Cow/Bytes/Arc only when justified.
- [ ] Avoid making APIs lifetime-heavy without measurement.

## 3. Verify

- [ ] Re-run benchmark/profile.
- [ ] Check readability did not significantly degrade.
- [ ] Run correctness tests.
