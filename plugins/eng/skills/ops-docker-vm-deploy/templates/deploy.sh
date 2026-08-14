#!/usr/bin/env bash
# shellcheck shell=bash
# Runs on the target host via appleboy/ssh-action (script_path).
# IMAGE_TAG, REGISTRY_USER, REGISTRY_TOKEN and IMAGE_LABEL arrive through the action's envs.

set -euo pipefail

# ---- change these four -------------------------------------------------------------
APP_DIR=/opt/myapp
REGISTRY=ghcr.io
CONTAINER=myapp          # container_name in docker-compose.yml
SERVICE=app              # service key in docker-compose.yml
# ------------------------------------------------------------------------------------

cd "$APP_DIR"

# Set up by hand once; docker-compose.yml requires it and CI never creates it.
if [ ! -f app.env ]; then
  echo "app.env is missing on the host — see SETUP.md"
  exit 1
fi

# Currently deployed tag, kept for rollback.
PREV_TAG=""
if [ -f .env ]; then
  PREV_TAG="$(sed -n 's/^TAG=//p' .env | head -n1)"
fi
echo "previous tag : ${PREV_TAG:-none}"
echo "new tag      : ${IMAGE_TAG}"

echo "$REGISTRY_TOKEN" | docker login "$REGISTRY" -u "$REGISTRY_USER" --password-stdin
trap 'docker logout "$REGISTRY" >/dev/null 2>&1 || true' EXIT

# Install the file staged by scp, keeping the live one for rollback.
if [ -f docker-compose.yml ]; then
  cp -f docker-compose.yml docker-compose.yml.prev
fi
mv -f incoming/docker-compose.yml docker-compose.yml
rmdir incoming 2>/dev/null || true

# Restore and abort if the new compose file does not parse.
if ! TAG="$IMAGE_TAG" docker compose config -q; then
  if [ -f docker-compose.yml.prev ]; then
    mv -f docker-compose.yml.prev docker-compose.yml
  fi
  echo "new compose file is invalid — restored the previous one, nothing was deployed"
  exit 1
fi

# Poll the container healthcheck for up to 200s.
wait_healthy() {
  for _ in $(seq 1 40); do
    status="$(docker inspect -f '{{.State.Health.Status}}' "$CONTAINER" 2>/dev/null || echo unknown)"
    echo "  health: $status"
    if [ "$status" = "healthy" ]; then return 0; fi
    if [ "$status" = "unhealthy" ]; then return 1; fi
    sleep 5
  done
  return 1
}

# Captured, not left to set -e, so a failure still reaches the rollback below.
deploy_ok=1

# Pull first; .env is rewritten only once the image is on the host.
TAG="$IMAGE_TAG" docker compose pull || deploy_ok=0

if [ "$deploy_ok" = 1 ]; then
  # .env holds TAG and nothing else. Secrets live in app.env, untouched.
  printf 'TAG=%s\n' "$IMAGE_TAG" > .env
  docker compose up -d || deploy_ok=0
fi

if [ "$deploy_ok" = 1 ] && wait_healthy; then
  echo "deploy succeeded: $IMAGE_TAG"
  rm -f docker-compose.yml.prev

  # Drop images older than 7 days; the label filter scopes this to our app.
  docker image prune -af \
    --filter "until=168h" \
    --filter "label=$IMAGE_LABEL" || true
  exit 0
fi

# Restore the previous tag and compose file, then fail the run.
echo "deploy failed — rolling back"
docker compose logs --tail 200 "$SERVICE" || true

if [ -f docker-compose.yml.prev ]; then
  mv -f docker-compose.yml.prev docker-compose.yml
fi

if [ -z "$PREV_TAG" ]; then
  echo "no previous tag to roll back to — the stack may be unusable, check by hand"
  exit 1
fi

printf 'TAG=%s\n' "$PREV_TAG" > .env
if docker compose up -d && wait_healthy; then
  echo "rolled back to $PREV_TAG — healthy"
else
  echo "ROLLBACK FAILED — the service is likely down, intervene now"
fi
exit 1
