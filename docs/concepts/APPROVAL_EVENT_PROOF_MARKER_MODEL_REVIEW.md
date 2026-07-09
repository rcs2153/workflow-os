# Approval Event Proof Marker Model Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

Proceed to approval-event proof marker runtime event wiring, limited to the existing opt-in approval-presentation decision path.

## 2. Scope Verification

The phase stayed within the approved model-only scope.

It implemented bounded approval decision proof marker vocabulary and validation. It did not implement:

- runtime approval event wiring;
- default approval behavior changes;
- public approval enforcement changes;
- automatic approvals;
- hidden approvals;
- inspect or projection changes;
- approval-card UI;
- CLI rendering changes;
- workflow schema changes;
- examples;
- provider writes;
- side-effect execution;
- report artifact writes;
- hosted or distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 3. Model Assessment

The model is domain-appropriate and minimal for the event-proof gap.

Implemented types:

- `ApprovalDecisionProofMarker`;
- `ApprovalDecisionProofMarkerDefinition`;
- `ApprovalDecisionProofEnforcementMode`;
- `ApprovalDecisionProofValidationPolicy`.

The first enforcement mode, `approval_presentation_required`, correctly represents the existing proof-enforced dogfood approval path without expanding default approval semantics.

The first validation policy, `approval_presentation_request_match`, correctly names the intended proof relationship without introducing broader approval-card or authority semantics.

The marker carries stable proof references and bounded proof metadata:

- presentation ID;
- presentation content hash;
- proof validation timestamp;
- optional proof age;
- optional freshness limit;
- proof record sensitivity;
- redaction metadata.

This is enough for future approval decision events to prove which presentation record was used without copying the approval-presentation payload.

## 4. Validation Assessment

Validation is deterministic and appropriately bounded.

The constructor validates:

- proof age upper bound;
- non-zero freshness limit;
- freshness limit upper bound;
- proof age not exceeding freshness limit when both are supplied;
- redaction metadata entry counts;
- redaction field bounds;
- redaction reason bounds;
- secret-like redaction fields and reasons.

Invalid serialized marker payloads fail closed through the same constructor path during deserialization.

Validation errors use stable, non-leaking codes for freshness failures:

- `approval_event_proof_marker.proof_age.too_large`;
- `approval_event_proof_marker.freshness_limit.invalid`;
- `approval_event_proof_marker.freshness_mismatch`.

Non-blocking follow-up: secret-like redaction metadata currently reuses the broader `approval_presentation.secret_like_value` code. That is safe and non-leaking, but a marker-specific code would make future diagnostics clearer.

## 5. Privacy And Redaction Assessment

The model is redaction-safe by construction.

It stores stable proof references and bounded metadata only. It does not store:

- raw approval handoff text;
- work summary;
- approved scope;
- strict non-goals;
- validation expectations;
- why-now text;
- chat transcripts;
- screenshots;
- local file paths;
- provider payloads;
- command output;
- source or spec contents;
- credentials, tokens, authorization headers, private keys, or secret-like values.

`Debug` redacts the presentation ID and redaction metadata contents. Serialization includes stable IDs and hashes but not presentation payload text.

Deserialization errors for invalid marker payloads are routed through stable validation errors and do not include the referenced presentation ID in the tested failure path.

## 6. Serde And Compatibility Assessment

Serde support is appropriate for a future event payload model.

Valid markers serialize and deserialize successfully. Invalid serialized markers fail closed through constructor validation.

The serialized field names are stable and sensible for future event payloads:

- `enforcement_mode`;
- `presentation_id`;
- `presentation_content_hash`;
- `proof_validated_at`;
- `proof_validation_policy`;
- `proof_age_ms`;
- `proof_freshness_limit_ms`;
- `proof_record_sensitivity`;
- `redaction`.

No workflow schema changes were introduced.

Existing event logs remain unaffected because the marker is not yet wired into runtime event payloads.

## 7. Runtime Boundary Assessment

The implementation preserves the runtime boundary.

It adds a marker model only. Constructing the marker does not:

- append approval events;
- resume runs;
- fail runs;
- alter approval requests;
- create report artifacts;
- emit inspect output;
- mutate state;
- change default approval behavior.

This is the correct boundary before adding proof marker construction to the opt-in approval-presentation decision path.

## 8. Test Quality Assessment

The focused tests cover the model slice well.

Tests verify:

- valid marker construction;
- enum vocabulary serialization;
- freshness mismatch rejection;
- secret-like redaction metadata rejection;
- serde round trip;
- invalid serialized marker failure;
- Debug redaction behavior.

Existing approval-presentation tests also continue to exercise the surrounding model vocabulary.

No blocking test gaps were found.

Non-blocking follow-ups:

- add marker-specific redaction error-code coverage if the error namespace is tightened;
- add runtime regression tests in the next phase proving marker construction failure prevents approval decision events and downstream runtime mutation;
- add compatibility tests for older approval events without markers when event payload wiring lands.

## 9. Documentation Review

The phase report accurately states:

- approval-event proof marker model vocabulary is implemented;
- runtime approval event wiring is not implemented;
- default public approval behavior is unchanged;
- inspect/projection output is not implemented;
- approval-card UI is not implemented;
- schemas are not changed;
- examples are not updated;
- provider writes, hosted behavior, reasoning lineage, side-effect execution, report artifact writes, and release posture changes remain out of scope.

`ROADMAP.md` correctly identifies the model-only status and the remaining event-marker limitation.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Consider replacing `approval_presentation.secret_like_value` with a marker-specific non-leaking error code for marker redaction metadata.
- Add runtime tests in the next implementation phase proving marker construction failure fails before approval decision event append, run resume, downstream skill invocation, side-effect events, report artifacts, provider calls, or other runtime mutation.
- Add inspect/projection exposure immediately after event payload wiring so dogfood phase-close can report `proof_enforced` from the event trail rather than proof-record correlation alone.
- Preserve backward compatibility for old approval decision events without proof markers.

## 12. Recommended Next Phase

Recommended next phase: approval-event proof marker runtime event wiring.

The next implementation should remain narrow:

- wire marker construction only into the existing opt-in approval-presentation decision path;
- keep default `decide_approval(...)` unchanged;
- append proof markers only after proof validation succeeds;
- fail closed before approval decision events if marker construction fails;
- keep inspect/projection, schemas, examples, writes, hosted behavior, reasoning lineage, and release posture changes out of scope unless separately approved.

## 13. Validation

Governed review run:

- dogfood workflow ID: `dg/review`;
- run ID: `run-1783607393438729000-2`;
- approval ID: `approval/run-1783607393438729000-2/review-scope-approved`;
- approval-presentation ID: `presentation/ea9664ad99c24bf7`;
- approval-presentation content hash: `ea9664ad99c24bf7e8867ab4ff239e573717a5ff773a47c7d83d0d1c10ed17e7`;
- approval outcome: granted;
- event summary: 39 events, including 1 approval request, 1 approval grant, 8 policy decisions, 6 scheduled steps, 6 skill invocation requests, 6 skill starts, 6 skill successes, resume, and completion;
- approval-presentation enforcement: `proof_record_present_granted_approval_seen`;
- approval-presentation event marker: `not_available`.

Commands run:

- passed: `npm run check:docs`;
- passed: `git diff --check`;
- passed: `npm run dogfood:benchmark -- phase-close run-1783607393438729000-2 --phase review`.
