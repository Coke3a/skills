# Advanced Patterns

## Generics and Static Dispatch

Generics are abstract stand-ins for concrete types. Rust performs monomorphization at compile time, generating specialized machine code for each concrete type used. No runtime cost.

```rust
fn specialized_sum<T: MyTrait>(iter: impl Iterator<Item = T>) -> T {
    iter.map(|x| x.random_mapping()).sum()
}
```

### When to use static dispatch
- Zero runtime cost needed (tight loops, performance-critical paths).
- Types are known at compile time.
- Single-use implementations (monomorphized).

### Trade-off: larger binary size and slower compile times due to per-type code generation.

## Trait Objects and Dynamic Dispatch

Dynamic dispatch uses `dyn Trait` behind a pointer (`Box<dyn Trait>`, `Arc<dyn Trait>`, `&dyn Trait`). Calls go through a vtable at runtime.

```rust
trait Animal {
    fn greet(&self) -> String;
}

struct Dog;
impl Animal for Dog {
    fn greet(&self) -> String { "woof".to_string() }
}

struct Cat;
impl Animal for Cat {
    fn greet(&self) -> String { "meow".to_string() }
}

fn all_greetings(animals: Vec<Box<dyn Animal>>) {
    for animal in animals {
        println!("{}", animal.greet());
    }
}
```

### When to use dynamic dispatch
- Runtime polymorphism is needed.
- Storing different implementations in one collection.
- Abstracting internals behind a stable interface.
- Plugin-style architectures.

## Static vs Dynamic Trade-offs

| Aspect              | Static (`impl Trait`)          | Dynamic (`dyn Trait`)           |
|---------------------|--------------------------------|---------------------------------|
| Performance         | Faster, inlined                | Slower: vtable indirection      |
| Compile time        | Slower: monomorphization       | Faster: shared code             |
| Binary size         | Larger: per-type codegen       | Smaller                         |
| Flexibility         | One type at a time             | Can mix types in collections    |
| Error clarity       | Clearer type errors            | Erased types can confuse errors |

Rule of thumb: start with generics, then use `dyn Trait` when flexibility outweighs speed.

## Architecture Note: Arc<dyn Trait> for Repositories

This architecture defaults to `Arc<dyn Trait>` for repository interfaces. Dynamic dispatch is chosen here because:
- Repositories are swapped between real and mock implementations.
- The flexibility of runtime polymorphism matters more than vtable overhead for I/O-bound operations.
- `Arc` provides shared ownership across async tasks and threads.

```rust
pub struct GetUserUsecase {
    user_repo: Arc<dyn UserRepository>,
}
```

## Dynamic Dispatch Ergonomics

- Prefer `&dyn Trait` over `Box<dyn Trait>` when you don't need ownership.
- Use `Arc<dyn Trait>` for shared access across threads.
- Don't use `dyn Trait` if the trait has methods returning `Self`.
- Avoid boxing too early -- use generics in structs unless runtime polymorphism is truly needed.
- Box at the boundary, not internally.
- Object safety requirements: no generic methods, no `Self: Sized`, methods use `&self`/`&mut self`/`self`.

```rust
// Object safe
trait Runnable { fn run(&self); }

// NOT object safe
trait Factory { fn create<T>() -> T; }
```

## Type State Pattern

Encodes states as types so illegal states are compile errors, not runtime bugs. Uses `PhantomData` for zero-cost state markers.

### Simple Example: File State

```rust
struct FileNotOpened;
struct FileOpened;

struct File<State> {
    path: PathBuf,
    handle: Option<std::fs::File>,
    _state: PhantomData<State>,
}

impl File<FileNotOpened> {
    fn open(path: &Path) -> io::Result<File<FileOpened>> {
        let file = std::fs::File::open(path)?;
        Ok(File { path: path.to_path_buf(), handle: Some(file), _state: PhantomData })
    }
}

impl File<FileOpened> {
    fn read(&mut self) -> io::Result<String> { /* only callable when opened */ }
}
```

### When to use type state
- Compile-time state safety is needed.
- Enforcing API constraints (e.g., must connect before sending).
- Replacing runtime booleans/enums with type-safe code paths.

### When to avoid
- Trivial states that enums handle fine.
- When it leads to overly complex generics.
- When runtime flexibility is required.

## Builder Pattern with Guaranteed Fields

Type state ensures required fields must be set before `.build()` compiles:

```rust
struct MissingName;
struct NameSet;
struct MissingAge;
struct AgeSet;

struct Builder<NameState, AgeState> {
    name: Option<String>,
    age: u8,
    email: Option<String>,
    _name: PhantomData<NameState>,
    _age: PhantomData<AgeState>,
}

impl Builder<MissingName, MissingAge> {
    fn new() -> Self { /* ... */ }
    fn name(self, name: String) -> Builder<NameSet, MissingAge> { /* ... */ }
    fn age(self, age: u8) -> Builder<MissingName, AgeSet> { /* ... */ }
}

impl Builder<NameSet, AgeSet> {
    fn build(self) -> Person { /* safe: both fields guaranteed */ }
}
```

```rust
// Compiles
let p = Builder::new().name("Alice".into()).age(30).build();

// Does NOT compile -- age required
let p = Builder::new().name("Alice".into()).build();
```

## Thread Safety: Send and Sync

Rust tracks thread safety through two auto-traits:
- `Send`: data can move across threads.
- `Sync`: data can be referenced from multiple threads.

A pointer is thread-safe only if the data behind it satisfies these traits.

## Pointer Types Overview

| Type         | Description                                      | Send+Sync? | Use Case                          |
|--------------|--------------------------------------------------|------------|-----------------------------------|
| `&T`         | Shared immutable reference                       | Yes        | Read-only shared access           |
| `&mut T`     | Exclusive mutable reference                      | Not Send   | Exclusive mutation                |
| `Box<T>`     | Heap-allocated owning pointer                    | If T is    | Heap allocation, recursive types  |
| `Rc<T>`      | Single-threaded reference counting               | Neither    | Multiple owners, single thread    |
| `Arc<T>`     | Atomic reference counting                        | Yes        | Multiple owners, multi-thread     |
| `Mutex<T>`   | Thread-safe exclusive interior mutability         | Yes        | Shared mutable state, threaded    |
| `RwLock<T>`  | Thread-safe read-many/write-one interior mutability | Yes     | Read-heavy shared state, threaded |
| `Cell<T>`    | Interior mutability for Copy types               | Not Sync   | Shared mutable, single thread     |
| `RefCell<T>` | Runtime borrow-checked interior mutability        | Not Sync   | Shared mutable, single thread     |

## When to Use Each Pointer Type

**`&T` / `&mut T`**: Default choice. Use borrows whenever ownership transfer is not needed.

**`Box<T>`**: Single owner on the heap. Required for recursive types. Good for large structs to avoid stack overflow.

**`Rc<T>`**: Multiple owners in single-threaded code (e.g., tree structures, linked lists).

**`Arc<T>`**: Multiple owners across threads. The go-to for sharing data in async/concurrent code. Often combined with `Mutex` or `RwLock`: `Arc<Mutex<T>>`.

**`Mutex<T>`**: When you need exclusive mutable access from multiple threads. Wrap in `Arc` for sharing.

**`RwLock<T>`**: When reads vastly outnumber writes. Multiple readers OR one writer at a time.

**`Cell<T>` / `RefCell<T>`**: Interior mutability on single-threaded code. `Cell` for `Copy` types (no runtime check), `RefCell` for others (runtime borrow check, can panic).

General rule: start with borrows, reach for `Arc`/`Box` only when ownership semantics demand it.
