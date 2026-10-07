# Trusted-Host Local Application Preparation Source Plan Review

## 1. Executive Verdict

**Needs planning blocker fixes. Do not implement the standalone source
bridge.**

The proposed opaque source preserves authority and dependency direction, but
its first implementation explicitly has no production issuer or caller. That
would add a production type, private issuer, local-host method, and test
surface that no real code can use. It would satisfy packaging semantics while
leaving the runtime-composition gap unchanged.

The plan must identify one actual Core-owned issuance and embedding boundary
before implementation. The corrected first code slice should prove one
end-to-end explicit source-to-operation path, while remaining local, injected,
non-discovering, and non-scheduling.

## 2. Scope Verification

The plan stays within planning scope and does not authorize implementation,
provider behavior, OpenShell, nested harnesses, CLI, schemas, hosted adoption,
or release change. Its authority and privacy constraints are appropriately
conservative.

The blocker is not scope expansion. It is that the proposed implementation is
too detached from a real runtime owner to satisfy the repository engineering
standard or reduce the documented gap between primitives and execution.

## 3. Dependency Direction Assessment

The dependency decision is correct:

- `workflow-local-host` depends on Core;
- Core must not depend on `workflow-local-host`;
- Core must remain the only preparation and authority issuer; and
- the local-host package may consume only an opaque source or prepared pair.

The source therefore belongs in Core. A concrete local-host convenience method
may consume it without receiving private preparation fields.

## 4. Authority Boundary Assessment

An opaque, unconstructible, one-shot source is a sound authority-custody shape.
It should privately own exact already-resolved preparation input and project
private errors to the existing bounded application failure vocabulary.

The plan correctly rejects public ID-based resolution, caller-authored
locators, backend handles, current authority, executor internals, and skill
bindings. Successful source issuance must remain weaker than successful
preparation, which remains weaker than consumed execution.

## 5. Blocking Finding: No Production Issuance Boundary

The proposed first implementation adds a crate-private issuer only for tests
and deliberately has no production call site. This creates three problems:

1. The source is unused production abstraction, contrary to the engineering
   standard's prohibition on speculative or unused abstractions.
2. Local-host tests still cannot prove the real Core-backed path without a
   synthetic seam, so the phase adds vocabulary more than runtime proof.
3. The roadmap would advance without answering the decisive ownership
   question: which process already possesses the exact backend, current
   authority, continuation posture, executor binding, and skill input required
   to issue the source?

This is a blocker because the project has explicitly prioritized composing
existing primitives into enforceable runtime paths instead of adding more
model-only layers.

## 6. Required Planning Correction

The blocker-fix plan must select one concrete owner and call path. It must
answer:

- Which existing or newly scoped local process owns the SQLite backend?
- How does Core receive an exact target without general discovery or arbitrary
  ID-based selection?
- Where do the attempt executor and validated `SkillInput` come from?
- How are fresh and existing continuation postures obtained without caller-
  authored authority?
- At what exact function does Core issue the opaque source?
- Which actual embedding component passes that source to
  `workflow-local-host`?
- How is one end-to-end test run without a public authority factory?
- What remains injected and explicit so the path does not become scheduling?

If no existing component can answer those questions, the next plan must select
the minimal embedding application boundary before source implementation.

## 7. Failure And Privacy Assessment

The proposed failure and privacy posture is accepted:

- source construction and drop are zero-write;
- source consumption occurs at most once;
- no pair escapes on preparation failure;
- private `WorkflowOsError` becomes fixed application failure before crossing
  Core;
- source and pair are non-cloneable, non-serde, and Debug-redacted; and
- no identifiers, paths, payloads, credentials, or provider values cross the
  boundary.

These requirements must survive the corrected operational slice.

## 8. Test Strategy Assessment

Core, package-private, external, and compile-fail layers are appropriate, but
they are insufficient without one real source-to-operation path. The corrected
plan must add a production-shaped integration test that uses the selected
issuer and embedding boundary, proves one consumed operation, and verifies
zero writes on pre-issuance failure and drop.

A test-only public factory remains prohibited.

## 9. Blockers

1. No actual Core-owned source issuance call site is selected.
2. No embedding component that receives and consumes the source is selected.
3. The first implementation would add unused production abstractions without
   an end-to-end runtime path.

## 10. Non-Blocking Follow-Ups

- Preserve the question of whether `from_prepared` remains visible after a
  real source path exists.
- Add checked-in default-feature negative fixtures when the corrected slice is
  implemented.
- Prove any future `Send`, `Sync`, signal, or cross-thread claim explicitly.
- Keep the fresh substitution non-mutation follow-up in Core coverage.

## 11. Recommended Next Phase

Perform a preparation-source planning blocker fix. Select the minimal actual
local embedding owner and one explicit Core issuance call site, then redefine
the first implementation as one end-to-end source-to-operation vertical slice.

Do not implement code until that plan is reviewed. Do not broaden into
discovery, scheduling, provider behavior, OpenShell, nested harnesses, hosted
parity, public configuration, or release change.

## 12. Validation

- `npm run check:docs`
- `git diff --check`

Runtime checks are not required because this review changes documentation
only.

## 13. Governed Review Record

- workflow: `dg/review`
- run: `run-1791389314393445000-2`
- approval:
  `approval/run-1791389314393445000-2/review-scope-approved`
- presentation: `presentation/0225375ea0f5aef9`
- presentation hash:
  `0225375ea0f5aef9ef028b31efb212bec13fc7bc6d694f88a280b0cf35c801e7`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: focused preparation-source plan review only
- phase status: completed
- event summary: 39 ordered events, one approval, zero retries, and zero
  escalations; approval-presentation proof was enforced with one presentation
  record
- validation summary: `npm run check:docs` and `git diff --check` passed
- out-of-kernel work: architecture, code, dependency, and plan inspection;
  review authoring; documentation validation; and later git or pull-request
  work
- report posture: no runtime WorkReport artifact was generated or persisted
