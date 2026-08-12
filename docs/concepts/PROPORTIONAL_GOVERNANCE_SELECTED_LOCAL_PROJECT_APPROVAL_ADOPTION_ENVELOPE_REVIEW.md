# Proportional-Governance Selected Local Project Approval Adoption Envelope Review

## 1. Executive Verdict

Phase accepted; proceed to combined selected CLI `run` and `approve` adoption.

The Core envelope correctly binds the selected project-validation path to
proof-enforced approval decisions, derives approval subject and artifact policy
from durable state, reruns the canonical check only for grants, and preserves
the distinction between transient aggregate authority and durable terminal
closure. No blocker remains before the separately scoped CLI cutover.

## 2. Scope Verification

The phase stayed within its approved Core-only boundary. It added an explicit
in-memory/store-injected envelope, bounded result vocabulary, capability-aware
validation plumbing, focused tests, and documentation.

It did not change CLI commands or output, workflow schemas, SDKs, provider or
SideEffect execution, automatic approval behavior, runtime configuration,
hosted behavior, examples, or release posture. Existing public executor and
approval APIs continue to use the default project-validation capability.

## 3. Approval Subject Assessment

Core derives `AggregateGovernance` versus `AuthoredWorkflowStep` from the
durable approval request after validating its subject. The caller cannot select
the gate kind. Approval-presentation proof is preflighted before a current-fact
source or local check can run.

An aggregate grant that advances to an authored gate returns its authority
receipt only in memory. It does not persist the receipt, project approval-proof
markers, generate a WorkReport, or write an artifact while the run remains
non-terminal. The later authored approval remains distinct and independently
proof-enforced.

## 4. Grant And Denial Assessment

A grant executes the canonical selected project-validation check exactly once
at decision time and constructs the report citation from the Core-produced
result identifier. The caller-authored placeholder result identifier is not
trusted. Current-fact reassessment remains bound to the immutable run bundle
and selected workflow identity.

A denial uses no current-fact source, does not rerun the local check, invokes no
workflow skill, and creates no authority receipt. Aggregate and authored
denials both close the run truthfully and can produce a report artifact from
the denial event trail without fabricating successful check evidence.

## 5. Capability And Immutable-Input Assessment

`ProjectValidationCapability::ReportArtifactCapable` is carried only through
the selected envelope's project load, resume planning, and immutable-bundle
reconstruction. It is bound to the run workflow and enables only the already
reviewed report-artifact/proof-marker declarations. Existing paths remain on
`ProjectValidationCapability::Default`.

The exact workflow is recovered from the stored immutable run bundle. Bundle
binding, workflow ID, workflow version, schema version, and source content hash
must match the durable run identity before artifact policy can be derived.

## 6. Report And Artifact Assessment

Terminal closure derives high-assurance disclosure and approval-proof-marker
requirements from the immutable workflow rather than caller-selected booleans.
Required proof-marker projections are persisted from the actual approval event
trail before the existing governed artifact writer evaluates them.

Report or artifact failure is retained as bounded result posture after the
approval decision; it does not rewrite the workflow's terminal decision.
Duplicate artifact writes reconcile only when the existing artifact is exactly
equal. Missing or unreadable outcomes become ambiguous, block retry, and do not
claim successful persistence.

A granted terminal authority receipt is persisted and checked for
receipt-to-artifact integrity. Denials remain receipt-free. Persisting a valid
grant receipt before later artifact closure is appropriate because the receipt
records the decision authority, not artifact-write success.

## 7. Error, Privacy, And Debug Assessment

Input Debug output redacts approval, execution, report, and local-check
reference fields. Result Debug output exposes only bounded posture, presence
booleans, run status, and stable error codes. Raw command output, report text,
paths, runtime facts, provider payloads, credentials, tokens, and caller
reference values are not surfaced.

Pre-decision proof, identity, immutable-bundle, current-check, and reassessment
failures return stable errors. Post-decision report and artifact failures are
captured without leaking raw values or pretending the workflow decision did
not occur.

## 8. Compatibility Assessment

The API is additive and exported consistently with existing Core executor
vocabulary. It does not alter `LocalExecutor::execute`, existing approval
helpers, existing report adapters, or CLI behavior. The capability-aware
private path preserves default-capability behavior for every existing caller.

This is the correct compatibility boundary before `run` and `approve` adopt
the selected envelope together. Adopting only one command would risk different
governance semantics across the same waiting run and remains disallowed by the
plan.

## 9. Test Quality Assessment

Focused tests prove:

- aggregate grant advances to a distinct authored approval;
- non-terminal receipt, report, projection, and artifact posture is truthful;
- the decision-time citation uses the Core-produced check result ID;
- authored grant closes with proof-marker projection and artifact gates;
- aggregate and authored denials do not rerun checks or invoke skills;
- denials remain receipt-free while producing truthful artifacts; and
- input and result Debug output do not expose secret-like reference content.

The complete workspace suite passed, including 373 local-executor tests, 227
WorkReport tests, 149 provider-write tests, 216 Core unit tests, hosted
OpenShell tests, immutable-run, approval-presentation, SideEffect, receipt,
projection, artifact, and compatibility coverage.

## 10. Documentation Assessment

The implementation report, adoption plan, and roadmap accurately state that
the envelope is implemented, CLI behavior is unchanged, and focused review is
the current gate. They do not claim provider execution, automatic approval,
or release readiness.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Remove or replace the vestigial caller-authored `result_id` field from the
  selected envelope input when the CLI request shape is finalized. Core
  correctly ignores it today, but the unused field is avoidable ambiguity.
- Add direct selected-envelope tests for high-assurance denial and exact
  duplicate-artifact reconciliation when this surface next changes. Existing
  helper and workspace coverage make these non-blocking.
- Preserve projection-before-artifact ordering and disclose partial closure if
  an artifact write fails after a truthful approval projection is persisted.

## 13. Recommended Next Phase

Implement the already-planned combined selected CLI `run` and `approve`
adoption. Both commands should use the reviewed Core route and approval
envelope in one compatibility-sensitive phase. Do not broaden provider,
schema, automation, hosted, or release behavior.

## 14. Governed Review Record

- Workflow: `dg/review`
- Run: `run-1786510272864886000-2`
- Approval: `approval/run-1786510272864886000-2/review-scope-approved`
- Presentation: `presentation/ffbc286f6c0eeb1b`
- Approval outcome: granted by delegated maintainer with persisted
  presentation proof
- Presentation content hash:
  `ffbc286f6c0eeb1b16cae9c5bb70f02b6b4d587ef147b7c5c80e2cf36895a24b`
- Phase status: completed
- Event summary: 39 events, one approval request, one approval grant, eight
  policy decisions, six scheduled steps, six successful skill invocations, no
  retries, and no escalations
- Approval-presentation enforcement: proof enforced with one persisted
  presentation record and a present event marker

## 15. Validation

The exact reviewed implementation passed:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`; and
- `git diff --check`.

The implementation commit is `7b90304`.

## 16. Out-Of-Kernel Disclosure

The kernel governed review scope, approval, and durable event history. Codex
inspected the implementation, tests, reports, and compatibility boundary;
formed the maintainer verdict; edited this review artifact and roadmap status;
and ran repository validation outside the kernel. No Workflow OS runtime state
was edited by hand.
