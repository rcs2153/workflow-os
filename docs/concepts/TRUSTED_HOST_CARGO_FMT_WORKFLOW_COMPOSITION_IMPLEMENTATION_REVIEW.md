# Trusted-Host Cargo Format Workflow Composition Implementation Review

## 1. Executive Verdict

**Needs blocker fixes.**

The additive three-step shape, closed profile registry, immutable bundle,
bounded host loop, terminal result truth, and v4 artifact path are directionally
correct. Existing one-step and two-step behavior remains intact, and ordinary
success, bounded failure, restart, replay, and pre-result integrity scenarios
pass.

The phase cannot yet be accepted because resumed admission does not validate
the complete authorized-dispatch history promised by the plan, malformed
failed terminal history can pass the three-step history validator, and the
pre-result report does not prove that its failed invocation is the invocation
authorized by the matching dispatch. The mandatory dispatch-substitution and
successor-boundary corruption tests are also absent.

This verdict does not authorize another profile, broader topology, providers,
writes, public CLI behavior, schemas, hosted behavior, or release changes.

## 2. Scope Verification

The implementation stayed within the approved private, local, no-write scope.
It added one exact three-step sibling path and did not generalize profile
registration, add arbitrary commands, expose a released CLI contract, enable
provider access, add external writes, or broaden execution topology.

The implementation report accurately preserves the trusted-host limitations:
there is no operating-system sandbox, final-state equivalence cannot detect a
transient restored write, and executable identity is self-committed rather
than independently attested.

## 3. Registry And Admission Assessment

`TrustedHostThreeStepNoWriteProfileRegistry` is closed to the exact project
validation, docs-check, and Cargo-format profiles. Construction verifies the
canonical profile identities and explicit-zero authority posture. Inventory
generation preserves the approved order and exposes no arbitrary map or
caller-selected command.

Fresh admission validates the exact plan, persists the complete immutable
bundle, and prepares only the first Core-derived operation. Resumed admission
loads the exact stored bundle and derives the current step from durable state.
The caller cannot select the step, profile, command, successor, result, or
report identity.

The admission boundary is blocked, however, by incomplete history validation
described below.

## 4. Topology And Progression Assessment

The accepted topology is represented exactly:

```text
project-validation -> docs-check -> cargo-format-check -> terminal
```

The host loop is bounded to three operations. Passing non-terminal results use
the existing atomic continuation path to commit result truth, invocation
terminal truth, and the exact successor schedule. Failure does not schedule a
later step. Restart tests demonstrate continuation after the first and second
commits, and terminal replay does not rerun a process.

Appending Cargo format is the correct compatibility-preserving order. The
existing two-step prefix and v3 report path remain unchanged.

## 5. Authorized Dispatch And History Assessment

### Blocker 1: resumed admission does not validate attempted-step dispatches

`validate_trusted_host_three_step_history` validates schedule, result,
invocation-terminal, and successor order, but it never examines
`AuthorizedExecutionAttemptDispatchAdmitted` events. Resumed admission calls
that validator before preparing the next step. As a result, duplicate,
substituted, missing, or reordered dispatch history for an already attempted
step is not rejected at the promised exact-history boundary before later work
can be admitted.

The blocker fix must make admission validate exactly one dispatch per attempted
step, its causal position, its immutable invocation commitment, and its durable
executor commitment under the accepted dispatch-binding policy. Unexpected
additional dispatches must fail closed rather than being ignored.

### Blocker 2: failed terminal history is under-validated

The three-step history validator requires three results for `Completed`, but it
does not require a `Failed` run to end in either:

- one bounded non-passing result with a matching failed invocation; or
- the exact pre-result shape of schedule, matching dispatch, matching failed
  invocation, and run failure.

A terminal failed run with a scheduled step and no result can therefore pass
history validation without proving the required pre-result failure events.
Report composition may reject some such shapes later, but terminal replay and
resumed admission must fail closed at the history boundary itself.

## 6. Pre-Result Failure Truth Assessment

The implementation correctly avoids fabricating a `LocalCheckResultRecorded`
event after an integrity failure and discloses that no bounded result was
accepted. It also uses the durable dispatch executor commitment rather than
claiming that a subsequently mutated executable can reproduce the admitted
identity.

### Blocker 3: failed invocation is not bound to the authorized attempt

For the pre-result branch, report composition selects exactly one
`SkillInvocationFailed` event by step ID and event order only. It does not prove
that the failed invocation is the invocation requested for, and authorized by,
the selected dispatch. A same-step failure with a different invocation
identity can therefore satisfy the report branch.

The blocker fix must establish the complete causal identity chain for the
pre-result attempt. It must reject missing, duplicate, or substituted
invocation-request and invocation-terminal identity without creating a result
or inferring an unpersisted failure cause.

## 7. V4 Report And Artifact Assessment

