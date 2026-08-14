# Go Idioms Required By This Architecture

Broad Go idiom guidance is out of scope — see `references/uber-go-style.md` for the distilled
style guide. This file lists only the idioms the architecture pattern directly depends on:

- **Accept interfaces, return structs.** Usecase and handler constructors take domain interfaces
  (`repository.ExampleRepository`) and return concrete pointers (`*CreateExampleEntityUsecase`).
- **Compile-time interface checks** on every repository/service implementation:
  `var _ repository.ExampleRepository = (*ExampleRepository)(nil)`.
- **`context.Context` first parameter** on every function that does IO, flowing unbroken from the
  Fiber request context to pgx so cancellation and deadlines propagate.
- **Constructor injection, wired once in `cmd/api/main.go`** — no globals, no `init()`, no service
  locator.
- **Unexported entity fields with getters**; value object constructors return `(T, error)` with a
  domain error; `...FromTrusted` variants exist only for reconstruction from persisted data.
- **Sentinel errors + `%w` wrapping + `errors.Is`/`errors.As`** to carry errors across layers
  (the Go equivalent of the Rust skill's `From` conversions with `?`).
- **Copies at boundaries**: entity getters returning slices/maps return copies; handler code
  copies fasthttp-backed strings (`c.Params`, `c.Query`) before they outlive the handler.
- **Owned goroutines**: any background work is a struct with `Start`/`Stop` where `Stop` signals
  and waits — never a bare `go func()` without lifecycle.
