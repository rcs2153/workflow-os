# SideEffect Store-Backed Lifecycle Transition Blocker Fix Review

## 1. Executive Verdict

Blocker fixed; proceed with PR 46 review and merge once CI is green.

The blocker fix closes the public store update loophole identified in the store-backed lifecycle transition helper review. Store-backed updates now enforce allowed lifecycle transitions at the storage boundary, not only through the pure helper path.

## 2. Scope Verification

The fix stayed within the approved blocker-fix scope.

No provider calls, runtime side-effect execution, executor attempted/completed/failed event append behavior, workflow run mutation, report artifact writing, persistence beyond the existing side-effect store boundary, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes were introduced.

## 3. Original Blocker Restatement

The original blocker was that `SideEffectRecordStore::update_side_effect_record(...)` was public and validated record shape and identity, but did not validate lifecycle transition semantics.

That meant a caller could bypass the reviewed pure transition helpers by constructing a valid same-ID, same-identity replacement record with an unsupported lifecycle state, such as replacing a `Proposed` record directly with `Completed`.

## 4. Fix Approach Assessment

The fix adds store-boundary lifecycle transition validation.

Allowed transitions for this phase are:

- `Proposed -> Attempted`;
- `Attempted -> Completed`;
- `Attempted -> Failed`.

All other updates fail closed. Same-state replacement is rejected because replay/reconciliation semantics have not been designed or reviewed.

The approach is minimal and appropriate: it preserves the existing helper API, keeps provider writes out of scope, and adds defense in depth at the public store boundary.

## 5. Validation Boundary Assessment

The validation boundary now ensures:

- missing records fail closed;
- identity mismatches fail closed;
- invalid replacement records fail closed;
- unsupported lifecycle transitions fail closed;
- same-state replacement fails closed;
- direct `Proposed -> Completed` replacement fails closed.

Validation errors use stable codes and do not include side-effect IDs, run IDs, target references, provider payloads, paths, tokens, snippets, command output, or secret-like values.

## 6. Test Quality Assessment

Focused tests cover:

- direct store update rejects `Proposed -> Completed`;
- direct store update rejects same-state replacement;
- store-backed helper still supports `Proposed -> Attempted`;
- store-backed helper still supports `Attempted -> Completed`;
- store-backed helper still supports `Attempted -> Failed`;
- shared backend contract accepts allowed lifecycle transition;
- shared backend contract rejects unsupported lifecycle transition;
- shared backend contract rejects same-state replacement;
- error messages are stable and non-leaking.

The coverage is adequate for the blocker. Replay-as-success and broader reconciliation behavior remain intentionally deferred.

## 7. Documentation Review

The blocker fix report documents:

- the blocker fixed;
- the allowed lifecycle transitions;
- fail-closed behavior;
- redaction and privacy posture;
- tests added;
- validation commands run;
- remaining limitations;
- governed dogfood run details.

The original helper report links to the blocker fix report instead of erasing the review finding.

## 8. Remaining Limitations

- Executor append behavior for attempted/completed/failed side-effect events remains deferred.
- Provider calls and runtime side-effect execution remain unsupported.
- Replay-as-success semantics remain deferred.
- Report artifact and audit ordering around lifecycle transitions remain future work.

## 9. Validation

The blocker fix branch passed:

- `cargo fmt --all`;
- `cargo test -p workflow-core --test side_effect`;
- `cargo test -p workflow-core state::tests::backend_contract_passes`;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Design replay/reconciliation semantics before allowing same-state store replacement.
- Keep executor event append behavior in a separately reviewed phase.

## 12. Recommended Next Phase

Recommended next phase: merge PR 46 after CI is green, then proceed to executor attempted/completed/failed side-effect event append planning.

That phase should compose the now-validated store-backed lifecycle transition boundary into the executor event/audit path without adding provider writes.

