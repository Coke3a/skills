# Comment shapes

A worked example of each of the six things code cannot say. The examples deliberately rotate across
languages — the point is the *shape* of the information, not the syntax. Every one of these transfers
unchanged to any language you are writing in.

## 1. Why this, not the obvious alternative

State the failure the obvious version causes. Without it, the next reader deletes the odd-looking line
as cleanup and reintroduces the bug — which is exactly why the comment exists.

```python
# The parser yields a sentinel that is not None but compares equal to it, so an
# `is not None` check passes and the value still poisons everything downstream.
if value is None or value == None:
    return None
```

The comment does not explain the syntax. It explains why the redundant-looking second test is not
redundant, which is the only thing a reader cannot work out from the line itself.

Do not comment ordinary idioms of the language — this shape is for code that *looks* wrong and is not.
The exception is audience: when the readers are novices, or the file is teaching material, an idiom
that is invisible to a fluent reader is exactly what needs explaining. Who reads it decides.

## 2. Where the code came from

Link it. The link carries three things the pasted code does not: the caveats in the replies, the
context of the original problem, and the attribution that Stack Overflow's Creative Commons licence
requires.

```typescript
/** Debounce that survives React 18 double-invoked effects.
 *  Adapted from <the answer URL you actually opened> — the leading-edge branch is ours. */
export function useDebounced<T>(value: T, ms: number): T {
```

The placeholder is deliberate, and it is the most important thing on this page. Paste the URL you
actually opened — never one reconstructed from memory. Answer IDs are short digit strings, so a
plausible-looking guess resolves to a real page about something else entirely, and the reader has no
way to tell. A link to the wrong question is worse than no link at all: it looks checkable, so it gets
trusted.

Note "the leading-edge branch is ours". Say what you changed, so a reader comparing against the source
is not left wondering which differences are deliberate.

Two failure modes bracket this shape. A bare "taken from a stackoverflow post" with no URL admits the
code is borrowed and gives the reader nothing to follow. A confident wrong URL gives them something
false to follow. And underneath both: never paste in code you do not understand — if you cannot say
what it does, the comment you attach to it is a guess wearing a citation.

## 3. Which external rule forces this

Put the link at the line it governs, not in a header far away. The reader who needs it is looking at
that line.

```go
// RFC 4180 specifies CRLF line endings, not \n; strict parsers reject the file otherwise.
// https://tools.ietf.org/html/rfc4180
buf.WriteString("\r\n")
```

This shape covers specs, RFCs, vendor API documentation, protocol requirements, regulatory rules, and
anything else where an external authority — not your team — decided the behaviour.

## 4. How it broke

The reproduction condition is the payload. Without it nobody can ever decide whether the workaround is
still needed, and it stays forever.

```rust
// Under tokio 1.35 a `select!` on a `broadcast::Receiver` that has already lagged
// returns `Ok` on the next poll instead of `Err(Lagged)`, so we re-check `len()`
// before trusting the value. Reproduces with >1024 queued messages. See #2841.
if rx.len() >= CHANNEL_CAPACITY {
```

Three parts, all load-bearing: the exact conditions (`>1024 queued messages`), the version it was
observed on (`tokio 1.35`), and the issue reference. A future reader can retest it in one step.

`// fix for bug` carries none of that and is indistinguishable from noise.

## 5. What is unfinished

Name an owner and what is missing. An unowned `TODO` is a wish; an owned one is a commitment someone
can act on.

```bash
# TODO(coke): rollback only restores the previous image tag, not the migration.
# A failed deploy after a destructive migration currently needs manual repair. See #312.
docker compose up -d --no-deps app
```

State the consequence, not just the task. "Rollback is incomplete" tells a reader nothing about whether
they can deploy today; "a failed deploy after a destructive migration needs manual repair" does.

## 6. What the domain demands

The number is in the code. The rule that produced the number cannot be.

```sql
-- Finance writes off receivables at 90 days, not 30 — policy changed 2026-Q1 and the
-- reporting pipeline keys off this exact threshold. Changing it silently changes revenue.
WHERE age(invoice.due_date) > interval '90 days'
```

The test for this shape: if the number changed, would something outside the codebase have to change
too? Then the reason is a domain rule and belongs in a comment. If any value would work, it is a
tuning constant and probably needs a named constant rather than prose.

---

## Comment language

These shapes are about information, so they hold in any human language. Match whatever the file already
uses — a reader switching languages at every comment pays for it, and the switch buys nothing.

```typescript
// LINE ตอบ 429 เมื่อยิงเกิน 200 req/นาที ต่อ channel และไม่ส่ง Retry-After กลับมา
// ค่า 300ms มาจากการวัดจริงบน production เมื่อ 2026-07 ไม่ใช่ค่าที่เดา — ดู #1180
await sleep(300);
```

The same three tests apply unchanged. This comment survives Delete (the code cannot state the rate
limit), survives Surprise (300 looks arbitrary and would otherwise be tuned away), and is pure
Archaeology (it was measured, and the measurement exists nowhere else).
