---
name: design-landing-page
description: Stack-agnostic landing page design and CRO guidance. Use when Codex needs to create, redesign, audit, improve, rewrite, or optimize a landing page, marketing page, product page, SaaS landing page, waitlist page, pricing landing page, launch page, or conversion-focused page for design concept, conversion strategy, copy structure, visual direction, messaging, CTA, social proof, trust, objection handling, CRO review, or landing page best practices without forcing any frontend stack.
---

# Landing Page Design Concepts

## Overview

Use this skill to make landing pages clear, persuasive, memorable, and credible without prescribing a technical stack. Start from audience, offer, proof, and conversion intent; then adapt recommendations to the current project, brand system, and implementation constraints.

The goal is not to force every landing page into the same template. The goal is to choose one strong message spine, one intentional visual direction, and a page flow that removes doubt while making the next action obvious.

## Workflow

1. Ground the page before proposing design.
   - Identify the audience, traffic source, offer, primary conversion goal, buyer stage, proof assets, brand constraints, and business model.
   - If a repo or existing page exists, inspect it first. Preserve the stack and design system unless the user asks for a new direction.
   - Ask only for missing product, audience, offer, proof, or brand details that materially change the recommendation.

2. Choose one message spine.
   - Use `references/conversion-frameworks.md` when copy structure, persuasion flow, or objection handling matters.
   - Pick one primary framework and, at most, one supporting pattern. Do not stack frameworks mechanically.
   - Match the framework to the audience's decision state: problem-aware, solution-aware, skeptical, high-intent, price-sensitive, risk-averse, or identity-driven.

3. Choose one visual direction.
   - Define the landing page's visual role: quiet trust, product clarity, editorial authority, premium confidence, operational utility, playful energy, or category disruption.
   - Commit to a coherent direction across typography, color, imagery, spacing, motion, and section rhythm.
   - Avoid generic template aesthetics: vague gradient backgrounds, stock business photos, interchangeable three-card grids, fake logos, filler testimonials, and decorative visuals that do not explain the offer.

4. Design the page flow.
   - Use `references/landing-page-anatomy.md` to decide what sections belong on this page.
   - Always account for value proposition, CTA, proof, friction, and trust.
   - Include other elements only when they serve the offer and traffic intent. A short waitlist page, a paid-search demo page, and a high-ticket SaaS page should not have the same section count.

5. Review conversion risks.
   - Use `references/cro-review-checklist.md` for audits, redesigns, or optimization requests.
   - Look for unclear goals, weak proof, unresolved fears, too many choices, hidden pricing or terms, weak CTA hierarchy, slow pages, mobile reading issues, and mismatched traffic intent.
   - Treat CRO advice as hypotheses to test when possible. Do not claim guaranteed conversion lift.

6. Adapt implementation guidance to the existing project.
   - Use the current stack, routing, design system, asset pipeline, accessibility patterns, and performance constraints.
   - If implementation is requested, give stack-specific steps only after the concept and flow are clear.
   - Do not introduce a new framework, component library, package manager, animation library, analytics product, or CMS unless the user explicitly asks or the existing project already uses it.

## Design Guardrails

- Make the first viewport explain who the page is for, what is offered, why it matters now, and what action comes next.
- Prefer authentic product visuals, real screenshots, real customer proof, credible numbers, and plain-language benefits over abstract decoration.
- Tie every major visual decision to brand position or conversion intent. If a flourish does not improve comprehension, trust, memorability, or action clarity, remove it.
- Keep one dominant CTA path. Secondary actions are allowed only when they support the same conversion journey, such as "watch demo" before "book call."
- Write benefit-led copy, but keep enough feature specificity that the offer feels concrete.
- Use social proof only when it is truthful and specific. Never invent customers, logos, ratings, awards, case studies, or usage numbers.
- Design for mobile reading first: visible CTA, scannable sections, no cramped buttons, no overlapping text, and no hero composition that hides the next section.
- Maintain accessibility and performance: readable contrast, semantic structure, meaningful alt text, reduced-motion respect, compressed media, and fast above-the-fold load.

## Output Expectations

Match the output to the user's request:

- For a new page concept, return a concept brief with audience, goal, message spine, visual direction, page flow, proof plan, CTA strategy, and validation checklist.
- For an audit, lead with prioritized findings tied to conversion impact and cite the page section or file when available.
- For copy work, state the chosen copy framework and provide revised copy by section.
- For redesign work, describe the visual system, section rhythm, media strategy, trust strategy, and friction removals before implementation details.
- For implementation work, translate the approved concept into the current stack's files and patterns.

## References

- Read `references/conversion-frameworks.md` when choosing a persuasion structure, rewriting copy, or addressing buyer psychology.
- Read `references/landing-page-anatomy.md` when planning sections, auditing page completeness, or deciding what to include or remove.
- Read `references/cro-review-checklist.md` when optimizing, reviewing, prioritizing tests, or diagnosing weak conversion performance.

## Source Basis

This skill paraphrases and synthesizes guidance from the original `landing-page-guide-v2` skill, its 11-elements reference, Reciprocal's PRESTO/AIDA/PAS article, 123Internet's conversion framework article, Invesp's 7-principle Conversion Framework, Matomo's CRO best practices, Leadpages' essential elements guide, and Webflow's high-converting landing page guide. Treat source claims about conversion lifts as directional marketing claims, not guarantees.
