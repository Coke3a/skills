# Entry Gate

This skill runs in one phase only: the design overview is settled, the deep behaviour is not.
Outside that phase it does more harm than good, so the gate is a hard stop.

**Too early** — the overview is incomplete. Tests written now encode guesses about endpoints,
payloads, and columns. Every guess that turns out wrong invalidates the tests built on it, and the
rework costs more than asking would have.

**Too late** — the behaviour already exists. Tests written now describe what the code does, not what
it should do, and they pass on the first run. That is characterisation, which is useful but is a
different workflow with different rules.

## The four preconditions

Check all four before writing anything. Each must be **quotable from a source** — a spec file, a
design doc, an OpenAPI file, a migration, or something the user states in the conversation. The
source does not have to be a file; a direct answer from the user counts.

| # | Precondition        | Satisfied when                                                                | Not satisfied by                                    |
| - | ------------------- | ----------------------------------------------------------------------------- | --------------------------------------------------- |
| 1 | Endpoints           | Method and path for every endpoint in scope                                    | "the auth endpoints"                                |
| 2 | Request/response    | Field names and types for each request and each response                       | "returns the user object"                           |
| 3 | Database schema     | Tables and columns this feature reads or writes, including nullability         | "stores it in Postgres"                             |
| 4 | Purpose             | One sentence you can quote on what the feature is for                          | the ticket title                                    |

Deliberately **not** required: acceptance criteria, method signatures, error cases, validation
rules, flow. Those are what this skill produces. Requiring them would mean the design work was
already done elsewhere.

## Running the gate

1. Read the design source end to end. Do not skim, and do not accept a summary of it.
2. Fill the table above with the actual quoted values, not with "yes".
3. Check the other direction: does the usecase already exist and work? A quick look at
   `internal/usecase/{feature}/` answers it.
4. Decide.

## On failure

Stop. Report, in this shape:

```
Entry gate failed — this skill needs the design overview settled first.

Present:
- Endpoints: POST /v1/sessions, DELETE /v1/sessions  (spec.md line 40)
- Purpose: "let a user sign in with email and password"  (spec.md line 12)

Missing:
- Request/response shapes — spec.md describes the login request but not the
  response body or its fields.
- Database schema — no table definition for sessions; migrations/ has nothing
  matching.

To proceed, either:
- point me at where these are defined, or
- answer them here, or
- run coke-eng:flow-spec-review over spec.md first if the whole document needs a pass.

I have not written any tests or code.
```

Rules for the failure report:

- Name the specific missing item and what you looked at. "The spec is incomplete" is not a report.
- List what *is* present, so the user can see how close it is.
- Offer the three routes: point at a source, answer inline, or run `coke-eng:flow-spec-review`.
- State plainly that nothing was written.
- Do not offer to "start with what we have and fill the rest in later". That is the failure mode
  the gate exists to prevent.

If the user explicitly overrides the gate after seeing the report, proceed — and record the
override and the unresolved assumptions in the final summary, so the risk is visible at review time.

## On the "too late" branch

If the behaviour already exists, do not run this loop. Say so and switch:

```
This behaviour is already implemented in internal/usecase/authfeature/login.go,
so there is nothing left to design. Switching to
workflows/add-tests-to-existing-feature.md — the tests will describe what the
code currently does, which is what you want before changing it.
```

Partial implementation is common and is not automatically the "too late" branch: if the usecase
exists but the criterion under design is not implemented, the loop applies normally to that
criterion.
