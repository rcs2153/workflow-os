#!/usr/bin/env bash

set -euo pipefail
set +x

: "${WORKFLOW_OS_HOSTED_DATABASE_ADMIN_PASSWORD:?database admin password is required}"
: "${WORKFLOW_OS_HOSTED_DATABASE_PASSWORD:?runtime database password is required}"
: "${WORKFLOW_OS_HOSTED_TOKEN:?hosted API token is required}"

for command_name in curl docker; do
  if ! command -v "${command_name}" >/dev/null 2>&1; then
    printf 'hosted alpha rehearsal dependency is unavailable: %s\n' "${command_name}" >&2
    exit 1
  fi
done

compose_file="deploy/hosted-alpha/compose.yml"
api_base_url="${WORKFLOW_OS_HOSTED_API_URL:-http://127.0.0.1:8080}"

compose() {
  docker compose --file "${compose_file}" "$@"
}

authenticated_get() {
  local url="$1"
  printf 'Authorization: Bearer %s\n' "${WORKFLOW_OS_HOSTED_TOKEN}" |
    curl \
      --fail \
      --silent \
      --show-error \
      --header @- \
      "${url}" >/dev/null
}

wait_for_api() {
  local attempts=0
  until authenticated_get "${api_base_url}/health/ready"; do
    attempts=$((attempts + 1))
    if [ "${attempts}" -ge 60 ]; then
      printf 'hosted alpha API did not become ready\n' >&2
      return 1
    fi
    sleep 1
  done
}

inspect_authenticated_surface() {
  authenticated_get "${api_base_url}/version"
  authenticated_get "${api_base_url}/api/v0alpha1/metrics"
}

compose up --detach --build
wait_for_api
inspect_authenticated_surface

compose restart api
wait_for_api
inspect_authenticated_surface

compose restart worker
wait_for_api
inspect_authenticated_surface

printf '%s\n' \
  'Hosted alpha API/worker restart and authenticated inspection rehearsal passed.' \
  'The compose topology remains running for operator inspection.' \
  "Stop it explicitly with: docker compose --file ${compose_file} down"
