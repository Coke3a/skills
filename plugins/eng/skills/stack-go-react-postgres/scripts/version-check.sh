#!/usr/bin/env bash
# version-check.sh — the `make version-check` gate.
#
# Node and Go are each pinned in several places. They drift silently and the symptom is
# "works on my machine". This asserts they agree:
#
#   Node — .nvmrc · any Dockerfile that pins it · the CI workflow · the compose image tag
#   Go   — go.mod · the backend Dockerfile · the CI workflow      (no Go image in compose)
#
# The frontend often has no Dockerfile of its own (it runs from a stock node: image in the
# local stack and ships as static files), so that source is optional — the script checks
# whatever it finds and only fails on a real disagreement.
#
# ⚠ A floating tag cannot satisfy this. `node:24-alpine` never "agrees with" 24.19.0, so a
#   tag has to be exact or the gate is theatre — a partial version is reported as a FAIL,
#   not quietly widened to match.
#
# Usage:  ./version-check.sh [repo-root]      (default: current directory)
# Exit:   0 all agree · 1 a disagreement or a floating tag · 2 nothing found to check

set -uo pipefail

ROOT="${1:-.}"
cd "$ROOT" || exit 2

FAIL=0
FOUND=0

note() { printf '  %-34s %s\n' "$1" "$2"; }
bad()  { printf '\033[31m  %-34s %s\033[0m\n' "$1" "$2"; FAIL=1; }

# Files worth reading, minus the ones that are never ours.
files() {
  find . \
    \( -name node_modules -o -name vendor -o -name .git -o -name dist -o -name build \) -prune \
    -o -type f \( "$@" \) -print 2>/dev/null
}

exact() { [[ "$1" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; }

# collect "<value>|<where>" pairs on stdout
collect_node() {
  local f v
  for f in $(files -name .nvmrc); do
    v="$(tr -d ' \nv' < "$f")"
    [ -n "$v" ] && echo "$v|$f"
  done
  for f in $(files -name 'Dockerfile*'); do
    grep -oiE '^FROM +node:[0-9][^ ]*' "$f" 2>/dev/null | while read -r m; do
      echo "${m##*node:}|$f"
    done
  done
  for f in $(files -name '*.yml' -o -name '*.yaml'); do
    grep -oE 'node-version: *.?[0-9][^"'\''[:space:]]*' "$f" 2>/dev/null | while read -r m; do
      echo "$(printf '%s' "${m##*: }" | tr -d "\"'")|$f"
    done
    grep -oE 'image: *node:[0-9][^ ]*' "$f" 2>/dev/null | while read -r m; do
      echo "${m##*node:}|$f"
    done
  done
}

collect_go() {
  local f v
  for f in $(files -name go.mod); do
    v="$(grep -m1 -E '^go [0-9]' "$f" | awk '{print $2}')"
    [ -n "$v" ] && echo "$v|$f (go directive — a MINIMUM, not a pin)"
  done
  for f in $(files -name 'Dockerfile*'); do
    grep -oiE '^FROM +golang:[0-9][^ ]*' "$f" 2>/dev/null | while read -r m; do
      echo "${m##*golang:}|$f"
    done
  done
  for f in $(files -name '*.yml' -o -name '*.yaml'); do
    grep -oE 'go-version: *.?[0-9][^"'\''[:space:]]*' "$f" 2>/dev/null | while read -r m; do
      echo "$(printf '%s' "${m##*: }" | tr -d "\"'")|$f"
    done
  done
}

report() {
  local label="$1" data="$2" base seen=""
  printf '\n%s\n' "$label"
  if [ -z "$data" ]; then
    echo "  nothing found — is this the repository root?"
    return
  fi
  FOUND=1
  while IFS='|' read -r raw where; do
    [ -z "$raw" ] && continue
    # strip an image suffix: 24.19.0-alpine -> 24.19.0
    base="${raw%%-*}"
    if exact "$base"; then
      note "$base" "$where"
    elif [[ "$where" == *"go directive"* ]]; then
      # `go 1.26` is a legitimate minimum, not a floating pin. It constrains the minor only.
      note "$base  (minimum)" "$where"
    else
      bad "$base  ⚠ floating — not an exact x.y.z" "$where"
    fi
    seen="$seen $base"
  done <<< "$data"

  # A go.mod directive of 1.26 legitimately covers a 1.26.5 toolchain; compare on x.y.
  local uniq_full uniq_minor
  uniq_full="$(printf '%s\n' $seen | sort -u | tr '\n' ' ')"
  uniq_minor="$(printf '%s\n' $seen | cut -d. -f1,2 | sort -u | tr '\n' ' ')"
  if [ "$(printf '%s\n' $seen | cut -d. -f1,2 | sort -u | wc -l | tr -d ' ')" -gt 1 ]; then
    bad "DISAGREE on major.minor:" "$uniq_minor"
  elif [ "$(printf '%s\n' $seen | sort -u | wc -l | tr -d ' ')" -gt 1 ]; then
    # same minor, different patch — fine only if the odd one out is go.mod's directive
    if printf '%s' "$data" | grep -q 'go directive'; then
      note "patch differs, same minor — check go.mod is the loose one:" "$uniq_full"
    else
      bad "DISAGREE on patch:" "$uniq_full"
    fi
  fi
}

report "Node" "$(collect_node)"
report "Go"   "$(collect_go)"

echo
if [ "$FOUND" -eq 0 ]; then
  echo "version-check: found nothing to compare — run it from the repository root."
  exit 2
elif [ "$FAIL" -eq 0 ]; then
  echo "version-check: OK"
  exit 0
else
  echo "version-check: FAILED — pin every place to the same exact version."
  exit 1
fi
