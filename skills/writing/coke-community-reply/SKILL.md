---
name: coke-community-reply
description: Draft a reply to a community or forum post in a real practitioner's voice, for pull-channel / answer-first participation on the n8n community, Reddit (r/n8n and similar), Make/Zapier forums, IndieHackers, Hacker News, or any Discourse thread. Claude drafts; Coke always posts. Use this whenever Coke wants help writing, drafting, refining, tightening, or "humanizing" a response to someone's forum/community/thread post, answering a community question, or doing answer-first outreach — including when he just pastes a thread or a link and says things like "how should I reply to this", "help me answer this post", "draft a forum reply", "ช่วยตอบโพสต์นี้", "ร่างคำตอบ community", "จะตอบเธรดนี้ยังไง", or "make this sound more human" — even without naming a skill. It enforces the answer-the-exact-question, sound-human, no-promo voice plus a re-check pass. Not for email replies or marketing/landing copy — those are separate writing skills.
---

# Community Post Reply

Draft a reply that Coke will post to a community or forum thread. This is **pull-channel** participation: you win by being genuinely useful, not by pitching. The output is one short reply in a real person's voice, ready for Coke to paste.

**Claude drafts. Coke posts. Always.** Never post, never suggest auto-posting.

## Why the bar is this high

The channels that matter (n8n community, r/n8n, Make/Zapier forums, IndieHackers) ban or punish overt promotion and reward real practitioner help. The wedge Checkilo plays in is already crowded, so credibility is the only edge left. A reply that drifts off the question, reads like marketing copy, speaks for everyone, or name-drops a product reads as self-interested — and burns the exact trust the channel is supposed to build. Every rule below exists to protect that credibility.

## The rules, in priority order

### 0. Answer the EXACT question, then re-check after drafting — this is the most important thing

Read what they actually asked, **and** read what they said is already handled so you don't answer a question they closed. If the post asks "how do *you* do this?", answer only "here's what I do" — do not explain how the system, the tool, or the world works in general.

Then, after the draft is finished, go back through it line by line and ask: *does every sentence actually answer their question?* Cut whatever drifted. The biggest misses hide in a paragraph that is true and well-written but answers a slightly different question. Re-reading catches them; the first pass usually doesn't.

> Real example. The OP wrote "when something breaks there's an alert, someone shouting. Fine. But when it's working fine the client sees nothing." A draft included a paragraph about catching failures before the client notices. That answers breakage — which the OP explicitly said was already handled. Cut it. The question was only about the working-fine state.

### 1. Answer from your own angle. Never speak for other people.

They asked for your experience, not a theory. Avoid universal claims about what "clients" or "everyone" think or feel — you can't know that, and it reads as posturing. Say what worked for *you*.

> "Clients don't care that it ran 4,000 times, that's our language" → "For me that's landed way better than trying to show it ran a few thousand times."

### 2. Sound like a human typing, not a document.

A real person dropping a helpful reply doesn't format it as "1. 2. 3." with bold headers. Write flowing prose, a couple of paragraphs. Use contractions (I've, that's, honestly, out of curiosity). Numbered lists and section headers are the single clearest tell that a reply was manufactured.

Let the length match what the question actually needs: a post that asks several things earns a fuller answer, a simple one earns two lines. Don't force brevity, and don't pad to seem thorough. What you cut is padding — restating their question, throat-clearing, hedging, covering things they didn't ask — never substance. If every sentence is doing real work, it's the right length. A reply that feels too long almost always has padding to cut, not ideas to drop.

### 3. Never compare to or reference anyone.

Not the asker's approach ("instead of waiting for them to ask" → just say what you do). Not other people who already replied ("good list above, as X said" → drop it). Referencing someone else pulls in side-issues, and praising or contrasting can look like angling for something. Just share what you do and stop.

### 4. Say actions plainly. Leave nothing to interpret.

Concrete beats abstract. The reader should picture the thing, not decode it.

> "put that in front of them" → "send them a short recap every month."

### 5. No pitch, no brand.

Do not name Checkilo, do not name competitors, no links. Refer to any tool generically if you must ("an external heartbeat service"). You may name Checkilo **only** if someone in the thread explicitly asks "is there a tool for this?", and even then only in a follow-up reply, never in the opening answer. This is answer-first, not answer-then-sell.

### 6. Style details.

- No em dashes. No emoji.
- Borrow the OP's own words to show you actually read the post ("ran 4,000 times"), but reframe them — don't swallow their framing whole.
- A genuine closing question is welcome when it fits, and it doubles as a quiet validation signal (e.g. asking whether they push updates on a schedule or share a link tells you what people actually want). It has to be real curiosity, never a setup for a pitch.

### 7. Process.

- Space posts out. Don't answer several threads from the same person back-to-back — it looks like following them around, or a bot.
- Claude drafts; Coke posts manually, every time.

## The pre-post checklist

Run this **after** drafting, not just before. Re-reading is where the real fixes come from.

- Does every paragraph answer the question they actually asked?
- Any sentence that speaks for clients / everyone instead of from your own experience?
- Any numbered list, header, or documentation tone?
- Referencing or comparing to anyone (the asker, another replier, a tool)?
- Any brand name, link, or sales smell?
- Any em dash or emoji?
- Read it out loud — does it sound like a person actually typed it?

## Workflow when Coke hands you a thread

1. **Read the whole thread first** (use the browser if it's a live link). Note the exact question and, crucially, anything the OP says is already solved.
2. **Draft in the voice above** — short, human, our-angle, on-question.
3. **Re-read against the checklist**, especially rule 0. Cut drift. This pass is not optional.
4. **Give Coke the draft plus a Thai translation** so he can sanity-check it before posting, and flag anything he should tune to his own phrasing.
5. **Remind him it's his to post**, and if he's replying to more than one thread, suggest spacing them out.

## Finished examples — length follows the question

**A quick single question earns a quick answer.** Don't inflate it to look thorough.

Thread: "Do you bother versioning your n8n workflows, or just rely on the built-in history?"

> For anything a client actually depends on I export the workflow JSON into a git repo, mostly so I can see what changed between versions and roll back without guessing. The throwaway stuff I just leave on the built-in history. Honestly never needed more ceremony than that.

Three sentences, first person, answers exactly what was asked, and stops. Adding a second "here's another tip" paragraph here would be padding, not help.

**A focused question stays tight too.**

Thread: "When everything's working fine, how does the client know?"

> Honestly what helped me most was changing what I report. Instead of run counts, I pull out one number that ties back to why they hired me, like "it brought in 1,523 leads this month." For me that's landed way better than trying to show it ran a few thousand times.
>
> Then I just send them a short recap every month, something like "here's what it did for you." It reminds them it's still doing its job, so a quiet stretch doesn't leave them wondering what they're paying for.
>
> Out of curiosity, do you send them something on a schedule, or give them a link they can check whenever they get nervous?

Why it works: it answers only the working-fine question, it's all first-person ("for me", "I just send"), it's plain prose with no list, it names no tool, it reuses the OP's "ran 4,000 times" without adopting their framing, and the closing question is genuine and quietly surfaces whether they lean push or pull. It's brief because that question was single-focus.

**A post that asks several distinct things earns a reply that answers each one.** That reply runs longer, and it's correct as long as every line is doing work and none of it is padding. Length tracks the number of real questions, not how thorough you want to appear. So: a quick either/or gets a few sentences, a focused question gets a tight paragraph or two, a genuine multi-part question gets a fuller answer. Match the reply to the question in front of you.
