#!/usr/bin/env bash

set -euo pipefail
set +x

: "${WORKFLOW_OS_POSTGRES_SOURCE_URL:?source PostgreSQL URL is required}"
: "${WORKFLOW_OS_POSTGRES_RESTORE_ADMIN_URL:?restore admin PostgreSQL URL is required}"
: "${WORKFLOW_OS_POSTGRES_RESTORE_URL:?restore PostgreSQL URL is required}"

for command_name in pg_dump pg_restore psql cargo; do
  if ! command -v "${command_name}" >/dev/null 2>&1; then
    printf 'recovery rehearsal dependency is unavailable: %s\n' "${command_name}" >&2
    exit 1
  fi
done

dump_file="$(mktemp "${TMPDIR:-/tmp}/workflow-os-postgresql-recovery.XXXXXX")"
trap 'rm -f "${dump_file}"' EXIT
chmod 600 "${dump_file}"

pg_dump \
  --format=custom \
  --no-owner \
  --no-acl \
  --file="${dump_file}" \
  "${WORKFLOW_OS_POSTGRES_SOURCE_URL}"

psql \
  "${WORKFLOW_OS_POSTGRES_RESTORE_ADMIN_URL}" \
  --set=ON_ERROR_STOP=1 \
  --command='DROP DATABASE IF EXISTS workflow_os_restore WITH (FORCE)'
psql \
  "${WORKFLOW_OS_POSTGRES_RESTORE_ADMIN_URL}" \
  --set=ON_ERROR_STOP=1 \
  --command='CREATE DATABASE workflow_os_restore'

pg_restore \
  --no-owner \
  --no-acl \
  --exit-on-error \
  --dbname="${WORKFLOW_OS_POSTGRES_RESTORE_URL}" \
  "${dump_file}"

WORKFLOW_OS_RECOVERY_POSTGRES_URL="${WORKFLOW_OS_POSTGRES_RESTORE_URL}" \
  cargo test \
  -p workflow-core \
  --test postgres_state_backend \
  restored_postgresql_database_passes_integrity_rehearsal \
  -- \
  --exact

printf 'PostgreSQL state backup, restore, and integrity rehearsal passed.\n'
