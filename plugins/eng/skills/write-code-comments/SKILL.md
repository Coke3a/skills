---
name: write-code-comments
description: Decides what earns a code comment and what to delete — a language-agnostic discipline that applies to any programming language (Rust, Go, TypeScript, Python, SQL, Bash, config) and to comments written in any human language. Covers the six things code cannot say (a non-obvious choice, copied code and its source, an external spec, a bug's reproduction condition, unfinished work, a domain rule behind a magic value) and the comments that should never ship (restating the code, narrating structure or the diff, propping up a bad name, or a stale claim an edit made false). Use whenever writing or editing comments, docstrings, doc comments, or TODOs; whenever adding a workaround, a bug fix, or code copied in from elsewhere; and when asked to audit, thin out, or clean up the comments a file already has — including phrasings like "add comments to this", "write docstrings for these functions", "this file has way too many comments", "is this worth commenting", "document what this does", or "leave a note about why we did it this way". Do not use for generating API documentation sites or READMEs, for commenting out code while debugging, for writing commit or PR descriptions, for explaining code to the user in chat rather than in the file, or for a full code review (Rust → coke-eng:rust-code-review).
---

# Writing Code Comments

## Use this when

- Writing or editing any comment, docstring, doc comment, or TODO.
- Adding a workaround, a bug fix, or code whose obvious simplification would be wrong.
- Copying code in from Stack Overflow, a gist, a blog post, or another repository.
- Asked to "add comments", "write docstrings", or "document this function" — that is, prose that lands in the file.
- Asked to clean up, thin out, or audit the comments a file already has.
- Deciding whether a given line is worth a comment at all.

## Do not use this when

- Generating an API documentation site or a README. That is docs tooling, not source comments.
- Commenting out code to bisect a failure → use `superpowers:systematic-debugging`.
- Writing a commit message or PR description. Those describe the change; comments describe the code.
- Explaining code to the user in chat. Nothing is being written to a file.
- Running a full code review — comments are one lens of many there. Rust → use `coke-eng:rust-code-review`.

## The one idea everything follows from

**"Code Tells You How, Comments Tell You Why."** — Jeff Atwood

What follows is a way of reaching a happy medium, not a way of writing fewer comments. Both extremes
fail: over-commenting buries the signal in noise that no compiler checks, and under-commenting throws
away context nobody can ever recover. An agent's habitual failure is the first; its most expensive one
is the second.

| Question | Answer |
| --- | --- |
| Why not comment generously and let readers skip what they know? | Every comment is a second source of truth that no compiler checks. It must be read, maintained, and re-verified on every edit, and it earns none of that when it repeats the line below it. |
| Why is a stale comment worse than no comment? | A missing comment makes a reader read the code. A wrong comment makes them trust it and stop reading. The first costs time; the second causes bugs. |
| Why treat "I can't word this clearly" as a signal? | Often the difficulty is in the code rather than the sentence — a function you struggle to describe in one line may be doing more than one thing. Treat it as a prompt to re-read the code, not as a verdict on it. |
| Is any of this tied to a language? | No. The tests below ask about **information**, not syntax — "can the code say this itself?" answers the same way in Rust, Go, TypeScript, Python, SQL, Bash or HCL, and the same way whatever human language the comment is written in. |
| Why does an agent need this at all? | An agent's default is inverted. It over-writes the comments that restate code, and almost never writes the one comment only it can write — what it learned while getting the code to work. |

## Three tests before a comment ships

| Test | Ask | If it fails |
| --- | --- | --- |
| **Delete** | Read the comment, then the code. Did the comment say anything the code did not? | Delete it. |
| **Surprise** | Would a competent reader be confused here, or "simplify" this into a bug? | Keep it, and say what would go wrong. |
| **Archaeology** | To write this line, did you learn something that is not in the file? | Put it in the file. |

Archaeology is the test that pays. A library that returns null in an undocumented case, a header a vendor
API silently requires, a retry count found by trial — that context lives in the session and dies with it.
Written down, it survives.

## What earns a comment

Six recurring things sit outside the code, where a reader cannot reconstruct them at any price. The
list is not exhaustive — it is the set worth checking against every time.

| It carries | Write it as | It prevents |
| --- | --- | --- |
| **Why this, not the obvious alternative** | The failure the obvious version causes | Someone "cleaning up" the guard and restoring the bug |
| **Where the code came from** | A link to the answer, gist, or repo | Losing the caveats in the replies, and the attribution the licence requires |
| **Which external rule forces this** | A link to the RFC, spec, or vendor doc, at the line it governs | A required oddity being read as an accident |
| **How it broke** | The reproduction condition, plus the issue reference | Nobody being able to tell whether the workaround is still needed |
| **What is unfinished** | `TODO(owner):` — what is missing and why — with an issue link | Known gaps becoming production discoveries |
| **What the domain demands** | The rule behind the number | A policy constant being tidied into a different policy |

Where the code came from and how it broke are the two agents skip most and readers need most.

## Core rules

- **A comment that restates the line below it is deleted, not improved.** `i += 1  // add one to i` costs a read and a maintenance obligation and returns nothing.
- **Never narrate structure.** `// Step 1: validate input`, `// Now loop over the items`, `// Import dependencies`. If the steps are hard to see, extract functions whose names are those labels.
- **Never narrate the diff.** `// Added null check`, `// Changed from map to filter`, `// New retry logic`. That belongs in the commit message; in the file it is false by the next commit. This is not the claim that version control replaces comments — commit messages are brief, and the change that explains a line is often not the most recent one touching it. That is precisely why a bug's *reason* belongs in the file even though its *diff* does not.
- **Fix the name before writing the comment.** A comment explaining what `n` holds is a rename waiting to happen. Kernighan and Plauger: "Don't comment bad code — rewrite it."
- **If the comment will not come out clearly, change the code.** Do not ship the tangled version with a paragraph on top.
- **A comment only its author can decode is worse than silence.** Cleverness, in-jokes, and private initialisms get cut.
- **Comments follow the codebase's language, not the author's.** If a file's comments are in Thai, write Thai. Switching language mid-file costs the reader a context switch at every comment. Nothing else changes: restating the code in a second language is still restating the code.
- **When editing code, re-read the comments around it.** Every comment inside the blast radius of a change is a claim that may have just become false. Comment rot is caused by edits, not by time.
- **Link, do not paraphrase.** A URL to the spec outlives a summary of it.
- **Never invent a reason.** If you do not know why a line is the way it is, say what you observed or leave it alone. A confident wrong "why" is the most expensive comment there is.
- **Never paste in code you do not understand.** If you cannot say what a borrowed block does, no comment you write about it can be trusted, and the source link you attach becomes a citation for a claim you never checked.
- **Paste only URLs you actually opened.** A reconstructed link is worse than no link: it looks checkable, so a reader follows it, and it sends them somewhere false. Without the real URL to hand, describe the source in words instead.
- Judge the result by whether each comment carries something the code cannot — not by how few you managed to ship.

## Workflow

### Lane A — writing or editing code

1. Write the code first, with names good enough that most of it needs no commentary.
2. Run the three tests on each comment you were about to write. Most will not survive Delete.
3. For each survivor, name which of the six things it carries. If none fits, it is narration — cut it.
4. Before finishing, re-read the comments adjacent to every line changed and correct the ones the change falsified.

### Lane B — auditing comments already in a file

1. Read the file, then read `references/comment-smells.md`.
2. Give each comment a verdict: **keep**, **delete**, **fix the code instead**, or **stale — correct or remove**.
3. Look for what is missing too. A subtle workaround with no explanation is a comment gap, and it is the more serious finding.
4. Report one line per finding: `file:line`, verdict, reason. Do not rewrite the file unless asked.
5. Never call a comment stale without reading the code it describes.

```text
src/billing/invoice.ts:42   delete   restates the line below
src/billing/invoice.ts:88   stale    says retries 3 times; the loop runs 5
src/billing/invoice.ts:130  missing  the `?? []` guard has a reason nobody can see
```

## Load more detail

| Read | For |
| --- | --- |
| `references/comment-smells.md` | The full anti-pattern list, one line each — the checklist for Lane B |
| `references/comment-shapes.md` | Worked examples of each of the six things code cannot say |
| `references/doc-comments.md` | What a doc comment on a public interface must carry beyond the signature |

## Related skills

- `coke-eng:rust-code-review` — owns full Rust review. This skill owns the comment lens it applies.
- `coke-eng:go-clean-architecture`, `coke-eng:rust-clean-architecture` — own naming and layer structure. When a comment is propping up a name, the fix is theirs.
- `karpathy-guidelines` — owns keeping the change surgical. This skill owns what the change leaves behind in prose.

## Source

The nine rules underneath this skill come from Stack Overflow's *Best practices for writing code
comments* — https://stackoverflow.blog/2021/12/23/best-practices-for-writing-code-comments/. Its rules
5–9 became "what earns a comment"; its rules 1–4 became the subtractive half of "core rules". Read it
when a judgement here seems arbitrary; it carries the worked examples and the reasoning.

These parts are **not** from that article and carry no authority from it: the archaeology test, the
sixth category (what the domain demands), the bans on narrating structure and the diff, the
comment-language rule, comment rot as edit-driven, the two rules about not inventing reasons or URLs,
and all of `references/doc-comments.md`. They are this skill's own positions, added because an agent
writing code fails differently from a human writing code.

## Definition of done

- Every surviving comment carries one of the six things code cannot say.
- No comment restates its code, narrates structure, or describes the diff.
- Every copied block links a source that was actually opened; every workaround names what broke; every TODO names an owner, and an issue where one exists.
- Comments adjacent to changed lines were re-read and are still true.
- No "why" was invented — anything uncertain was left out, or marked as observed rather than concluded.
