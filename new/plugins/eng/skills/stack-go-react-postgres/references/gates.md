# Gates

Green from a clean checkout, or the batch is not done. These are `make` targets so CI and a
laptop run the identical command — `templates/Makefile` is the starting file.

⚠ **The targets wrap the skills' own commands; they do not replace them.** Every Go skill in
Phase B ends with `go build ./... · go vet ./... · go test -race ./... · golangci-lint run`.
If `make test` runs something weaker than what the skill told the implementer to run, the
skill and the gate disagree — and the gate is the one that decides whether the work shipped.

| Target | Asserts |
|---|---|
| `make lint` | `golangci-lint run` · `tsc --noEmit` · eslint |
| `make test` | unit tests, both sides — **`go test -race ./...`** and Vitest. ⚠ **`-race`, never plain `go test`**: every Go skill in Phase B makes that the standard, and a data race that only appears under load is exactly the defect this catches for free |
| `make test-db` | integration tests **against a real PostgreSQL, connecting as the application's own roles** — never as owner. ⚠ **This is the layer no skill owns**: grants, RLS, constraints and pgx mapping are proven here or nowhere. ⚠ **Call the repository the way the application does — through its real signature** — or the test proves the policy instead of the code path |
| `make rls-check` | ⚠ **every table is covered by a policy, or is on an explicit exemption list.** `make test-db` only reaches tables somebody wrote a test for; the table added in this PR is the one at risk, and a table with RLS left *off* fails as quietly as the pool trap — it just returns too many rows |
| `make migrate-check` | ⚠ **`up → down → up` on a scratch database.** Proves every `down` actually reverses its `up`, and that a schema builds from zero. Without this, "we have rollback" is a claim about files nobody has run. ⚠ **Two ways it goes red for the wrong reason:** a `down` that *refuses* on a destructive change fails this gate forever unless the refusal is conditional on there being data to lose; and golang-migrate's `down` **prompts on stdin** unless told how far to go, so in CI it dies at EOF |
| `make contract-check` | **the implementation matches the contract**, in that direction — the API convention is contract-first. ⚠ **Nothing in the pinned set can run this.** It needs a machine-readable contract, something that checks the implementation against it, and a breaking-change diff against the contract on `main`. **No tool is chosen here** — until a project picks them this gate is a name with nothing behind it |
| `make secret-scan` | no credential, token or `.env` in the tree **or in the built image** — two different artifacts, two different scanners, and ⚠ **nothing is pinned for either.** ⚠ It also goes red on the local compose file, whose credentials are unusable anywhere else by construction. That exemption is a decision to write down, not to discover on the first run |
| `make version-check` | **Node agrees across four places** — `.nvmrc`, the Dockerfile, the CI workflow, `docker-compose.yml`'s image tag — and ⚠ **Go across three**: `go.mod`, the Dockerfile, the CI workflow. There is no Go image in the compose. ⚠ **A floating tag cannot satisfy this**: `node:24-alpine` never "agrees with" `24.19.0`, so the tag has to be exact or the gate is theatre. `scripts/version-check.sh` implements it |
| `make arch-test` | architecture-fitness: layer boundaries hold, and the logging enforcement point has exactly one door. ⚠ **The import half is reachable with the `golangci-lint` already pinned** — its import-restriction and forbidden-identifier linters express layer rules directly. **The logging half is not:** *"a raw `string` cannot reach a log line"* is a rule about **types**, which no import linter can see, and nothing is pinned for it |

⚠ **Five of these nine exit non-zero as shipped, and the two reasons are different.** Do not
treat them as one pile.

| | Gates | Why | What closes it |
|---|---|---|---|
| **No tool chosen** | `contract-check` · `secret-scan` · half of `arch-test` | nothing in the pinned stack implements them, and choosing is a project decision | pick the tools, wire them, record the choice |
| **Body is project-specific** | `rls-check` · `migrate-check` | the tools exist and are already pinned — what is missing is *this project's* table list and scratch-database command | write the body; no new dependency needed |

**They are all listed anyway, on purpose.** A gate named and unimplemented is a known hole; a
gate quietly dropped is a rule nobody remembers deciding to stop enforcing.

⚠ **The second row is the more dangerous one, because it looks finished.** `rls-check` is the
control that catches the table somebody adds next quarter with RLS left off — and it is an
`exit 1` echo. A `pg_class` / `pg_policy` catalog query plus an exemption list is the whole
body; it is small, and until it exists the coverage claim is aspirational.

⚠ **A gate that has never gone red has not been shown to be a gate.** Once, on a scratch
branch, break each rule on purpose and record which check caught it. Any sabotage that is
*not* caught means the rule is being enforced by attention rather than by a test — fix the
gate before continuing. Re-run that table at release; it is the regression test for the gates
themselves.
