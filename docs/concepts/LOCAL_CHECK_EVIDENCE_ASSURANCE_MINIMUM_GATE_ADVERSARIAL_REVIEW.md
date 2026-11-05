# Local Check Evidence Assurance Minimum Gate Adversarial Review

## Verdict

No P0 or P1 defect was found in the tested minimum-gate boundaries.

This was a maintainer-operated adversarial pass against exact merged `main`, not
independent assurance. The delegated maintainer implemented and reviewed the
slice, so an external reviewer must still reproduce the attacks before the
roadmap's independent-review requirement is closed.

## Attacks Exercised

### Declaration substitution

The review constructed otherwise equivalent declarations with:

- no evidence-assurance minimum;
- the current weak producer and attestation minimum; and
- a stronger registered-producer minimum.

Each posture produced a distinct canonical declaration-set fingerprint. Editing
the serialized minimum without recomputing the complete declaration-set
identity failed deserialization with the existing bounded invalid-record error.

### Serialized projection escalation

The review changed a valid weak serialized projection from
`operator_selected_content_bound` to `registered_content_bound` while retaining
the original fingerprint. Deserialization rejected the projection. Unknown
producer vocabulary remains rejected by the same closed parser.

### Same-run downgrade after denial

The review started an authoritative project-validation run whose immutable
declaration required `registered_content_bound`. The current weak projection
was persisted and the run failed with:

```text
executor.local_check_evidence_assurance.minimum_not_satisfied
```

The workflow declaration was then weakened to
`operator_selected_content_bound`, and the same run identity was retried. The
route rejected the attempt as an existing immutable run before invoking the
validator again. Across both attempts there was one validator call, zero skill
calls, and zero workflow events.

### Denial ordering

Existing and rerun coverage confirms that the bounded validator executes so the
kernel can classify its result, then the gate persists or resolves the durable
projection and rejects insufficient assurance before workflow events or skill
execution.

### Compatibility and ambiguity

Declarations without `evidence_assurance_minimum` preserve prior behavior. The
closed authoritative route already requires one declaration-set record and one
selected local-check declaration; missing, duplicate, or unresolved selection
fails closed before the minimum gate can authorize work.

## Evidence Boundary

These tests prove identity-bound downgrade and ordinary serialized-tamper
resistance inside the current local trust domain. They do not prove:

- semantic trustworthiness of an operator-selected validator;
- independence of the trusted host;
- resistance to a privileged storage owner who can replace all records and
  recompute every unkeyed commitment;
- authenticity of the future registered-producer vocabulary;
- cryptographic signing or external witnessing; or
- live provider or hosted execution safety.

The gate remains a truthful classifier and enforcement consumer for durable
local assurance posture. It is not an independent evidence authority.

## Permanent Regression Coverage

The review added coverage for:

- declaration-set identity changes across absent, weak, and registered producer
  minimums;
- minimum substitution without matching canonical identity;
- serialized weak-to-registered projection tampering; and
- same-run declaration downgrade after an assurance denial.

## Validation

Focused adversarial tests passed for canonical declaration identity,
projection serde validation, and authoritative executor downgrade rejection.
The complete Core all-feature suite passed, including 792 library tests, 351
local-executor integration tests with one opt-in live GitHub test ignored, and
the remaining Core integration and documentation tests. Warnings-denied Core
lint, repository formatting, documentation validation, and diff checks also
passed.

Live credential-backed and external provider tests are out of scope because the
minimum gate adds no transport or provider mutation behavior.

## Governed Review

- Workflow: `dg/review`
- Run: `run-1793901390393951000-2`
- Approval: `approval/run-1793901390393951000-2/review-scope-approved`
- Presentation: `presentation/b2f021d6c4b9537c`
- Presentation hash:
  `b2f021d6c4b9537c90f866378f8ae6b573bda220a2e724552ce6d4755d0af8ba`
- Approval outcome: granted by the delegated maintainer after complete handoff
  presentation
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- Approval-presentation enforcement: proof enforced

Repository edits, adversarial tests, and validation were performed outside the
kernel under the approved scope. The kernel coordinated the review and durable
approval proof; it did not execute the attacks or manufacture their results.

## Remaining Requirement

An external reviewer must reproduce or extend these attacks from a clean exact-
main checkout. Until that occurs, the roadmap must describe this pass as
maintainer-operated adversarial evidence rather than independent assurance.
