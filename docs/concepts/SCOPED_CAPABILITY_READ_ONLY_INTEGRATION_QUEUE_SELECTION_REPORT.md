# Scoped Capability Read-Only Integration Queue Selection Report

## 1. Executive Summary

The next executable phase is one opt-in, Core-owned GitHub read-only adapter
integration that enforces scoped capability authority immediately before the
adapter call and issues an evidence-only local authority receipt only after a
successful outcome.

This selection moves existing governance primitives into a real runtime path.
It does not add another model family, provider mutation, public configuration,
or live-network requirement.

## 2. Why This Phase Is Next

The dedicated draft-pull-request sandbox proof remains dependency-tracked
because least-privilege access has not been verified. That blocked proof must
not stall unrelated no-write runtime composition.

The scoped runtime authority plan has completed and reviewed the prerequisites
for a first integration:

- capability grants and availability;
- pure capability resolution;
- non-authorizing capability requests;
- step-scoped capability projection;
- governed context-access projection;
- local unsigned evidence-only authority receipts;
- complete current-authority fact-set modeling; and
- source-backed current-authority resolution foundations.

The plan's next uncompleted sequence item is one opt-in harness or adapter
integration before any write-path adoption. The existing GitHub read-only
adapter is the smallest useful boundary because it already has injected
clients, fixture coverage, bounded outputs, telemetry, and a stable
`GitHubRead` capability vocabulary.

## 3. Selected Vertical Slice

The milestone should define, review, and implement one internal Core helper or
service that:

1. accepts an exact workflow, run, step, actor, repository resource,
   sensitivity, and correlation binding;
2. freshly reads a complete current-authority fact set from an injected
   Core-owned source;
3. resolves the exact GitHub repository-read capability from current grants
   and availability;
4. rejects missing, stale, revoked, expired, ambiguous, sensitivity-exceeding,
   or independently conditioned authority before the adapter client is called;
5. creates the step-scoped projection from that fresh authorized resolution;
6. invokes the existing `GitHubReadOnlyAdapter` through an injected client;
7. binds one successful bounded read outcome into a local unsigned authority
   receipt; and
8. returns the bounded adapter outcome, projection, and receipt without
   treating either projection or receipt as reusable permission.

The focused integration contract and security review belong at the beginning
of this same vertical-slice milestone. A valid blocker must be corrected before
implementation, but the work must not fragment into an open-ended sequence of
documentation-only phases.

## 4. Authority Boundary

Caller-supplied grants, availability records, projections, and receipts are not
authoritative runtime inputs. The integration must consume the existing
Core-owned complete fact-set source boundary and use fresh time and exact
execution identity.

For the first slice, a grant with independent policy, approval, evidence, or
check requirements is not silently upgraded to authorized. Such posture fails
closed before adapter invocation until a separately reviewed same-call
composition supplies independently evaluated prerequisites.

The receipt is evidence only. It records that one exact source-backed authority
assessment preceded one successful read-only operation. It cannot authorize a
retry or a later call.

## 5. Required Failure Ordering

The implementation must prove:

- identity and source validation precede authority resolution;
- fresh authority resolution precedes capability projection;
- authorized projection precedes adapter invocation;
- successful bounded adapter outcome precedes receipt issuance; and
- no denied or failed path fabricates a projection, receipt, telemetry success,
  or evidence claim.

Source read failure, incomplete inventory, duplicate facts, stale facts,
revocation, expiry, unavailable capability, wrong repository, actor, workflow,
run, step, sensitivity, or capability must fail without calling the adapter
client.

## 6. Privacy And Evidence Posture

The slice may store or return stable references, commitments, bounded adapter
metadata, and the existing redaction-safe telemetry. It must not copy raw
GitHub bodies, credentials, authorization headers, environment values, source
contents, command output, or token-like values into errors, receipts, Debug,
or serialization.

The required test path uses injected fixtures. Live GitHub access is optional
and is not acceptance evidence for this phase.

## 7. Strict Non-Goals

This selection does not authorize:

- GitHub comments, pull requests, merges, or any provider mutation;
- another adapter or capability family;
- CLI or workflow-schema configuration;
- automatic capability grants or inferred authority;
- automatic approvals;
- ambient tool loading or workspace access;
- durable receipt storage or audit export;
- OpenShell integration;
- nested harness execution;
- hosted or enterprise administration; or
- release posture changes.

## 8. Test Expectations

The implementation milestone must include focused tests for:

- exact authorized read success;
- complete fact-set source use;
- no client call for every denied or source-failure posture;
- exact repository, actor, workflow, run, step, sensitivity, and capability
  substitution rejection;
- revocation, expiry, unavailable capability, ambiguity, and stale assessment;
- independently conditioned grant rejection;
- adapter failure without receipt issuance;
- receipt issuance only after success;
- receipt/projection non-reusability;
- no raw payload or secret leakage; and
- existing adapter, authority, context, receipt, telemetry, and workspace
  regressions.

## 9. Governed Planning Record

- workflow ID: `dg/d`
- run ID: `run-1792688668759908000-2`
- approval ID: `approval/run-1792688668759908000-2/planning-approved`
- presentation ID: `presentation/fe515423bb9bb896`
- approval outcome: granted by delegated maintainer with persisted presentation
  proof
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- approval-presentation enforcement: proof enforced with one matching persisted
  presentation record and event marker
- kernel boundary: governance coordination only; evidence inspection,
  documentation, validation, Git, and pull-request work remain out of kernel

## 10. Validation

The planning phase requires:

- `npm run check:docs`; and
- `git diff --check`.

Both checks passed. No implementation or runtime test was required because this
phase changed only roadmap sequencing and its evidence report.

## 11. Recommended Next Phase

Execute the scoped capability GitHub read-only integration vertical slice.
Start with its focused integration contract and security review, then implement
the accepted boundary in the same governed milestone. Stop for a demonstrated
authority or privacy blocker; do not broaden to provider writes.
