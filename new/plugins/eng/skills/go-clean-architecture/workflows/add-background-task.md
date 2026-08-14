# Add a background task

Use this when adding a background worker (periodic job, queue consumer, outbox processor) that
preserves dependency direction and goroutine lifecycle rules.

## 1. Define the job

- [ ] Name the job and its trigger: interval tick, queue message, or startup
- [ ] Define the per-run timeout and what happens when a run fails (retry policy, logging)
- [ ] Decide idempotency: a rerun after crash or retry must be safe

## 2. Put business logic in a usecase

- [ ] Add `internal/usecase/{feature}/{action}.go` owning the actual work
- [ ] The usecase depends on domain repository/service interfaces only
- [ ] The worker will call `Execute(ctx)` — no business decisions in the worker loop

## 3. Implement the worker

- [ ] Add `internal/worker/{name}_worker.go` following `templates/background_job.go`
- [ ] Struct owns `stop` and `done` channels; `Start()` launches, `Stop()` closes `stop` and
      blocks on `done`
- [ ] Each run gets `context.WithTimeout` and a `defer cancel()`
- [ ] Worker errors are logged and degraded — never both logged and returned
- [ ] No goroutines in `init()`; no fire-and-forget spawns inside the run
- [ ] For queue consumers: bounded concurrency (worker count or semaphore), defined queue-full
      behavior, capped backoff with jitter for retries

## 4. Wire lifecycle

- [ ] Construct and `Start()` the worker in the composition root (`cmd/api/main.go` or a dedicated
      `cmd/worker`)
- [ ] Call `Stop()` in `run()` after `Listen` returns — not from a Fiber `OnPostShutdown` hook,
      which fires twice and may not finish before the process exits
      (see `references/architecture.md`)
- [ ] Confirm shutdown order: stop accepting HTTP → stop workers → close pool (`defer pool.Close()`
      in `run()` gives this ordering for free)

## 5. Verify

- [ ] Dependency direction preserved: worker → usecase → domain; worker does not import infra
      directly (repositories arrive via the usecase)
- [ ] A test or `goleak` check confirms the worker goroutine exits on `Stop()`
- [ ] `go build ./...`, `go vet ./...`, `go test -race ./...` pass
