# Write the remote deploy script

The script runs on the host with its contents piped over SSH. Start from
`templates/deploy.sh`; this file explains the ordering, which is where the correctness
lives. Read `references/rollback-safety.md` alongside it.

## The order is the design

```
1. cd to the app directory
2. preflight            — host prerequisites; nothing mutated yet
3. read previous state  — the tag currently deployed, for rollback
4. registry login       — can fail freely; nothing mutated yet
5. install config       — FIRST MUTATION; back up what you replace
6. validate config      — on failure: restore, exit
7. define wait_healthy  — one function, called twice
8. pull                 — captured, not left to set -e
9. write tag, start     — captured
10. healthy? → success (clean up, prune, exit 0)
11. otherwise → rollback: restore config + tag, restart, VERIFY, exit 1
```

Steps 2–4 come before step 5 so that everything which can fail independently of host state
fails while the host is still untouched. That eliminates most of the cases that would
otherwise need capturing, and the ones left (8 and 9) are the two you must capture.

## Preflight

Check what the host must already have and name it in the error:

```bash
if [ ! -f app.env ]; then
  echo "app.env is missing on the host — see SETUP.md"
  exit 1
fi
```

Without this, the missing file surfaces later as a config validation error and the script
blames the wrong file. Cheap checks with accurate messages pay for themselves the first
time someone else runs the deploy.

## Installing a shipped config file

The copy step already ran, so the new file is sitting in a staging directory. Back up the
live one first, then install, then validate:

```bash
if [ -f docker-compose.yml ]; then
  cp -f docker-compose.yml docker-compose.yml.prev
fi
mv -f incoming/docker-compose.yml docker-compose.yml
rmdir incoming 2>/dev/null || true

if ! TAG="$IMAGE_TAG" docker compose config -q; then
  if [ -f docker-compose.yml.prev ]; then
    mv -f docker-compose.yml.prev docker-compose.yml
  fi
  echo "new compose file is invalid — restored the previous one, nothing was deployed"
  exit 1
fi
```

Pass the tag explicitly to the validation so it checks the file as it will actually be used.
Note that an unset variable is only a *warning* to compose, not an error — validation is not
a guard against a missing tag. The pull is what catches that.

## Capture what would otherwise trip `set -e`

```bash
ok=1
TAG="$IMAGE_TAG" docker compose pull || ok=0

if [ "$ok" = 1 ]; then
  printf 'TAG=%s\n' "$IMAGE_TAG" > .env
  docker compose up -d || ok=0
fi

if [ "$ok" = 1 ] && wait_healthy; then
  ...success...
  exit 0
fi
...rollback...
```

Pull before writing the tag file: if the image is not available, the host is left pointing
at the tag that still works.

## One health function, called twice

```bash
wait_healthy() {
  for _ in $(seq 1 40); do
    status="$(docker inspect -f '{{.State.Health.Status}}' <name> 2>/dev/null || echo unknown)"
    echo "  health: $status"
    if [ "$status" = "healthy" ];   then return 0; fi
    if [ "$status" = "unhealthy" ]; then return 1; fi
    sleep 5
  done
  return 1
}
```

Use `if ... fi`, not `[ ... ] && return`, so the function does not interact with `set -e` in
surprising ways. Check the budget against the healthcheck's worst case — see
`references/healthchecks.md`.

## Rollback restores everything, then verifies

```bash
echo "deploy failed — rolling back"
docker compose logs --tail 200 <service> || true

if [ -f docker-compose.yml.prev ]; then
  mv -f docker-compose.yml.prev docker-compose.yml
fi

if [ -z "$PREV_TAG" ]; then
  echo "no previous tag to roll back to — check by hand"
  exit 1
fi

printf 'TAG=%s\n' "$PREV_TAG" > .env
if docker compose up -d && wait_healthy; then
  echo "rolled back to $PREV_TAG — healthy"
else
  echo "ROLLBACK FAILED — the service is likely down, intervene now"
fi
exit 1
```

Capture logs *before* restoring, so they describe the failed release. The two distinct
outcome messages are the point: one is "handled", the other is "get up".

## Comments

Comment the non-obvious mechanism, not the syntax. Lines worth a comment: why the pull comes
before the tag write, why an outcome is captured instead of left to `set -e`, what the
prune's label filter scopes, which file holds secrets and is never touched. Skip commentary
on what the shell is doing — it is right there.

## After success

```bash
rm -f docker-compose.yml.prev
docker image prune -af --filter "until=168h" --filter "label=<label>" || true
exit 0
```

The label filter is not optional on a shared host. See `references/host-exposure.md`.
