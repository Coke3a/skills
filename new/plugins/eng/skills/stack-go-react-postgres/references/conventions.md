# House conventions

## API

- **Every response uses one envelope**, success and error alike, and one error-code table.
  Both are implemented in **one middleware** — not per handler, where they drift.
- **Pagination has a ceiling.** A page size above the ceiling is **rejected with `400`**,
  never silently truncated — a truncated page is a client that believes it has seen
  everything.
- **`X-Request-Id` on every response**, and shown on the error screen so a user can quote it.
  It is the join key for the observability stores.
- **Idempotency**: writes that can be retried carry `Idempotency-Key`, and the record is
  **durable** — stored transactionally with the effect it guards, not in an evicting cache.
- **The API path carries no tenant, customer or account segment.** Identity comes from the
  session. A URL segment is a thing a user can edit.

⚠ **Where `api-and-interface-design` differs, this section wins on shape and the skill wins
on reasoning.** The skill's worked examples return a bare resource on success,
`{data, pagination}` on a list, and `{error: {code, message, details}}` on failure — **three
shapes, where the first rule above says one.** What to take from it instead is the part this
section does not argue: contract-first, Hyrum's Law (every observable behaviour becomes a
commitment), validation at the boundary and nowhere inside, and additive-only change. Two
smaller divergences to settle once, in the project's own contract document: it names the
page-size parameter differently and puts no ceiling on it, and it offers **both `400` and
`422`** for invalid input — pick one and put it in the error-code table.

## Logging

Structured JSON to `stdout`/`stderr`. Nothing writes a log file.

```json
{ "level": "error", "service": "backend", "env": "production",
  "request_id": "01J…", "trace_id": "01J…", "message": "database connection failed" }
```

⚠ **Four rules, and the first is the one that gets skipped:**

| | |
|---|---|
| **Strip the query string before the line is written** | not in a collector pipeline. A pipeline is a filter somebody can reorder or misconfigure; **a line that never contained the value cannot leak it** |
| **Never log a request body** on an auth endpoint or any write carrying personal data | |
| **Never log a decrypted value or a hash/index input** | the input to a lookup index *is* the identifier |
| **Never log a token, link or credential** | an emailed link is a credential — it appears in the message and nowhere else |

**The logger is the enforcement point, not a code-review convention.** Give it a typed field
API so a raw `string` cannot reach a log line, and put an architecture-fitness test on it. A
rule a reviewer has to notice is a rule that fails on a Friday.

## ⚠ Row-level security fails silently — settle it before you rely on it

**This is a warning, not a design.** How an actor reaches the database is a project decision
and belongs in the design phase's schema lane. What belongs here is the trap, because it is
the same on every Go + pgx + sqlc project and it is invisible until production.

`set_config(…, true)` and `SET LOCAL` last exactly **one transaction**. sqlc builds its
`Queries` from a `DBTX`, `*pgxpool.Pool` satisfies that interface, and a query issued on a
pool runs in **its own** implicit transaction on an arbitrary connection. ⇒ **A query on the
pool cannot see an actor set beforehand**, and the usual policy shape then returns **zero
rows with no error** — a `200` and an empty list, from a system whose authorization is doing
nothing whatsoever.

⚠ **Nothing catches that on its own.** The TDD loop uses fake repositories, the local
superuser bypasses RLS entirely, and a `test-db` test that sets the actor itself proves the
*policy* rather than the *code path*. The two database gates in `gates.md` are worded against
exactly that — read them before the first policy is written.

⚠ **Four more ways a correct policy set is still inert.** Each is a *fact about PostgreSQL*,
not a design; how the project answers them belongs to the schema lane.

| | |
|---|---|
| **Owning a table bypasses its policies** | "never connect as superuser" is only half the rule. If the application's role *owns* the tables, its policies do nothing — unless the table is set to `FORCE ROW LEVEL SECURITY`. Whoever runs migrations owns what they create, which is why that is not the application's role |
| **A view runs as its definer** | so a view over a protected table serves rows the caller could not select directly, unless it is defined with `security_invoker`. A materialised view cannot enforce policies at all |
| ⚠ **An external transaction-mode pooler can break the mechanism outright** | `SET LOCAL` needs the query to be in the same transaction on the same connection. A pooler in *statement* mode cannot hold one. This is the baseline's own lane, not the specialist's — see the connection budget |
| **A policy is not the whole grant story** | `TRUNCATE` is governed by privileges rather than policies, and uniqueness and foreign-key checks run without row security — so a `23505` on someone else's row is an existence oracle. Decide the grant list alongside the policies |

⚠ **And one API consequence, because it lands in the error-code table:** under RLS, *"not
yours"* and *"does not exist"* are indistinguishable to the query. Pick one response for both
and keep them identical — a `403` where the rest of the system returns `404` tells the caller
the row exists.
