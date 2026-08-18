# Comment smells

One line per smell. Use as the checklist for Lane B audits. Nothing here depends on a programming
language or on the language the comment is written in.

## Restating the code

- The comment is the line below it, in prose — `count += 1  // increment count`.
- The comment is the function signature, in prose — "takes a user id and returns a user".
- The comment names the control flow the reader can already see — "if the list is empty, return early".
- The comment translates an operator — "compare the two values", "concatenate the strings".
- A closing-brace or `end`-of-block marker that only repeats what opened it — `} // if`.
- A section banner over code whose function name already says the same thing.
- A doc comment that lists parameters and gives each one its own name back.

## Narrating instead of informing

- Step labels the reader can count themselves — `// Step 1:`, `// Step 2:`, `// Finally`.
- Tour-guide phrasing — "Now we loop over…", "Here we check…", "Next, we build…".
- Labels over standard file furniture — `// Imports`, `// Constants`, `// Helper functions`.
- A comment introducing a block that should have been a named function instead.
- Narrating what the *reader* should do rather than what the code does — "note that below we…".

## Narrating the change rather than the code

- `// Added null check`, `// Changed from map to filter`, `// Removed the old branch`.
- `// New` / `// Updated` / `// Fixed` with no statement of what is now true.
- A date or author stamp that duplicates version control — `// 2026-08-18, added by …`.
- A changelog accumulating at the top of a file that version control already holds.
- Commented-out code kept "in case we need it" — version control is where that belongs.

## Covering for code that should change

- The comment explains what a badly named variable holds, instead of the variable being renamed.
- The comment explains what an overloaded function does in each case, instead of it being split.
- The comment apologises for the code — "sorry, this is messy", "hacky but works".
- The comment is long because the code is tangled, and shortening the comment is impossible.
- The comment defines a domain term that should have been a named type or constant.

## Misleading the reader

- The comment is confidently wrong about why — a reason invented rather than established.
- The comment is cryptic: an in-joke, an initialism, or a reference only its author can expand.
- The comment states an intention the code does not implement.
- The comment describes the ideal behaviour rather than the shipped behaviour.
- The comment claims a guarantee (thread-safe, idempotent, never null) that nothing enforces.
- A `TODO` with no owner, no issue, and no statement of what is missing — indistinguishable from noise.
- A `TODO` old enough that nobody can say whether it is still true.

## Rot

- The comment describes a parameter, branch, or return value that no longer exists.
- The numbers disagree — the comment says three retries, the loop runs five.
- The comment references a file, function, ticket, or URL that has moved or died.
- The comment describes a workaround for a bug that has since been fixed upstream.
- The comment sits above code that a later edit replaced wholesale.

## Gaps — the missing comment is also a finding

- A magic number with no stated source (a timeout, a threshold, a page size, a retry count).
- A guard or defensive branch whose triggering case is not described, inviting removal.
- Code copied in from elsewhere with no link to where it came from.
- A workaround with no statement of what broke and under what conditions.
- A deliberate deviation from the codebase's normal pattern, unexplained.
- An ordering dependency between statements that nothing signals.
- A public interface whose failure modes, units, or ownership rules are undocumented.

## Form

- Comments in a different human language from the rest of the file, forcing a context switch.
- A wall of prose where two sentences would do.
- Formatting decoration (boxes, rules, ASCII art) that adds bulk and no information.
- A comment far enough from the code it describes that the two drift apart independently.
