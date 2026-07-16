# Capability Grant And Availability Core Model Blocker Fix Review

## 1. Executive Verdict

**Blocker fixed; proceed to pure capability resolution helper.**

## 2. Scope Verification

The fix stayed within the approved blocker boundary. It changed capability
availability vocabulary and focused tests, then updated honest documentation.
It added no resolver, enforcement, projection, request, receipt, tool,
connector, provider write, schema, CLI, hosted behavior, or unrelated redesign.

## 3. Original Blocker Restatement

The pre-fix availability record could assert `available_and_authorized`,
`denied`, or `expired_or_revoked` without a grant or decision reference. This
allowed inventory metadata to manufacture an authority outcome and violated
the planned source-of-truth separation.

## 4. Fix Assessment

`CapabilityAvailability` now contains only inventory/connectivity facts:

- `available`;
- `declared_not_connected`;
- `known_unsupported`;
- `unknown`.

This is the smallest correct fix. It avoids adding a premature proof-bearing
combined record and preserves future authority derivation for the pure
resolver.

## 5. Validation And Serde Assessment

Constructors and deserialization still validate capability references,
resources, timestamps, and redaction metadata. Serialized former authority
states now fail closed as unknown enum variants. There is no alternate public
field or serde path that can reintroduce those states.

## 6. Source-Of-Truth Assessment

The corrected boundary is coherent:

- availability reports bounded inventory/connectivity posture;
- grants report scoped authority declarations;
- policy, approval, evidence, and checks remain independent prerequisites;
- the future resolver derives an authority decision from explicit validated
  inputs.

`Available` is not permission, successful connectivity forever, or invocation
readiness.

## 7. Privacy And Redaction Assessment

No payload-bearing fields were added. Debug output remains redacted and serde
errors do not echo raw capability, resource, or secret-like metadata values.
The model still excludes credentials, provider payloads, source contents,
command output, environment values, and unrestricted metadata.

## 8. Regression Assessment

- Focused capability-authority tests: 16 passed.
- Workspace tests: passed.
- Explicit live provider tests remained ignored by design.
- Formatting and clippy with warnings denied passed.
- Documentation and diff checks passed.

## 9. Test Quality Assessment

The new regression test verifies that `available` remains valid and that all
four former authority-bearing wire values fail deserialization. This directly
guards the original bypass surface. Existing construction, serde, scope,
lifecycle, delegation, redaction, and non-leakage coverage remains intact.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- The pure resolver must accept an explicit evaluation timestamp and evaluate
  availability observation freshness rather than assuming current posture.
- Resolver tests must prove `Available` alone never yields authority.
- Unknown and unsupported availability must remain fail closed.

## 12. Governed Review Record

- workflow: `dg/review`
- run ID: `run-1784164876526894000-2`
- approval ID:
  `approval/run-1784164876526894000-2/review-scope-approved`
- presentation ID: `presentation/a5d5e3fba4f37bd1`
- approval outcome: granted under delegated maintainer authority
- phase status: completed before repository phase close

## 13. Recommended Next Phase

Implement the pure capability resolution helper. It must consume explicit
validated inputs, derive a deterministic non-mutating result, and keep
availability separate from authority. Do not add runtime state mutation, tool
projection or execution, capability requests, receipts, connectors, provider
writes, schemas, CLI behavior, or hosted features.
