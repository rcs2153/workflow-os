# Authoritative Local-Check Reassessment Binding Plan Blocker Fix Review

## 1. Executive Verdict

Plan blocker fix accepted; proceed to the private authoritative local-check
reassessment binding implementation.

The corrected plan closes both original planning blockers without broadening
scope. It now requires complete deterministic wrapper preflight before any
clock or local process use and returns one private bound-assessment value whose
fact and reassessment identities cannot be consumed independently as authority.

## 2. Scope Verification

The fix stayed within planning-only scope.

It did not add:

- Rust implementation;
- executor integration or automatic local checks;
- runtime quiet-success activation or disclosure presentation;
- persistence, events, evidence, reports, or artifacts;
- schemas, CLI behavior, examples, providers, or OpenShell;
- SideEffects, writes, hosted behavior, or release changes.

## 3. Original Blocker Verification

The original review correctly rejected two boundaries:

1. deterministic wrapper mismatches could have been discovered only after
   local-check composition started a process; and
2. a raw assessment set plus separate fingerprint could let a future
   crate-internal caller ignore the binding.

Those findings remain preserved in the original review.

## 4. Complete Preflight Assessment

The corrected plan requires pure preflight of:

- exact stored bundle and workflow identity;
- selected-step membership and canonical declarations;
- immutable workflow, skill, and policy resolution;
- exactly one runtime-fact record for every immutable workflow step;
- selected-step runtime-fact uniqueness; and
- absence of caller-selected evidence/check posture for the selected step.

Only after that preflight succeeds may the accepted local-check composition
helper perform its own requirement and command preflight and start a process.

This is implementable against the current source. Immutable-bundle
reassessment already performs pure workflow, skill, policy, and exact
runtime-fact resolution. The implementation may factor that resolution without
duplicating its semantics. The existing local-check composition helper already
preflights its complete check universe before its first process.

The plan also requires the resulting aggregate fact's candidate identity to
match the exact preflighted bundle and step before reassessment. This prevents
late substitution of a different check universe.

## 5. Bound Authority Assessment

The corrected candidate model is appropriately narrow:

```text
AuthoritativeLocalCheckBoundAssessment {
  local_check_fact,
  assessment_set,
  binding_fingerprint,
}
```

The value remains crate-private, non-serializable, and read-only. It owns the
authoritative aggregate fact, complete immutable assessment set, and binding
fingerprint. The outer outcome does not expose the raw assessment set or fact
as independently reusable authority.

This is stronger than convention alone. A future consumer must accept the
bound value or use a separately reviewed durable projection. Compile-time
privacy and focused module tests can enforce the intended accessor surface.

## 6. Identity And Monotonicity Assessment

The binding inputs are sufficient and appropriately explicit:

- immutable bundle, workflow, run, and selected step;
- aggregate fact algorithm and complete fact identity;
- candidate-set and structural-coverage identities;
- selected workload-assessment identity; and
- complete assessment-set algorithm and aggregate identity.

The local-check fact replaces only the selected step's absent evidence/check
axis. Authority, sensitivity, SideEffect, prior-decision, runtime-escalation,
profile, policy, and steward minima remain under the accepted selector.

No check result can grant authority, approve a SideEffect, lower sensitivity,
or override denial.

## 7. Failure And Privacy Assessment

The corrected failure boundary is explicit:

- wrapper preflight failure uses no clock or process;
- local-check failure returns no reassessment or bound value;
- reassessment or binding failure returns no bound value; and
- the helper mutates no workflow state, persistence, events, evidence, report,
  artifact, provider, or external system.

Static error codes and redacted `Debug` remain required. Commands, paths,
output, source, environment, identities, fingerprints, credentials, and
natural-language report content remain excluded.

## 8. Test Quality Assessment

The corrected test plan covers both original blockers directly:

- every deterministic wrapper mismatch must fail before clock or process use;
- selected-step caller posture must fail closed;
- the output must not expose a raw assessment set as independent authority;
- equal complete inputs must produce equal binding identity;
- every binding input must independently invalidate identity;
- non-check governance axes must remain monotonic; and
- errors and `Debug` must remain non-leaking.

The planned stable vector and framing tests are appropriate for a new versioned
binding algorithm.

## 9. Blockers

None.

## 10. Non-Blocking Follow-Ups

- Keep the implementation selected-step and crate-private.
- Keep runtime facts for other steps explicit without calling them
  authoritative local-check facts.
- Review any future durable projection separately.
- Plan executor consumption only after the private implementation passes
  phase-level review.

## 11. Governed Review Record

- workflow: `dg/review`
- run: `run-1785020026396294000-2`
- approval:
  `approval/run-1785020026396294000-2/review-scope-approved`
- presentation: `presentation/87c127a0b98e6219`
- approval outcome: granted by delegated maintainer through proof enforcement
- review status: completed and accepted
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- validation: `npm run check:docs` and `git diff --check`
- out-of-kernel work: source inspection, architecture review, documentation
  updates, and validation
- missing coverage: the kernel coordinated governance only; it did not perform
  architecture analysis, implement code, or generate a WorkReport artifact

## 12. Recommended Next Phase

Implement the accepted private authoritative local-check reassessment binding.

Do not combine implementation with executor integration, automatic checks,
runtime quiet-success activation, persistence, events, schemas, providers,
OpenShell, SideEffects, writes, hosted behavior, or release changes.
