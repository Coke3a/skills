#!/usr/bin/env bash

set -euo pipefail

# Customize endpoints for the target project.
# Keep smoke checks read-only and avoid printing secrets.

BASE_URL="${BASE_URL:?BASE_URL is required}"

curl --fail --silent --show-error "${BASE_URL}/healthz"
curl --fail --silent --show-error "${BASE_URL}/readyz"

echo "Smoke test passed for ${BASE_URL}"
