# Explicit Zero Required-Context Contract Implementation Report

## 1. Executive Summary

The explicit-zero required-context prerequisite is implemented in Core. One
private canonical docs-check path can now prove that an exact immutable
execution binding requires zero capability grants, zero capability
availability facts, and zero governed-context references.

The existing public contract constructor still rejects empty requirements.
Established non-empty contracts retain their existing hash algorithm and
serialized shape. Explicit zero uses a distinct marker and hash domain, may
consume exactly zero projections, and can resolve through a private registered
authority source whose configuration commitment includes all three complete
empty inventory declarations.

This phase does not resume trusted-host admission or add an application binary.

## 2. Scope Completed

- Added a private closed requirement posture distinguishing canonical
  non-empty requirements from explicit zero.
- Preserved the existing public non-empty constructor and validation behavior.
- Added a crate-private explicit-zero constructor gated by private docs-check
  profile authorization.
- Added compatibility-preserving custom serde for contract bindings.
- Added exact zero-projection consumption semantics.
- Added explicit-zero current-authority query-set and source-request posture.
- Added a private registered-source constructor for complete empty docs-check
  authority inventories.
- Bound the execution binding, contract identity and hash, docs-check command
  fingerprint, and three empty fact-family declarations into source
  configuration identity.
- Added focused compatibility, malformed-wire, consumption, profile-issuance,
  source-resolution, and regression tests.

## 3. Scope Explicitly Not Completed

- No `workflow-local-host` binary or trusted-host admission API was added.
- No public empty-contract constructor or workflow-authored zero posture exists.
- No additional local-check profile can issue explicit zero.
- No caller-authored authority inventory was exposed.
- No provider access, provider mutation, OpenShell, nested harness, hosted,
  distributed, CLI, SDK, schema, example, or release behavior changed.
- Policy, approval, evidence, checks, operational opening, and dispatch remain
  independent requirements.

## 4. Model And Compatibility Summary

`RequiredContextContractBinding::new` still requires at least one canonical
requirement. Non-empty contracts continue to use the unchanged
`required-context-contract-v1` hash path and serialize the original four
fields. A golden regression test proves the established fixture hash remains:

```text
f7f7aedb81d2f26171bcd352dd1523cff4ad56a5c06f61703b8263cc9e33cfdf
```

Explicit-zero contracts use the domain
`required-context-contract-explicit-none-v1`, serialize
`requirement_posture: explicit_none`, and require an empty requirement list.
An empty list without the marker, an unknown marker, or a marker paired with a
non-empty list fails closed.

## 5. Consumption And Authority Summary

An explicit-zero contract accepts exactly zero context projections and returns
the existing `Satisfied` posture with zero satisfactions and zero gaps. Any
projection supplied to that contract fails closed. Ordinary contracts retain
their existing non-empty projection requirement.

The current-authority query set and source request use explicit markers and
separate commitment domains only for the zero posture. The canonical docs-check
profile is the sole private issuer. The registered source can commit complete
empty inventories only when the execution binding and explicit-zero contract
match exactly. Focused resolution proves a `Ready` assessment with no invented
grant, reference, satisfaction, or evidence record.

## 6. Privacy And Failure Posture

Debug output exposes only bounded posture and counts. Validation and serde
errors remain payload-free. Unknown, omitted, mismatched, or non-empty zero
declarations fail before execution. No source path, command argument,
environment value, context target, token, credential, or private policy value
is included in errors.

## 7. Tests Added

- legacy non-empty contract hash and four-field wire-shape regression;
- public empty-constructor rejection;
- explicit-zero serde round trip and marker requirement;
- unknown and non-empty explicit-zero posture rejection;
- zero-projection satisfaction and supplied-projection rejection;
- explicit-zero authority query-set round trip and marker enforcement;
- canonical docs-check private issuance without process execution;
- exact complete-empty registered-authority resolution; and
- existing current-authority source regression coverage.

## 8. Validation

Focused validation completed during implementation:

- isolated `cargo check -p workflow-core`: passed;
- focused explicit-zero contract tests: passed;
- legacy required-context golden regression: passed;
- current-authority query-set tests: passed;
- canonical docs-check issuer test: passed;
- complete empty registered-authority resolution test: passed;
- existing `current_authority_source` integration suite: 10 passed; and
- focused `cargo clippy -p workflow-core --all-targets -- -D warnings`: passed.

Full validation completed:

- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed;
- `npm run check:docs`: passed; and
- `git diff --check`: passed.

## 9. Remaining Limitations

- The explicit-zero source is crate-private and not wired into an operational
  application.
- Trusted-host admission remains blocked pending focused review of this
  prerequisite.
- No public configuration or schema can request explicit zero.
- No additional profile has been reviewed for zero-authority eligibility.

## 10. Governed Phase Record

- workflow: `dg/implement`
- run: `run-1791391705389027000-2`
- approval: `approval/run-1791391705389027000-2/implementation-approved`
- presentation: `presentation/7e1813f87c8c4f85`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: explicit-zero prerequisite model, consumption, private
  docs-check issuance, exact empty authority resolution, tests, and docs
- out-of-kernel work: Rust implementation, test execution, documentation, and
  later git or pull-request operations

## 11. Recommended Next Phase

Perform a focused maintainer/security implementation review. If accepted,
resume the trusted-host local application admission vertical slice without
broadening profiles, runtime surfaces, provider behavior, or release posture.
