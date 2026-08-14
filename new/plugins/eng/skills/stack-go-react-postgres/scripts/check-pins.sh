#!/usr/bin/env bash
# check-pins.sh — the six upstream lookups from references/stack-pins.md, in one pass.
#
# A version table read as current when it is not is worse than no table at all, because it
# carries authority it no longer has. Run this before writing a version anywhere.
#
# It reports what upstream says TODAY beside what this skill pins. It edits nothing and
# decides nothing: "latest" and "production-ready" are not the same thing, and the peer
# constraints in references/stack-pins.md are what settle the difference.
#
# Usage:  ./check-pins.sh [npm|go|toolchain|node|pg|images]     (default: all)
# Needs:  curl, python3.  npm is optional — its section is skipped without it.
#
# ⚠ set -e is deliberately OFF. One failed network lookup must not kill the whole report.

set -uo pipefail

SKILL_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PINS_MD="$SKILL_DIR/references/stack-pins.md"
SECTION="${1:-all}"

bold() { printf '\n\033[1m%s\033[0m\n' "$1"; }
row()  { printf '  %-22s %-14s %s\n' "$1" "$2" "$3"; }
get()  { curl -fsS --max-time 20 "$1" 2>/dev/null; }
pyj()  { python3 -c "$1"; }

# The version this skill pins for a given table row, or "—" if it cannot be read.
pinned() {
  [ -f "$PINS_MD" ] || { echo "—"; return; }
  local v
  v="$(grep -m1 -F -- "| **$1** |" "$PINS_MD" 2>/dev/null \
       | sed -E 's/^\| \*\*[^*]+\*\* \| *`?v?([0-9][^ |`]*)`?.*/\1/')"
  case "$v" in [0-9]*) echo "$v" ;; *) echo "—" ;; esac
}

# ---------------------------------------------------------------- npm packages

NPM_PKGS=(vite @vitejs/plugin-react react react-router typescript typescript-eslint eslint tailwindcss vitest)

read -r -d '' P_NPM <<'PY'
import json, sys
d = json.load(sys.stdin)
if not isinstance(d, dict):
    print(d); sys.exit()
out = []
for k, v in (d.get("peerDependencies") or {}).items():
    out.append(k + "@" + v)
node = (d.get("engines") or {}).get("node")
if node:
    out.append("engines.node " + node)
print(d.get("version", "?"))
print(" · ".join(out) or "-")
PY

check_npm() {
  bold "npm — latest, and the peers that are the half that matters"
  if ! command -v npm >/dev/null 2>&1; then
    echo "  npm not on PATH — skipped"; return
  fi
  row "package" "latest" "pinned here | peers · engines"
  local p json parsed ver peers disp
  for p in "${NPM_PKGS[@]}"; do
    json="$(npm view "$p" version peerDependencies engines --json 2>/dev/null)"
    if [ -z "$json" ]; then row "$p" "?" "lookup failed"; continue; fi
    parsed="$(printf '%s' "$json" | pyj "$P_NPM" 2>/dev/null)"
    ver="$(printf '%s' "$parsed" | sed -n 1p)"
    peers="$(printf '%s' "$parsed" | sed -n 2p)"
    disp="$p"
    case "$p" in
      tailwindcss) disp="Tailwind" ;;
      react)       disp="React · react-dom" ;;
      typescript)  disp="TypeScript" ;;
      vite)        disp="Vite" ;;
      eslint)      disp="ESLint" ;;
      vitest)      disp="Vitest" ;;
    esac
    row "$p" "${ver:-?}" "$(pinned "$disp") | ${peers:--}"
  done
  echo "  ⚠ a newer version is only usable if its peers moved with it. Read the"
  echo "    compatibility table in references/stack-pins.md before taking any of these."
}

# ---------------------------------------------------------------- Go modules

GO_MODS=(
  "Fiber|github.com/gofiber/fiber/v3"
  "pgx|github.com/jackc/pgx/v5"
  "sqlc|github.com/sqlc-dev/sqlc"
  "golang-migrate|github.com/golang-migrate/migrate/v4"
  "golangci-lint|github.com/golangci/golangci-lint/v2"
  "testcontainers-go|github.com/testcontainers/testcontainers-go"
  "OpenTelemetry Go|go.opentelemetry.io/otel"
)

check_go() {
  bold "Go modules — latest, and that version's own 'go' directive (the floor it imposes)"
  row "module" "latest" "pinned here · needs go"
  local entry disp mod ver floor
  for entry in "${GO_MODS[@]}"; do
    disp="${entry%%|*}"; mod="${entry#*|}"
    ver="$(get "https://proxy.golang.org/${mod}/@latest" \
           | pyj 'import json,sys; print(json.load(sys.stdin)["Version"])' 2>/dev/null)"
    if [ -z "$ver" ]; then row "$disp" "?" "lookup failed"; continue; fi
    floor="$(get "https://proxy.golang.org/${mod}/@v/${ver}.mod" \
             | grep -m1 -E '^go [0-9]' | awk '{print $2}')"
    row "$disp" "$ver" "$(pinned "$disp") · needs go ${floor:-?}"
  done
}

