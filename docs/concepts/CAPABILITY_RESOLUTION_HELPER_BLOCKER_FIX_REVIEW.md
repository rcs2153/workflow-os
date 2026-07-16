# Capability Resolution Helper Blocker Fix Review

## 1. Executive Verdict

**Blocker fixed; proceed to capability request model and review-only projection.**

The fix closes the semantic integrity gap identified in the original helper
review. `CapabilityResolution` now rejects every contradictory
`NotAuthorized` availability/reason shape supported by the current vocabulary,
while preserving valid resolver output and deterministic multi-grant rejection
reasons.

## 2. Scope Verification

The fix remained within the approved validation and regression-test boundary.
It introduced no executor integration, capability requests, tool projection,
authority receipts, persistence, events, schemas, CLI behavior, connectors,
provider writes, hosted administration, RBAC, IdP integration, or release
changes.

## 3. Original Blocker Restatement

The resolver generated correct denial states, but the custom serde boundary
accepted impossible states because `NotAuthorized` validation did not bind
reasons to availability. A wire payload could claim available inventory while
reporting `capability_not_connected`, or absent inventory while reporting a
grant-matching result.

That ambiguity could undermine future persistence or schema consumers even
though it did not authorize execution.

## 4. Fix Assessment

The implementation uses one private predicate from the existing
`CapabilityResolution::validate` boundary. This is the smallest idiomatic fix:
constructors, runtime results, serde round trips, and future stored values do
not acquire competing validation paths.

The accepted matrix is:

- absent availability requires `availability_record_missing`;
- `declared_not_connected` requires `capability_not_connected`;
- `known_unsupported` requires `capability_unsupported`;
- `unknown` requires `capability_availability_unknown`;
- `available` permits `no_matching_grant` alone or ordered revoked, expired,
  and sensitivity-insufficient grant reasons.

The empty-reason and ordered-unique invariants remain enforced by the parent
validator. Authorization and independent-prerequisite reasons remain forbidden
for `NotAuthorized`.

## 5. Regression Assessment

Valid helper behavior is unchanged:

- exact active grants can authorize;
- inventory alone never authorizes;
- narrower grant tiers override broader tiers;
- independent prerequisites remain unevaluated obligations;
- revoked, expired, and sensitivity-insufficient grants deny;
- ambiguous inventory and duplicate grant identity fail closed;
- result round trips remain valid.

The full workspace suite passed after the fix.

## 6. Privacy And Error Assessment

The fix operates only on bounded enums and introduces no caller-supplied text.
Invalid wire combinations continue to map to the stable
`capability_authority.resolution.inconsistent` boundary without echoing
capability, resource, actor, workflow, run, step, harness, or grant values.

No credential, environment value, provider payload, source content, command
output, prompt, transcript, token, or authorization header is stored or logged.

## 7. Test Quality Assessment

The focused suite has 30 passing tests. New tests cover:

- available inventory with a not-connected reason;
- an unavailable enum with the wrong unavailable reason;
- absent inventory with a grant-matching reason;
- mixed no-match and rejected-grant reasons;
- non-leaking deserialization errors.

Existing resolver tests cover every valid availability state and the permitted
grant rejection reasons, providing positive coverage for the accepted matrix.

## 8. Documentation Assessment

The original review remains intact and links to the fix report. The fix report,
plan, and roadmap distinguish the accepted pure helper from unimplemented
runtime enforcement, requests, projection, connectors, and writes.

## 9. Validation

- `cargo test -p workflow-core --test capability_authority --quiet`: passed,
  30 tests.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace --quiet`: passed.
- `npm run check:docs`: passed before this review document was added and must
  run again at review close.
- `git diff --check`: passed before this review document was added and must run
  again at review close.

## 10. Remaining Blockers

None for the pure capability resolution helper.

## 11. Non-Blocking Follow-Ups

- Define caller-controlled availability freshness in a later bounded phase.
- Preserve most-specific-tier precedence in all future runtime consumers.
- Keep independent policy, approval, evidence, and check evaluation outside
  this helper.
- Bind runtime authority decisions to immutable run context before invocation.

## 12. Recommended Next Phase

Implement the capability request model and review-only projection as bounded,
payload-free model/helper work. A missing or insufficient grant may become a
reviewable request posture, but it must not auto-grant authority, activate a
connector, expose a tool, execute a provider call, or mutate runtime state.

## 13. Governed Review Evidence

- Workflow: `dg/review`.
- Run ID: `run-1784172581739523000-2`.
- Approval ID:
  `approval/run-1784172581739523000-2/review-scope-approved`.
- Approval presentation: `presentation/e7a79357211a12ca`.
- Approval outcome: granted with persisted presentation proof under delegated
  maintainer authority.
- Event summary: 39 events, one approval, zero retries, zero escalations.
- Out-of-kernel work: Codex reviewed the fix, tests, reports, and validation
  results and authored this review. The kernel coordinated governance only.
