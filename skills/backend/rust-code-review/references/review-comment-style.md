# Review Comment Style

- Make comments actionable.
- Explain why the issue matters.
- Suggest a concrete fix.
- Use severity consistently.
- Distinguish blockers from optional suggestions.
- Avoid vague comments like "This is bad", "Improve this", or "Looks weird".
- Avoid personal tone; review the code and behavior.
- Praise good decisions when useful.
- Avoid performance nitpicks unless cost is plausible.

Prefer:

- "Move this validation into `ExampleEntityName` because handlers must not own domain
  invariants."
- "This usecase test asserts a fake call count, but the observable behavior is the
  returned `UsecaseError`. Assert the error instead."
- "This repository maps `diesel::result::Error::NotFound` into `RepoError::Internal`. It
  should map to `Ok(None)` for `find_by_id`."
- "This handler performs CPU-heavy parsing directly inside an async request path. Move
  it to `spawn_blocking` or a bounded worker if this is expected to be expensive."
- "This `std::sync::MutexGuard` is held across `.await`, which can deadlock or starve
  the runtime. Limit the lock scope before the await or use an async-aware design."
