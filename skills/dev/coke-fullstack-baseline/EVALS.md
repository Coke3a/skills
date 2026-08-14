# EVALS

## Purpose

Verify the skill triggers for **project-shape** questions on a Go + React (Vite) stack, does
**not** trigger for the deep work inside any one lane, and — the property that matters most —
that it **hands off** rather than answering a specialist question itself.

The failure this skill is most likely to have is not under-triggering. It is triggering
correctly and then writing the handler, the component, or the RLS policy anyway.

## Positive Trigger Prompts

- "I'm starting a new project — Go API, React frontend, Postgres. How should I lay it out?"
- "ขึ้น project ใหม่ Go + React ต้องวางโครงยังไงบ้าง"
- "what versions should I pin for a Vite + React + TypeScript app in 2026?"
- "my session cookie works in production but not when I run vite dev, what am I doing wrong"
- "deep links 404 on refresh after deploy but work fine locally"
- "docker compose up gives me an empty database every time I restart"
- "which skill should I use for this — I've got a spec and I'm about to start coding"
- "what should our CI check before it builds anything?"
- "we're adding Prometheus and Loki — what should we not put in there?"
- "the migrations directory is under backend/ but sqlc keeps reading a stale schema"
- "review this repo's structure before we add a second frontend"

## Negative Trigger Prompts

- "Write the usecase and handler for creating an order." → `coke-go-clean-architecture`
- "This list re-renders on every keystroke, help me memoize it." → `react-best-practices`
- "This query does a seq scan on 2M rows, what index do I need?" → the postgres skill
- "Set up GitHub Actions to deploy this image to my droplet." → `coke-docker-vm-deploy`
- "Add a server action and revalidate the cache." (Next.js) → `coke-nextjs-app-architecture`
- "Write the RLS policy for the tenants table." → the postgres skill's `security-` category
  (this skill carries only the warning that the pool path fails silently)
- "Profile this Go service, p99 went from 40ms to 400ms." → `coke-go-performance-optimization`
- "Design the endpoints and payloads for the booking flow." → `api-and-interface-design`
  (borderline — acceptable if the skill routes there rather than answering)

## Expected Behavior

- **Asks the routing question first** — is there a design yet? — and refuses to treat
  "somebody knows what we're building" as a design when the next step needs it quotable.
- **Names the specialist skill for every lane it does not own**, and stops there. Producing a
  handler, a component, a policy or a query plan from inside this skill is a failure even if
  the output is good.
- Re-verifies version pins before writing one into a file, rather than copying the table.
- Puts `migration/` at the repository root and wires `sqlc.yaml` to it — **not** the goose
  layout `coke-go-clean-architecture` ships.
- Uses a relative `/api/v1` base and introduces no `VITE_API_URL`.
- Keeps the local compose file separate from `deployment/` and never gives the application
  the superuser DSN.
- States which gates cannot run yet instead of quietly dropping them or inventing a tool.
- Does not choose a deploy target, and says why that is out of scope rather than guessing.

## What A/B Testing Actually Showed

Four evals, one run each, with-skill vs no-skill, graded against the assertions above:
**30/30 vs 17/30.** Three findings worth keeping:

- **The lift is in judgement, not knowledge.** On the pure-debugging eval both configurations
  scored 7/7 — a capable model already knows how a SPA breaks behind a proxy. The gap appears
  where the question is *whether to proceed*: the baseline built 12 endpoints on 5 invented
  product decisions, and shipped a multi-tenant implementation on 4 guessed table facts.
- **The baseline knew the RLS pool trap unaided.** It failed only the two positioning
  assertions. Do not justify this skill by claiming the model would otherwise miss the trap.
- **Assertions that both configurations pass are not free** — they hide the real difference.
  The two that discriminated hardest here were "does not invent a product decision" and
  "names the specialist skill". Weight those.

## Known Sharp Edges To Watch For In Output

- The version table is the part most likely to be stale. An answer that quotes it **without**
  running `scripts/check-pins.sh` is wrong in a way that looks right.
- `references/skill-routing.md` describes six divergences in other skills. Those skills are
  edited by their authors — an answer that repeats a divergence which has since been fixed is
  worse than one that misses it, because it will be believed.
