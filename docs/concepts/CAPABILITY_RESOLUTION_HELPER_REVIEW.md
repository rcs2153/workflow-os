# Capability Resolution Helper Review

## 1. Executive Verdict

**Needs blocker fixes.**

The pure resolver is appropriately narrow, deterministic, payload-free, and
non-mutating. Its runtime path now correctly prevents a broader grant from
bypassing a more-specific grant tier. One serialization-boundary blocker
remains: `CapabilityResolution::validate` accepts semantically impossible
`NotAuthorized` availability/reason combinations.

## 2. Scope Verification

The phase stayed within the approved model/helper boundary. It added no
executor integration, capability requests, tool or context projection,
authority receipts, persistence, events, schemas, CLI behavior, connectors,
provider mutations, hosted administration, RBAC, IdP integration, or release
changes.

The external dogfood reconciliation is documentation-only and accurately
distinguishes implemented behavior from remaining runtime gaps.

## 3. Resolver Assessment

The helper accepts explicit capability, resource, actor, workflow, run, step,
optional harness, sensitivity, evaluation time, inventory, and grant inputs.
It does not read ambient state or mutate any runtime object.

The three postures are appropriately bounded:

- `Authorized` requires available inventory and an active matching grant with
  no declared independent prerequisites;
- `RequiresIndependentEvaluation` preserves policy, approval, evidence, and
  check obligations without pretending they passed;
- `NotAuthorized` represents absent, unavailable, revoked, expired, or
  insufficient authority.

Availability remains inventory rather than permission. The resolver never
creates grants, evidence, approvals, checks, connectors, or provider calls.

## 4. Scope And Precedence Assessment

Matching is exact for actor, capability, resource, and workflow, with optional
grant restrictions for run, step, and harness. Unknown requested sensitivity
fails closed.

The implementation now evaluates only the highest-specificity matching grant
tier. This is security-significant: a broad prerequisite-free grant cannot
bypass a narrower grant that requires policy, approval, evidence, or checks.
Grant identity provides deterministic ordering within the selected tier.

The regression test
`broader_grant_cannot_bypass_more_specific_prerequisites` directly covers this
boundary.

## 5. Validation And Serde Assessment

Constructors validate grants, availability records, resource scopes,
prerequisite references, redaction metadata, lifecycle, expiry, and
sensitivity. Duplicate grant IDs, ambiguous inventory, future-dated
observations, and unknown requested sensitivity fail with stable errors.

### Blocker

`CapabilityResolution::validate` validates `Authorized` and
`RequiresIndependentEvaluation` combinations strictly, but its
`NotAuthorized` branch currently checks only that no grant is selected and no
authorization/prerequisite reason is present. It therefore accepts impossible
wire combinations such as:

```text
posture: not_authorized
availability: available
reasons: [capability_not_connected]
```

It can similarly accept a missing-inventory reason with a present availability
record or an unavailable-inventory reason that does not match the availability
enum. Custom deserialization therefore does not yet fail closed for every
semantically inconsistent result.

Required blocker fix:

1. bind `AvailabilityRecordMissing` exclusively to `availability: None`;
2. bind each unavailable reason exclusively to its matching availability enum;
3. permit grant rejection reasons only with `availability: Available`;
4. preserve deterministic multi-grant rejection reasons;
5. add invalid-wire regression tests for each impossible combination;
6. keep errors stable and non-leaking.

## 6. Privacy And Redaction Assessment

Inputs and results contain bounded identifiers, enums, and timestamps rather
than credentials or provider payloads. `Debug` redacts capability, resource,
actor, workflow, run, step, harness, and selected grant identifiers. Errors do
not echo caller-supplied identifiers. The model stores no command output,
source content, prompt, transcript, token, authorization header, environment
value, or provider payload.

The blocker is semantic integrity, not a discovered data leak.

## 7. Test Quality Assessment

The focused suite covers:

- exact active authorization;
- availability without authority;
- unavailable and missing inventory;
- prerequisite disclosure;
- actor, run, step, and harness mismatch;
- revoked, expired, and sensitivity-insufficient grants;
- deterministic specificity and broad-grant bypass prevention;
- duplicate and ambiguous inputs;
- serde round trip and selected invalid states;
- Debug and error non-leakage.

The missing tests are the impossible `NotAuthorized` availability/reason wire
matrix described in the blocker.

## 8. Documentation Assessment

The roadmap, implementation plan, implementation report, and external dogfood
reconciliation accurately state that the helper is pure and not consumed by
runtime execution. They do not claim connector activation, provider mutation,
runtime enforcement, or enterprise authority support.

## 9. Validation

- `cargo test -p workflow-core --test capability_authority`: passed, 29 tests.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed before this review document was added and must
  run again at review close.
- `git diff --check`: passed before this review document was added and must run
  again at review close.

## 10. Blockers

One blocker: tighten `CapabilityResolution` validation so deserialization fails
closed for impossible `NotAuthorized` availability/reason combinations.

## 11. Non-Blocking Follow-Ups

- Add caller-defined availability maximum-age policy in a later phase.
- Bind future runtime consumption to immutable run inputs.
- Keep policy, approval, evidence, and check evaluation separate from this pure
  helper.
- Decide same-tier grant conflict semantics before runtime enforcement if
  future grant types introduce explicit deny authority.

## 12. Recommended Next Phase

Implement a focused capability-resolution wire-invariant blocker fix. After
that fix is reviewed, proceed to capability request model and review-only
projection planning or implementation. Do not begin tool invocation or a new
provider mutation family first.

## 13. Governed Review Evidence

- Workflow: `dg/review`.
- Run ID: `run-1784170688904119000-2`.
- Approval ID:
  `approval/run-1784170688904119000-2/review-scope-approved`.
- Approval presentation: `presentation/f6b770bfda46fc36`.
- Approval outcome: granted with persisted presentation proof under delegated
  maintainer authority.
- Event summary: 39 events, one approval, zero retries, zero escalations.
- Out-of-kernel work: Codex inspected the implementation, tests, plans, reports,
  and validation results and authored this review. The kernel coordinated
  governance only.

## 14. Fix-Forward Status

The blocker identified by this review is fixed in
[Capability Resolution Helper Blocker Fix Report](CAPABILITY_RESOLUTION_HELPER_BLOCKER_FIX_REPORT.md).
This original verdict remains the review record; acceptance requires a focused
blocker-fix review.
