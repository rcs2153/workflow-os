#!/usr/bin/env bash

set -euo pipefail
set +x

: "${WORKFLOW_OS_HOSTED_DATABASE_ADMIN_PASSWORD:?database admin password is required}"
: "${WORKFLOW_OS_HOSTED_DATABASE_PASSWORD:?runtime database password is required}"
: "${WORKFLOW_OS_HOSTED_TOKEN:?hosted API token is required}"

for command_name in curl docker jq; do
  if ! command -v "${command_name}" >/dev/null 2>&1; then
    printf 'hosted alpha rehearsal dependency is unavailable: %s\n' "${command_name}" >&2
    exit 1
  fi
done

compose_file="deploy/hosted-alpha/compose.yml"
api_base_url="${WORKFLOW_OS_HOSTED_API_URL:-http://127.0.0.1:8080}"
compose_project="${WORKFLOW_OS_HOSTED_COMPOSE_PROJECT:-workflow-os-hosted-recovery}"
run_suffix="${WORKFLOW_OS_HOSTED_REHEARSAL_RUN_SUFFIX:-$(date -u +%Y%m%d%H%M%S)}"
run_id="run-hosted-recovery-${run_suffix}"
bundle_id="bundle-hosted-recovery-${run_suffix}"
bundle_version="v1"
correlation_id="correlation-hosted-recovery-${run_suffix}"
idempotency_key="hosted-recovery-create-${run_suffix}"
created_at="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
response_file="$(mktemp)"

cleanup() {
  rm -f "${response_file}"
  if [ "${WORKFLOW_OS_HOSTED_CLEANUP:-0}" = "1" ]; then
    compose down --volumes
  fi
}

trap cleanup EXIT

compose() {
  docker compose --project-name "${compose_project}" --file "${compose_file}" "$@"
}

authenticated_get() {
  local url="$1"
  printf 'Authorization: Bearer %s\n' "${WORKFLOW_OS_HOSTED_TOKEN}" |
    curl \
      --fail-with-body \
      --silent \
      --show-error \
      --header @- \
      "${url}"
}

authenticated_post_json() {
  local url="$1"
  local body="$2"
  printf 'Authorization: Bearer %s\n' "${WORKFLOW_OS_HOSTED_TOKEN}" |
    curl \
      --fail-with-body \
      --silent \
      --show-error \
      --header @- \
      --header 'Content-Type: application/json' \
      --request POST \
      --data "${body}" \
      "${url}"
}

wait_for_api() {
  local attempts=0
  until authenticated_get "${api_base_url}/health/ready" >/dev/null; do
    attempts=$((attempts + 1))
    if [ "${attempts}" -ge 60 ]; then
      printf 'hosted alpha API did not become ready\n' >&2
      compose ps >&2
      compose logs --no-color --tail 200 api postgres >&2
      return 1
    fi
    sleep 1
  done
}

inspect_authenticated_surface() {
  printf 'hosted_rehearsal_step: inspect_version\n'
  authenticated_get "${api_base_url}/version" >/dev/null
  printf 'hosted_rehearsal_step: inspect_metrics\n'
  authenticated_get "${api_base_url}/api/v0alpha1/metrics" >/dev/null
}

read_run() {
  authenticated_get "${api_base_url}/api/v0alpha1/runs/${run_id}"
}

wait_for_run_status() {
  local expected_status="$1"
  local attempts=0
  local actual_status=""
  until [ "${actual_status}" = "${expected_status}" ]; do
    read_run >"${response_file}"
    actual_status="$(jq --raw-output '.snapshot.status' "${response_file}")"
    attempts=$((attempts + 1))
    if [ "${attempts}" -ge 60 ]; then
      printf 'hosted alpha run did not reach expected status: %s\n' "${expected_status}" >&2
      return 1
    fi
    sleep 1
  done
}

