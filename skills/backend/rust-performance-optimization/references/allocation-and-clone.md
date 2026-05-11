# Allocation and Clone

Optimize allocations only when they are in a hot path or profiling shows allocation cost.

Check for:

- Unnecessary `clone()`.
- Unnecessary `to_string()` / `to_owned()`.
- Unnecessary `format!()`.
- Unnecessary `collect()`.
- Repeated allocation in loops.
- `Vec` / `String` created without capacity when size is known.
- Row -> domain -> DTO mapping that clones large values unnecessarily.
- Cloning large domain entities when borrowing or moving would work.
- `Arc`, `Rc`, or `Box` without clear need.
- Repeated serialization/deserialization.

Options:

- Borrow with `&str` / `&[T]` where lifetimes stay simple.
- Move values instead of cloning when ownership allows.
- Use `String::with_capacity` / `Vec::with_capacity` when size is known.
- Use `Cow` when mixed borrowed/owned data is useful and justified.
- Use `Bytes` for cheap shallow clone of byte buffers.
- Use `Arc` for shared immutable large values when appropriate.
- Use `clone_from` when reusing allocation is useful.

Rules:

- Do not make APIs lifetime-heavy for tiny unmeasured gains.
- Do not replace clear owned DTOs with borrowed DTOs unless measurement proves benefit.
- Document before/after measurement.
