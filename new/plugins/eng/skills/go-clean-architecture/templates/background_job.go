// Template: background worker owned by a struct with explicit Start/Stop.
// Target location: internal/worker/{name}_worker.go
// Replace "app" in imports with the project's module path.
//
// Follows the uber-go goroutine lifecycle rules: no fire-and-forget, no
// goroutines in init(), and Stop signals the goroutine AND waits for exit.
// Business logic stays in the usecase — the worker owns only scheduling,
// timeout, and lifecycle.
package worker

import (
	"context"
	"log/slog"
	"sync"
	"time"

	"app/internal/usecase/examplefeature"
)

type ExampleWorker struct {
	interval   time.Duration
	jobTimeout time.Duration
	usecase    *examplefeature.ProcessPendingUsecase

	stop     chan struct{}
	done     chan struct{}
	stopOnce sync.Once
}

func NewExampleWorker(
	interval time.Duration,
	jobTimeout time.Duration,
	usecase *examplefeature.ProcessPendingUsecase,
) *ExampleWorker {
	return &ExampleWorker{
		interval:   interval,
		jobTimeout: jobTimeout,
		usecase:    usecase,
		stop:       make(chan struct{}),
		done:       make(chan struct{}),
	}
}

// Start launches the worker goroutine. Called from cmd/api/main.go (or a
// dedicated cmd/worker binary), never from init().
func (w *ExampleWorker) Start() {
	go w.run()
}

// Stop signals the worker and blocks until the goroutine has exited, so
// shutdown never leaves a half-finished job behind. sync.Once makes it safe to
// call from more than one shutdown path — closing an already-closed channel
// panics, and shutdown paths have a habit of multiplying.
func (w *ExampleWorker) Stop() {
	w.stopOnce.Do(func() { close(w.stop) })
	<-w.done
}

func (w *ExampleWorker) run() {
	defer close(w.done)

	ticker := time.NewTicker(w.interval)
	defer ticker.Stop()

	for {
		select {
		case <-ticker.C:
			w.runOnce()
		case <-w.stop:
			return
		}
	}
}

func (w *ExampleWorker) runOnce() {
	ctx, cancel := context.WithTimeout(context.Background(), w.jobTimeout)
	defer cancel()

	if err := w.usecase.Execute(ctx); err != nil {
		// Log-and-degrade is this worker's error handling; the error is not
		// returned anywhere, so logging here does not double-handle it.
		slog.Error("example worker run failed", "err", err)
	}
}
