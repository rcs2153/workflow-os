# Project Approval Route Authority And PostgreSQL Integration Review

## 1. Executive Verdict

Phase accepted with a required live-PostgreSQL merge gate and non-blocking
follow-ups.

The implementation closes the approved authority and durable-state gaps without
turning a stored route into current approval authority. Local deterministic
validation is green. The dedicated `Shared PostgreSQL State` CI job must still
prove live conformance and backup/restore behavior before merge because the
review workstation has no PostgreSQL server/client or container runtime.

## 2. Scope Verification

The phase stayed within its approved boundary. It added canonical hosted
authority derivation, revision-bound authority snapshots, pre-serve durable
high-watermark activation, schema-v2 migration, PostgreSQL route persistence,
authenticated internal composition, tests, runtime/security documentation, and
an implementation report.

It did not add an approval inbox, route-authorized decisions, notifications,
automatic route creation, dynamic identity, enterprise administration, provider
mutation expansion, workflow schemas, CLI behavior, examples, or release
changes.

## 3. Authority Assessment

`HostedCredentialRegistry` is the one hosted source for credential
authentication bindings, the sanitized Core principal registry, and the
revision-bound authority snapshot. Credential digests are excluded from the
Core registry and authority commitment, so credential rotation does not change
authority identity when grants are unchanged.

The deployment must activate the authority snapshot against the durable
organization high watermark before `CollaborativeHostedApiState` can exist.
Exact replay succeeds; rollback and same-revision content changes fail closed.
Route creation requires the exact current revision, commitment algorithm, and
authority-view fingerprint inside the serializable transaction.

## 4. Durable Context Assessment

The Core composer accepts stable lookup subjects rather than caller-authored
route outcomes. It reconstructs an exact pending approval request from durable
events, requires an immutable run-bundle binding, reads the exact bundle,
selects ownership from the frozen workflow definition, requires an active exact
project run binding, and reconstructs the exact escalation event when
escalation routing is requested.

The complete source commitment binds the approval subject, approval-request
event, immutable bundle, project binding, escalation proof when present, route
outcome, and revision-bound authority snapshot. The generic route-store trait
remains explicitly documented as not being an authentication boundary; hosted
callers use the authenticated composer.

## 5. PostgreSQL Assessment

Schema bootstrap metadata is separated from versioned DDL. Fresh installation
and exact v1-to-v2 migration occur under the migration advisory lock and commit
the v2 metadata only with successful DDL. Newer, unknown, checksum-mismatched,
or recovery-required state fails closed.

The route adapter uses a dedicated immutable table rather than replacement
semantics in the generic record table. Creation runs in a bounded serializable
transaction and rechecks current authority, pending approval history, and the
active exact run binding before insert. Exact decision-equivalent retries return
the first canonical record; conflicting content fails closed.

Reads verify deserialization, canonical reserialization, a domain-separated
payload hash, and every duplicated relational index column. Recipient and
approval listings are exact-scope, bounded, deterministic, and fail the whole
operation on corrupt rows.

## 6. Security And Privacy Assessment

No credential, token, provider payload, approval payload, workflow content, or
raw project content is added to route storage. Debug implementations redact
identities and fingerprints. Errors use stable bounded codes and do not echo
database, identity, path, or payload values.

Stored routes remain historical routing evidence. They do not prove that a
recipient still has authority, grant decision permission, record notification
delivery, or bypass the ordinary approval authorization path.

## 7. Test Assessment

Focused tests cover:

- authority revision and snapshot validation and tamper rejection;
- credential rotation without authority-identity drift;
- ordinary and escalation composition from durable facts;
- missing approval events, cross-project or inactive bindings, and authority
  divergence;
- fresh schema installation and exact v1-to-v2 migration;
- authority replay, advance, rollback, and same-revision conflict;
- route create, read, bounded lists, exact reconciliation, and concurrency;
- restart behavior, unresolved routes, relational filtering, and row tampering;
- restored route and authority integrity in the recovery rehearsal.

The non-live workspace suite proves compilation and local model behavior but is
not a substitute for the environment-gated PostgreSQL bodies.

## 8. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.
- Live PostgreSQL conformance: required in CI before merge.
- PostgreSQL backup/restore rehearsal: required in CI before merge.

## 9. Blockers

No implementation blocker was found. A pull request must not merge until the
required live PostgreSQL conformance and recovery jobs pass.

## 10. Non-Blocking Follow-Ups

- Keep the generic `ProjectApprovalRouteStore` clearly documented as storage,
  not an authentication or decision-authority boundary.
- Design dynamic authority refresh and revocation separately before allowing
  registry changes in a live hosted process.
- Add a principal-filtered inbox only as a separately planned read surface with
  an independent current-authority check.
- Continue to treat routed records as historical evidence when later decision
  APIs are designed.

## 11. Recommended Next Phase

After CI acceptance and merge, return to the authoritative runtime-composition
queue. Do not broaden provider mutations merely because routing is durable. The
next phase should continue composing existing governance primitives into
enforced runtime behavior, with scoped authority and proportional-governance
foundations sequenced according to the roadmap.
