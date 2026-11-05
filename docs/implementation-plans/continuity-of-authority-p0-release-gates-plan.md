# Continuity Of Authority P0 Release Gates Plan

Status: Accepted P0 product-direction and release-gate plan. All three gates are
accepted from deterministic current-main reproductions. No gate is rejected or
deferred.

Related work:

- [Current Product Contract](../user-guide/current-product-contract.md)
- [Evidence Provenance And Report Integrity P0 Plan](evidence-provenance-and-report-integrity-p0-plan.md)
- [Authoritative Agent Continuation Context And Rehydration Plan](authoritative-agent-continuation-context-rehydration-plan.md)
- [Compact Recovery Checkpoint Evaluation Plan](compact-recovery-checkpoint-evaluation-plan.md)
- [Roadmap](../../ROADMAP.md)

## 1. Product Thesis

Workflow OS's differentiated value is continuity of authority across people,
agents, processes, retries, recovery, and interruptions.

```text
The kernel preserves what was authorized, what executed, which evidence and
reports belong to the work, and what remains lawful.
```

This is stronger than durable state alone. Durable but incompletely bound state
can preserve a stranded run, accept a report from another run, or continue work
whose project authorization was removed. Continuity requires exact binding and
truthful transitions across the complete lifecycle.

Expanded evaluation supports the broader thesis while preserving product
limits:

- twelve stages required twelve distinct approvals;
- eleven attempts to reuse earlier approvals were rejected;
- denial at gate seven stopped after six executions;
- eight competing hosts entered execution once;
- host loss produced an explicit blocked state;
- PostgreSQL backup and recovery preserved state; and
- under actual Codex compaction, a compact kernel checkpoint reduced gross
  recovery tokens from 167,470 to 83,857 across three paired scenarios while
  both conditions made all nine decisions correctly.

The 49.93% result is bounded experimental evidence, not a universal token,
billing, latency, or completion-cost claim. The correctness gates in this plan
take precedence over efficiency positioning.

## 2. Gate Dispositions

### 2.1 Accepted Means Dispatched

Disposition: **accepted**.

Owner: hosted runtime and PostgreSQL hosted-state composition.

Reproduction on current main:

1. reject only the `hosted_work_item` insert after the run prefix persists;
2. observe HTTP `409`, a `Running` snapshot, and five events through
   `PolicyDecisionRecorded`;
3. remove the fault and retry the exact request;
4. observe HTTP `201` on retries, zero queued work records, and an idle worker.

Violated invariant:

```text
A hosted create or exact retry reports success only when the exact run is
approval-waiting, durably dispatched, terminal, or explicitly recovery-blocked.
```

Blocked product claims:

- reliable unattended hosted completion;
- truthful hosted submission acknowledgment;
- automatic recovery from partial pre-dispatch failure; and
- general hosted-runtime readiness.

### 2.2 Verified Means Correctly Bound

Disposition: **accepted**.

Owner: Core WorkReport artifact identity and every filesystem, SQLite,
PostgreSQL, transaction-local, inspect, recovery, reconciliation, and replay
consumer.

Reproduction on current main:

1. create valid completed report artifacts for runs A and B;
2. place run B's complete unmodified artifact in run A's filesystem report slot;
3. inspect run A and observe run B's report and run identity with verified body-
   commitment posture;
4. replay run A and observe `already_persisted` for run B's artifact.

Violated invariant:

```text
A verified artifact must match the exact requested storage key, run, workflow,
report identity, and immutable execution context before projection or replay.
```

Blocked product claims:

- sole enterprise audit authority;
- complete report-integrity enforcement;
- trustworthy cross-backend restore without identity validation; and
- bounded replay correctness for persisted reports.

### 2.3 Approved Means Still Authorized

Disposition: **accepted**.

Owner: project provider-resource restriction, immutable-run binding, approval-
resume reassessment, and provider-admission composition.

Reproduction on current main:

1. start a repository-metadata fixture run for `acme/widgets`;
2. persist the approval presentation;
3. change only the project resource restriction to `acme/replacement`;
4. grant the original approval and resume;
5. observe successful completion against `acme/widgets`.

Violated invariant:

```text
Every project-level restriction presented as a condition of approval remains
committed and valid at approval acceptance and immediately before provider
admission.
```

Blocked product claims:

- fully committed repository-resource approval;
- unattended live-provider adoption;
- current-project allowlist enforcement across approval waits; and
- approval presentation statements not backed by exact revalidation.

