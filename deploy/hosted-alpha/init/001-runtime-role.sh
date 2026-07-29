#!/bin/sh
set -eu

psql \
  --set=ON_ERROR_STOP=1 \
  --set=runtime_password="$WORKFLOW_OS_HOSTED_DATABASE_PASSWORD" \
  --username "$POSTGRES_USER" \
  --dbname "$POSTGRES_DB" <<'SQL'
CREATE ROLE workflow_os_runtime
  LOGIN
  PASSWORD :'runtime_password'
  NOSUPERUSER
  NOCREATEDB
  NOCREATEROLE
  NOINHERIT;

GRANT CONNECT, CREATE, TEMPORARY
  ON DATABASE workflow_os
  TO workflow_os_runtime;

GRANT USAGE, CREATE
  ON SCHEMA public
  TO workflow_os_runtime;
SQL
