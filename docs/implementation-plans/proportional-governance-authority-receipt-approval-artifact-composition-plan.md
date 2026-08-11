# Proportional-Governance Authority-Receipt Approval-Artifact Composition Plan

Status: implemented and accepted after focused maintainer review.

Implementation evidence is recorded in the
[phase report](../concepts/PROPORTIONAL_GOVERNANCE_AUTHORITY_RECEIPT_APPROVAL_ARTIFACT_COMPOSITION_REPORT.md)
and [focused review](../concepts/PROPORTIONAL_GOVERNANCE_AUTHORITY_RECEIPT_APPROVAL_ARTIFACT_COMPOSITION_REVIEW.md).

## 1. Executive Summary

Workflow OS already has reviewed, explicit boundaries for proof-enforced
approval resume with fresh registered runtime facts, trusted decision-time
authority-receipt construction, receipt-bearing terminal WorkReport
generation, and receipt-plus-report-artifact persistence. Callers still have
to compose those three stages manually.

This phase adds one Core-owned, executor-adjacent call that preserves their
accepted order and failure semantics. It remains explicit, local, opt-in, and
store-injected. It does not change executor or CLI defaults.

## 2. Goals

- Close the manual call-graph gap among the accepted decision, report, and
  persistence helpers.
- Require proof-enforced presentation before a granted approval can mutate the
  run.
- Resolve fresh registered runtime facts at decision time.
- Issue a trusted authority receipt only from successful Core-owned grant
  proof.
- Generate the terminal report and derive its receipt citation in the same
  composition.
- Persist the trusted receipt before artifact-integrity and artifact-write
  gates.
- Preserve terminal workflow and approval truth across later failures.
- Return stable, bounded, non-leaking errors and posture.

## 3. Non-Goals

This phase does not add:

- default executor or CLI consumption;
- automatic approvals or automatic persistence for existing paths;
- store discovery, runtime configuration, or workflow schema fields;
- a transaction spanning receipt and artifact stores;
- reusable or ambient authority;
- providers, OpenShell execution, or new mutation families;
- SideEffect execution;
- hosted expansion, examples, SDK changes, or release changes.

## 4. Accepted Inputs

The new input owns:

- the existing proof-enforced current-runtime-fact approval decision request;
- explicit terminal WorkReport inputs;
- explicit SideEffect citation and approval-linkage gate booleans; and
- explicit high-assurance disclosure policy.

The call also receives the existing executor, immutable-bundle store,
registered runtime-fact source, receipt store, report-artifact store, and
SideEffect record store as explicit dependencies.

It accepts no public serialized receipt, prebuilt receipt citation, detached
assessment, inferred store, provider request, or payload.

## 5. Required Ordering

The order is fixed:

1. Validate approval presentation proof.
2. For a grant, resolve fresh facts and reproduce the exact durable governance
   binding before approval mutation.
3. Apply the existing approval decision and resume behavior.
4. Derive a trusted decision-time receipt from the successful proof-marked
   grant event. Denial produces no receipt.
5. Generate the terminal WorkReport and derive the receipt citation inside
   Core.
6. Construct the artifact before writes.
7. Persist or exactly reconcile the receipt.
8. Validate receipt referential integrity and selected existing gates.
9. Write or exactly reconcile the report artifact.

No artifact write may precede the receipt and gate boundaries.

## 6. Failure Semantics

- Proof, immutable-context, source, freshness, or reassessment failure returns
  `Err` before approval mutation.
- Denial remains a successful governed decision, invokes no decision-time
  source, emits no receipt, and writes neither receipt nor artifact.
- Report-generation failure preserves the terminal decision and trusted
  receipt in memory but writes neither store.
- Receipt persistence or integrity failure prevents artifact writing.
- Artifact conflicts fail closed.
- An unreadable or potentially started artifact write blocks automatic retry.
- No later failure rewrites the workflow result, approval result, or event
  history.

## 7. Privacy And Redaction

The new input Debug representation must redact the decision and report inputs.
The existing result representation exposes only status, counts, presence,
posture, and error codes. Errors must not include IDs, paths, commitments, raw
facts, report text, command output, environment values, credentials, or
secret-like values.

## 8. Tests

Focused tests must prove:

- a proof-enforced grant completes and persists one receipt and one artifact;
- denial remains source-free at decision time and writes nothing;
- missing presentation proof blocks before source access, mutation, and writes;
- report failure preserves terminal truth and writes nothing;
- existing receipt/artifact idempotency, integrity, gate, ambiguity, and
  privacy tests remain green; and
- the full workspace validation remains green.

## 9. Validation

Run:

- focused local-executor composition tests;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`; and
- `git diff --check`.

## 10. Acceptance Criteria

- One explicit Core call owns the complete accepted composition sequence.
- Pre-decision failures cannot reach approval mutation or either store.
- Granted decisions can produce and persist exactly bound receipt evidence and
  a governed terminal report artifact.
- Denial and report failure perform no durable receipt or artifact writes.
- Existing APIs and defaults remain unchanged.
- No external effect is authorized by this composition helper.

## 11. Recommended Next Phase

After focused review, return to the active runtime roadmap. Do not treat this
explicit helper as authorization for default persistence or provider-mutation
expansion. The next product consumer must be independently scoped and must
prove why its selected runtime path needs this evidence closure.