## 3. Gate 1 Implementation: Hosted Retry Reconciliation

Implementation status: **closed on exact main `61de8882` after governed
implementation and review, exact-head pull-request validation, and an
independent exact-main rerun.** See the
[Hosted Accepted Means Dispatched Implementation
Report](../concepts/HOSTED_ACCEPTED_MEANS_DISPATCHED_IMPLEMENTATION_REPORT.md)
and [focused
review](../concepts/HOSTED_ACCEPTED_MEANS_DISPATCHED_IMPLEMENTATION_REVIEW.md).

Trace hosted submission from request identity through run-prefix persistence,
approval-wait posture, queue insertion, dispatch event projection, worker claim,
terminal state, and exact retry.

The create/retry result must derive one closed disposition:

- `approval_waiting`: the exact run is waiting on a valid approval;
- `dispatched`: the exact hosted work item and dispatch proof exist;
- `terminal`: the exact run is durably terminal;
- `recovery_blocked`: the run records a typed bounded recovery condition; or
- `not_acknowledgeable`: success is forbidden.

An existing nonempty event stream is not sufficient evidence of dispatch. The
fix may atomically repair missing dispatch when all exact immutable bindings are
available and repair is provably idempotent. Otherwise it must return a bounded
recovery-required result and preserve enough state for explicit reconciliation.
It must never fabricate queue state or silently restart potentially dispatched
work.

Required tests:

- fault injection after run-prefix commit and before work-item insert;
- exact retry after pre-dispatch failure;
- concurrent exact submissions with one durable queue outcome;
- changed request or immutable binding rejection;
- approval-wait and terminal retry success;
- queue item without matching run identity rejection;
- Running without approval, queue, active invocation, recovery, or terminal
  posture fails closed; and
- worker and API reconciliation after process restart.

Gate 1 acceptance:

- no `2xx` response exists without one accepted disposition;
- exact retry is idempotent and never strands acknowledged work;
- concurrent submission produces one dispatch or one explicit recovery posture;
- hosted-alpha documentation states the repaired and still-limited boundary;
- exact-head hosted and PostgreSQL CI pass; and
- an independent fault-injection rerun passes.

Gate 1 acceptance evidence:

