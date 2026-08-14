# Bounds Checks and Hot Loops

- Slice/Vec indexing can introduce bounds checks.
- Prefer iterator-based loops when they clearly express the hot path.
- Slice before a loop when it helps the compiler prove bounds.
- Add assertions before the loop if that helps bounds-check elimination and remains safe.
- Avoid repeated indexing when iterators or zipped slices express the same work.
- Benchmark loop micro-optimizations; do not assume they matter.
- Use `unsafe` / `get_unchecked` only as a last resort after safe approaches fail and benchmarks
  prove meaningful benefit.
- Unsafe optimization requires explicit safety comments, tests, benchmark evidence, and final code
  review.
