# Rust Idioms

## Borrowing vs cloning decision tree

**Default to borrowing (`&T`).** Only clone when you have a specific reason.

### When to clone
- You need to change the object AND preserve the original (immutable snapshots).
- Sharing data across threads via `Arc` (clone the Arc, not the inner data).
- The underlying API requires owned data and you cannot change the API.
- Caching results that will be returned multiple times.

### When NOT to clone
- Function only needs read access -- use `&T`.
- Prefer `&str` over `String`, `&[T]` over `Vec<T>` in function parameters.
- Do not clone a reference argument; if you need ownership, make the parameter owned for the caller to decide.

```rust
// GOOD: borrow when you only read
fn process(name: &str) {
    println!("Hello {name}");
}

// BAD: unnecessary clone
fn process_owned(name: String) { /* only reads name */ }
let user = String::from("foo");
process_owned(user.clone()); // wasteful
```

### Clone traps to avoid
- Auto-cloning inside loops: `.map(|x| x.clone())` -- prefer `.cloned()` or `.copied()`.
- Cloning large data structures (`Vec<T>`, `HashMap<K, V>`).
- Cloning because of bad API design instead of adjusting lifetimes.

## Copy trait guidance

If a type is small and cheap to copy, pass by value instead of reference.

### When to derive `Copy`
- All fields are themselves `Copy`.
- Struct is small: up to ~24 bytes (2-3 machine words).
- Struct represents plain data with no heap allocations (no `Vec`, `String`, `Box`).

```rust
// GOOD: small, all-Copy fields
#[derive(Debug, Copy, Clone)]
struct Point { x: f32, y: f32, z: f32 }  // 12 bytes

// GOOD: tag enum
#[derive(Debug, Copy, Clone)]
enum Direction { North, South, East, West }

// BAD: contains String (heap-allocated)
#[derive(Debug, Clone)]
struct User { age: i32, name: String }  // Cannot be Copy
```

Enum size is based on the largest variant. Keep variant payloads small if deriving `Copy`.

### Primitive sizes for reference

| Type | Size | Type | Size |
|------|------|------|------|
| `bool` | 1B | `i8`/`u8` | 1B |
| `i16`/`u16` | 2B | `i32`/`u32`/`f32` | 4B |
| `char` | 4B | `i64`/`u64`/`f64` | 8B |
| `isize`/`usize` | arch | `i128`/`u128` | 16B |

## Option/Result handling

### Pattern selection guide

**`match`** -- when pattern matching against inner variants:
```rust
match result {
    Ok(Direction::North) => { /* ... */ },
    Ok(other) => { /* ... */ },
    Err(e) => { /* ... */ },
}
```

**`let-else`** -- when divergent code is a simple return/break/continue:
```rust
let Some(item) = collection.get(id) else {
    return Err(NotFound("item not found".into()));
};
// item is now available in scope
```

**`if let`** -- when the else branch needs computation:
```rust
if let Some(value) = cache.get(&key) {
    process(value);
} else {
    let computed = expensive_computation();
    cache.insert(key, computed);
}
```

**`?` operator** -- when you do not care about the Err value and want to propagate:
```rust
let data = fetch_data(url).await?;
let parsed = parse(data)?;
```

### Anti-patterns
- Using `match` to convert between Result and Option -- prefer `.ok()`, `.ok_or()`, `.ok_or_else()`.
- Using `unwrap`/`expect` outside tests -- use `let-else`, `?`, or `unwrap_or_default`.

## Preventing early allocation

When using `or`, `map_or`, `unwrap_or`, `ok_or`, the fallback is evaluated eagerly. Use the `_else` variants when the fallback involves allocation or computation:

```rust
// BAD: allocates format! string even on Ok path
x.ok_or(ParseError::Info(format!("value {x}")))

// GOOD: only allocates on None/Err
x.ok_or_else(|| ParseError::Info(format!("value {x}")))

// GOOD: no allocation needed, eager is fine
x.ok_or(ParseError::ValueAbsent)

// GOOD: default collection only created on None
x.unwrap_or_else(Vec::new)
// or even better:
x.unwrap_or_default()
```

