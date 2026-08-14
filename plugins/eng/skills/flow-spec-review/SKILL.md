---
name: flow-spec-review
description: Reviews specification documents before implementation by verifying current system state against spec claims, running tests to confirm the before-state, then walking the user through before/after approval of every change. Hard-stops when testing scope cannot be met or when spec's description of current behavior doesn't match reality. Use when the user wants to review a spec, validate a spec, check a spec document before implementation, audit a task spec, verify spec accuracy, or asks to "review spec", "check this spec", "check the spec document", "review this document", "validate spec document", "spec review", "check before implementing spec". Also triggers when the user provides a spec file and asks whether it makes sense, is accurate, or is ready for implementation.
---

# Spec Review

Validate a specification document against reality before anyone writes a line of implementation code. The goal is to catch misunderstandings, stale assumptions, and unclear before/after descriptions early — when fixing them costs a conversation, not a rollback.

## Why this matters

Specs rot. The system described in a spec may have drifted since the spec was written. If implementation starts from a wrong "before" picture, the "after" will be wrong too — and nobody notices until QA or production. This workflow forces verification: does the spec's description of today's system actually match today's system?

## Operating principles

- **Verify, don't assume.** The spec says "currently the API returns X" — confirm it actually does by testing.
- **Hard-stop on mismatch.** If reality doesn't match what the spec claims is the current state, stop and surface the discrepancy to the user. Do not proceed with a review built on a false premise.
- **Big picture first, details second.** Get alignment on the overall before/after story before nitpicking field names.
- **The user is the final authority.** Every change in the spec requires explicit user approval. Loop until you get it.
- **No implementation.** This skill reviews the spec. It does not implement anything.

---

## Workflow

Run in strict order. Do not skip steps or reorder.

### Phase 1: Comprehension — Understand the spec

Read the spec document the user provides. Extract and internalize:

- **What is being changed?** (feature, API, logic, UI, workflow)
- **Why is it being changed?** (business goal, user pain point, technical debt)
- **What is the business logic involved?**
- **What is the scope?** (which systems, services, screens, endpoints are touched)

Summarize your understanding back to the user in 3-5 sentences. Confirm you've got it right before continuing.

### Phase 2: Validate current state — Test what exists today

The spec claims something about how the system works *right now* (the "before" state). Your job is to verify that claim is accurate by actually testing the current system.

#### 2a. Determine test scope

Ask the user:

1. **What testing is available?** (unit tests, integration tests, E2E, browser automation, API requests)
2. **What is the test environment?** (local dev, staging, production — what URLs, what ports)
3. **If E2E or browser testing is needed:** What are the login credentials for the test environment? (The user can provide username/password directly in the session.)
4. **What is off-limits?** (production writes, destructive operations, rate-limited endpoints)

Wait for the user's answer. Do not guess scope.

#### 2b. Execute verification tests

Dispatch subagent(s) to verify the current state. The subagent's job is to:

- Run existing test suites (unit, integration) to confirm they pass
- For frontend specs: use browser automation (Playwright) to navigate the current UI and document the existing behavior
- For backend API specs: make actual API requests to the test environment and record the responses
- For backend logic specs: run relevant tests or trace the code path

The subagent must report back:

1. **Test results** — what passed, what failed, what couldn't run
2. **Observed current behavior** — what the system actually does today (screenshots, response bodies, test output)
3. **Comparison to spec** — does the observed behavior match what the spec says is the "current" state?

#### 2c. Hard-stop gates

**GATE 1: Testing scope not achievable**

If the required tests cannot be executed within the scope the user defined — STOP IMMEDIATELY.

Tell the user exactly what couldn't be tested and why. Do not proceed. The user must either:
- Expand the test scope, or
- Explicitly approve continuing without that verification

Without explicit approval, the workflow ends here.

**GATE 2: Current state doesn't match spec**

If the actual current behavior differs from what the spec describes as the "before" state — STOP IMMEDIATELY.

Present the discrepancy clearly:
- Spec says: "[quote from spec about current behavior]"
- Reality shows: "[what testing actually revealed]"

