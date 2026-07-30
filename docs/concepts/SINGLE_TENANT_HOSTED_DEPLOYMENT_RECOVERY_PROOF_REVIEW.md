# Single-Tenant Hosted Deployment And Recovery Proof Review

Review date: 2026-07-29

## 1. Executive Verdict

**Single-tenant no-write hosted evaluation milestone accepted; return to
runtime proportional-governance composition.**

The implementation closes the deployed recovery and terminal report gap
without expanding provider authority. A real server-owned governed run now
survives API restart, completes through a separately running worker, persists
its report atomically with terminal state, and remains inspectable after a
database interruption.

## 2. Scope Verification

The phase stayed within the approved deployment/recovery proof. It did not add
provider writes, credentials, OpenShell, another provider, multi-tenancy,
enterprise identity, hosted UI, schemas, CLI behavior, examples, HA, PITR, or
production claims.

The hosted project under `deploy/hosted-alpha/project` is an internal
deployment fixture, not a public workflow example or workflow-schema change.

## 3. Runtime Integration Assessment

The API creates a run from the server-controlled project root and immutable
bundle. The worker remains stateless and claims only the Core-created hosted
work item under the accepted database-time lease and fence.

The recovery rehearsal starts the worker only after the API restart check,
which proves that API process memory is not required to preserve queued work.
Worker restart occurs after completion and cannot duplicate terminal events.
Database interruption makes readiness unhealthy and recovered state is read
from `PostgreSQL`, not reconstructed from process memory.

## 4. Terminal Report Assessment

The report helper accepts only completed, exactly bound, no-write projections.
It reuses the existing validated report generator and artifact constructor.
It cites terminal workflow events plus stable payload-free hosted execution
references and does not recreate `EvidenceReference` values.

Report persistence occurs inside the same serializable fenced transaction as
the receipt and terminal run projection. This avoids a post-terminal crash gap.
Exact replay requires the identical report artifact; substitution fails
closed.

The scope is intentionally asymmetric: only successfully receipted no-write
runs receive a report. Failed, canceled, pre-start-rejected, ambiguous, and
reconciliation-required report semantics remain deferred rather than
fabricated.

## 5. Recovery And Fencing Assessment

The deployed rehearsal and live database suite form one complementary proof:

- compose rehearsal: API restart, queued-run survival, worker completion,
  worker restart, dependency-readiness failure, database process recovery, and
  terminal report readability;
- live `PostgreSQL` suite: lease expiry/takeover, monotonic fencing,
  stale-fence rejection, schema mismatch closure, logical backup/restore,
  projection rebuild, and immutable-bundle readability.

The implementation does not translate these results into HA, PITR, RTO, RPO,
capacity, or production disaster-recovery claims.

## 6. Security And Privacy Assessment

The provider remains no-write, credential-free, payload-free, and unable to
run shell, filesystem, process, network, browser, or model work. The report
contains stable references rather than raw output. The compose API remains
host-bound, and CI-only credentials are not emitted by the rehearsal.

Static bearer authentication, shared API/worker database identity, absent TLS
termination, and absent access-material resolution remain explicit evaluation
limitations.

## 7. Test Quality Assessment

Focused tests cover report derivation, citation shape, ambiguous-outcome
rejection, atomic storage, exact replay, conflicting replay, authentication,
and existing hosted transaction compatibility.

The dedicated hosted CI job is material rather than cosmetic: it builds the
real evaluation image and runs the API, worker, and `PostgreSQL` as separate
processes. The existing shared-state job continues to run the deeper database
recovery and concurrency proof.

Local container execution was unavailable because Docker is not installed on
the development machine. This is disclosed, and merge requires the Linux CI
rehearsal rather than substituting a mock.

## 8. Documentation Assessment

The roadmap, hosted plan, runtime guide, threat model, phase report, and this
review now state:

- the single-tenant no-write evaluation milestone is implemented;
- terminal report persistence is atomic with receipt projection;
- deployed restart and database-interruption recovery are proved in CI;
- production identity, credentials, HA, PITR, multi-tenancy, OpenShell, and
  writes remain unsupported;
- the result is not production ready.

## 9. Blockers

No blocker remains inside the single-tenant no-write evaluation milestone.

The following remain blockers to production or provider mutation, not to this
evaluation acceptance:

1. production-suitable identity and time-of-use authority;
2. scoped access-material isolation and resolution;
3. separate least-privilege service identities;
4. TLS and reviewed network policy;
5. mutation-specific idempotency, reconciliation, and threat proof;
6. HA, backup retention, recovery objectives, capacity, and operations proof.

## 10. Non-Blocking Follow-Ups

- Add failed/canceled/reconciliation report semantics as a separately reviewed
  report-contract expansion.
- Add fault injection between provider return and transaction commit.
- Move bounded event pagination into the database query.
- Separate API and worker database roles.
- Make container image identity immutable for later rollback rehearsal rather
  than relying on a mutable evaluation tag.

## 11. Product Feedback Assessment

Fresh-pull review confirms that Workflow OS is already coherent and honest as
a local governance kernel. Its next product pressure is reducing ceremony for
low-risk work without losing evidence.

That feedback aligns with the accepted proportional-governance and
quiet-success roadmap. It does not justify weakening fail-closed execution,
skipping immutable/fenced hosted proof, or pretending the kernel is a turnkey
agent runtime.

## 12. OpenShell Assessment

OpenShell is architecturally aligned as a future optional execution-provider
adapter. Workflow OS should remain the source of governed intent, authority,
approval, evidence obligations, SideEffect posture, and reports; OpenShell
would enforce runtime containment and return bounded receipts.

A fork is not justified. It would transfer container lifecycle, filesystem and
network isolation, credential injection, platform support, CVE response, and
runtime hardening into Workflow OS. Reconsider only if upstream cannot expose
required effective-policy identity, enforcement/degradation posture, durable
events, receipt binding, artifact manifests, and reconciliation hooks.

## 13. Recommended Next Phase

Proceed with **runtime proportional-governance composition**, followed by the
accepted scoped runtime authority and capability projection sequence.

Use the hosted alpha as an integration target, but do not broaden provider
mutations or add OpenShell before those governance decisions are enforced at
time of use.
