# Current-Authority WorkReport Artifact Metadata Read Plan Review

## 1. Executive Verdict

Plan accepted; proceed to the private exact-target WorkReport artifact metadata
read implementation.

The selected consumer is appropriately narrow, useful, and consistent with the
current-authority architecture. No planning blocker remains.

## 2. Scope Verification

The plan stays within the approved planning-only scope. It does not authorize a
public authority API, generic callback, report-body access, executor behavior,
local-check execution, provider or OpenShell integration, sandbox execution,
SideEffect execution, writes, event append behavior, persistence changes,
schemas, SDKs, CLI behavior, examples, dependencies, or release changes.

## 3. Consumer Selection Assessment

An exact read of bounded metadata for one immutable `WorkReport` artifact is an
appropriate first Core-owned consumer because:

- the target already has typed `WorkReport` context-reference vocabulary;
- bounded-metadata access is already distinct from reference-only access;
- `WorkReportArtifactStore` already exposes an exact run/report read;
- the stored record has an existing validation boundary;
- the result can support later governed handoff and context composition;
- the operation introduces no process, network, provider, or mutation surface.

A local-check or skill invocation would be a materially larger authority
surface and is correctly deferred.

## 4. Contract And Identity Assessment

The plan correctly requires:

- one exact `GovernedContextReferenceTarget::WorkReport(report_id)`;
- `GovernedContextAccessLevel::BoundedMetadata`;
- `RequiredContextObligation::Required`;
- a contract identity and content hash matching the immutable execution binding;
- the execution binding's run ID for the store lookup;
- artifact identity matching the requested run and report.

Reference-only access and optional-only declaration do not authorize this
consumer.

## 5. Same-Call Authority Assessment

The required order is sound: validate the concrete request, resolve current
authority from a fresh registered-source snapshot, consume the exact context
contract, stop unless ready, then perform one exact store read.

One implementation constraint needed to be made explicit. The existing private
`use_current_authority` helper returns a bounded use posture rather than the
consumer's value. The concrete method must reuse that helper by capturing one
private store-read result inside its `FnOnce`, mapping the read to the existing
consumer result, and reconciling the captured result with the returned use
posture. It must not duplicate the authority resolver or expose a generic
value-returning callback. The plan now states this requirement.

## 6. Zero-Read Blocking Assessment

The plan requires all target-shape, contract, source, authority, prerequisite,
availability, access-level, obligation, sensitivity, freshness, and coherence
failures to stop before the artifact store is touched.

The proposed instrumented store fixture and explicit read-count assertions are
necessary. Tests must prove zero reads for every blocked and source-failure
family rather than inferring this only from an outcome enum.

## 7. Output And Privacy Assessment

The proposed view is bounded to:

- report ID;
- run ID;
- terminal report status;
- sensitivity.

The contained `WorkReport`, sections, citations, summaries, notes, limitations,
risks, disclosures, hashes, paths, raw redaction metadata, provider payloads,
command output, and secrets remain unavailable. The view is private,
non-serializable, and redacts both identifiers in Debug output.

This is a suitable first metadata boundary.

## 8. Error Assessment

The plan correctly separates not-found, blocked, source-failure, and
store-failure postures. No missing artifact or failed read may fabricate
metadata or evidence.

The implementation should prefer one bounded private store-failure posture and
must never preserve or format the underlying store error. A captured store
result that conflicts with the same-call use posture must fail closed with a
stable non-leaking internal error.

## 9. Test Assessment

The proposed tests cover the material behavior:

- ready exact read and exact field bounds;
- reference-only, optional-only, target, run, and contract mismatch;
- unavailable targets and unresolved prerequisites;
- expired or revoked grants;
- stale, incomplete, future-dated, or incoherent sources;
- not-found and store-failure behavior;
- corrupt or identity-mismatched artifacts;
- sensitivity mismatch;
- fresh resolution on repeated calls;
- zero store reads on every blocked path;
- Debug non-leakage and private visibility;
- no events, writes, executor mutation, provider calls, files, or CLI output;
- existing authority, required-context, WorkReport, store, and workspace
  regression coverage.

Implementation tests should additionally prove that the captured store result
and returned use posture cannot disagree silently.

## 10. Documentation Assessment

The roadmap, plan, and planning report accurately say that planning is complete
and implementation has not started. They do not overclaim runtime, provider,
OpenShell, sandbox, or write behavior.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Keep the first outcome and view private until a separate compatibility and
  privacy review authorizes exposure.
- Later production consumers will need source/store consistency and restart
  semantics; this slice may claim only same-process, same-call gating.
- Do not generalize this operation into arbitrary governed-context dereference
  during implementation.

## 13. Governed Review Record

- workflow ID: `dg/review`
- run ID: `run-1785179497620648000-2`
- approval ID:
  `approval/run-1785179497620648000-2/review-scope-approved`
- approval-presentation ID: `presentation/f96ff2dd73c00ada`
- approval outcome: granted by delegated maintainer
- governed status: completed
- out-of-kernel work: repository and implementation-plan review, documentation
  edits, validation, git, and later PR operations remain external execution
  coordinated by the kernel

## 14. Validation

- `npm run check:docs`
- `git diff --check`

## 15. Recommended Next Phase

Implement only the private exact-target WorkReport artifact metadata read.

Do not add public APIs, generic callbacks, report-body access, executor
integration, local-check execution, providers, OpenShell, sandboxes,
SideEffects, writes, events, persistence changes, schemas, CLI behavior,
dependencies, or release changes.