For accepted result paths, the v4 report binds the immutable bundle, exact
ordered step and profile vocabulary, command identity, dispatch event,
durable executor commitment, result ID, invocation terminal, and terminal run
event. Stable result references are emitted in workflow order. Event citations
are causal. Create-only artifact persistence and exact duplicate reconciliation
reuse the accepted artifact gate.

The report path is not accepted until the pre-result invocation linkage above
is fixed and tested. The schedule event is validated during composition and
the scheduled step ID is committed, but the review recommends also committing
and citing the schedule event ID so the report's statement that it binds the
scheduled step is literal and independently inspectable.

## 8. Replay And Recovery Assessment

Ordinary restart after the first and second successful commits is covered.
Ordinary terminal replay returns without running a process, and missing
artifact recovery uses the private create-only path.

Corrupt-history replay is not sufficiently covered for the new topology. The
review found no three-step tests for dispatch substitution, duplicate dispatch,
or causal corruption at either successor boundary. Existing two-step and
generic corruption tests do not prove the new three-step validator.

## 9. Privacy Assessment

The new public Debug implementations remain bounded and redacted. Stable error
families do not include paths, process output, command transcripts, source or
toolchain material, environment values, or credentials. Reports cite stable
references and do not create `EvidenceReference` values implicitly.

No privacy blocker was found in the reviewed happy and bounded-failure paths.
The blocker regressions must assert that rejection errors and serialized
surfaces remain non-leaking.

## 10. Test Quality Assessment

The focused tests cover:

- exact successful order and three dispatches;
- first-, second-, and third-step bounded failure;
- caller-selected step rejection before state creation;
- restart after the first and second committed results;
- terminal replay;
- Cargo pre-result identity failure without a fake result;
- v4 artifact creation and stable result citations; and
- existing one-step and two-step compatibility through the workspace suite.

The required plan matrix remains materially incomplete. Missing focused
regressions include:

- dispatch executor-commitment substitution at admission, history validation,
  and report composition;
- duplicate or unrelated dispatch events;
- malformed failed terminal history with no accepted result and no exact
  pre-result chain;
- pre-result failed-invocation identity substitution;
- causal corruption at both successor boundaries;
- explicit result/profile/command/bundle/invocation substitution in the new
  three-step path;
- timeout behavior in the composed third step;
- conflicting v4 artifact posture; and
- comprehensive new-path Debug, event, report, artifact, path, and raw-output
  non-leakage assertions.

These are blocker gaps because they protect security-sensitive claims made by
the accepted plan, not optional test-depth improvements.

## 11. Documentation Assessment

The plan and implementation report correctly describe the private no-write
boundary and its operating-system, transient-write, and attestation
limitations. The implementation report overstates acceptance when it says
complete causal-history validation and exact pre-result dispatch/invocation
binding are implemented. This review and the roadmap now record those claims
as blocked pending fix-forward work; the original implementation report is
preserved as the phase author's record.

## 12. Blockers

1. Validate exactly one causally and cryptographically matching authorized
   dispatch for every attempted step during resumed admission and terminal
   history validation.
2. Require failed terminal runs to match either the exact bounded-result
   failure shape or the exact pre-result failure shape.
3. Bind the pre-result failed invocation to the invocation request and
   authorized dispatch for that attempt.
4. Add the missing dispatch-substitution, successor-corruption, substitution,
   timeout, artifact-conflict, and privacy regression matrix required by the
   accepted plan.

## 13. Non-Blocking Follow-Ups

- Include schedule event IDs in v4 report commitment material and citations so
  the scheduled-step binding is directly inspectable.
- Keep the exact sibling implementation until the accepted three-step path has
  enough evidence to justify any private helper extraction.
- Preserve the distinction between trusted-host identity/final-state checks
  and operating-system containment.

## 14. Validation

The implementation revision reviewed here had already passed the complete
workspace validation matrix and the opt-in bundled-toolchain Cargo-format
smoke recorded in the implementation report. Review documentation is validated
separately at phase close.

## 15. Governed Review Record

- Workflow ID: `dg/review`
- Run ID: `run-1792683652708766000-2`
- Approval ID:
  `approval/run-1792683652708766000-2/review-scope-approved`
- Presentation ID: `presentation/2b33d7a42861c2a8`
- Presented content hash:
  `2b33d7a42861c2a87261a6d92a1f753833d6debb685c031c763cff9d72dd4e01`
- Approval outcome: granted under delegated-maintainer authority for the exact
  persisted review-only scope
- Phase status: `Completed`
- Event summary: 39 events, one approval, zero retries, zero escalations, one
  persisted presentation record, and a present approval-event proof marker
- Out-of-kernel work: source, test, report, artifact, and runtime inspection;
  review writing; and validation were performed by the delegated maintainer.
  The kernel governed scope and approval but did not perform those actions.

## 16. Recommended Next Phase

Implement a narrow trusted-host Cargo-format workflow composition blocker fix.
Do not add another check profile, broaden topology, expose a public surface,
enable providers or writes, or begin the later capability-authority lane until
this exact three-step history and report boundary passes focused review.
