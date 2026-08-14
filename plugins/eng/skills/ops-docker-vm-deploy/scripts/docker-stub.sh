#!/usr/bin/env bash
# Fake `docker`, placed on PATH by test-deploy-script.sh.
#
# Behaviour is driven by environment variables set per scenario:
#   FAIL_LOGIN=1        `docker login` fails
#   FAIL_CONFIG=1       `docker compose config` fails
#   FAIL_PULL=1         `docker compose pull` fails
#   FAIL_UP=1           the FIRST `docker compose up` fails   (the deploy)
#   FAIL_ROLLBACK_UP=1  the SECOND `docker compose up` fails  (the rollback)
#   HEALTH=...          health status reported before the second `up`   (default healthy)
#   ROLLBACK_HEALTH=... health status reported from the second `up` on  (default healthy)
#   REAL_CONFIG=1       delegate `compose config` to the real docker CLI (no daemon needed)
#   STATE=<dir>         where the invocation counter lives (set by the harness)

sub="$1 ${2:-}"

case "$sub" in
  "compose config")
    [ "${FAIL_CONFIG:-0}" = 1 ] && { echo "stub: invalid compose file" >&2; exit 1; }
    if [ "${REAL_CONFIG:-0}" = 1 ] && [ -n "${REAL_DOCKER:-}" ]; then
      exec "$REAL_DOCKER" "$@"
    fi
    exit 0 ;;

  "login "*|"login")
    cat >/dev/null
    [ "${FAIL_LOGIN:-0}" = 1 ] && { echo "stub: login refused" >&2; exit 1; }
    echo "stub: login ok"; exit 0 ;;

  "logout "*|"logout")
    exit 0 ;;

  "compose pull")
    echo "stub: pulled TAG=${TAG:-<unset>}"
    [ "${FAIL_PULL:-0}" = 1 ] && exit 1
    exit 0 ;;

  "compose up")
    n=$(cat "$STATE/up_count" 2>/dev/null || echo 0); n=$((n + 1)); echo "$n" > "$STATE/up_count"
    echo "stub: up -d (call #$n, .env=$(tr -d '\n' < .env 2>/dev/null))"
    if [ "$n" = 1 ] && [ "${FAIL_UP:-0}" = 1 ]; then exit 1; fi
    if [ "$n" = 2 ] && [ "${FAIL_ROLLBACK_UP:-0}" = 1 ]; then exit 1; fi
    exit 0 ;;

  "inspect "*|"inspect")
    n=$(cat "$STATE/up_count" 2>/dev/null || echo 0)
    if [ "$n" -ge 2 ]; then echo "${ROLLBACK_HEALTH:-healthy}"; else echo "${HEALTH:-healthy}"; fi
    exit 0 ;;

  "compose logs")
    echo "stub: <container logs>"; exit 0 ;;

  "image prune")
    echo "stub: pruned"; exit 0 ;;
esac

echo "stub: UNHANDLED -> docker $*" >&2
exit 127
