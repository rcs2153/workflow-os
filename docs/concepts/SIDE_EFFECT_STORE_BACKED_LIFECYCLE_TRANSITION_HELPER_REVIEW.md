# SideEffect Store-Backed Lifecycle Transition Helper Review

## 1. Executive Verdict

Needs blocker fixes.

The helper implementation is directionally correct and remains within the approved pre-write scope, but the new public store update boundary must enforce lifecycle transition semantics instead of relying on documentation intent alone.

## 2. Scope Verification

The phase stayed within the approved store-backed helper scope.

No accidental provider calls, runtime side-effect execution, executor attempted/completed/failed event append behavior, workflow run mutation, report artifact writing, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes were introduced.

## 3. Helper API Assessment

The implemented helper APIs are small and explicit:

- `transition_side_effect_to_attempted_in_store(...)`
- `transition_side_effect_to_completed_in_store(...)`
- `transition_side_effect_to_failed_in_store(...)`

They load prior records by stable `SideEffectId`, delegate transition construction to the pure helpers, update the store, and return reference-only event payloads. This is the right shape for future executor integration because it keeps provider calls and event appends out of this phase.

## 4. Store Boundary Assessment

Blocker: `SideEffectRecordStore::update_side_effect_record(...)` is public and currently validates identity and record shape, but it does not enforce allowed lifecycle transitions.

That means a caller could bypass the reviewed pure transition helpers by constructing another valid record with the same ID and identity, then replacing the stored record directly. Documentation says the method is intended for reviewed lifecycle transitions only, but the implementation must fail closed at the store boundary as well.

Required fix:

- reject direct update transitions outside the accepted lifecycle moves;
- allow only `Proposed -> Attempted`, `Attempted -> Completed`, and `Attempted -> Failed` for this phase;
- preserve same-state replay/update behavior only if explicitly designed and test-covered, otherwise reject it;
- add focused tests proving invalid direct store updates fail closed without leaking values.

## 5. Lifecycle Assessment

The store-backed helper path itself uses the pure transition helpers, so helper-mediated attempted/completed/failed transitions are valid.

The blocker is the lower-level update method, not the helper functions. Since future code may call the store trait directly, the store boundary must not become a loophole around transition validation.

## 6. Event Boundary Assessment

The helpers correctly return reference-only event payloads and do not append them. Executor event append behavior remains future work.

## 7. Privacy And Redaction Assessment

The helper error mapping is bounded and non-leaking. Store read/write failures map to stable transition errors, and tests cover non-leakage for failing reads and writes.

The blocker fix must preserve this posture and avoid leaking side-effect IDs, target references, provider payloads, paths, tokens, or secret-like values.

## 8. Test Quality Assessment

Existing new tests cover:

- successful store-backed attempted transition;
- successful store-backed completed transition;
- successful store-backed failed transition;
- missing prior failure;
- store read failure mapping;
- store write failure mapping without partial update;
- repeated helper-mediated transition rejection;
- non-leaking error messages.

Missing blocker test:

- direct `update_side_effect_record(...)` rejects invalid lifecycle jumps.

## 9. Documentation Review

Docs correctly state:

- store-backed helpers are implemented;
- provider calls are not implemented;
- runtime side-effect execution is not implemented;
- executor attempted/completed/failed event append behavior is not implemented;
- CLI behavior, schemas, examples, hosted behavior, reasoning lineage, writes, and release posture changes remain out of scope.

## 10. Blockers

1. Public `SideEffectRecordStore::update_side_effect_record(...)` must enforce allowed lifecycle transitions instead of only documenting that callers should use it for reviewed transitions.

## 11. Non-Blocking Follow-Ups

- Consider explicit replay semantics later, but do not add them until reviewed.
- Consider adding a narrower trait method name or helper-specific wrapper later if lifecycle updates become more complex.

## 12. Recommended Next Phase

SideEffect store-backed lifecycle transition blocker fix.

Fix the store update lifecycle enforcement before merging PR 46 or planning executor attempted/completed/failed event append behavior.

## 13. Validation

Before this review, the implementation branch passed:

- `cargo fmt --all`;
- `cargo test -p workflow-core --test side_effect`;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`.

## 14. Dogfood Governance

- Workflow: `dg/review`
- Run: `run-1783267598129772000-2`
- Approval: `approval/run-1783267598129772000-2/review-scope-approved`
- Approval outcome: granted by delegated maintainer.
- Phase close status: completed.
- Event summary: 39 events total; 1 approval; 0 retries; 0 escalations.
- Event kinds: `ApprovalGranted`, `ApprovalRequested`, `PolicyDecisionRecorded`, `RunCompleted`, `RunCreated`, `RunResumed`, `RunStarted`, `RunValidated`, `SkillInvocationRequested`, `SkillInvocationStarted`, `SkillInvocationSucceeded`, and `StepScheduled`.
- Out-of-kernel work: repository review, blocker identification, documentation updates, and validation command review were performed by the agent outside the kernel under the governed review phase boundary.
