# Karpathy filter for reviewer findings

Most reviewer findings have to be filtered through a Karpathy-YAGNI lens before deciding to act on them. Without this filter, you'll either auto-merge every finding (scope creep, broken surgical discipline) or ignore real bugs.

## Core rule

**Every changed line should trace to the spec.** Findings that ask you to add lines without spec backing → reject. Findings that catch real spec gaps or correctness bugs → fix.

## Decision matrix (full)

| Finding | Action | Rationale |
|---|---|---|
| Code in this batch produces wrong output for an input the spec covers | **FIX** | This is a correctness bug. Always. |
| Code in this batch breaks an unrelated test in `tests/` | **FIX** | You broke something. |
| Spec field X exists in code but with wrong type / wrong serialization | **FIX** | Spec compliance failure. |
| Test asserts on a deleted field | **FIX** | Test was on the deletion list; finish the cleanup. |
| Pre-existing pattern in code that's not introduced by this batch (e.g., `pub use` in `mod.rs` that was already there) | **REJECT** | Out of scope; clean up under a different PR if it matters. |
| Compile error in a file the implementation checklist explicitly says a LATER batch will fix | **REJECT** | Transitional state; documented in the plan. |
| "While we're here, refactor X" — X is not in spec | **REJECT** | Surgical scope. The PR is for the spec, not opportunistic improvements. |
| "Add defensive validation" for an input the only caller already guards | **REJECT** | YAGNI; defense-in-depth for single callers is speculative. |
| Spec table or example contains a number/word typo | **REPORT to user**, don't auto-fix | The spec is the user's artifact; flag for them to update separately. Code stays correct. |
| Suggested rename for clarity | **REJECT unless** the current name is actively misleading | Code review (not this batch) catches naming. |
| "Use `match` instead of `if let`" or other stylistic preferences | **REJECT** | Final code review catches style. |
| `i32` narrowing cast where the only caller passes a tiny value | **REJECT** | YAGNI; if the trait accepts u32 and the caller passes 30, no real risk. |
| Inline comment requesting the doc-comment elaborate on a load-bearing invariant | **MAYBE fix** if one line | Borderline; do if cheap. |
| "Use Promise.all" or "use try_join!" for two independent awaits | **FIX if introduced by this batch; REJECT if pre-existing** | New code should be parallel; don't bundle pre-existing serial code with the spec's scope. |
| Mock impl semantics diverge from real DB impl in a way the trait contract forbids | **FIX** | Tests can pass against the mock and fail in prod; this is a real bug. |

## Worked examples

### Example A — REJECT (transitional state)

**Reviewer says:** "BLOCKER: `list_cards.rs` line 61 calls `bucket_counts_in_window` which was removed from the trait. `cargo check` will fail."

**You check:** The implementation checklist says: "B1 removes `bucket_counts_in_window`. B3 rewrites `list_cards.rs`. Between B1 and B3, `list_cards.rs` does not compile — this is expected."

**Action:** Reject. Tell the reviewer this is documented transitional state. Don't fix in B1 — wait for B3.

### Example B — REJECT (pre-existing convention drift)

**Reviewer says:** "BLOCKER: `domain/value_objects/automation/mod.rs` has `pub use` statements which violate CLAUDE.md's `mod.rs is declaration-only` rule."

**You check:** `git diff main -- domain/value_objects/automation/mod.rs` — your change only removed `pub mod list_window;` and `pub use list_window::ListWindow;`. The remaining `pub use AutomationListParams` etc. existed on `main` before your work.

**Action:** Reject as out of scope. The convention drift was pre-existing. Cleaning it up belongs to a separate PR. Karpathy: don't refactor things this PR didn't touch.

### Example C — FIX (real bug introduced this batch)

**Reviewer says:** "MAJOR: The new SQL in `runs_per_day_last_7d` does not exclude `in_progress` runs, but the mock impl does. Postgres will emit a `(0,0)` row for a day with only in_progress runs; mock returns nothing. This is a semantic divergence."

**You check:** Trace through the Postgres SQL — yes, the GROUP BY includes `in_progress` rows even though the FILTER aggregates correctly skip them. The empty bucket row is emitted. The mock skips `in_progress` in the match arm so doesn't emit the row.

**Action:** Fix. The trait docstring claims "in-progress excluded"; the Postgres impl violates that. Add `AND state != 'in_progress'::run_state` to the SQL WHERE clause. The mock and the docstring agree; bring Postgres in line.

### Example D — REPORT TO USER (spec typo)

**Reviewer says:** "BLOCKER: The spec's example table row says '5 success + 2 failed + 3 in_progress → 71% over 7 → Unstable', but with the documented thresholds (≥0.80 → Unstable), 71% < 80% would derive Failing."

**You check:** Walk the math — 5/7 = 71.4%, which is < 0.80, so the code correctly returns Failing per the documented thresholds. The spec's example row in the table mislabeled the outcome.

**Action:** Don't change the code (it's correct). Don't auto-edit the spec (it's the user's artifact). Add a line to the Final Report's "Audit findings reported" section: "Spec table at line N row 1 says 'Unstable' for 71%; thresholds say Failing. Code matches thresholds. User may want to fix the table." Leave for the user.

### Example E — FIX (cascade cleanup)

**Reviewer says:** "NIT: `mock_automation_with_status` in `test_support.rs:201` is unused after `list_with_status.rs` was deleted in this batch. Dead code."

**You check:** `grep -rn "mock_automation_with_status"` — only reference was in the now-deleted `list_with_status.rs` test module. Confirmed orphan.

**Action:** Fix. The deletion you performed orphaned this helper; finish the cleanup. This is not scope creep — it's completing what the deletion implicitly requires.

### Example F — REJECT (defense-in-depth for single caller)

**Reviewer says:** "MAJOR: `per_automation_limit as i32` truncates for values above `i32::MAX`. Use `BigInt` and `i64`, or `debug_assert!`."

**You check:** The only caller passes `30`. The trait accepts `u32` but realistic values are tiny.

**Action:** Reject. Karpathy: no defense-in-depth for a single internal caller passing a known small value. If someone later wires this to user input, the type system already says u32 not String, so we're not exposed to arbitrary values.

## The discipline

When in doubt about a finding, ask:
1. Does this fix a real spec gap or correctness bug? → Fix.
2. Does this expand surface area without spec backing? → Reject.
3. Would `git blame main -- <file>` show this issue existing before our PR? → Reject.
4. Is this style preference or alternative phrasing? → Reject.
5. Does fixing this require touching files outside the batch's scope? → Almost always reject.

The reviewer's job is to surface; yours is to filter. A reviewer that finds nothing in a 500-line diff is suspicious. A controller that accepts every finding is even more suspicious.