Ask the user to clarify. Keep asking until the picture is clear:
- Is the spec outdated?
- Has the system changed since the spec was written?
- Is the spec describing a different environment?

If the user's answers contradict the spec's before/after description — STOP. Tell the user the spec's overview of the current process is inaccurate and should not be used as a basis for implementation. The user must either:
- Update the spec, or
- Explicitly approve continuing with the known inaccuracy

Without explicit approval, the workflow ends here.

### Phase 3: Code comprehension — Big picture to small

Once the "before" state is confirmed accurate, understand the implementation scope:

1. **Architecture level** — which services, modules, layers are involved?
2. **Component level** — which files, classes, functions are touched?
3. **Line level** — what specific code paths change?

Map the spec's proposed changes onto the actual codebase. Identify:
- Entry points affected
- Data flow changes
- Side effects and downstream impacts
- Integration boundaries that shift

### Phase 4: Before/After approval loop — Big picture

Present the spec's changes to the user as a series of before/after comparisons at the **overview level**. Adapt the framing to the type of change:

| Change type | Before/After framing |
|---|---|
| Design/UX | UX flow before → UX flow after (screens, interactions, user journey) |
| Frontend process | Render behavior before → Render behavior after (what changes visually, performance impact, why) |
| Backend API | Request/response shape before → Request/response shape after (endpoints, payloads, status codes) |
| Backend logic | Processing logic before → Processing logic after (business rules, data transformations, side effects) |
| Database | Schema/query before → Schema/query after (migrations, indexes, constraints) |

For each change in the spec:

1. State the before (confirmed by Phase 2 testing)
2. State the after (as described in the spec)
3. Explain why the change achieves the spec's stated goal
4. Ask: **"Do you approve this change?"**

Wait for explicit approval on each change before moving to the next.

### Phase 5: Handle disputes

If the user disputes any before/after description:

1. Acknowledge the disagreement
2. Re-list the spec changes incorporating the user's correction
3. Return to Phase 4 and re-present the updated before/after for approval

Repeat until every change is approved.

### Phase 6: Detail review

Once all big-picture changes are approved, examine small details:

- Edge cases not covered by the spec
- Error handling gaps
- Performance implications
- Security considerations
- Backward compatibility concerns
- Missing validation rules
- Race conditions or concurrency issues
- Incomplete rollback scenarios

For each issue found, present it to the user:
- What the issue is
- Why it matters
- Suggested improvement (if you have one)
- Ask: "Should this be addressed in the spec?"

Loop until the user confirms no more detail changes are needed.

### Phase 7: Final summary

Once everything is approved, produce a final summary:

**Overview**
- One-paragraph description of what the spec accomplishes

**Changes approved**
- Bulleted list of each before/after change the user approved

**Risks**
- What could go wrong during implementation
- What could go wrong in production
- Dependencies or timing sensitivities

**Pros**
- Benefits of implementing this spec as-is

**Cons**
- Drawbacks, tradeoffs, or technical debt introduced

**Things to know**
- Non-obvious implementation details
- Things the implementing engineer should be aware of
- Monitoring or observability needs post-deploy

---

## Subagent dispatch pattern

When dispatching a subagent for Phase 2 testing, provide:

```
You are verifying the current state of a system against a spec document's claims.

Spec claims the current behavior is:
[paste relevant "before" section from spec]

Your job:
1. Run the following tests: [list based on user-defined scope]
2. If browser testing: navigate to [URL], log in with [credentials], and document what you see
3. If API testing: send requests to [endpoints] and record responses
4. Report back: what is the ACTUAL current behavior? Does it match what the spec claims?

Test environment: [user-provided details]
Credentials: [user-provided if applicable]
Off-limits: [user-provided constraints]

Save all evidence (screenshots, response bodies, test output) and summarize findings.
```

The subagent returns its findings. You then compare against the spec in the main session.

---

## Key behaviors

- Never proceed past a hard-stop gate without explicit user approval
- Never implement code — this skill is review-only
- Never guess what the current system does — test it
- Always present changes at the big-picture level first
- Always get per-change approval before moving on
- If in doubt, ask the user — they are the authority on intent
