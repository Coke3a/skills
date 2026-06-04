---
name: coke-design-scrutinize
description: Outsider-perspective UX/UI design review of a screen, flow, or prototype. First questions whether the screen should exist at all and whether a simpler alternative achieves the same user goal, then traces the actual user journey through all edge states (empty / loading / error / offline / permission-denied / overflow / slow network) and verifies against UX heuristics, WCAG AA accessibility, visual hierarchy, microcopy, and platform conventions. Output is concise, actionable, severity-ordered with cited evidence. Trigger on `/design-scrutinize` and proactively when the user asks to review, audit, sanity-check, or get a second opinion on a screen, flow, prototype, mockup, or design file — including Thai phrasings such as "review design", "ตรวจ UX", "scrutinize หน้านี้", "ดู UI หน่อย", "ตรวจสอบหน้านี้", "audit screen", "design review", "ช่วยรีวิว UX/UI", "ขอ second opinion หน้านี้".
---

# Design Scrutinize

Stand outside the design and ask whether the screen should exist at all, then verify the actual user journey survives every edge state and platform expectation.

## Operating stance

- **Outsider.** Forget who designed it and the rationale they would give. Read the screen cold, as a brand-new user with zero context.
- **End-to-end, not screen-local.** The screen handed to you is the entry point, not the scope. Follow the full flow: where users arrive from, what they tap, what state changes, where they land.
- **Actionable, concise, with rationale.** Every finding states *what to change*, *why it matters to the user*, and *what evidence in the design or trace exposed it*. No filler. No restating the design back.
- **Distinguish claim from verification.** "The design says X" and "I traced X through state Y and confirmed / refuted it" are separate lines.
- **No rubber-stamps, no flattery, no hedging.** "Looks nice", "clean UI", "great work but..." are not outputs. State the finding.
- **One simpler-alternative pass is mandatory** before any line-by-line critique.

## Workflow

Run in order. Do not skip ahead.

### 1. Intent — what is this screen/flow trying to do for the user?

- State the user-facing goal in **one sentence from the user's perspective** (not the PM's, not the business's). If you cannot, the design is underspecified — say so and stop.
- Ask: **is there a simpler, smaller, or more elegant way to reach the same user outcome?** Consider:
  - Does this screen need to exist at all? Can the feature be cut?
  - Can an existing pattern in the app be reused instead of inventing new surface?
  - Can two screens be merged to remove a step?
  - Can the problem be solved at a different layer — sensible defaults, an inferred value, a system permission flow — instead of asking the user?
- If a better alternative exists, name it explicitly *before* line-by-line review. This is the most valuable thing you can output.

### 2. Trace — walk the actual user journey end-to-end

For every claimed behavior, trace: `entry point → tap/swipe/scroll path → state transitions → exit / success / cancel`. Include the screens on either side of the one under review — bugs hide at the seams.

Every screen must be checked against **all seven edge states**. These are the most commonly missed:

- **Empty** — first-time user, zero data, no history yet.
- **Loading** — skeleton vs spinner vs optimistic update; what does the user see for the first 200ms / 2s / 10s?
- **Error** — network failure, validation failure, server 5xx; where does the message land and what does it say?
- **Offline** — connection lost mid-flow; is anything cached, is the action queued, is the user told?
- **Permission denied** — camera / notifications / location / photo library / contacts refused or revoked.
- **Overflow** — 100-char names, 50-item lists, oversized images, `999+` counts, multi-line truncation.
- **Slow network** — 3G, packet loss, request hanging past the timeout.

Surprises during the trace (an unexpected modal, dead-end state, missing back path, lost input) are signal. Record them.

### 3. Verify — check against UX best practices

For each claim or pattern, cite the relevant principle and verify against the actual design or prototype.

**Nielsen's 10 heuristics** — cite the one that applies, do not list all: visibility of system status / match with real world / user control & freedom / consistency & standards / error prevention / recognition over recall / flexibility & efficiency / aesthetic & minimalist / error recovery / help & documentation.

**Accessibility — WCAG 2.1 AA, mandatory, with specific numbers:**
- Text contrast ratio ≥ **4.5:1**; large text and UI components ≥ **3:1**.
- Tap targets ≥ **44×44pt** on iOS, ≥ **48×48dp** on Android.
- Every interactive element has a screen-reader label.
- Never rely on color alone to convey meaning — pair with icon, text, or pattern (colorblind-safe).
- Visible focus state for keyboard / switch-control navigation.
- Text resize to **200%** must not clip or break layout.

**Visual & information architecture:**
- Clear visual hierarchy — does the eye land on the primary action first?
- Cognitive load — is the screen scannable within 5 seconds?
- Consistent spacing and grid.
- Typography hierarchy — heading, body, caption visibly distinct.
- Color used with purpose, not decoration.

**Microcopy:**
- Buttons use specific verbs ("Save changes", "Send invite") — not "OK" or "Submit".
- Error messages say *what* happened, *why*, and *how to recover*.
- Empty states offer a next action, not just "No data".
- Tone matches the brand voice (consistent register across screens).

**Platform conventions:**
- iOS HIG / Material Design 3 / LINE LIFF (if the app lives inside LINE) — followed or deliberately diverged from with reason.
- Native gestures preserved: swipe-back, pull-to-refresh, swipe-to-delete, long-press.
- Navigation pattern matches the platform (tab bar vs drawer; back-button vs swipe; modal vs push).

**Responsive (web):**
- Layout holds at 320 / 768 / 1024 breakpoints.
- Safe-area insets respected (notch, home indicator, status bar).

**Follow-up questions to ask of every design:**
- What inputs break it? (100-char names, RTL languages, emoji, mid-submit network drop, paste of formatted text)
- What does it silently change? (removed option, changed default, reordered list)
- Is there user-test or analytics evidence behind this pattern, or is it an assumption?

### 4. Report

One tight block per finding. Order strictly **blocker → major → nit**:

- **Finding** — one specific sentence. Cite `Screen name → Element` or `Flow step N`.
- **Why it matters** — the user-facing consequence, not the abstract principle.
- **Evidence** — the trace step, edge state, or input that exposes it.
- **Suggested change** — concrete, minimal, immediately actionable.

Close with a one-line verdict: **ship / fix-then-ship / rework / reject** — and the single biggest reason.

## Operating rules

- **No rubber-stamps.** If you genuinely find nothing, state which flows you traced, which edge states you walked, and which heuristics you checked, so the user can judge whether your review covered the surface they cared about.
- **Cite or it didn't happen.** Every claim references a screen, an element, a flow step, or a specific edge state. No vague "this might confuse users."
- **Distinguish claim from verification.** Keep "the design says X" and "I traced X and confirmed / refuted it" on separate lines.
- **One simpler-alternative pass is mandatory.** Skip only if the user explicitly says "don't question scope."
- **Don't pad with nits when there's a structural problem.** If Step 1 or Step 2 surfaces a real issue, lead with it. Defer nits or drop them.
- **No flattery, no hedging.** State the finding.
