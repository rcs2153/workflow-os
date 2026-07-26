# Governance Disclosure Delivery Model Report

## 1. Executive Summary

The first prerequisite for authoritative `Proceed + Visible` routing is
implemented as a domain-neutral, payload-free model in `workflow-core`.

The model binds one complete, source-bound governance assessment to one
explicit injected-local disclosure surface. A validated receipt proves only
that the configured surface accepted the exact delivery request. It does not
claim that a human saw, understood, acknowledged, or approved anything.

This phase does not integrate the executor or deliver disclosures at runtime.

## 2. Scope Completed

- Added a versioned disclosure delivery identifier.
- Added one explicit injected-local disclosure surface kind and bounded surface
  reference.
- Added a delivery request bound to the exact authoritative governance
  assessment.
- Required complete `Proceed + Visible` posture and authoritative source
  binding.
- Added sensitivity and reference-only redaction posture.
- Added a surface-acceptance receipt that embeds the exact request.
- Added explicit `NotClaimed` human-observation and acknowledgement fields.
- Added deterministic validation, custom fail-closed deserialization,
  redaction-safe `Debug`, and focused tests.

## 3. Scope Explicitly Not Completed

The phase did not add:

- executor routing or disclosure callbacks;
- CLI, terminal, UI, notification, or hosted delivery surfaces;
- persistence, workflow events, audit projection, or WorkReport integration;
- approval routing, approval decisions, or automatic approval;
- denial routing;
- workflow or policy schema fields;
- providers, OpenShell, sandbox execution, or credential injection;
- SideEffect execution or writes;
- human-observation, understanding, acknowledgement, or presentation claims;
- reasoning lineage, enterprise administration, or release changes.

## 4. Model And API Summary

The implementation adds:

- `GovernanceDisclosureDeliveryVersion`;
- `GovernanceDisclosureDeliveryId`;
- `GovernanceDisclosureSurfaceKind`;
- `GovernanceDisclosureSurface`;
- `GovernanceDisclosureSensitivity`;
- `GovernanceDisclosureRedactionPosture`;
- `GovernanceDisclosureDeliveryRequest`;
- `GovernanceDisclosureDeliveryStatus`;
- `GovernanceDisclosureHumanObservation`;
- `GovernanceDisclosureAcknowledgement`; and
- `GovernanceDisclosureDeliveryReceipt`.

`GovernanceDisclosureDeliveryRequest` requires the existing
`GovernanceAssessmentBinding` rather than a caller-selected interaction enum.
`GovernanceDisclosureDeliveryReceipt` embeds the full validated request so a
receipt cannot be detached from or substituted across another run,
assessment, or surface.

## 5. Validation Boundary

Construction and deserialization fail closed unless:

- identifiers and references are canonical, bounded, and not secret-like;
- the assessment is complete;
- execution disposition is `Proceed`;
- disclosure requirement is `Visible`;
- the assessment carries authoritative source binding;
- the surface is explicit and supported;
- redaction posture is `ReferenceOnly`;
- receipt status is `SurfaceAccepted`;
- human observation and acknowledgement remain `NotClaimed`;
- receipt acceptance does not precede the request timestamp; and
- receipt/request equality is exact when validated together.

Unknown top-level fields are rejected without echoing their names or values.
Validation errors use stable codes and static non-leaking messages.

## 6. Semantic Claim Boundary

The receipt proves:

```text
the configured disclosure surface accepted this exact bounded request
```

It does not prove:

- delivery to a person;
- display or notification success;
- human observation;
- human understanding;
- acknowledgement;
- approval; or
- authorization to execute.

This distinction preserves the routing plan's separation between
non-blocking disclosure, approval presentation, durable audit, and execution
authority.

## 7. Privacy And Redaction

The model has no fields for rendered prose, source/spec contents, commands,
process output, filesystem paths, environment values, provider payloads,
credentials, authorization headers, private keys, or tokens.

`Debug` redacts bounded caller-controlled identifiers and references.
Serialization contains typed identity and posture only. Secret-like values are
rejected before storage, output, or receipt construction.

## 8. Test Coverage

Focused tests cover:

- exact valid request binding;
- incomplete, quiet, approval-required, and source-unbound rejection;
- surface acceptance with explicit human non-claims;
- mismatched request/receipt rejection;
- timestamp ordering;
- valid serde round trip;
- fail-closed invalid wire claims without secret echo;
- secret-like identifier, surface, and correlation rejection;
- redaction-safe `Debug` and payload-free serialization; and
- unknown-field and tampering rejection without value echo.

Full workspace validation passed.

## 9. Governed Phase Record

- workflow: `dg/implement`
- run: `run-1785033169817760000-2`
- approval:
  `approval/run-1785033169817760000-2/implementation-approved`
- presentation: `presentation/27a464b2926ca3d4`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: completed
- out-of-kernel work: source inspection, Rust implementation, tests,
  documentation, validation, and later git/PR work
- missing coverage: the kernel coordinated governance only; it did not edit
  files, execute checks, create a WorkReport artifact, or perform git actions

## 10. External User Feedback Reconciliation

Fresh-clone evaluation of current `main` confirms that existing-repo
scaffolding, preserved agent guidance, concise first-run posture,
safe-metadata recommendations, inactive authoring, preflight, approvals, and
durable event inspection form a credible local governance loop.

The review reinforces the proportional-governance direction: the next product
problem is reducing low-risk ceremony while retaining evidence and audit
truth. It also identified two bounded product-hardening follow-ups that are not
part of this model phase:

- make the Node 20 integration-check requirement harder to miss or enforce it
  through repository tooling; and
- remove the duplicated missing-manifest diagnostic in the pre-scaffold
  validation path.

Mock skill runs remain demonstrations of approval and audit mechanics, not
execution evidence.

## 11. Remaining Known Limitations

- No runtime surface implements this model.
- No receipt is persisted or projected through events, audit, or reports.
- No human delivery or acknowledgement proof exists.
- Only one injected-local surface vocabulary is modeled.
- The authoritative executor path remains fresh-run, local,
  `DocsCheck`-only, and quiet-`Proceed`-only.
- Approval and denial routes remain separate future phases.

## 12. Recommended Next Phase

The focused maintainer review is complete and accepted the model with
non-blocking constraints in
[Governance Disclosure Delivery Model Review](GOVERNANCE_DISCLOSURE_DELIVERY_MODEL_REVIEW.md).

Proceed to one explicit injected-local `Proceed + Visible` executor path that
uses this model before skill execution. Continue to exclude approval routing,
denial routing, persistence, CLI/UI behavior, providers, OpenShell, SideEffect
execution, writes, schemas, hosted behavior, reasoning lineage, and release
changes.
