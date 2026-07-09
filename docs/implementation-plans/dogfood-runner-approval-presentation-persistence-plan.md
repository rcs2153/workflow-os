# Dogfood Runner Approval-Presentation Persistence Plan

Status: Planned; not implemented.

Related work:

- [Approval Gate Presentation Enforcement Gap](../concepts/APPROVAL_GATE_PRESENTATION_ENFORCEMENT_GAP.md)
- [Approval Gate Presentation Persistence And Enforcement Plan](approval-gate-presentation-persistence-enforcement-plan.md)
- [Approval Gate Presentation Persistence Report](../concepts/APPROVAL_GATE_PRESENTATION_PERSISTENCE_REPORT.md)
- [Approval Gate Presentation Persistence Review](../concepts/APPROVAL_GATE_PRESENTATION_PERSISTENCE_REVIEW.md)
- [Approval Gate Presentation Opt-In Enforcement Plan](approval-gate-presentation-opt-in-enforcement-plan.md)
- [Approval Gate Presentation Opt-In Enforcement Implementation Report](../concepts/APPROVAL_GATE_PRESENTATION_OPT_IN_ENFORCEMENT_IMPLEMENTATION_REPORT.md)
- [Approval Gate Presentation Opt-In Enforcement Review](../concepts/APPROVAL_GATE_PRESENTATION_OPT_IN_ENFORCEMENT_REVIEW.md)
- [Self-Governed Build Benchmark](../user-guide/self-governed-build-benchmark.md)

## 1. Executive Summary

Workflow OS can now model, locally persist, and explicitly enforce approval-presentation proof for local approval decisions. The repo-local dogfood runner still only emits `approval_handoff` and `copy_safe_approval_request` text; it does not persist the exact presented approval scope as an `ApprovalPresentationRecord`.

The next implementation should add dogfood-runner proof persistence before approval commands are submitted. The runner should create a validated, durable presentation record from the exact bounded handoff it emits, print the resulting `presentation_id`, and make the future approval command able to use that proof.

This plan does not implement runner persistence, automatic approvals, default approval enforcement, CLI approval cards, schemas, examples, provider writes, side effects, hosted behavior, reasoning lineage, or release posture changes.

## 2. Goals

- Persist the exact dogfood approval handoff emitted by `phase-start`.
- Persist proof before a maintainer or delegated maintainer runs the approval command.
- Use existing `ApprovalPresentationRecord` constructors and local store APIs.
- Print the stable `presentation_id` and content hash for audit.
- Keep approval explicit and human/delegated-maintainer controlled.
- Prepare dogfood approvals to call the reviewed opt-in enforcement path later.
- Preserve current `phase-start`, `approve`, and `phase-close` semantics unless explicitly extended.
- Keep all errors stable and non-leaking.
- Keep the runner as repo-local development tooling, not public product CLI.

## 3. Non-Goals

This phase must not implement:

- hidden approval;
- automatic approval;
- default approval behavior changes;
- runtime approval semantic changes outside the repo-local dogfood helper;
- CLI approval-card UX;
- workflow schema fields;
- public product CLI guarantees;
- WorkReport citation changes;
- report artifacts;
- provider writes;
- side effects;
- hosted or distributed runtime behavior;
- reasoning lineage;
- recursive agents, agent swarms, or Level 3/4 autonomy;
- release posture changes.

## 4. Current Baseline

Implemented baseline:

- `ApprovalPresentationRecord` model and validation;
- deterministic presentation content hash;
- local `ApprovalPresentationRecordStore`;
- opt-in local executor approval enforcement through `LocalExecutor::decide_approval_with_presentation(...)`;
- dogfood phase runner work-context fields;
- dogfood `approval_handoff` and `copy_safe_approval_request` emission;
- dogfood phase close event summaries.

Current gap:

- the dogfood runner emits the required approval presentation but does not persist it;
- the printed approval command cannot yet reference a persisted `presentation_id`;
- dogfood approvals still use the default approval command, so the exact presented handoff is not durably proven before approval.

## 5. Proposed First Implementation

Add proof persistence to `scripts/self-governed-benchmark.mjs` for `phase-start`.

Recommended behavior:

1. Build the same bounded work context currently used to emit `approval_handoff`.
2. After the workflow run reaches `WaitingForApproval`, construct a deterministic approval presentation payload from the exact emitted approval block.
3. Persist an `ApprovalPresentationRecord` through a small repo-local helper path.
4. Print:
   - `presentation_id`;
   - `presentation_content_hash`;
   - `presentation_record_persisted: true`;
   - a future enforcement-ready approval command variant.
5. Preserve the current explicit approval command for compatibility during the first persistence implementation, unless the reviewed implementation explicitly switches dogfood approval to the opt-in enforcement API.

The first implementation should prefer a Rust-backed helper or existing CLI/API path if available. If no public CLI path exists, add the narrowest repo-local helper necessary without creating a stable public command.

## 6. Presentation Content Policy

The persisted record should represent what the maintainer was asked to approve:

