# Current Authority Same-Call Time-Of-Use Resolver Plan Review

## 1. Executive Verdict

**Plan accepted; proceed to the private test-only same-call resolver
implementation.**

The plan preserves the accepted distinction between caller-owned commitment
vocabulary and Core-owned source completeness. It does not expose a public
authority shortcut, and it keeps target dereference and runtime integration
separate.

## 2. Scope Verification

The phase stayed within planning and documentation scope.

It did not implement:

- a resolver;
- target dereference;
- runtime or executor wiring;
- a public authority source;
- persistence, events, reports, receipts, or artifacts;
- providers, OpenShell, sandbox execution, or external mutation;
- schemas, SDKs, CLI, UI, or examples;
- SideEffect execution or writes;
- hosted behavior, enterprise administration, or lineage; or
- release changes.

## 3. Trust-Boundary Assessment

The plan correctly identifies the central hazard:
`CurrentAuthorityFactSet::new` validates a caller-supplied commitment but does
not prove trusted completeness.

The proposed first implementation remains private, test-only, and co-located
with the accepted Core-owned source. Callers cannot pass an arbitrary fact set
and receive `Ready`. This is the correct first boundary.

No new readiness type should be exported from `workflow-core` in the first
implementation.

## 4. Same-Call Assessment

The required sequence is complete and appropriately ordered:

1. validate exact binding and contract;
2. derive the exact query set;
3. query complete Core-owned authority and reference inventories;
4. resolve current capability authority;
5. reconstruct fresh projection candidates;
6. project by access level;
7. rerun required-context consumption; and
8. derive one payload-free assessment.

Prior resolutions, projections, and consumption results are explicitly
excluded. No reusable lease or cache is introduced.

## 5. Source And Reference Completeness

The plan accounts for the missing reference source required by projection. A
private complete reference inventory is appropriate for the test-only proof.

The inventory must commit the full owned reference set, reject duplicate
targets, and select exactly one reference per contract target. It must not
accept an already-filtered caller slice as complete.

This proves composition mechanics only. It does not establish production
source trust.

## 6. Capability Resolution Assessment

The plan correctly keeps `resolve_capability_authority` as the source of truth
for:

- availability posture;
- grant specificity;
- lifecycle;
- expiry;
- revocation;
- sensitivity; and
- independent prerequisites.

The implementation may extract one crate-private actor/execution-scope
matching predicate for source filtering, but it must not duplicate or replace
the resolver's specificity and terminal-posture logic.

## 7. Required-Context Assessment

The plan preserves:

- exact typed target and access-level matching;
- exact actor/workflow/run/step/harness context;
- required-gap blocking;
- optional-gap disclosure;
- maximum-sensitivity enforcement; and
- rejection of undeclared or duplicate context.

Grouping fresh candidates by declared access level before projection is
consistent with the current projection API.

## 8. Independent-Prerequisite Assessment

The plan correctly treats policy, approval, evidence, and check references as
unresolved independent obligations.

`RequiresIndependentEvaluation` never produces projected authority. It blocks
when the affected contract requirement is required and remains an explicit
non-blocking gap when the requirement is optional. IDs alone do not prove
decisions or accepted evidence. No caller boolean or model judgment can weaken
this posture.

Trusted prerequisite facts remain a later separately reviewed phase.

## 9. Readiness Semantics

`Ready` is narrowly defined as current payload-free contract satisfaction
under the exact immutable binding and complete private source facts.

It is not:

- a target-existence guarantee;
- a payload-integrity guarantee;
- a dereference lease;
- provider authority;
- sandbox policy;
- tool execution; or
- write permission.

This distinction is explicit enough for implementation.

## 10. Determinism And Privacy Assessment

The planned result commits every decision-relevant source and output with
fixed-width framing and a versioned domain separator.

The privacy boundary is adequate. No raw target data, repository content,
provider output, command output, credentials, approval prose, policy input,
check output, sandbox data, or paths are permitted. Debug and errors remain
bounded and non-leaking.

## 11. Test Quality Assessment

The planned tests cover:

- positive same-call readiness;
- required and optional gaps;
- availability and reference failures;
- grant lifecycle, specificity, scope, and sensitivity;
- all independent prerequisite families;
- contract, binding, time, and source substitution;
- rejection of caller-owned public fact sets;
- canonical ordering and hashing;
- privacy and non-dereference; and
- full regression coverage.

The matrix is phase-ready.

## 12. Product And Roadmap Assessment

The plan supports the proportional-governance and quiet-success direction
without prematurely changing user-facing behavior.

Fresh-pull feedback says the product now explains itself and should reduce
low-risk ceremony. This resolver is a prerequisite for doing that safely:
quiet execution must rely on current Core-owned authority, not a stale or
caller-asserted result.

## 13. Blockers

None after correcting one review finding: the draft plan initially said every
unresolved independent prerequisite blocked the overall assessment, which
contradicted accepted optional-requirement semantics. The corrected plan never
projects unresolved authority, blocks required requirements, and retains
optional requirements as explicit non-blocking gaps.

## 14. Non-Blocking Follow-Ups

- A production source will need authenticated source identity, freshness,
  snapshot/high-watermark semantics, concurrency, retry, and operational
  failure behavior.
- Policy, approval, evidence, and check prerequisites need independently
  trusted fact sources.
- A future production reference source must define target-existence and
  reference-freshness semantics.
- One-time-use or replay-prevention semantics should be decided before target
  dereference.

## 15. Implementation Guardrails

- Keep all new resolver and reference-source vocabulary under `#[cfg(test)]`.
- Do not export a readiness API.
- Do not accept `CurrentAuthorityFactSet` as a public authoritative input.
- Keep the existing capability resolver authoritative for specificity and
  terminal posture.
- Return valid negative facts as `Blocked`; reserve errors for malformed or
  inconsistent invocation.
- Do not add target dereference or runtime behavior.

## 16. Recommended Next Phase

Implement the private test-only same-call resolver exactly as planned.

After focused implementation review, plan one production source boundary
before selecting any read-only runtime consumer.

## 17. Validation

- `npm run check:docs`: passed with the repository Node 20 toolchain.
- `git diff --check`: passed.
- Plan claims were checked against current capability, projection,
  required-context, immutable-binding, fact-set, and private-source APIs.

## 18. Dogfood Governance

- workflow: `dg/review`
- run ID: `run-1785153082518012000-2`
- approval ID:
  `approval/run-1785153082518012000-2/review-scope-approved`
- presentation ID: `presentation/52be483e71e73efa`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted review handoff was presented
- phase status: completed
- event posture: approval-presentation proof persisted and enforced
- out-of-kernel work: source inspection, review writing, documentation edits,
  and validation were performed by the delegated maintainer; the kernel
  governed scope and approval but did not edit files, run checks, or mutate git
