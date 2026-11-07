# Local Fixture Replacement Authority Review

Initial verdict: **not accepted for delivery**. See the correction and
verification below.

## Findings

1. **P1: Staged activation does not verify that new authority tables are empty.**
   The filesystem migration does not import fixture grants or registrations,
   but `destination_content_digest` omits both tables. Activation checks the
   schema manifest and existing digest, then marks the database ready without
   rejecting authority rows added after verification. In a disposable V10
   migration, a test registration was inserted after receipt creation. The
   unchanged receipt activated the database, the health check returned healthy,
   and the added row survived (`ready|1`). This is an activation integrity
   failure, not proof of a successful delegated continuation or a provider call.
   Verify that both tables are empty inside the activation transaction and
   when resuming an existing staged import. Do not rewrite the receipt or
   silently discard unexpected authority.
2. **P2: An exact V9 verified-inactive destination cannot finish activation.**
   The V10 activator only accepts the V10 manifest. The explicit V9 upgrade
   requires `ready`, so it cannot upgrade a verified-inactive destination.
   A V10 control activated with its exact receipt. A copy with the exact V9
   schema manifest and V9 metadata, but identical staged content and receipt,
   returned `state.migration.activation.schema_invalid` and stayed
   `9|verified_inactive`. The review constructed the V9 schema in a disposable
   copy; it did not claim an old-binary migration rerun. Preserve verification
   and activation for the exact V9 staged contract, or provide an explicit,
   tested recovery path that never silently rewrites the receipt. Provide a
   documented operator path for arbitrary ready V9 destinations as well as the
   dedicated fixture database.

## Accepted Boundaries And Next Check

The prior test evidence for exact fixture validation, principal and harness
binding, and final-snapshot revocation is useful. The new CLI does not take an
actor override or credential in argv, and the legacy fixture-only path remains
separate. The local operator and writable storage remain trusted; this is not
an OS isolation or vendor-account authentication claim. The Claude Desktop tool
surface and independent corpus have not been exercised.

## Correction

Filesystem staging imports no fixture authority. Staging and activation reject
any principal or grant rows. Activation now verifies the exact persisted receipt,
versioned schema, imported content, and authority emptiness in one write
transaction. It does not discard the unexpected rows or rewrite the receipt.

An exact V9 verified-inactive destination now verifies against its original
manifest and receipt, then upgrades and activates atomically. A ready V9
destination can be upgraded explicitly with `state upgrade-sqlite --database
<path>`. An absent path is not created, and an inactive destination still
requires the receipt-bearing activation command.

The cross-binary verification built `d1db7a3c` and used it to produce actual V9
inactive and ready destinations. The fixed CLI activated the original inactive
receipt and explicitly upgraded the ready database. Both preserved all 11
events and had zero principal or grant rows; the external and stored activation
receipt hashes were unchanged.

The migration runtime and SQLite backend suites passed all 23 tests, including
separate post-receipt principal and grant insertion cases. The 12 selected CLI
state tests passed, including the explicit upgrade command. Warnings-denied
Clippy, formatting, docs, and whitespace checks passed. This resolves the two
migration findings for delivery; it is not independent app-corpus acceptance.

Correction governance: `dg/blocker`, `run-1794038833836412000-2`, delegated
approval with persisted presentation proof. Build, tests, and scratch database
operations ran outside the kernel. No app setup or provider access occurred.

Review governance: `dg/review`, `run-1794038668984781000-2`, delegated
approval with persisted presentation proof. Source inspection and disposable
CLI/SQLite reproduction ran outside the kernel; no runtime code was changed.