- workflow ID;
- phase;
- run ID;
- approval ID;
- approval reason;
- work summary;
- approved scope;
- strict non-goals;
- expected touched surfaces;
- validation required;
- why-now context;
- approval allows;
- approval does not allow;
- next action after approval;
- redaction note.

The record must not store:

- raw provider payloads;
- command output;
- raw spec contents;
- local filesystem secrets;
- tokens;
- private keys;
- unbounded chat transcript;
- browser state;
- model hidden reasoning;
- arbitrary terminal output.

The content should be bounded, deterministic, and validated through existing approval-presentation constructors.

## 7. Identity And Matching Rules

The persisted record must bind to:

- workflow ID;
- workflow version when available;
- schema version;
- run ID;
- approval ID;
- step ID when available;
- presentation channel;
- presented actor/system actor;
- presented timestamp;
- sensitivity;
- redaction metadata.

The runner should fail closed if it cannot derive the required identity fields from the started run and pending approval output.

## 8. Error Handling

Runner persistence errors must:

- use stable codes;
- avoid leaking run IDs, approval IDs, presentation IDs, handoff text, local paths, corrupt payloads, command output, provider payloads, or secret-like values;
- fail before presenting an enforcement-ready approval command;
- disclose whether the phase can continue only under the old non-enforced approval path.

Recommended conservative first behavior:

- if proof persistence fails, `phase-start` exits non-zero for material phases;
- it may still print the bounded diagnostic and next action;
- it must not fabricate a `presentation_id`;
- it must not auto-approve.

## 9. Approval Command Policy

The first implementation should not hide the approval boundary.

Preferred path:

- `phase-start` prints the existing explicit approval command;
- `phase-start` also prints an enforcement-ready approval command that includes or references `presentation_id`, if an execution path exists;
- a later reviewed phase can switch the benchmark default to the enforcement-ready command.

If the enforcement-ready approval command is implemented in the same phase, it must call the existing opt-in enforcement API and preserve compatibility with the current explicit approval posture.

## 10. Dogfood Runner Boundary

The runner remains governance coordination only.

It must not:

- approve automatically;
- run arbitrary shell commands on behalf of the kernel;
- edit repository files;
- perform git operations;
- open or merge PRs;
- write report artifacts;
- perform provider writes;
- execute side effects;
- change workflow schemas;
- claim hosted or production self-hosting behavior.

## 11. Test Plan

Future implementation tests should cover:

- `phase-start` persists an approval presentation record for material phases;
- persisted record matches the emitted `approval_handoff`;
- persisted record matches `copy_safe_approval_request` scope;
- `presentation_id` is printed;
- content hash is printed and deterministic;
- persistence happens before approval command output is treated as ready;
- missing work summary fails closed;
- missing approval ID fails closed;
- secret-like work context is rejected without leakage;
- proof persistence failure does not fabricate proof;
- no automatic approval occurs;
- no repo files, git state, report artifacts, provider state, or workflow schemas are mutated by proof persistence;
- existing phase-start/phase-close tests still pass;
- `npm run check:docs` passes.

If a Rust helper is added, also run the focused Rust tests that cover the new helper boundary.

## 12. Documentation Updates

Future implementation must update:

- `docs/user-guide/self-governed-build-benchmark.md`;
- `ROADMAP.md`;
- this plan;
- an end-of-phase implementation report.

Docs must say:

- dogfood runner approval-presentation persistence is implemented only after that phase lands;
- default approval behavior remains unchanged;
- automatic approval remains unsupported;
- CLI approval-card rendering remains unsupported;
- report artifacts, schemas, examples, provider writes, side effects, hosted behavior, reasoning lineage, and release posture changes remain unsupported.

## 13. Proposed Implementation Sequence

1. Add a small persistence helper for the dogfood runner that constructs/stores `ApprovalPresentationRecord` from explicit phase-start context.
2. Wire `phase-start` to persist proof after approval ID discovery and before printing final approval instructions.
3. Print `presentation_id`, content hash, and persistence posture.
4. Add focused tests for persistence, failure, non-leakage, and no automatic approval.
5. Review.
6. Only after review, plan switching dogfood approval commands to the opt-in enforcement API by default.

## 14. Open Questions

- Should `phase-start` fail closed if proof persistence fails, or allow an explicit compatibility-only fallback?
- Should the runner use a Rust helper binary path, a private CLI subcommand, or a Node wrapper over serialized model input?
- Should presentation IDs be deterministic from run/approval/context or generated uniquely per presentation?
- Should delegated-maintainer approvals require a freshness window immediately, or defer freshness until enforcement defaults are planned?
- How should future WorkReports cite approval-presentation proof?

## 15. Final Recommendation

Proceed next to dogfood runner approval-presentation persistence implementation, with fail-closed persistence for material phases if feasible.

Do not implement automatic approval, public approval-card UX, default approval enforcement, schemas, examples, provider writes, side effects, hosted behavior, reasoning lineage, or release posture changes.
