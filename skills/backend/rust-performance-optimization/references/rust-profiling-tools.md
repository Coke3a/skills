# Rust Profiling Tools

Choose the smallest useful tool for the observed issue:

- `perf`: Linux CPU profiling.
- `cargo flamegraph` / flamegraph: CPU flamegraphs for release builds.
- `samply`: Firefox profiler capture, useful on Linux/macOS.
- Instruments: macOS CPU, allocation, and time profiling.
- VTune: detailed CPU and threading analysis.
- DHAT: allocation profiling through Valgrind.
- heaptrack: allocation profiling on Linux.
- bytehound: Rust allocation profiling on Linux.
- `tracing` spans and metrics: request, worker, async, and IO timing.
- DB `EXPLAIN` / `EXPLAIN ANALYZE`: query planning and execution evidence.

## Build Notes

- Profile release builds unless debug-only behavior is the target.
- Enable debug line info or frame pointers when the profiler needs better symbols.
- Record tool, command, workload, build profile, and environment.
- Use allocation tools for allocation rate/peak memory, not CPU-only profilers.
- Use spans/metrics when async or IO latency is distributed across awaits.
