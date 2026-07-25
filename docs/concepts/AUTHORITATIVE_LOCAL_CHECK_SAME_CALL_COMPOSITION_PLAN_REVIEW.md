# Authoritative Local-Check Same-Call Composition Plan Review

## 1. Executive Verdict

**Plan accepted; proceed to the private same-call composition helper.**

The plan defines the smallest safe runtime composition boundary from canonical
stored local-check declarations through existing `DocsCheck` execution,
attestation, freshness gating, exact structural coverage, and authoritative
aggregate-fact conversion.

## 2. Scope Verification

The plan stays within planning scope. It does not authorize:

- executor wiring or automatic local-check execution;
- proportional-governance selector invocation or quiet-success behavior;
- workflow, policy, or report schema changes;
- CLI, UI, onboarding, or example behavior;
- persistence, events, evidence records, reports, or artifacts;
- provider execution, OpenShell integration, SideEffects, or writes;
- hosted or distributed behavior; or
- automatic approvals, defaults, or release changes.

The proposed helper remains crate-private, explicit, local, sequential, and
unwired.

## 3. Source-Of-Truth Assessment

The plan correctly derives the obligation universe from validated
`StoredImmutableRunBundle` canonical declaration records. It does not accept
mutable project files, inferred commands, caller counts, or detached posture
enums as authority.

The helper must preserve one important implementation invariant: complete
batch preflight must compare each supplied attestation requirement and handler
command-contract fingerprint with the matching canonical declaration record,
not only with the derived obligation fingerprint. The current canonical record
retains both fingerprints, so this can be implemented without broadening the
model.

## 4. Preflight Assessment

The plan requires all deterministic checks to finish before the first process
starts. That includes:

- stored-bundle, workflow, run, and step identity;
- authoritative declaration-source completeness;
- exact requirement identity;
- exact command-contract identity;
- supported handler family;
- duplicate and unexpected execution inputs; and
- canonical execution ordering.

This is the correct fail-closed boundary. Preflight must operate over the full
batch. Per-item validation immediately before each invocation is insufficient
because an earlier process could run before a later mismatch is discovered.

## 5. Execution And Freshness Assessment

The accepted
`execute_docs_check_governance_contribution(...)` path remains the sole owner
of process execution, observation, attestation verification, gate consumption,
and leaf contribution. Calling it exactly once per preflighted input preserves
the existing same-call freshness boundary.

Sequential execution in canonical obligation order is appropriate for v1.
The plan is honest that a later execution error may occur after earlier
non-source-writing checks have run. It does not claim transactional rollback,
return a partial aggregate fact, or execute later checks after the error.

## 6. Requirement-Level Assessment

The authoritative path removes caller-selected required-versus-optional
posture. A focused Core-owned adapter derives the level from the matching
canonical obligation.

The resulting semantics are correct:

- omitted required obligations become `RequiredUnavailable`;
- omitted optional obligations become `OptionalUnavailable`;
- executed optional failures remain `Failed`; and
- a canonical authoritative empty set may become `Satisfied`.

Missing, incomplete, legacy, or unresolved declaration sources still fail
closed and are not equivalent to an authoritative empty set.

## 7. Aggregate Authority Assessment

The plan preserves exact structural coverage and returns the existing
`AuthoritativeLocalCheckEvidenceCheckFact`, including exact counts and
candidate, coverage, and fact fingerprints. It does not return a detached
`GovernanceWorkloadEvidenceCheckPosture` as reusable authority.

The strict precedence remains:

```text
Failed
  > RequiredUnavailable
  > OptionalUnavailable
  > Satisfied
```

This is compatible with future proportional-governance composition without
invoking that selector in this phase.

## 8. Privacy And Error Assessment

The proposed errors are stable, bounded, and static. `Debug` and errors must
not expose:

- workflow, run, step, invocation, or result identities;
- commands, arguments, paths, or captured output;
- requirement, contract, candidate, coverage, or fact fingerprints;
- provider data, environment values, credentials, or tokens.

The aggregate fact remains payload-free. Bounded `LocalCheckResult` values may
be returned only through their existing validated and redaction-safe model.

## 9. Test Assessment

The planned tests cover the important behavioral boundaries:

- completed populated and canonical-empty sets;
- required and optional omission;
- optional executed failure;
- duplicate, unexpected, cross-bundle, cross-run, and cross-step inputs;
- requirement and command-contract mismatch before any process starts;
- canonical ordering independent of caller order;
- exact one-call execution through the accepted contribution path;
- freshness expiry;
- mid-batch execution failure with no fact and no later execution;
- caller inability to select requirement level;
- deterministic aggregate identity;
- privacy-safe errors and `Debug`; and
- regressions across runtime, declaration, structural coverage, aggregate
  conversion, and proportional-governance models.

Implementation tests should use an invocation-counting handler so the
full-batch preflight guarantee is observable, including a mismatch placed
after an otherwise valid first input.

## 10. Planning Blockers

None.

## 11. Non-Blocking Follow-Ups

- Keep canonical declaration-record access private; expose only the minimum
  accessor or adapter required by the composition helper.
- Return results in canonical execution order so output ordering and aggregate
  derivation cannot diverge.
- Keep the current result-on-error decision explicit: no partial result vector
  or aggregate fact is returned, even though earlier checks may have executed.
- Reassess runtime integration only after this private helper passes its own
  maintainer review.

## 12. Governed Review Record

- workflow: `dg/review`
- run: `run-1785013819087649000-2`
- approval:
  `approval/run-1785013819087649000-2/review-scope-approved`
- presentation: `presentation/14dcae289f4de108`
- approval outcome: granted by delegated maintainer through proof enforcement
- review status: completed
- event summary: 39 events, one approval, zero retries, zero escalations
- validation: `npm run check:docs` and `git diff --check`
- out-of-kernel work: source and plan inspection, review authoring,
  documentation validation, and diff validation
- missing coverage: the kernel coordinated governance only; it did not perform
  the maintainer analysis, author the review, or generate a WorkReport artifact

## 13. Recommended Next Phase

Implement the crate-private same-call composition helper and focused tests
only.

Do not combine that implementation with executor wiring, automatic check
execution, proportional-governance invocation, schemas, CLI behavior,
persistence, events, evidence records, reports, artifacts, providers,
OpenShell, SideEffects, writes, hosted behavior, or defaults.
