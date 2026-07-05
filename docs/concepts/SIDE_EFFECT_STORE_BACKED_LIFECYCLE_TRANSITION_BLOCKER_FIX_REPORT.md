# SideEffect Store-Backed Lifecycle Transition Blocker Fix Report

## 1. Executive Summary

The store-backed lifecycle transition helper review found a blocker: the public `SideEffectRecordStore::update_side_effect_record(...)` method validated identity and record shape, but did not enforce allowed lifecycle transitions. That could let a caller bypass the reviewed pure transition helpers by constructing another valid record with the same identity and replacing the stored record directly.

The blocker is fixed. Store updates now fail closed unless the lifecycle transition is one of the explicitly supported phase transitions: `Proposed -> Attempted`, `Attempted -> Completed`, or `Attempted -> Failed`.

## 2. Blocker Fixed

Fixed blocker:

- public store update boundary could be used for unsupported lifecycle replacement.

Required behavior now enforced:

- missing record still fails closed;
- identity mismatch still fails closed;
- invalid record still fails closed;
- unsupported lifecycle transition fails closed with `side_effect_record.update.invalid_lifecycle_transition`;
- same-state replacement is rejected for this phase;
- direct `Proposed -> Completed` replacement is rejected for this phase.

## 3. Implementation Approach

The fix adds an internal lifecycle predicate in `state.rs` and applies it in both local and in-memory `SideEffectRecordStore` update implementations.

The update boundary now allows only:

- `Proposed -> Attempted`;
- `Attempted -> Completed`;
- `Attempted -> Failed`.

All other transitions remain unsupported until a separate reviewed phase defines replay or reconciliation semantics.

## 4. Validation Boundary Summary

The store-backed helper path still delegates transition construction to the pure helpers. The store update boundary now independently prevents direct public update misuse.

This is intentionally defense in depth: helper validation remains the primary construction path, and the store boundary now rejects unsupported replacement if future code calls it directly.

## 5. Redaction And Privacy Summary

The new invalid-lifecycle error is stable and non-leaking. It does not include side-effect IDs, run IDs, target references, provider payloads, paths, tokens, snippets, command output, or secret-like values.

## 6. Test Coverage Summary

Added or updated tests cover:

- direct store update rejects `Proposed -> Completed`;
- direct store update rejects same-state replacement;
- shared backend contract accepts `Proposed -> Attempted`;
- shared backend contract rejects unsupported lifecycle transitions;
- shared backend contract rejects same-state replacement;
- focused store-backed helper tests still pass.

## 7. Commands Run And Results

- `cargo fmt --all`: passed.
- `cargo test -p workflow-core --test side_effect`: passed.
- `cargo test -p workflow-core state::tests::backend_contract_passes`: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.

## 8. Remaining Known Limitations

- Replay-as-success semantics remain deferred.
- Executor event append behavior remains deferred.
- Provider calls and runtime side-effect execution remain unsupported.
- Report artifact and audit ordering around attempted/completed/failed transitions remains future work.

## 9. Recommended Next Phase

Recommended next phase: rerun focused helper review after blocker fix.

## 10. Dogfood Governance

- Workflow: `dg/blocker`
- Run: `run-1783267723125073000-2`
- Approval: `approval/run-1783267723125073000-2/fix-approved`
- Approval outcome: granted by delegated maintainer.
- Phase close status: completed.
- Event summary: 39 events total; 1 approval; 0 retries; 0 escalations.
- Event kinds: `ApprovalGranted`, `ApprovalRequested`, `PolicyDecisionRecorded`, `RunCompleted`, `RunCreated`, `RunResumed`, `RunStarted`, `RunValidated`, `SkillInvocationRequested`, `SkillInvocationStarted`, `SkillInvocationSucceeded`, and `StepScheduled`.
- Out-of-kernel work: repository edits, test execution, documentation updates, and validation commands were performed by the agent outside the kernel under the governed blocker phase boundary.
