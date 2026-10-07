# Trusted-Host Local Application Admission Plan Blocker Fix Review

## 1. Executive Verdict

**Blocker fix accepted; proceed to the trusted-host local application admission
vertical slice.**

The corrected plan now identifies a real process owner, a bounded
current-authority source for the exact no-authority profile, and a typed result
that preserves lawful workflow posture. The first implementation remains
unpublished, SQLite-only, synchronous, no-write, and restricted to the closed
docs-check profile.

## 2. Review Scope

This review assessed:

- the selected unpublished binary owner;
- process, backend, handler, and cancellation lifetime;
- current-authority inventory provenance and completeness;
- distinction between no required facts and fabricated authority;
- admission outcomes for prepared, waiting, terminal, blocked, denied, and
  failed posture;
- fresh, existing, and waiting-run re-entry;
- privacy and failure behavior;
- production-shaped proof; and
- scope discipline.

It did not implement or authorize broader runtime behavior.

## 3. Foreground Owner Assessment

The owner is now concrete: the unpublished `workflow-local-host` binary target.
It owns `main`, process exit, one SQLite backend, one resolved closed handler,
and cancellation custody. The end-to-end test must invoke that target.

This resolves the prior ambiguity. A package-private helper may support the
binary, but cannot replace it as the process owner. The target remains outside
the released `workflow-os` CLI and carries no stable compatibility promise.

## 4. SQLite Lifecycle Assessment

The plan correctly requires one backend instance from open through consumed
operation return. Schema and health validation occur before admission.
Unavailable, unhealthy, and incompatible databases fail before project or
handler execution. Migration, repair, fallback, filesystem translation, and
PostgreSQL reuse remain prohibited.

## 5. Current-Authority Assessment

The no-authority projection is acceptable only because it is narrower than a
grant. Core may create complete empty inventories when the exact immutable
required-context contract and the exact closed profile explicitly require:

- zero capability grants;
- zero capability-availability facts; and
- zero governed-context references.

The implementation must encode that zero requirement explicitly in the
private closed-profile contract. It must not infer zero from an absent field,
an empty caller vector, a missing registry, or lack of current integrations.

Policy, approval, evidence, checks, operational-window authority, and dispatch
reservation remain independent. Empty current-authority inventories cannot
bypass them. Any non-empty, unknown, stale, incomplete, optional-but-policy-
significant, or conflicting requirement blocks before preparation.

The application cannot provide or amend the source. Additional profiles need
a separately reviewed production authority source.

## 6. Admission Result Assessment

The typed result correctly separates:

- `Prepared` with the opaque one-shot pair;
- `Waiting` with a fixed bounded reason;
- `TerminalReplay`;
- `Blocked` with a fixed bounded reason;
- `Denied`; and
- fixed application failure through `Result`.

This prevents healthy wait or terminal replay from being mislabeled as an
application error. Only `Prepared` may enter `LocalHostPreparedOperation`.

## 7. Fresh, Existing, And Re-Entry Assessment

The plan preserves the correct behavior:

- fresh runs establish durable validated state before operational preparation;
- policy and approval stops occur before preparation;
- existing runs are selected explicitly, never discovered;
- terminal replay invokes nothing;
- ambiguous or substituted state fails closed; and
- a waiting run re-enters only after a separate governed operation changes
  durable state and the binary is explicitly invoked again.

Admission does not poll, wait interactively, or grant approval.

## 8. Executor And Input Assessment

The application selects only the one enum-valued closed profile. Core resolves
the handler, verifies the immutable step identity, constructs `SkillInput`,
constructs the private attempt executor, and binds the canonical command
contract into its commitment.

No arbitrary command, argument, JSON payload, environment value, copied event,
mock handler, or caller-provided executor is accepted.

## 9. Cross-Crate, Privacy, And Failure Assessment

The existing opaque prepared pair remains the only authority-bearing value
crossing into local host. It remains non-cloneable, non-serde, lifetime-bound,
one-shot, and redacted. Application results and failures remain fixed and
payload-free.

No identifiers, paths, commands, arguments, policy text, payloads, outputs,
credentials, or private Core diagnostics may appear in application-visible
Debug, Display, or errors.

## 10. Test Assessment

The corrected test plan is sufficient. It requires actual binary invocation,
fresh and existing operation, approval and policy stops, terminal replay,
substitution rejection, cancellation, state drift, at-most-once dispatch,
authority completeness/freshness failures, backend failures, privacy, and
unchanged CLI/hosted behavior.

Direct calls to the private preparation function, a test-only issuer, or an
internal helper without binary invocation do not satisfy the end-to-end proof.

## 11. Scope Verification

The blocker fix remained documentation-only. It added no Rust, target,
command, source, issuer, runtime configuration, released CLI behavior, hosted
behavior, discovery, scheduling, provider access, mutation, OpenShell, nested
harnesses, schemas, examples, or release changes.

## 12. Blockers

None for beginning the bounded implementation vertical slice.

## 13. Non-Blocking Follow-Ups

- Keep the explicit zero-authority contract private and closed to docs-check.
- Choose existing durable event vocabulary for lawful pre-preparation stops;
  add vocabulary only if current events cannot state the truth.
- Retain the resolved handler safely for the prepared-session lifetime without
  widening public handler APIs.
- Keep process restart, general authority registration, and public operator
  UX separate.

## 14. Recommended Next Phase

Implement the complete trusted-host local application admission vertical slice
as one phase:

```text
unpublished workflow-local-host binary
  -> Core admission over one SQLite backend
  -> explicit zero-authority docs-check contract
  -> opaque prepared pair
  -> LocalHostPreparedOperation
  -> synchronous bounded result
```

Do not split out a standalone source or issuer. Do not broaden profiles,
authority, released CLI behavior, hosted behavior, discovery, scheduling,
providers, writes, OpenShell, nested harnesses, schemas, or release posture.

## 15. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 16. Governed Review Record

- workflow: `dg/review`
- run: `run-1791390933694246000-2`
- approval: `approval/run-1791390933694246000-2/review-scope-approved`
- presentation: `presentation/c4afb34f6ec32bb1`
- presentation hash:
  `c4afb34f6ec32bb1b41952f57154fc36b28f005f320132531780630fd2d71e0b`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: focused documentation-only blocker-fix review
- phase status: completed
- event summary: 39 ordered events, one approval, zero retries, and zero
  escalations; approval-presentation proof was enforced with one presentation
  record
- validation summary: documentation and diff checks passed; Rust checks were
  not run because the reviewed phase changed documentation only
- out-of-kernel work: source and plan inspection, review authoring, validation
  commands, and later git or pull-request work
- report posture: no runtime WorkReport artifact was generated or persisted
