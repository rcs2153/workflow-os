# Visible Proceed Executor Integration Report

## 1. Executive Summary

Workflow OS now has one explicit local executor path for a complete,
source-bound proportional-governance result whose route is
`Proceed + Visible`.

The path reuses the accepted immutable-run-bundle and authoritative
`DocsCheck` reassessment boundary. Core constructs the exact payload-free
delivery request, invokes one explicitly injected local disclosure handler,
constructs and validates the surface-acceptance receipt from the handler's
bounded timestamp, and only then creates run events or invokes workflow
skills.

The receipt proves only that the configured surface accepted the exact
request. It does not prove human observation, understanding, acknowledgement,
approval, persistence, or audit projection.

## 2. Scope Completed

- Added an explicit local disclosure-delivery handler contract.
- Added bounded executor disclosure inputs.
- Added an explicit authoritative visible-governance request and result.
- Reused the existing authoritative immutable-bundle and local-check
  reassessment path.
- Required a complete, source-bound aggregate `Proceed + Visible` assessment.
- Constructed the delivery request inside Core from the exact assessment and
  execution correlation identity.
- Limited the injected handler result to an acceptance timestamp or error.
- Constructed and validated the receipt inside Core.
- Required successful receipt construction before `RunCreated` or skill
  invocation.
- Preserved the existing quiet execution API and behavior.

## 3. Scope Explicitly Not Completed

The phase did not add:

- approval-required routing or approval decisions;
- denial lifecycle routing;
- receipt persistence, workflow events, audit projection, or WorkReport
  projection;
- CLI, terminal, UI, notification, or hosted surfaces;
- workflow or policy schema fields;
- providers, OpenShell, sandbox execution, or credentials;
- SideEffect execution or writes;
- human observation, understanding, acknowledgement, or approval claims;
- retry/resume support for the authoritative fresh-run consumer;
- reasoning lineage or release changes.

## 4. API Summary

The implementation adds:

- `GovernanceDisclosureDeliveryHandler`;
- `LocalExecutionGovernanceDisclosureInputs`;
- `LocalExecutionWithAuthoritativeDocsCheckVisibleGovernanceRequest`;
- `LocalExecutionWithAuthoritativeDocsCheckVisibleGovernanceResult`; and
- `execute_with_authoritative_docs_check_visible_governance(...)`.

The delivery handler receives a borrowed
`GovernanceDisclosureDeliveryRequest` and can return only `Timestamp` or
`WorkflowOsError`. It cannot return or supply a
`GovernanceDisclosureDeliveryReceipt`.

## 5. Runtime Ordering

The explicit path orders work as follows:

1. Require a fresh run identity and empty durable event state.
2. Prepare and validate the local execution plan.
3. Build, claim, and reread the immutable run bundle.
4. Execute the canonical `DocsCheck` contribution.
5. Derive the aggregate authoritative governance binding.
6. Require complete source-bound `Proceed + Visible`.
7. Construct the exact delivery request in Core.
8. Invoke the injected local disclosure handler.
9. Construct and validate the exact surface-acceptance receipt in Core.
10. Persist or validate the governance assessment binding.
11. Append run-start events.
12. Execute sequential workflow skills.

Delivery failure therefore creates no run event and invokes no workflow skill.
The immutable bundle may already have been claimed because it is the
fresh-run concurrency boundary and the authoritative local check precedes
routing.

## 6. Authority And Claim Boundary

The delivery receipt remains non-authoritative. It proves:

```text
the configured local surface accepted this exact bounded disclosure request
```

It does not prove:

- delivery to a person;
- human observation or understanding;
- acknowledgement;
- approval;
- durable audit recording; or
- permission to execute.

Execution authority remains the accepted authoritative governance assessment.
Visible delivery is a required non-blocking condition for this explicit route.

## 7. Failure Behavior

The path fails closed before run events and skills when:

- the run is not fresh;
- the immutable bundle or canonical check declaration is invalid;
- the local check fails;
- aggregate facts are incomplete;
- execution requires approval or is denied;
- disclosure is quiet rather than visible;
- authoritative source binding is missing;
- the surface rejects or fails delivery;
- acceptance time predates the request;
- receipt construction or exact request validation fails; or
- assessment-binding persistence fails.

Handler errors are mapped to the stable non-leaking code
`executor.authoritative_local_check.disclosure_delivery_failed`; handler
messages and codes are not propagated.

## 8. Privacy And Redaction

The path uses the existing payload-free disclosure model. It does not pass
rendered prose, commands, process output, source/spec contents, filesystem
paths, environment values, provider payloads, credentials, authorization
headers, private keys, or tokens through the delivery contract.

Request, result, assessment, surface, and receipt `Debug` boundaries redact
caller-controlled identities. Failed injected-handler text is discarded at
the Core boundary.

## 9. Test Coverage

Focused tests prove:

- visible `Proceed` produces a completed run;
- the surface receives the exact Core-owned request;
- the exact authoritative assessment, delivery identity, surface, and
  correlation are bound into the request;
- delivery happens while workflow skill invocation count is still zero;
- Core constructs and validates the receipt;
- delivery failure blocks run events and skills without leaking handler text;
- invalid acceptance time blocks run events and skills;
- quiet, approval-required, and denied routes do not invoke the surface;
- exact fresh-run reuse does not repeat checks, delivery, or skills;
- result and request `Debug` do not expose bounded identities; and
- existing quiet authoritative execution remains unchanged.

## 10. External Feedback Reconciliation

Fresh-pull evaluation confirms that current onboarding is coherent and that
the next product problem is reducing low-risk ceremony without losing
evidence. This path advances that objective by making visible non-blocking
delivery executable without turning it into approval.

The same review's Node-version and duplicate missing-manifest findings are
already fixed on current `main`. General turnkey execution, broader handlers,
and provider integration remain separate roadmap work.

## 11. Validation

Required phase validation:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`; and
- `git diff --check`.

Results are recorded at governed phase close.

All required commands passed. The focused visible-governance executor tests
also passed after the final identity-binding assertions were added.

## 12. Governed Phase Record

- workflow: `dg/runtime-composition`
- run: `run-1785036579170637000-2`
- approval:
  `approval/run-1785036579170637000-2/composition-approved`
- presentation: `presentation/3e150ff278ef0065`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval-presentation event marker: present
- out-of-kernel work: source inspection, Rust implementation, focused tests,
  documentation, validation, and later git/PR work
- missing coverage: the kernel coordinated governance only; it did not edit
  files, execute checks, persist a receipt, create a WorkReport artifact, or
  perform git actions

## 13. Remaining Limitations And Recommendation

This slice is fresh-run-only, local, `DocsCheck`-only, injected-surface-only,
and in-memory for disclosure receipt posture.

Proceed next to a focused maintainer review of this visible route. Do not begin
approval-required or denial routing until that review accepts runtime ordering,
authority, failure, privacy, and concurrency behavior.