### Mapping errors
Use `inspect_err` to log and `map_err` to transform:
```rust
result
    .inspect_err(|err| tracing::error!("operation failed: {err}"))
    .map_err(|err| UsecaseError::from(err))?;
```

## Iterator patterns vs manual loops

Both `for` and `.iter()` chains are idiomatic. Choose based on context:

### Prefer `for` loops when
- You need early exits (`break`, `continue`, `return`).
- Simple iteration with side-effects (logging, IO).
- Readability matters more than chaining.

### Prefer iterators when
- Transforming collections (filter/map/collect).
- Composing multiple steps elegantly.
- Using `.enumerate()`, `.windows()`, `.chunks()`.
- Combining data from multiple sources without intermediate allocations.

```rust
// Iterator: transform + collect
let names: Vec<_> = users.iter()
    .filter(|u| u.is_active())
    .map(|u| u.name())
    .collect();

// For loop: early exit
for item in &items {
    if item.is_invalid() {
        return Err(ValidationError);
    }
    process(item);
}
```

### Iterator anti-patterns
- Do not collect into a Vec just to iterate again -- pass the iterator directly.
- Prefer `.iter()` over `.into_iter()` unless you need to consume ownership.
- Prefer `.sum()` over `.fold(0, |acc, x| acc + x)` -- `.sum()` is specialized.
- Format chained calls one-per-line for readability.

```rust
// BAD: useless intermediate allocation
let doubled: Vec<_> = items.iter().map(|x| x * 2).collect();
process(doubled.iter());

// GOOD: pass iterator directly
let doubled = items.iter().map(|x| x * 2);
process(doubled);  // fn process(items: impl Iterator<Item = i32>)
```

Iterators are lazy -- `.filter()`, `.map()` etc. do nothing until consumed by `.collect()`, `.sum()`, `.for_each()`, etc. The compiler fuses iterator chains into a single tight loop.

## Performance mindset

### Golden rule: don't guess, measure
- Always build with `--release` for benchmarks.
- `cargo clippy -- -D clippy::perf` for performance lint hints.
- `cargo bench` for micro-benchmarks.
- `cargo flamegraph` (or `samply` on macOS) for profiling.

### Flamegraph interpretation
- y-axis = stack depth (main at bottom, callees stacked above).
- Box width = total CPU time for that function.
- Wide boxes = hot functions to investigate.
- Color is random and not significant.

## Avoiding redundant cloning

- Clone only when you truly need a new owned copy.
- If you only need read access, use `.iter()` or slices (`&[T]`).
- If you need to clone, do it at the last possible moment.
- Use `Cow<'_, str>` when ownership is sometimes needed but not always:

```rust
use std::borrow::Cow;

fn greet(name: Cow<'_, str>) {
    println!("Hello {name}");
}

greet(Cow::Borrowed("Alice"));
greet(Cow::Owned(format!("User-{id}")));
```

## Stack vs heap decisions

### Keep on the stack
- Small types implementing `Copy` (`usize`, `bool`, small structs).
- Return small types by value.

### Use the heap
- Large types (> 512 bytes) -- avoid passing by value.
- Recursive data structures (`Box<Node>`).
- Dynamically sized data (`Vec<T>`, `String`).

```rust
// Heap-allocate recursive structures
enum TreeNode<T> {
    Leaf(T),
    Branch(Box<TreeNode<T>>, Box<TreeNode<T>>),
}
```

### Be mindful
- Only use `#[inline]` when benchmarks prove it helps.
- Avoid massive stack allocations; box them or use `vec![0; size].into_boxed_slice()`.
- For large const arrays, consider `smallvec`.

## Zero-cost abstractions

Rust iterators, generics, and trait dispatch are compiled away into efficient machine code:
- Iterator chains are fused into single loops at compile time.
- Monomorphized generics have zero runtime overhead.
- Prefer iterators over manual `for` loops for collection transforms -- the compiler optimizes them equally well or better.
- Avoid creating intermediate collections unless truly needed.
