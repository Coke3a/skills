# Optimize Allocation / GC Pressure

## 1. Get evidence

- [ ] Benchmark allocs/op (`-benchmem`) or heap profile `alloc_space` under load
- [ ] GC share in CPU profile or `GODEBUG=gctrace=1` — allocation work comes before GC-knob work
- [ ] `go build -gcflags='-m'` on the hot files to see what escapes and why

## 2. Candidate optimizations (smallest first)

- [ ] Preallocate slices/maps where size is known (`make([]T, 0, n)`)
- [ ] `strings.Builder` + `Grow`; `strconv` over `fmt` for primitives
- [ ] Hoist repeated `[]byte`/`string` conversions and compiled regexes out of loops
- [ ] Remove interface boxing from inner loops (concrete types or generics)
- [ ] Reuse buffers across iterations; `sync.Pool` only for measured hot-path transients
      (pointer types, `Reset()` on reuse, nothing retained after `Put`)
- [ ] Trim mapping chains that copy the same data repeatedly across layers
- [ ] After allocation work: GOGC/GOMEMLIMIT tuning per `references/gc-and-memory.md`

## 3. Keep the architecture and safety

- [ ] DTO ownership and boundary copies stay — never alias fasthttp buffers to save an allocation
- [ ] Domain invariants and validation stay
- [ ] No `unsafe` string/slice tricks without explicit approval

## 4. Verify

- [ ] `go test -race ./...`
- [ ] benchstat shows allocs/op and time/op improvement; `~` reported honestly
- [ ] Heap profile / RSS confirms the effect at process level if that was the goal
