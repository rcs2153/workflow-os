# Governed Context Access Projection Report

## 1. Executive Summary

Workflow OS now has a domain-neutral, payload-free governed context model and a
pure helper that projects authorized stable references for one exact actor,
workflow, run, step, and optional harness context.

The helper supports only `reference_only` and `bounded_metadata`. It reuses
existing scoped capability resolutions, retains the complete ordered evaluated
candidate set, and derives exact authorized entries plus bounded gaps. It does
not read or copy the target behind any reference.

## 2. Scope Completed

- Added a fixed typed target taxonomy for:
  - EvidenceReference;
  - workflow events;
  - audit events;
  - validation diagnostics;
  - approval decisions;
  - policy decisions;
  - SideEffects;
  - typed handoffs;
  - WorkReports.
- Added `CapabilityResourceKind::ContextReference`.
- Added exact Core-owned capability mapping:
  - `reference_only` requires `context.reference.view`;
  - `bounded_metadata` requires `context.metadata.view`.
- Added canonical `<target-kind>/<stable-id>` resource derivation.
- Added explicit available, unavailable, and unknown target posture.
- Added fixed bounded metadata containing only target kind, declared
  sensitivity, and availability observation time.
- Added complete evaluated candidates, authorized entries, and bounded gaps.
- Added pure `project_step_scoped_context`.
- Added deterministic ordering, validated serde, safe Debug, stable errors, and
  focused behavior tests.
- Hardened reused capability, sensitivity, and redaction enum deserialization
  so invalid wire values fail with static non-leaking errors.

## 3. Scope Explicitly Not Completed

- No target, source, evidence, event, report, handoff, or SideEffect payload
  dereference.
- No repository inspection, source reading, memory retrieval, or transcript
  access.
- No tool loading, command execution, connector activation, provider calls, or
  sandbox lifecycle.
- No OpenShell integration.
- No runtime consumption or time-of-use authorization.
- No persistence, workflow events, audit projection, or authority receipts.
- No schemas, SDKs, CLI behavior, UI, or workflow-spec fields.
- No SideEffect execution, provider mutation, or writes.
- No hosted administration, enterprise identity, or release posture changes.

## 4. Model And Helper Summary

`GovernedContextReferenceTarget` has no generic string variant. Every target
uses an existing validated Core identity. `GovernedContextReference` adds
known sensitivity, current declared availability, and validated redaction
metadata without storing target contents.

Each `GovernedContextProjectionCandidate` retains the reference, availability
observation time, requested access level, and exact `CapabilityResolution`.
Candidate validation requires the fixed capability for the access level, the
exact canonical context resource, and matching sensitivity.

`GovernedContextProjection` retains the complete ordered candidate set. Its
validation and deserialization recompute entries and gaps and reject
inconsistent entry omission, substitution, or candidate reordering.

## 5. Authority Boundary

Only an `authorized` current capability resolution can produce an entry.
Unavailable or unknown targets, missing authority, unresolved policy,
approval, evidence, or check prerequisites, and sensitivity-ceiling failures
produce bounded gap reasons.

The projection is not a lease, durable receipt, or payload-access grant. Any
future dereference must independently re-resolve authority and target posture
at time of use.

## 6. Privacy And Redaction

- Debug output redacts target IDs, actor, workflow, run, step, harness,
  capability, resource, grant, and redaction text.
- Secret-like stable IDs, redaction fields, and reasons fail with stable
  non-leaking errors.
- Serialization contains stable references and fixed typed metadata only.
- Invalid local or nested enum values fail with static errors that do not echo
  caller-supplied wire values.
- No arbitrary metadata map, summary, path, URL, title, diagnostic message,
  report text, event payload, provider payload, command output, environment
  value, or credential field exists in the model.
- Gaps disclose target kind and bounded reason, not the rejected target ID.

## 7. Test Coverage

Focused tests cover:

- authorized reference-only projection;
- authorized bounded-metadata projection;
- all nine fixed target variants;
- exact access-level capability mapping;
- canonical context-resource derivation;
- unavailable, unknown, missing-authority, independent-policy, and sensitivity
  gaps;
- deterministic candidate ordering;
- duplicate candidate rejection;
- wrong actor/context and access-level rejection;
- serde round trip;
- inconsistent entry omission and candidate reordering rejection;
- secret-like ID and redaction-metadata rejection without leakage;
- safe Debug and absence of forbidden raw-payload fields;
- existing capability-authority behavior.

## 8. Commands And Results

- `cargo fmt --all`: passed.
- `cargo clippy -p workflow-core --test governed_context_access -- -D warnings`:
  passed.
- `cargo test -p workflow-core --test governed_context_access -- --nocapture`:
  passed, 11 tests.
- `cargo test -p workflow-core --test capability_authority --test governed_context_access`:
  passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace --quiet`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 9. Dogfood Governance

- workflow: `dg/implement`
- run ID: `run-1785125207546229000-2`
- approval ID:
  `approval/run-1785125207546229000-2/implementation-approved`
- presentation ID: `presentation/b132a7b2febcab2b`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted approval handoff was reviewed and presented.

## 10. Remaining Limitations

- Required-context contracts do not consume this projection.
- A projection is not bound to an immutable run bundle.
- Candidate completeness is enforced relative to the candidates supplied to
  the helper; this model does not prove that a caller supplied every
  real-world context target.
- No time-of-use re-resolution or authority receipt exists.
- No audited dereference path exists.
- Later target variants need stable typed IDs and separately reviewed access
  semantics.
- A future sandbox integration must prevent unprojected workspace exposure.

## 11. Recommended Next Phase

Perform a focused governed context-access projection maintainer review.

Review exact authority composition, target taxonomy, complete-candidate wire
integrity, gap derivation, sensitivity, redaction, serde safety, and the
non-dereferencing runtime boundary before planning required-context contract
consumption or any payload-access path.
