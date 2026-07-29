# PostgreSQL State Recovery

This runbook describes the bounded logical backup/restore rehearsal for the
explicit Workflow OS `PostgreSQL` shared-state preview.

It is a correctness rehearsal, not a production disaster-recovery system,
high-availability design, point-in-time recovery guarantee, or hosted
operations claim.

## Preconditions

The operator supplies three connection references through the environment:

- `WORKFLOW_OS_POSTGRES_SOURCE_URL`: initialized source database;
- `WORKFLOW_OS_POSTGRES_RESTORE_ADMIN_URL`: administration database able to
  recreate the isolated rehearsal destination;
- `WORKFLOW_OS_POSTGRES_RESTORE_URL`: fixed isolated destination named
  `workflow_os_restore`.

The script also requires `pg_dump`, `pg_restore`, `psql`, and `cargo`.
Connection references are caller-owned secrets. The script disables shell
tracing and never prints them intentionally. Database/server logs remain an
operator responsibility.

Do not point the restore variables at a database containing retained data. The
rehearsal drops and recreates `workflow_os_restore`.

## Rehearsal

Run:

```sh
bash scripts/rehearse-postgresql-state-recovery.sh
```

The script:

1. creates a mode-`0600` temporary custom-format logical dump;
2. drops and recreates the isolated restore database;
3. restores without ownership or ACL transfer;
4. runs the exact restored-state integrity test;
5. verifies managed schema health;
6. plans and performs deterministic projection rebuild;
7. verifies a restored immutable run bundle;
8. removes the temporary dump on exit.

Any command failure stops the rehearsal. The script does not activate a
destination, change runtime selection, mutate the source database, or repair
corrupt authority.

## Recovery-Required Posture

Managed schema metadata includes a `recovery_required` flag. Normal
initialization and health checks fail closed when this flag is set. An operator
must diagnose and complete or reverse the interrupted schema operation before
ordinary use resumes. Workflow OS does not silently clear this posture.

Checksum mismatch, a newer schema version, invalid canonical records, identity
mismatch, projection-rebuild failure, and missing immutable references also
fail closed with stable errors that omit connection data and stored payloads.

## CI Boundary

Repository CI runs the shared-state conformance test against the official
`PostgreSQL` 17 service image, then executes this rehearsal against a separate
restore database.

Passing CI proves the reviewed schema and sample authority can be backed up,
restored, rehydrated, and projection-rebuilt. It does not prove:

- production recovery time or recovery point objectives;
- physical backup or point-in-time recovery;
- replication, failover, or cross-region behavior;
- compatibility with every managed `PostgreSQL` provider;
- production credential, TLS, pooling, or retention configuration.
