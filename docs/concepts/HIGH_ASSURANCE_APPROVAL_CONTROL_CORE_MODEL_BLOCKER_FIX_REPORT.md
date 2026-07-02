# High-Assurance Approval Control Core Model Blocker Fix Report

## 1. Executive Summary

The high-assurance approval control core model blocker is fixed.

`HighAssuranceApprovalRequiredReference` now enforces constructor-backed deserialization at its own public type boundary. Standalone serialized required-reference values can no longer bypass reference-name validation before entering the model.

This fix remains model-only. It does not implement runtime high-assurance approval enforcement, approval identity enforcement, RBAC, IdP integration, quorum approval, write-capable adapters, provider mutations, runtime side-effect execution, workflow schemas, CLI behavior, examples, hosted behavior, reasoning lineage, or release posture changes.

## 2. Blocker Fixed

The review found that `HighAssuranceApprovalRequiredReference` derived `Deserialize` directly.

That was unsafe for the public model surface because callers could deserialize the nested exported type directly with an invalid or secret-like `name`, bypassing `HighAssuranceApprovalRequiredReference::new(...)` until the value was embedded in a validated top-level control.

The fixed behavior is:

- standalone required-reference deserialization calls the validated constructor;
- invalid reference names fail closed;
- secret-like reference names fail closed;
- valid serialized references still round trip;
- error messages do not include the rejected raw value.

## 3. Implementation Approach

The fix removes direct derived `Deserialize` from `HighAssuranceApprovalRequiredReference`.

Deserialization now uses a small internal wire struct and then calls `HighAssuranceApprovalRequiredReference::new(...)`. That keeps the public serialization shape unchanged while ensuring the same validation boundary applies to in-memory construction and deserialization.

This is the smallest fix because it avoids redesigning the high-assurance approval model, changing target reference vocabulary, or changing top-level control construction.

## 4. Validation Boundary Summary

The required-reference type now validates:

- reference name is present;
- reference name is bounded;
- reference name uses the supported identifier character set;
- reference name is not secret-like;
- target deserializes through existing target primitives;
- required flag remains explicit.

Validation errors continue to use stable non-leaking `high_assurance_approval.*` codes from the existing identifier validation path.

## 5. Privacy And Redaction Summary

The fix prevents secret-like reference names from being silently stored through standalone deserialization.

Regression tests cover non-leakage for:

- invalid serialized reference names;
- secret-like serialized reference names;
- `Debug` output for the high-assurance approval control model;
- serialization avoiding forbidden raw payload field markers.

The model remains reference-only. It does not store raw provider payloads, command output, CI logs, Jira or GitHub bodies, raw spec contents, parser payloads, environment variable values, credentials, authorization headers, private keys, or token-like values.

## 6. Test Coverage Summary

Added focused blocker regression tests for:

- standalone required-reference serde round trip through the validated shape;
- invalid serialized required-reference name failing closed without leaking the raw value;
- secret-like serialized required-reference name failing closed without leaking the raw value.

Existing high-assurance approval tests continue to cover top-level controls, target vocabulary, future write vocabulary as model-only, requester/approver vocabulary, disclosure vocabulary, redaction metadata rejection, redaction-safe `Debug`, and serialization non-leakage.

## 7. Commands Run And Results

Commands run:

- `cargo fmt --all --check` - passed
- `cargo clippy --workspace --all-targets -- -D warnings` - passed
- `cargo test -p workflow-core --test high_assurance_approval` - passed
- `cargo test --workspace` - passed
- `npm run check:docs` - passed

## 8. Remaining Known Limitations

- High-assurance approval controls remain model-only.
- Runtime approval enforcement is not implemented.
- Requester/approver separation is not enforced at runtime.
- Approval expiration and revocation are not enforced at runtime.
- Evidence sufficiency is not enforced at runtime.
- Approval report disclosure population is not automatic.
- Write-capable adapters remain unsupported.
- Runtime side-effect execution remains unsupported.

## 9. Recommended Next Phase

Recommended next phase: **High-assurance approval control core model blocker fix review**.

After this fix is reviewed, the roadmap should continue toward connecting already-built primitives into opt-in runtime paths before any write-capable adapter work.
