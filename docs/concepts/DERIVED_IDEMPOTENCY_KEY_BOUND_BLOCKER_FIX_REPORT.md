# Derived Idempotency Key Bound Blocker Fix Report

## 1. Executive Summary

The local executor no longer fails valid long run IDs because of internally concatenated derived idempotency keys.

The blocker was that `invocation_idempotency_key` built a readable path from run ID, workflow ID, workflow version, step ID, skill ID, and skill version. Each identifier could be valid on its own while the composed idempotency key exceeded the 128-byte identifier bound and failed with `identifier.too_long`.

The fix changes executor-derived invocation idempotency keys to deterministic SHA-256-backed keys:

```text
skill-invocation/<sha256>
```

This keeps duplicate-run and restart idempotency stable while preventing a valid external run ID from breaking an internal derived-key boundary.

## 2. Blocker Fixed

Fixed blocker:

- valid long run IDs could cause executor startup to fail before the governed workflow reached its intended approval checkpoint;
- the failure came from internal derived-key construction, not from invalid user input;
- workarounds required shortening run IDs, which undermined the dogfood governance experience.

## 3. Implementation Approach

The executor now hashes the non-secret identity components that define a skill invocation:

- run ID;
- workflow ID;
- workflow version;
- step ID;
- skill ID;
- skill version.

Attempt, retry, and hook idempotency keys remain derived from the invocation key, but the base invocation key is now bounded enough that those suffixes stay under the identifier limit.

No public identifier validation rules were widened.

## 4. Validation Boundary Summary

The fix preserves:

- deterministic idempotency for duplicate run execution;
- stable keys across rehydration/retry paths;
- existing `IdempotencyKey` validation;
- existing duplicate invocation behavior.

The fix avoids:

- concatenating full run IDs into derived idempotency keys;
- leaking long run IDs through idempotency-key text;
- requiring callers to choose artificially short run IDs.

## 5. Test Coverage Summary

Added regression coverage:

- `long_valid_run_id_keeps_derived_idempotency_keys_bounded`

The test proves:

- a valid 128-byte run ID executes successfully;
- duplicate execution rehydrates through idempotency instead of invoking the skill twice;
- generated event idempotency keys remain under the 128-byte bound;
- generated idempotency keys do not concatenate the full run ID.

## 6. Commands Run

- `cargo test -p workflow-core --test local_executor long_valid_run_id_keeps_derived_idempotency_keys_bounded` - passed.
- `cargo fmt --all` - applied mechanical formatting after `cargo fmt --all --check` identified formatting diffs.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test -p workflow-core --test local_executor` - passed.
- `cargo test --workspace` - passed.
- `cargo build -p workflow-cli --bin workflow-os` - passed.
- `target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance --state-dir /tmp/workflow-os-long-id-regression --mock-all-local-skills run dg/implement --run-id run/dogfood-phase2-workflows` - passed and reached `WaitingForApproval` at `approval/run/dogfood-phase2-workflows/implementation-approved`.
- `npm run dogfood:benchmark -- validate --no-build` - passed with expected experimental lifecycle warnings.
- `npm run check:docs` - passed.

## 7. Remaining Known Limitations

- Derived keys are intentionally opaque. They are stable and auditable as IDs, but no longer human-readable paths.
- The generic identifier length limit remains unchanged.
- This fix does not add new idempotency store behavior, new event types, CLI behavior, schemas, persistence changes, or release posture changes.

## 8. Recommended Next Phase

Recommended next phase: continue with `dg/runtime-composition` for the next code-bearing runtime-composition lane.

The idempotency blocker is no longer a reason to shorten governed run IDs during dogfood work.
