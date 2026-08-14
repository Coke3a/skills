# <project>/Makefile — the gate set.
#
# These exist so CI and a laptop run the IDENTICAL command. A gate that differs between
# them is not a gate.
#
# ⚠ FIVE targets below exit non-zero as shipped, for two different reasons:
#     no tool chosen yet   — contract-check, secret-scan, the type half of arch-test
#     body is yours to write — rls-check, migrate-check (the tools are already pinned)
#   They are present and loud on purpose: a gate named and unimplemented is a known hole;
#   a gate quietly dropped is a rule nobody remembers deciding to stop enforcing.

.PHONY: all db lint test test-db rls-check migrate-check contract-check secret-scan \
        version-check arch-test gates

BACKEND  ?= backend
FRONTEND ?= frontend

gates: lint test test-db rls-check migrate-check contract-check secret-scan version-check arch-test

# --- local stack ------------------------------------------------------------

# ⚠ Postgres alone. `docker compose up` needs migration/ to hold at least one up/down pair —
#   `migrate up` on an empty directory FAILS rather than no-ops, and the backend then waits
#   on a one-shot that never succeeds. Until the schema design lane delivers that first pair,
#   this is how you get a database to work against.
db:
	docker compose up -d postgres

# --- static -----------------------------------------------------------------

lint:
	cd $(BACKEND)  && golangci-lint run ./...
	cd $(FRONTEND) && npx tsc --noEmit && npx eslint .

# ⚠ -race, never plain `go test`. Requires cgo and a C toolchain — a CI image built only
#   for CGO_ENABLED=0 cannot run this.
test:
	cd $(BACKEND)  && go build ./... && go vet ./... && go test -race ./...
	cd $(FRONTEND) && npx vitest run

# --- database ---------------------------------------------------------------

# ⚠ Against a real PostgreSQL, connecting as the APPLICATION's own roles — never as owner.
#   This is the layer no skill owns: grants, RLS, constraints and pgx mapping are proven
#   here or nowhere. Call the repository through its real signature, or the test proves
#   the policy instead of the code path.
test-db:
	cd $(BACKEND) && go test -race -tags=integration ./...

# ⚠ Every table is covered by a policy, or is on an explicit exemption list. test-db only
#   reaches tables somebody wrote a test for; the table added in this PR is the one at risk.
# ⚠ No new tool is needed for this one — it is a catalog query plus a list. The shape:
#     SELECT c.relname FROM pg_class c JOIN pg_namespace n ON n.oid = c.relnamespace
#      WHERE n.nspname = 'public' AND c.relkind = 'r'
#        AND (NOT c.relrowsecurity
#             OR NOT EXISTS (SELECT 1 FROM pg_policy p WHERE p.polrelid = c.oid))
#        AND c.relname <> ALL($EXEMPT);
#   Non-empty result = fail, and print the table names. Keep $EXEMPT in the repo, reviewed.
rls-check:
	@echo "TODO: catalog query above + this project's exemption list. No dependency needed."
	@exit 1

# ⚠ up -> down -> up on a scratch database. Two ways this goes red for the wrong reason:
#   a `down` that refuses on a destructive change fails forever unless the refusal is
#   conditional on there being data to lose; and golang-migrate's `down` PROMPTS ON STDIN
#   unless told how far to go, so in CI it dies at EOF. Hence the explicit count/-all.
migrate-check:
	@echo "TODO: bring up a scratch database, then:"
	@echo "  migrate -path migration -database \$$SCRATCH_DSN up"
	@echo "  migrate -path migration -database \$$SCRATCH_DSN down -all   # -all: never prompts"
	@echo "  migrate -path migration -database \$$SCRATCH_DSN up"
	@exit 1

# --- gates with no tool chosen yet ------------------------------------------

# Needs: a machine-readable contract, something that checks the IMPLEMENTATION against it
# (contract-first, that direction), and a breaking-change diff against the contract on main.
contract-check:
	@echo "contract-check: no tool chosen for this project yet."
	@echo "  pick one, wire it here, and record the choice in the project's own setup."
	@exit 1

# Needs TWO scanners: one for the tree, one for the built image — different artifacts.
# ⚠ It also goes red on docker-compose.yml, whose credentials are unusable anywhere else
#   by construction. That exemption is a decision to write down, not to discover on run 1.
secret-scan:
	@echo "secret-scan: no tool chosen for this project yet (tree scanner + image scanner)."
	@exit 1

# ⚠ The import half is reachable with golangci-lint's import-restriction and
#   forbidden-identifier linters. The logging half is not: "a raw string cannot reach a log
#   line" is a rule about TYPES, which no import linter can see.
arch-test:
	cd $(BACKEND) && golangci-lint run --enable-only=depguard,forbidigo ./...
	@echo "TODO: the type half — assert no raw string reaches the logger's field API"
	@exit 1

# --- consistency ------------------------------------------------------------

# Copy scripts/version-check.sh out of this skill into the project's own scripts/ — a gate
# has to live in the repository it gates, or CI and a laptop are not running the same thing.
version-check:
	./scripts/version-check.sh
