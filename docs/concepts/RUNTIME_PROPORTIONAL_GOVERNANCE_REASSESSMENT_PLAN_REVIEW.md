# Runtime Proportional-Governance Reassessment Plan Review

## 1. Executive Verdict

**Plan accepted; proceed to the pure immutable-bundle reassessment helper.**

The plan closes the right gap: the proportional-governance model is accepted,
but runtime work is not yet bound to a reassessment derived from immutable run
definitions and explicit runtime facts. The first implementation remains a
pure helper and does not prematurely authorize executor behavior.

## 2. Scope Verification

The plan stays within planning scope. It does not authorize implementation in
the planning phase and does not introduce default runtime behavior, schemas,
CLI or UI behavior, provider calls, provider writes, automatic approvals,
hosted operation, or enterprise administration.

## 3. Product-Feedback Assessment

The plan interprets the external feedback correctly:

- execution disposition and disclosure obligation remain independent axes;
- visible disclosure is not a separate blocking execution mode;
- an operator UI may display quiet decisions without changing authority;
- ordinary onboarding should derive most posture instead of requiring users to
  construct a complete decision input manually;
- explicit governance minima may hold or raise inferred posture but may never
  be weakened by inference;
- relevant changes should invalidate an accepted assessment.

The remaining gap is runtime composition and invalidation, not another redesign
of the accepted proportional-governance model.

## 4. Source-Of-Truth Assessment

Using a validated `StoredImmutableRunBundle` as the static source of truth is
appropriate. It prevents mutable project files from silently changing the
definitions assessed for a run. Explicit typed runtime facts remain necessary
for authority, evidence and checks, `SideEffect` reversibility, escalation,
prior posture, and steward minima.

The review added an exactness requirement: the pure helper must receive exactly
one matching runtime-fact record for each ordered workflow step. Missing,
duplicate, extra, or mismatched records fail closed.

## 5. Assessment And Fingerprint Boundary

The helper should reuse the accepted classifiers and workload assessment rather
than create a parallel governance interpretation. Shared derivation internals
may be factored over resolved canonical definitions if needed, but the helper
must not serialize and reparse definitions merely to manufacture a
`ProjectBundle`.

The aggregate assessment-set fingerprint is justified. The review clarified
that it must use a new versioned domain separator, fixed-width length framing,
a stable known test vector, and delimiter-collision tests. This preserves the
fingerprint hardening already established elsewhere in the kernel.

## 6. Freshness And Time-Of-Use Assessment

The pure helper can validate and fingerprint supplied facts; it cannot prove
their independent freshness. That limitation is now explicit. Trusted fact
references, validity windows, durable bindings, and time-of-use reassessment
remain later phases. The helper must report assessment, not freshness proof.

## 7. Runtime Sequencing Assessment

The sequence is appropriately conservative:

1. pure immutable-bundle reassessment helper;
2. maintainer review;
3. durable assessment-binding model and event vocabulary;
4. maintainer review;
5. one opt-in executor path before `RunCreated`;
6. retry and resume reassessment hardening;
7. complete local-path review;
8. only then consider defaults, schema, CLI, UI, or provider adoption.

This ordering responds directly to the approval/resume time-of-check and
time-of-use risk without widening provider mutation capability.

## 8. Privacy And Error Assessment

The plan keeps raw definitions, evidence, check output, provider payloads,
source content, paths, environment values, tokens, and credentials outside the
assessment result and errors. Stable codes and redacted `Debug` behavior remain
required. Fingerprints and identifiers are treated as sensitive operational
metadata.

## 9. Test-Plan Assessment

The planned tests cover deterministic ordering, immutable source use,
decision-relevant invalidation, irrelevant-change stability, monotonic minima,
independent disclosure behavior, stale fingerprint rejection, retry/resume
hardening, malformed bundle input, runtime-fact exactness, framing collisions,
known vectors, and non-leakage. Existing runtime and governance regressions
remain required in later integration phases.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Decide the minimal trusted reference and validity-window model before durable
  fact binding is implemented.
- Reuse or factor canonical-definition derivation logic so project onboarding
  and immutable-bundle reassessment cannot drift.
- Decide which local profile first enables the future opt-in executor path only
  after the helper and binding models pass review.

## 12. Recommended Next Phase

Implement the **pure immutable-bundle proportional-governance reassessment
helper only**.

Do not add executor enforcement, durable bindings, events, schemas, CLI, UI,
provider calls, writes, automatic approvals, enterprise administration, or
default runtime behavior in that phase.

## 13. Review Record

- Dogfood workflow: `dg/review`
- Run ID: `run-1784182704865928000-2`
- Approval ID:
  `approval/run-1784182704865928000-2/review-scope-approved`
- Approval outcome: granted with persisted presentation proof
- Event summary: 39 events, one approval, zero retries, zero escalations
- Out-of-kernel work: documentation inspection, review drafting, documentation
  validation, and diff inspection
