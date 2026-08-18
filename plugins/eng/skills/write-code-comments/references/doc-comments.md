# Doc comments

A doc comment sits on a declaration — a function, type, module, endpoint, table, or exported constant —
and is read by people who will never open the body. That changes the calculation. An inline comment
competes with code the reader can see; a doc comment is often the *only* thing they will see.

So "don't restate the code" still holds, but it applies to the signature, not the implementation. The
signature already states names, types, arity, and visibility. Repeating those is the same waste as
`i += 1  // add one to i`. Everything the signature *cannot* express is what the doc comment is for.

## What the signature cannot express

Work down this list. Most declarations need two or three of these, not all of them.

| It carries | Why the signature cannot | Skip it when |
| --- | --- | --- |
| **What it is for** | A name compresses; one line of intent decompresses it | The name genuinely says it all |
| **Failure modes** | An error type names a category, not the conditions that produce it | It cannot fail |
| **Units, ranges, formats** | `int` does not say milliseconds, cents, or 1-based | The type is already precise (a `Duration`, a `Money`) |
| **Invariants and preconditions** | Types rarely encode "must be sorted" or "call `connect` first" | Nothing is assumed |
| **Ownership and lifetime** | Who closes, frees, or must not retain this | Nothing is owned |
| **Concurrency** | Safe to call from several tasks? Reentrant? Holds a lock? | Single-threaded by construction |
| **Side effects** | Writes, network calls, mutations of arguments, global state | It is pure |
| **Cost** | Surprising complexity, an N+1, a blocking call in an async context | Cost is unremarkable |
| **An example** | Only when correct usage is not obvious from the signature | Usage is obvious |

## The bar differs by audience

- **Public API — other teams, other repositories, published packages.** Nothing here can be discovered by reading the implementation, because the reader will not have it. Failure modes, units, and invariants are mandatory when they exist. This is the one place where thoroughness beats brevity.
- **Internal but cross-module.** Document the contract; skip the tutorial. Callers can read the body if they need to, but should not have to in order to call it correctly.
- **Private helpers.** A good name usually suffices. Add a doc comment when the helper carries a non-obvious invariant, not as a matter of policy.

Blanket rules like "every exported symbol needs a doc comment" reliably produce a file of restated
signatures. Coverage is not the goal; a caller who cannot misuse the thing is.

## Follow the file's existing convention

Every ecosystem has its own doc comment marker, its own tooling that renders them, and its own habits
about ordering, section headings, and whether the first sentence must be a particular shape. Some
compile the comments; some test the examples inside them.

Read what the file and its neighbours already do, and match it — marker, structure, section names, and
the human language they are written in. A doc comment that breaks the local convention may silently
drop out of generated documentation, and an inconsistent file is harder to read than a plainly
formatted one.

When a project's tooling checks doc comments (a linter, a doc build, a doctest runner), that check is
the authority on form. Satisfy it, then spend your effort on the content above.

## Smells specific to doc comments

- A parameter list that gives each parameter its own name back.
- "Returns the result" on a function whose return type already says what it returns.
- A description of the implementation rather than the contract — it goes stale on the first refactor.
- A promise nothing enforces: thread-safe, idempotent, never returns null.
- An example that no longer compiles or runs.
- Documentation on a private helper that only restates its name, added to satisfy a coverage rule.
