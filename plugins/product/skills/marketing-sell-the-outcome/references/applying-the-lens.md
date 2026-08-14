# Applying the lens

> Per-context guidance, diagnostic questions, and anti-patterns. The questions exist to *provoke thinking* — they are not a checklist to score, and there is no passing grade. Use judgment, and always explain the user-facing reason behind a finding.

## Contents

- The five contexts, in practice
- Diagnostic questions to think with
- Anti-patterns: selling the product, not the outcome
- Honest-aspirational vs misleading
- Boundaries: this skill vs its siblings

## The five contexts, in practice

### 1. Planning / positioning
**The lens asks:** What one outcome should this own for the user? Against what alternative (including "do nothing")? Who cares about that outcome most?
**Example:** Positioning a status-page feature → not "a page that shows check states," but "prove your reliability to *your* clients." That outcome then shapes everything — make it shareable, branded, and tell an uptime story.
**Good output:** a crisp value theme + the alternative it beats + the segment that cares most — not a feature list.

### 2. A new feature idea
**The lens asks:** Will the user perceive a result, and is it the result they want? What makes the value visible at first use? Could the outcome be delivered more directly?
**Example (Checkilo weekly digest):** Raw idea = "email a summary of pings." Through the lens, the owner's outcome is *reassurance without effort* → lead the digest with "All 12 automations healthy this week ✓ you didn't lift a finger," surface only the exceptions, and make the felt result (peace of mind, time saved) the content. The pings are evidence, not the message.
**Good output:** the user outcome, where the current idea hides it, and one or two concrete ways to make it visible.

### 3. Existing content / page / message
**The lens asks:** Does the first thing the user sees state *their* outcome? Where does it talk about the product instead of the user? What's the highest-leverage reframe?
**Example:** Hero = "The complete automation monitoring platform" → sells the category, not the result. Reframe to the user's outcome: "Know the second a Zap silently fails."
**Good output:** the biggest reframe first, then a few supporting ones, each with the *why* and a suggested angle.

### 4. PR / spec review (PO / marketing angle)
**The lens asks:** Separate from correctness — does this change make the user's outcome clearer or more visible, or does it just add capability? What's the user-facing story? Is anything claimed that isn't shipped?
**Example:** A spec adds "configurable retry thresholds." Lens read: the *user outcome* is "fewer false alarms I can trust" → does the spec surface that (sensible defaults, plain-language copy), or just expose a knob? And make sure release notes / landing don't describe it as live before it ships.
**Good output:** the user-value read on the change, plus any honesty flags — explicitly *not* an engineering review.

### 5. Companion to a creation skill
**The lens asks:** Given what the other skill produced, where is it selling the tool instead of the result — and how do we flip it?
**How to behave:** contribute only the outcome read; don't redo the other skill's structure, visual, or code work. One tight pass, highest-leverage first.

## Diagnostic questions to think with

Grouped under the four core questions. They prompt thinking; don't tally them into a score.

**Outcome clarity**
- Can you state the user's desired outcome in one sentence, in their words?
- Does the first thing they encounter point at that outcome, or at the product?
- Is there a single dominant outcome, or several competing for attention?

**Customer-centricity**
- Is the user the subject, or is the product / company the subject?
- Does it lead with the user's problem / desire, or with the build?

**Feature → outcome**
- Is each feature laddered to a result the user cares about, or left as a spec to self-translate?
- Does the benefit reach the emotional / identity level, or stop at "it does X"?

**Believability & honesty**
- Is the outcome specific enough to picture *and* to believe?
- Could a competitor say the exact same line? (If yes, it isn't positioned yet.)
- Is every present-tense claim true today? Is anything aspirational clearly labeled?

**Calibration**
- What's the audience's awareness stage — and is the outcome / feature balance right for it?

## Anti-patterns: selling the product, not the outcome

Each is smell → example → reframe. Keep the awareness caveat in mind: feature detail on a product-aware page is *not* an anti-pattern.

1. **Feature-dumping with no benefit** — "5GB storage, OAuth, REST API." → ladder each to a result, or cut it.
2. **"We / our"-centric brag** — "We're a leading provider of innovative solutions." → make the user the subject: "You ship faster."
3. **Vague abstraction / buzzwords** — "world-class, seamless, next-gen, robust." → replace with a number or a concrete after-state.
4. **Jargon / curse of knowledge** — write for the non-expert buyer; spell out the "so what."
5. **Claims with no proof** — a big promise with no evidence. → attach data / testimonial / demo, or make it specific.
6. **Overpromise / vaporware** — describing the unshipped as live. → label "coming soon"; sell what's real today.
7. **Feature dressed as the outcome** — "real-time dashboards" sold as *the* benefit. → the dashboard is the mechanism; the outcome is "know everything's healthy without checking."
8. **Generic, unbelievable outcomes** — "grow your business," "save time & money." → make it specific and scoped to the user's situation.
9. **Outcome buried under the mechanism** — leading with how it works before why it matters. → outcome first, mechanism as support.

## Honest-aspirational vs misleading

A future / aspirational claim is **acceptable** when all hold:
- It's framed as vision or roadmap ("coming soon," "beta," "our mission"), not present-tense fact.
- The user gets real value *today* regardless of the unshipped part.
- The purchase decision doesn't hinge on the unshipped thing.

It is **misleading** when an unbuilt or partial capability is stated as a live, current feature, or a result is implied as typical without basis. When in doubt, label it and move the aspiration into vision language.

## Boundaries: this skill vs its siblings

| The question being asked | Skill |
|---|---|
| Does this make the user see / feel the outcome (vs. the product)? | **coke-product:marketing-sell-the-outcome** (this one) |
| Is the UX / flow / accessibility sound? | `coke-product:design-scrutinize` |
| How should the whole landing page be designed (structure, visual, CRO)? | `coke-product:design-landing-page` |
| Is the code correct / well-architected? | `coke-eng:rust-code-review` |
| Is the spec accurate vs. reality? | `coke-eng:flow-spec-review` |

The two `coke-eng:` rows ship in a sibling plugin — hand off only if it is installed.

Compose freely: `coke-product:design-landing-page` drafts the page, then this lens passes over the message; or `coke-product:design-scrutinize` checks the UX while this checks whether the value lands. Stay in your lane — contribute only the outcome read.

## Sources

Julian Shapiro, startup landing pages (julian.com/guide/startup/landing-pages) · CXL high-converting landing pages (cxl.com/blog/how-to-build-a-high-converting-landing-page) · Harry Dry / Marketing Examples — visualize / falsify / unique (upgrow.io/blog/harry-dry-copywriting-3-rules) · StoryBrand "your brand is not the hero" (storybrand.com) · Schwartz five stages of awareness (indiehackers.com/post/the-5-stages-of-awareness) · selling what's on the truck today / roadmap honesty (clickinsights.asia). Treat conversion-lift claims as directional, not guarantees.
