#!/usr/bin/env bash
# Exercise a remote deploy script's failure paths with no Docker daemon and no server.
#
#   ./test-deploy-script.sh path/to/deploy.sh
#   REAL_CONFIG=1 ./test-deploy-script.sh path/to/deploy.sh    # validate the real config file
#
# It puts a fake `docker` on PATH, rewrites only the script's APP_DIR to a temp
# directory, runs the real script, and asserts the host state left behind.
#
# Assumes the script under test: reads the deployed tag from `TAG=` in .env, installs a
# config file staged in a subdirectory, and requires a host-only secrets file. Adjust the
# names below if the project differs.

set -uo pipefail

DEPLOY_SCRIPT="${1:?usage: test-deploy-script.sh <path-to-deploy.sh>}"
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

CONFIG_FILE="${CONFIG_FILE:-docker-compose.yml}"
SECRET_FILE="${SECRET_FILE:-app.env}"
STAGING_DIR="${STAGING_DIR:-incoming}"
NEW_TAG="${NEW_TAG:-sha-NEW}"
OLD_TAG="${OLD_TAG:-sha-OLD}"

WORK="$(mktemp -d)"
BIN="$WORK/bin"
mkdir -p "$BIN"
cp "$HERE/docker-stub.sh" "$BIN/docker"
chmod +x "$BIN/docker"
export REAL_DOCKER="$(command -v docker || true)"
trap 'rm -rf "$WORK"' EXIT

pass=0; fail=0

# run_case <name> <seed_env: old|none> <expect_rc> <expect_tag> <expect_config> <expect_grep> [VAR=VAL ...]
run_case() {
  local name="$1" seed="$2" want_rc="$3" want_tag="$4" want_cfg="$5" want_grep="$6"; shift 6
  local d="$WORK/case"; rm -rf "$d"; mkdir -p "$d/$STAGING_DIR" "$d/.state"

  echo "OLD-CONFIG" > "$d/$CONFIG_FILE"
  echo "NEW-CONFIG" > "$d/$STAGING_DIR/$CONFIG_FILE"
  echo "SECRET=x"   > "$d/$SECRET_FILE"
  [ "$seed" = old ] && printf 'TAG=%s\n' "$OLD_TAG" > "$d/.env"

  sed "s#^APP_DIR=.*#APP_DIR=$d#" "$DEPLOY_SCRIPT" > "$d/run.sh"

  local out rc
  out=$(cd "$d" && env PATH="$BIN:$PATH" STATE="$d/.state" \
        REAL_CONFIG="${REAL_CONFIG:-0}" \
        IMAGE_TAG="$NEW_TAG" REGISTRY_USER=u REGISTRY_TOKEN=t IMAGE_LABEL=l \
        "$@" bash "$d/run.sh" 2>&1)
  rc=$?

  local got_tag got_cfg errs=""
  got_tag=$(tr -d '\n' < "$d/.env" 2>/dev/null || echo "<no .env>")
  got_cfg=$(cat "$d/$CONFIG_FILE" 2>/dev/null || echo "<missing>")

  [ "$rc" = "$want_rc" ]           || errs+=" rc=$rc(want $want_rc)"
  [ "$got_tag" = "$want_tag" ]     || errs+=" tag='$got_tag'(want '$want_tag')"
  [ "$got_cfg" = "$want_cfg" ]     || errs+=" config='$got_cfg'(want '$want_cfg')"
  grep -q -- "$want_grep" <<<"$out" || errs+=" missing-output='$want_grep'"
  [ -f "$d/$SECRET_FILE" ]         || errs+=" SECRETS-FILE-DESTROYED"
  [ -f "$d/$CONFIG_FILE.prev" ]    && errs+=" stale-.prev-left-behind"

  if [ -z "$errs" ]; then
    printf '  ok   %-42s rc=%s  tag=%-8s config=%s\n' "$name" "$rc" "$got_tag" "$got_cfg"
    pass=$((pass + 1))
  else
    printf '  FAIL %-42s%s\n' "$name" "$errs"
    sed 's/^/         | /' <<<"$out"
    fail=$((fail + 1))
  fi
}

echo "deploy script: $DEPLOY_SCRIPT"
echo

run_case "1  happy path"                old 0 "TAG=$NEW_TAG" NEW-CONFIG "succeeded"
run_case "2  shipped config invalid"    old 1 "TAG=$OLD_TAG" OLD-CONFIG "nothing was deployed"   FAIL_CONFIG=1
run_case "3  up -d fails"               old 1 "TAG=$OLD_TAG" OLD-CONFIG "rolled back"            FAIL_UP=1
run_case "4  container unhealthy"       old 1 "TAG=$OLD_TAG" OLD-CONFIG "rolled back"            HEALTH=unhealthy
run_case "5  rollback unhealthy"        old 1 "TAG=$OLD_TAG" OLD-CONFIG "ROLLBACK FAILED"        HEALTH=unhealthy ROLLBACK_HEALTH=unhealthy
run_case "6  rollback up -d fails"      old 1 "TAG=$OLD_TAG" OLD-CONFIG "ROLLBACK FAILED"        HEALTH=unhealthy FAIL_ROLLBACK_UP=1
run_case "7  no previous tag"          none 1 "TAG=$NEW_TAG" OLD-CONFIG "no previous tag"        HEALTH=unhealthy
run_case "8  registry pull fails"       old 1 "TAG=$OLD_TAG" OLD-CONFIG "rolled back"            FAIL_PULL=1
run_case "9  registry login fails"      old 1 "TAG=$OLD_TAG" OLD-CONFIG "login refused"          FAIL_LOGIN=1

# Missing host prerequisite: nothing may be mutated, and the message must name the file.
d="$WORK/case"; rm -rf "$d"; mkdir -p "$d/$STAGING_DIR" "$d/.state"
echo "OLD-CONFIG" > "$d/$CONFIG_FILE"; echo "NEW-CONFIG" > "$d/$STAGING_DIR/$CONFIG_FILE"
printf 'TAG=%s\n' "$OLD_TAG" > "$d/.env"
sed "s#^APP_DIR=.*#APP_DIR=$d#" "$DEPLOY_SCRIPT" > "$d/run.sh"
out=$(cd "$d" && env PATH="$BIN:$PATH" STATE="$d/.state" IMAGE_TAG="$NEW_TAG" \
      REGISTRY_USER=u REGISTRY_TOKEN=t IMAGE_LABEL=l bash "$d/run.sh" 2>&1); rc=$?
if [ "$rc" = 1 ] && grep -qi "$SECRET_FILE" <<<"$out" && [ "$(cat "$d/$CONFIG_FILE")" = OLD-CONFIG ]; then
  printf '  ok   %-42s rc=1  error names %s, nothing mutated\n' "10 host prerequisite missing" "$SECRET_FILE"
  pass=$((pass + 1))
else
  printf '  FAIL %-42s rc=%s (expected 1, an error naming %s, and no mutation)\n' \
         "10 host prerequisite missing" "$rc" "$SECRET_FILE"
  sed 's/^/         | /' <<<"$out"
  fail=$((fail + 1))
fi

echo
echo "passed=$pass failed=$fail"
[ "$fail" = 0 ]
