# Runtime Proportional-Governance Retry/Resume Reassessment Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The explicit opt-in executor path now closes the intended retry and
approval-resume integrity gap. Exact retries and approval decisions re-read the
stored immutable bundle, recompute the proportional-governance assessment from
current typed facts, and require exact equality with both the snapshot and
durable assessment binding before rehydration or approval mutation.

## 2. Scope Verification

The phase stayed within the approved hardening scope. It added one additive
approval helper and exact retry behavior to the existing opt-in assessment-bound
executor path. It did not change legacy executor or approval defaults, enforce
assessment dispositions, infer fact freshness, add schema or CLI behavior,
invoke providers, add writes, broaden automatic approvals, or redesign
persistence.

## 3. Retry Assessment

An existing assessment-bound run is rehydrated only after:

- the snapshot carries an immutable bundle binding and assessment binding;
- the stored bundle manifest matches the snapshot bundle binding;
- the retry request matches the exact stored bundle and execution posture;
- the stored assessment binding equals the snapshot binding;
- current typed facts produce a reassessed binding exactly equal to the durable
  binding; and
- any supplied expected aggregate fingerprint matches reassessment.

An exact retry returns the durable run without duplicate events or skill
invocation. Changed facts or a changed supplied fingerprint fail first.

## 4. Approval-Resume Assessment

`decide_approval_with_governance_reassessment` prepares the existing pending
approval without mutating state, performs the same immutable-bundle and durable
binding reassessment, and only then enters the existing approval application
path.

For grants, the existing path reconstructs the current resolved execution
context and compares its commitment with the context originally approved
before appending `ApprovalGranted`, policy, `RunResumed`, or skill events. This
ordering addresses both proportional-governance reassessment and the previously
reported approval/resume TOCTOU boundary.

Denials through the additive helper are also reassessed. Existing denial and
legacy approval APIs remain unchanged.

## 5. Error And Privacy Assessment

New failures use stable codes for missing bundle binding, missing assessment
binding, durable binding mismatch, supplied fingerprint mismatch, and
reassessment mismatch. Messages are bounded and do not include IDs, hashes,
paths, definitions, runtime facts, provider output, command output, credentials,
or payloads. Failed reassessment does not append partial approval or resume
events.

## 6. Test Quality Assessment

Focused tests prove:

- exact retry rehydrates without duplicate invocation;
- changed retry facts append no new events;
- a changed supplied retry fingerprint appends no new events;
- matching approval-resume facts complete the run;
- changed approval-resume facts preserve the waiting run and exact prior event
  history; and
- pre-run fingerprint and binding event ordering remain intact.

The full workspace suite also covers existing immutable bundle, approval,
evidence, report, SideEffect, capability, adapter, and runtime behavior.

## 7. Compatibility Assessment

The integration is additive. Existing `LocalExecutor::execute(...)`,
`LocalExecutor::decide_approval(...)`, immutable-bundle execution, and all
default behavior retain their existing contracts. The new path remains local,
explicit, and API-only.

## 8. Non-Blocking Follow-Ups

- The plan originally said a caller-supplied expected fingerprint should be
  mandatory on retry/resume. The implementation instead treats the stored
  durable binding as the mandatory expected value and accepts a caller
  fingerprint as optional additional confirmation. This preserves the runtime
  integrity invariant, but the public contract should be settled before schema,
  CLI, or default exposure.
- Current runtime facts are typed but not independently freshness-attested.
- Assessment execution dispositions remain recorded rather than enforced.
- Add focused persisted-corruption and missing-binding tests before broader
  default adoption.

## 9. Blockers

None.

## 10. Recommended Next Phase

Proceed to the next roadmap-authoritative bounded runtime phase. Do not broaden
provider mutation families or make proportional governance a default until
trusted fact freshness and disposition-enforcement posture are separately
planned and reviewed.

## 11. Validation Reviewed

- Focused governance-assessment executor tests: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 12. Governed Review Record

- Dogfood workflow: `dg/review`
- Run ID: `run-1784507064720166000-2`
- Approval ID:
  `approval/run-1784507064720166000-2/review-scope-approved`
- Presentation ID: `presentation/1caaaa5cbaff91f0`
- Approval outcome: granted with persisted presentation proof
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
- Phase-close limitation: the helper reported `proof_record_read_error` after
  reaching 250 accumulated presentation records; the proof-enforced approval
  itself succeeded and this known dogfood store-scaling defect remains open.
- Out-of-kernel work: source and test inspection, validation-result review,
  documentation updates, and maintainer verdict
