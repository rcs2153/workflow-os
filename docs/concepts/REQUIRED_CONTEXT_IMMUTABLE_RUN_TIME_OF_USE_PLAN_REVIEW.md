# Required Context Immutable-Run And Time-Of-Use Plan Review

## 1. Executive Verdict

**Plan accepted; proceed to the immutable execution-binding core model.**

## 2. Scope Verification

The plan remains planning-only. It does not authorize dereference, executor
integration, persistence, events, schemas, CLI behavior, providers, sandboxes,
process execution, SideEffects, writes, hosted behavior, reasoning lineage, or
release changes.

## 3. Problem Assessment

The plan addresses the correct gap. Existing required-context consumption
proves exact point-in-time satisfaction but is not bound to the stored
immutable run bundle and does not re-resolve authority at use time.

Without both boundaries, a future consumer could accept a coherently
substituted contract/context or reuse stale availability and grant posture.

## 4. Immutable Binding Assessment

The proposed separate `RequiredContextExecutionBinding` is appropriately
narrow. It binds the stored immutable bundle root, exact contract hash, and
execution identity without falsely claiming current immutable bundles already
contain canonical harness contract records.

Deferring bundle taxonomy expansion is correct because adding harness
definitions would affect canonical hashing, storage, compatibility, and replay
semantics.

## 5. Time-Of-Use Assessment

Same-call re-resolution is the right first posture. Fresh capability
resolution, projection reconstruction, and contract consumption should occur
at the consuming boundary. A reusable TTL lease would add ambiguity and replay
risk before there is a concrete need.

The plan correctly prevents prior projections and consumption results from
serving as current authority.

## 6. Completeness Assessment

The plan correctly identifies candidate-set completeness as an unresolved
authority boundary. Arbitrary caller slices cannot prove global grant or
availability posture.

This is not a blocker for the first immutable binding model, but it is a
blocker for any authoritative time-of-use `Ready` result. The re-resolution
implementation must use a validated complete-set source or remain explicitly
non-authoritative.

## 7. Governance Composition Assessment

The plan preserves ownership boundaries:

- capability resolution owns grant and availability posture;
- policy, approval, evidence, and checks own their accepted records;
- governed projection owns bounded visibility;
- required-context consumption owns contract satisfaction;
- a future consumer owns actual read-only dereference.

No boolean shortcut is allowed to collapse those sources.

## 8. Privacy And Error Assessment

The proposed models remain payload-free and require redacted Debug plus stable
non-leaking errors. The forbidden-data list covers source, provider, command,
parser, environment, credential, log, path, and target payload risks.

## 9. Product Alignment

The plan supports the product's quiet-success direction without confusing
quiet UX with weak enforcement. Low-risk work may eventually proceed quietly,
but only after immutable identity and current authority are proven.

An optional sandbox execution provider remains complementary: Workflow OS
governs authority and evidence while the provider enforces containment. No
sandbox integration is authorized here.

## 10. Planning Blockers

None for the first immutable execution-binding model.

Authoritative time-of-use re-resolution remains blocked on a validated complete
current authority-fact set and exact accepted prerequisite records.

## 11. Non-Blocking Follow-Ups

- Decide crate visibility for the Core-owned binding constructor.
- Define the complete grant/availability set model before authoritative
  re-resolution.
- Evaluate canonical harness contract bundle records separately.
- Select the first read-only dereference target only after re-resolution
  review.

## 12. Recommended Next Phase

Proceed to **required-context immutable execution-binding core model only**.

Keep target dereference, runtime consumption, persistence, events, schemas,
CLI behavior, providers, OpenShell, process execution, SideEffects, writes,
hosted behavior, reasoning lineage, and release changes out of scope.