- PR [#739](https://github.com/rcs2153/workflow-os/pull/739) passed all nine
  required checks on exact reviewed head
  `c2f3640d3035482d8451b4fc5c359dc070e76149`;
- squash-merged main is
  `61de8882f8b8dd2a2e2bbb81f5668ece581053b9`;
- independent exact-main Actions run `42019866185` passed all nine jobs,
  including the Rust scheduling regression, live PostgreSQL trigger and
  concurrency coverage, and Hosted Alpha Recovery; and
- this evidence closes Gate 1 only. Gate 2, **Verified Means Correctly Bound**,
  is the next authorized implementation lane.

## 4. Gate 2 Implementation: WorkReport Identity Binding

Define one Core-owned artifact-location expectation that binds:

- requested run identity;
- requested report identity where applicable;
- workflow and immutable execution identity;
- artifact kind and storage slot/key;
- retained body commitment posture; and
- backend or transaction-local locator class without exposing local paths.

Validation order must be explicit:

1. decode and structurally validate;
2. verify external key/slot and internal run/report identity;
3. verify workflow and immutable execution binding where the consumer has that
   expectation;
4. verify retained complete body commitment;
5. only then compare, project, reconcile, recover, or replay.

Apply the same contract to:

- local filesystem read and list;
- SQLite read and list;
- PostgreSQL read and list;
- transaction-local PostgreSQL reads;
- injected store reconciliation;
- human and JSON inspect;
- current-authority metadata projection;
- authoritative-artifact recovery;
- duplicate reconciliation; and
- terminal/provider replay.

Legacy body-commitment posture remains explicit, but legacy does not excuse
identity mismatch. A structurally valid artifact in the wrong slot fails closed
with a stable payload-free code before any body or identity is projected.

Required tests:

- run B artifact placed under run A key for every backend;
- valid report stored under another report ID;
- wrong workflow or immutable execution identity;
- list containing one misplaced artifact;
- transaction-local replay substitution;
- restore/migration misplacement;
- body substitution and identity substitution together;
- injected trait store returning a valid wrong-run artifact;
- legacy artifact in the correct and incorrect slot; and
- no leaked run, report, path, or payload in errors and debug output.

Gate 2 acceptance:

- every durable and injected read boundary rejects wrong-slot valid artifacts;
- inspect and replay fail before projection or `already_persisted` acceptance;
- body commitment and external identity binding are independently tested;
- the roadmap and product contract distinguish local integrity from storage-
  owner tamper resistance; and
- exact-head CI plus an independent filesystem/backend misplacement rerun pass.

Implementation status:

- one Core-owned `WorkReportArtifactLocationExpectation` now carries the
  requested run, optional report, available immutable workflow identity, and a
  payload-free locator class;
- filesystem, SQLite, PostgreSQL, PostgreSQL transaction-local, and injected
  store reads use the common ordered validator;
- CLI inspect, first-run terminal replay, current-authority metadata,
  continuation collection, and duplicate reconciliation supply the strongest
  identity expectation available at their boundary;
- valid wrong-run, wrong-report, wrong-workflow, combined body-and-location,
  relational-key, injected-store, and legacy misplacement cases fail closed
  with `work_report_artifact.read.location_mismatch` before projection or body
  reuse;
- body-only substitution remains `work_report_artifact.read.corrupt`; and
- focused review, exact-head CI, maintainer-operated post-merge evidence, and
  an independent clean-room backend-misplacement rerun passed. Gate 2 is
  closed on exact `main`; these results do not close Gate 3.

See the [Gate 2 implementation
report](../concepts/WORK_REPORT_ARTIFACT_IDENTITY_BINDING_IMPLEMENTATION_REPORT.md).

## 5. Gate 3 Implementation: Project Resource Authorization Continuity

Identify every project-level capability and resource restriction that the
approval presentation claims will remain in force. Derive a canonical project-
restriction commitment bound to:

- project and provider family;
- operation and resource kind;
- exact normalized resource set;
- enabled/disabled posture;
- relevant policy and definition commitments;
- immutable run and approval request; and
- restriction algorithm/version identity.

The commitment must be captured when the approval request is created and
revalidated from current project declarations:

1. before accepting approval; and
2. immediately before provider admission.

Removal, replacement, ambiguous broadening, duplicate declaration, disablement,
or parser/validation failure blocks before approval events or provider entry.
An unchanged exact restriction proceeds. A strictly narrower restriction may
proceed only if the already-approved exact resource remains allowed and the
reviewed rule explicitly defines that monotonic case; otherwise require a new
presentation and approval.

Required tests:

- approved resource removed;
- approved resource replaced;
- resource set broadened;
- duplicate or ambiguous declarations;
- provider read disabled;
- policy/reference commitment changed;
- change before approval and change after approval but before provider entry;
- unchanged restriction succeeds;
- workflow target and fixture/provider payload substitutions remain blocked;
- no approval-granted event or provider call on failure; and
- equivalent project-level resource restrictions for every existing provider-
  read family are inventoried before live adoption.

Gate 3 acceptance:

- approval presentation names only mechanically committed facts;
- current restrictions are revalidated at both boundaries;
- stale or ambiguous restriction fails closed without provider entry;
- fixture and opt-in live-read contract tests pass; and
- an independent drift rerun reproduces the old failure on the parent commit and
  passes on the fix.

Implementation status: **closed for the documented GitHub repository-metadata
boundary.** The
existing approval `resolved_execution_context_hash` now commits a versioned
canonical GitHub repository-metadata restriction containing the project ID,
exact run and deterministic approval ID, provider family, operation and
resource kind, enabled posture, endpoint, access-reference identity, sorted
exact resource set, current workflow/skill/policy context, and fixture payload
commitment. Approval acceptance reconstructs that commitment from the current
validated project before appending `ApprovalGranted`. The atomic fixture and
opt-in live-read admission boundary reconstructs it again and requires equality
with both the immutable run bundle and granted approval before provider entry.

Focused public CLI regressions cover replacement, broadening, removal,
duplicate declaration, disablement, endpoint and access-reference substitution,
stable project-identity drift, referenced-policy drift, normalized resource
ordering, fixture drift, unchanged completion, and post-approval resource drift
with no provider invocation or telemetry. The private live-read boundary also
proves that current-project drift rejects before transient access resolution or
provider entry. Focused maintainer/security review found no authorization
blocker and corrected the field-level evidence gap before acceptance. Full Core,
CLI, warnings-denied, documentation, release-state, formatting, and diff
validation pass. The project manifest currently declares only the GitHub
repository-metadata provider-read family; Jira and GitHub Actions adapter models
are not equivalent project-declared governed read families and are not claimed
as covered by this gate.

PR #744 merged the unchanged reviewed head as exact `main` commit
`442b4a4435d65d66b974c11d17047e15327af8f5` after all nine exact-head checks in
Actions run `42045120157` passed. An independent disposable-checkout rerun
reproduced the parent defect at `a84cbaaba663f09075a810fb71c7963562779663`:
approval was granted and the original provider operation completed after the
current repository restriction changed. The same scenario on exact merged
`main` failed with `executor.approval.resume_context_mismatch` before
`ApprovalGranted`, provider dispatch, audit, or telemetry. The normalized-order
control approved and completed. Gate 3 acceptance is complete. See the
[independent rerun
report](../concepts/PROJECT_RESOURCE_AUTHORIZATION_CONTINUITY_INDEPENDENT_RERUN.md).

This closure does not claim equivalent project-level enforcement for provider
families that do not yet expose an authoritative manifest declaration, live
credential-backed acceptance evidence, or provider-write authorization.

## 6. Governed Delivery Sequence

Execute one reviewed phase per gate:

1. Gate 1 implementation, focused review, exact-head CI, independent fault-
   injection rerun, and merge.
2. Gate 2 implementation, focused review, exact-head CI, independent artifact-
   misplacement rerun, and merge.
3. Gate 3 implementation, focused review, exact-head CI, independent project-
   restriction drift rerun, and merge.
4. Cross-gate integration review proving a replacement executor cannot inherit
   success, evidence, report identity, or resource authority from a different
   lifecycle binding.
5. Product-contract, hosted, integration, and release-admission reconciliation.

Cross-gate review status: **blocked pending one narrow implementation
increment.** The review confirmed the three individual gate closures but found
that the private replacement-executor proof does not yet compose their exact
lifecycle bindings into its fresh-authority subject. See the [Continuity Of
Authority Cross-Gate
Review](../concepts/CONTINUITY_OF_AUTHORITY_CROSS_GATE_REVIEW.md). Step 5 remains
blocked until the required Core-owned lifecycle binding and integrated
substitution matrix pass focused review.

Do not bundle all three implementations into one unreviewable patch. Each gate
must close its own invariant and tests without claiming that the other two are
fixed. The Active Delivery Queue remains blocked until all three merge.

## 7. Release And Product Contract

Until all gates pass:

- do not call Workflow OS v1;
- do not call hosted execution reliable or unattended;
- do not present Workflow OS as a sole enterprise audit authority;
- do not claim fully committed live-provider resource approval;
- do not convert the 49.93% compaction result into a universal savings claim;
- do not broaden provider mutation or nested-harness autonomy; and
- do not let release pressure weaken fail-closed behavior.

After all gates pass, release admission still requires the existing preview
checks, exact release identity, clean-room packaging, product-contract review,
and independent security/recovery reruns. Passing these gates is necessary, not
sufficient, for v1.

## 8. Cross-Framework Recovery Evaluation

The comparative LangGraph and Sentence Governors evaluation belongs after the
three correctness gates. It must use the same recovery corpus and distinguish:

- framework-native authority/state guarantees;
- evaluator-authored integration and validation logic;
- decision correctness and authority preservation;
- duplicate SideEffects and stale-intent use;
- cached and uncached tokens, billed cost when authoritative, latency, recovery
  turns, and whole-task completion; and
- automatic checkpoint injection versus on-demand lookup.

The comparison must not reward Workflow OS merely because the harness supplies
its record more consistently, but that consistency is itself a product surface
to test and document.

## 9. Final Acceptance

The P0 preemption closes only when:

- all three gate implementations and focused reviews are merged;
- every adversarial regression passes on exact-head CI;
- independent reruns confirm the three original reproductions no longer pass;
- no accepted fix weakens approval, evidence, retry, replay, or provider safety;
- the current product contract and roadmap accurately describe residual limits;
- compaction evidence is preserved under its bounded claim contract; and
- release admission explicitly records that gate closure does not equal v1.

## 10. Final Recommendation

Treat continuity of authority as the organizing product thesis and these three
failures as its immediate proving ground. Fix truthful hosted acknowledgment,
exact report identity, and current resource authorization in sequence. Then use
the recovery checkpoint and cross-framework evaluation to demonstrate the
benefit of a governed record that survives interruption without turning that
benefit into an authority shortcut or unsupported readiness claim.