run_request="$(
  jq \
    --null-input \
    --compact-output \
    --arg run_id "${run_id}" \
    --arg workflow_id "hosted/recovery-proof" \
    --arg bundle_id "${bundle_id}" \
    --arg bundle_version "${bundle_version}" \
    --arg created_at "${created_at}" \
    --arg correlation_id "${correlation_id}" \
    --arg idempotency_key "${idempotency_key}" \
    '{
      run_id: $run_id,
      workflow_id: $workflow_id,
      bundle_id: $bundle_id,
      bundle_version: $bundle_version,
      created_at: $created_at,
      correlation_id: $correlation_id,
      idempotency_key: $idempotency_key,
      sensitivity: "internal",
      redaction_required: true
    }'
)"

compose up --detach --build postgres api
printf 'hosted_rehearsal_step: wait_for_initial_api\n'
wait_for_api
inspect_authenticated_surface

printf 'hosted_rehearsal_step: create_run\n'
if ! authenticated_post_json "${api_base_url}/api/v0alpha1/runs" "${run_request}" >"${response_file}"; then
  printf 'hosted_api_error_code: %s\n' \
    "$(jq --raw-output '.code // "hosted.unknown"' "${response_file}")" >&2
  exit 1
fi
if [ "$(jq --raw-output '.snapshot.identity.run_id' "${response_file}")" != "${run_id}" ]; then
  printf 'hosted alpha API returned an unexpected run identity\n' >&2
  exit 1
fi
if [ "$(jq --raw-output '.snapshot.status' "${response_file}")" != "running" ]; then
  printf 'hosted alpha run was not queued in running posture\n' >&2
  exit 1
fi

printf 'hosted_rehearsal_step: restart_api\n'
compose restart api
wait_for_api
inspect_authenticated_surface
read_run >"${response_file}"
if [ "$(jq --raw-output '.snapshot.status' "${response_file}")" != "running" ]; then
  printf 'hosted alpha run did not survive API restart\n' >&2
  exit 1
fi

printf 'hosted_rehearsal_step: start_worker\n'
compose up --detach worker
wait_for_run_status completed
printf 'hosted_rehearsal_step: inspect_terminal_run\n'
authenticated_get "${api_base_url}/api/v0alpha1/runs/${run_id}/events?limit=50" >"${response_file}"
if [ "$(jq --raw-output '.events[-1].kind.kind' "${response_file}")" != "RunCompleted" ]; then
  printf 'hosted alpha terminal event trail is incomplete\n' >&2
  exit 1
fi
authenticated_get "${api_base_url}/api/v0alpha1/runs/${run_id}/report" >"${response_file}"
if [ "$(jq --raw-output '.run_id' "${response_file}")" != "${run_id}" ]; then
  printf 'hosted alpha terminal report metadata has an unexpected run identity\n' >&2
  exit 1
fi
report_id="$(jq --raw-output '.report_id' "${response_file}")"
if [ -z "${report_id}" ] || [ "${report_id}" = "null" ]; then
  printf 'hosted alpha terminal report metadata is missing report identity\n' >&2
  exit 1
fi

printf 'hosted_rehearsal_step: restart_worker\n'
compose restart worker
wait_for_api
inspect_authenticated_surface
wait_for_run_status completed

printf 'hosted_rehearsal_step: interrupt_database\n'
compose stop postgres
if authenticated_get "${api_base_url}/health/ready" >/dev/null 2>&1; then
  printf 'hosted alpha readiness stayed healthy while PostgreSQL was unavailable\n' >&2
  exit 1
fi
compose start postgres
printf 'hosted_rehearsal_step: recover_database\n'
wait_for_api
wait_for_run_status completed
authenticated_get "${api_base_url}/api/v0alpha1/runs/${run_id}/reports/${report_id}" >"${response_file}"
if [ "$(jq --raw-output '.run_id' "${response_file}")" != "${run_id}" ]; then
  printf 'hosted alpha terminal report did not survive database interruption\n' >&2
  exit 1
fi

printf '%s\n' \
  'Hosted alpha governed run, API/worker restart, and database interruption rehearsal passed.' \
  "run_id: ${run_id}" \
  "report_id: ${report_id}" \
  'The compose topology remains running for operator inspection.' \
  "Stop it explicitly with: docker compose --project-name ${compose_project} --file ${compose_file} down"
