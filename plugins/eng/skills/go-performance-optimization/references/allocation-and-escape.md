# Allocation and Escape Analysis

Optimize allocations only when they are in a hot path or profiling shows allocation cost
(allocs/op in benchmarks, `alloc_space` in heap profiles, GC share in CPU profiles).

## Escape analysis

`go build -gcflags='-m'` (or `-m -m`) prints what escapes to the heap. Common causes:

- Returning pointers to locals.
- Storing values into `interface{}`/`any` (boxing) — including every `fmt` call argument.
- Closures capturing variables by reference.
- Slices that grow beyond what the compiler can prove fits on the stack.

## Check for

- Slices/maps created without capacity when size is known.
- `fmt.Sprintf` / string concatenation in loops.
- Repeated `[]byte(...)` / `string(...)` conversions of the same value — each copies.
- Interface boxing inside hot loops.
- Row → entity → output → DTO mapping chains that copy large values more than needed.
- Repeated serialization/deserialization of the same data.
- Per-iteration allocation of buffers, encoders, or regexes that could be reused.
- The uber-go hot-path trio: `fmt.Sprint` where `strconv` suffices; un-cached string→byte
  conversions; missing container capacity.

## Options

- Preallocate: `make([]T, 0, n)`, `make(map[K]V, n)` — slice capacity is a guarantee (appends up
  to capacity allocate nothing); map hints reduce rehashing.
- `strings.Builder` (+ `Grow(n)`) for concatenation; `strconv.Itoa`/`AppendInt` over `fmt` for
  primitives.
- Keep one representation (`string` or `[]byte`) end-to-end; the compiler elides some conversions
  (`m[string(b)]`, `switch string(b)`) — rely on those before reaching for tricks.
- `sync.Pool` for transient hot-path buffers (`*bytes.Buffer`, `[]byte`, encoder state): store
  pointer types, `Reset()` before reuse, never retain pooled memory after `Put`. Only for measured
  hot paths — pools add complexity and drain on GC.
- Use concrete types or generics in inner loops; keep interfaces at architecture boundaries.
- Struct field alignment (large-to-small) only for huge `[]struct` slices — rarely worth it;
  do not apply the `fieldalignment` vet check reflexively.
- `unsafe.String`/`unsafe.SliceData` are a last resort requiring ownership discipline — not by
  default.

## Rules

- Do not contort APIs or clone-free-at-any-cost the domain model for tiny unmeasured gains.
- Do not replace clear owned DTOs with aliased buffers — especially not fasthttp-backed ones
  (see `references/fiber-performance.md`).
- Document before/after measurement (benchstat allocs/op and time/op).
