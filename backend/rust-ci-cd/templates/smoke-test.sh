#!/usr/bin/env bash
set -euo pipefail

# Customize endpoints for the project.
# Avoid destructive checks and do not print secrets.

BASE_URL="${BASE_URL:?BASE_URL is required}"

curl --fail --silent --show-error "$BASE_URL/healthz"
curl --fail --silent --show-error "$BASE_URL/readyz"

echo "Smoke test passed for $BASE_URL"
