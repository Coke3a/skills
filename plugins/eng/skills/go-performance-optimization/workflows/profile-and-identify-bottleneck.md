# Profile and Identify Bottleneck

## 1. Choose the profile for the question

- [ ] Where does CPU go → CPU profile (30s under load)
- [ ] Why is RSS high → heap `inuse_space`
- [ ] What churns the GC → heap `alloc_space` / allocs
- [ ] Are goroutines leaking → goroutine profile, two snapshots minutes apart
- [ ] Is something blocked/contended → block + mutex profiles (enable temporarily)
- [ ] pprof looks fine but latency is bad → execution trace (`go tool trace`)

If the symptom is "fine in isolation, bad under load" or "more cores don't help", the workload
under the profiler must itself be concurrent — a serial run cannot produce the contention you are
hunting. A CPU profile dominated by `pthread_cond_wait`/`futex` frames rather than application
frames means the process is parked on a lock, not computing.

## 2. Capture correctly

- [ ] pprof endpoint on an internal port only, never public
- [ ] Capture **during** a realistic workload (load test or production traffic)
- [ ] Record tool, command, workload, Go version, and environment

## 3. Read the profile

- [ ] `go tool pprof -http=:8080 profile.pb.gz` — flamegraph, then `top`, `list` on suspects
- [ ] Distinguish app frames from runtime frames (`gcBgMarkWorker` = GC pressure → allocation
      work, not GC-knob work first)
- [ ] Attribute the cost to the owning layer (handler serialization? usecase mapping? repository
      query wait? worker lock?)
- [ ] For DB-shaped waits, move to `EXPLAIN (ANALYZE, BUFFERS)` — Go profiles only show waiting

## 4. Conclude

- [ ] State the bottleneck with evidence (frames, percentages, snapshots)
- [ ] State the falsification check (what would disprove it)
- [ ] Fill in `templates/profiling-report.md`
- [ ] Pick the matching optimize-* workflow; change one thing, then re-profile with `-diff_base`