# ---------------------------------------------------------------- Go toolchain

read -r -d '' P_TOOLCHAIN <<'PY'
import json, sys
d = json.load(sys.stdin)
stable = [r["version"] for r in d if r.get("stable")][:3]
rc = [r["version"] for r in d if not r.get("stable")][:3]
print("  stable:               " + (", ".join(stable) or "none"))
print("  NOT stable (rc/beta): " + (", ".join(rc) or "none"))
PY

check_toolchain() {
  bold "Go toolchain — stable versus rc, which the download page does not make obvious"
  get "https://go.dev/dl/?mode=json&include=all" | pyj "$P_TOOLCHAIN" 2>/dev/null \
    || echo "  lookup failed"
  echo "  pinned here:          $(pinned "Go")"
  echo "  ⚠ a release candidate is not a production floor."
}

# ---------------------------------------------------------------- Node

read -r -d '' P_NODE <<'PY'
import json, sys, datetime
d = json.load(sys.stdin)
today = datetime.date.today().isoformat()
for c in d[:5]:
    lts, sup, eol = c.get("lts"), c.get("support"), c.get("eol")
    if not lts:
        state = "Current — NOT LTS"
    elif isinstance(lts, str) and lts > today:
        state = "LTS from " + lts + " — not yet"
    elif isinstance(sup, str) and sup < today:
        state = "Maintenance LTS"
    else:
        state = "Active LTS"
    print("  {:<5} latest {:<10} {:<26} eol {}".format(
        c["cycle"], str(c.get("latest")), state, eol))
PY

check_node() {
  bold "Node — LTS status per major, so Current is not mistaken for LTS"
  get "https://endoflife.date/api/nodejs.json" | pyj "$P_NODE" 2>/dev/null \
    || echo "  lookup failed"
  echo "  pinned here: $(pinned "Node")"
}

# ---------------------------------------------------------------- PostgreSQL

read -r -d '' P_PG <<'PY'
import json, sys
for c in json.load(sys.stdin)[:4]:
    print("  {:<4} latest {:<10} released {}  eol {}".format(
        c["cycle"], str(c.get("latest")), c.get("latestReleaseDate"), c.get("eol")))
PY

check_pg() {
  bold "PostgreSQL — the upstream release line"
  get "https://endoflife.date/api/postgresql.json" | pyj "$P_PG" 2>/dev/null \
    || echo "  lookup failed"
  echo "  pinned here: $(pinned "PostgreSQL")"
  echo "  ⚠ the pin follows the IMAGE tag list, not this — see the images section."
}

# ---------------------------------------------------------------- image tags

IMAGES=("postgres|18" "node|24" "caddy|2")

read -r -d '' P_TAGS <<'PY'
import json, sys, re
names = [t["name"] for t in json.load(sys.stdin).get("results", [])]
alpine = [n for n in names if n.endswith("-alpine") and re.match(r"^\d+\.\d+\.?\d*-alpine$", n)]
print("    " + (", ".join(alpine[:8]) or ", ".join(names[:8]) or "none"))
PY

check_images() {
  bold "Container tags — the registry's own list, before any tag is written into compose"
  local spec repo filter
  for spec in "${IMAGES[@]}"; do
    repo="${spec%%|*}"; filter="${spec#*|}"
    echo "  $repo (tags matching '$filter'):"
    get "https://hub.docker.com/v2/repositories/library/${repo}/tags?page_size=100&name=${filter}" \
      | pyj "$P_TAGS" 2>/dev/null || echo "    lookup failed"
  done
  echo "  ⚠ docker-library lags upstream by weeks and SKIPS minors. A plausible-looking"
  echo "    tag that upstream released may simply not exist here, and compose fails to pull."
}

case "$SECTION" in
  npm)       check_npm ;;
  go)        check_go ;;
  toolchain) check_toolchain ;;
  node)      check_node ;;
  pg)        check_pg ;;
  images)    check_images ;;
  all)       check_npm; check_go; check_toolchain; check_node; check_pg; check_images ;;
  *)         echo "usage: $0 [npm|go|toolchain|node|pg|images]"; exit 2 ;;
esac

printf '\n⚠ Nothing here decides a pin. Take the newest release whose WHOLE dependency set\n'
printf '  supports it, and let something newer sit until its ecosystem catches up.\n'
