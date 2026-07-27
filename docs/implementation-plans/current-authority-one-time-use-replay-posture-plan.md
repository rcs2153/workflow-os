# Current-Authority One-Time-Use And Replay Posture Plan

Status: Planning complete. No runtime code, public model, persistence, event,
schema, CLI behavior, provider integration, sandbox integration, or write
behavior is implemented by this plan.

Related foundations:

- [Required Context Immutable-Run Binding And Time-Of-Use Plan](required-context-immutable-run-time-of-use-plan.md)
- [Production Current-Authority Source Boundary Plan](production-current-authority-source-boundary-plan.md)
- [Registered Current-Authority Source Resolver Composition Review](../concepts/REGISTERED_CURRENT_AUTHORITY_SOURCE_RESOLVER_COMPOSITION_REVIEW.md)
- [Scoped Runtime Authority And Capability Projection Plan](scoped-runtime-authority-capability-projection-plan.md)
- [Proportional Governance And Quiet Success Plan](proportional-governance-quiet-success-plan.md)
- [Immutable Run Bundle Boundary Plan](immutable-run-bundle-boundary-plan.md)

## 1. Executive Summary

Workflow OS can now derive one private source-backed current-authority
assessment from an exact immutable execution binding, required-context
contract, coherent source snapshot, capability grants, availability, governed
context references, and one explicit evaluation time.

That assessment is not permission that may be cached, serialized, persisted,
retried, resumed, or passed to a later consumer. `Ready` means only that the
exact required context was satisfied during that resolution call.

The safe next boundary is not a reusable token with a time-to-live. It is a
private Core-owned resolve-and-use operation:

```text
exact immutable binding
  -> read current registered source
  -> resolve current authority
  -> project exact governed context
  -> consume exact required-context contract
  -> borrow one non-reusable use capability
  -> invoke one bounded consumer in the same call
  -> return a payload-free outcome
```

Every retry, approval resume, worker restart, or later use must repeat the
entire source-backed resolution chain. Durable replay prevention cannot be
claimed until a future authoritative store records use identity and
consumption atomically with the consuming boundary.

This plan does not implement anything.

## 2. Goals

- Prevent a private `Ready` assessment from becoming ambient or reusable
  authority.
- Require fresh registered-source resolution for every attempted use.
- Define deterministic semantics for retries, approval resume, cancellation,
  worker restart, and duplicate calls.
- Keep authority material private, borrowed, non-cloneable, and
  non-serializable.
- Preserve exact immutable run, actor, workflow, run, step, harness, contract,
  sensitivity, source snapshot, fact-set, and evaluation-time binding.
- Fail closed when freshness, source, authority, prerequisites, projection, or
  consumption is unknown or changed.
- Separate same-call non-reuse from durable cross-process replay prevention.
- Prepare one future opt-in read-only consumer without authorizing it.
- Preserve proportional-governance and quiet-success correctness: lower
  friction may consume current authority but cannot weaken it.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- a public readiness or authorization API;
- a bearer token, lease, session token, or TTL-based authority cache;
- persistence or an authority-use ledger;
- executor integration;
- target dereference or payload access;
- provider or adapter invocation;
- OpenShell or another sandbox integration;
- SideEffect execution;
- writes;
- runtime events or audit projection;
- report artifacts;
- workflow schema or SDK changes;
- CLI or UI behavior;
- hosted or distributed runtime;
- enterprise identity or administration;
- cryptographic receipts;
- reasoning lineage; or
- release posture changes.

## 4. Threat Model

The boundary must prevent:

- replaying a prior `Ready` assessment after a grant expires or is revoked;
- reusing readiness after capability availability changes;
- resuming under a changed policy, approval, evidence, or check prerequisite;
- substituting actor, workflow, run, step, harness, contract, sensitivity, or
  target;
- caching a projection or consumption result and treating it as current;
- serializing an assessment and restoring it after worker restart;
- using one assessment for multiple targets or invocations;
- using a source snapshot commitment as if it were authority;
- treating matching deterministic commitments as proof that no time or source
  change occurred;
- caller-chosen nonces that falsely imply replay protection;
- retrying after an ambiguous consumer outcome without reconciliation; and
- treating logs, reports, or evidence citations as permission.

## 5. Source-Of-Truth Boundaries

| Concern | Source of truth | Must not be treated as |
| --- | --- | --- |
| Run and definition identity | Validated stored immutable run bundle | Current mutable project files |
| Exact required context | Immutable execution binding and exact contract | A prior projection |
| Current authority | Same-call registered source selection and resolution | A stored assessment commitment |
| Current availability | Exact source-backed availability records | Prior success |
| Current prerequisites | Accepted records from their owning boundaries | Caller booleans or report text |
| Same-call use eligibility | Private resolved assessment in the active call | A lease or bearer token |
| Durable replay history | Future authoritative use store | An in-memory set or caller nonce |
| Consumer outcome | Future bounded consumer result | Source readiness |

## 6. Core Semantic Decision

The first boundary must use **re-resolve per use**.

There is no reusable validity window. A source may have a bounded freshness
policy, but freshness controls whether current facts may be assessed; it does
not make the resulting assessment reusable.

The following are separate:

- **source freshness**: whether the source observation is acceptable at one
  evaluation time;
- **assessment currency**: whether the assessment was derived during the
  current use call;
- **same-call non-reuse**: whether authority can escape or be invoked more than
  once inside one process;
- **durable replay prevention**: whether a prior use identity can be detected
  across retries, workers, or process restarts; and
- **consumer idempotency**: whether repeating a consumer request can duplicate
  an external effect.

The first implementation may prove only same-call non-reuse. It must explicitly
disclaim durable replay prevention and consumer idempotency.

## 7. Candidate Private API Shape

The first implementation should remain crate-private and likely add:

- `RegisteredCurrentAuthorityUseInput`;
- `RegisteredCurrentAuthorityUsePosture`;
- `RegisteredCurrentAuthorityUseReason`;
- `RegisteredCurrentAuthorityUseOutcome`;
- a private borrowed `RegisteredCurrentAuthorityUseCapability<'call>`; and
- `with_registered_current_authority`.

The operation should accept:

- exact `RequiredContextExecutionBinding`;
- exact `RequiredContextContractBinding`;
- one injected resolution/use timestamp;
- validated redaction metadata; and
- one `FnOnce` consumer owned by Core.

The operation should:

1. call the private registered-source resolution path;
2. stop on source failure or `Blocked`;
3. construct a private use capability only for `Ready`;
4. borrow that capability only for the lexical lifetime of one `FnOnce`
   consumer;
5. prevent the assessment or capability from being cloned, serialized,
   persisted, or returned;
6. invoke the consumer at most once;
7. return only a bounded payload-free outcome; and
8. map all errors to stable non-leaking codes.

The private capability must not expose `authorize`, `permit`, `token`,
`lease`, or unrestricted target methods. It should expose only the exact
binding and requirement references needed by the future consumer, preferably
through Core-owned operations rather than raw accessors.

The callback invocation itself must be the single governed use. `FnOnce`
prevents a second callback invocation, but it does not by itself prevent code
inside the callback from repeating a privileged operation. The first
implementation therefore must not give the callback a general-purpose
authority object with repeatable operation methods. The bounded consumer and
the exact operation it performs must remain Core-owned.

## 8. Why A Borrowed `FnOnce` Boundary

A private borrowed capability and `FnOnce` consumer provide useful
compile-time constraints:

- the consumer cannot be invoked twice;
- the capability cannot outlive the call;
- no serializer can persist it;
- no clone can create parallel authority;
- the source-backed assessment never becomes public input; and
- a future consumer can be integrated without changing the public contract.

This is not durable replay protection. A new process can call the resolver
again. That is correct: it must obtain current facts again rather than restore
old permission.

## 9. Freshness And Change Invalidation

Every use gets one explicit `evaluated_at` supplied at the consuming boundary.
The source and resolver must revalidate:

- binding creation time;
- source observation and valid-through bounds;
- Core maximum source age;
- source snapshot generation or watermark posture;
- grant lifecycle, expiry, revocation, scope, delegation, and sensitivity;
- capability availability;
- exact governed context reference coverage;
- required policy, approval, evidence, and check prerequisites; and
- required-context satisfaction.

Any changed input produces a new assessment or a failure. A deterministic
assessment commitment may repeat when every committed field repeats. That does
not prove the assessment is reusable, and no consumer may accept the
commitment as authority.

Changes that always require a new resolution include:

- actor, workflow, run, step, or harness;
- immutable bundle or contract identity/hash;
- requested sensitivity;
- source observation, validity, generation, watermark, or inventory;
- grant, availability, or context-reference records;
- prerequisite decisions or evidence/check posture;
- retry attempt or resumed execution boundary; and
- any later consumer invocation.

## 10. Retry Semantics

A retry must never reuse the prior assessment, projection, consumption result,
or private capability.

Before each retry:

1. reload the accepted immutable run boundary;
2. revalidate the exact execution binding and contract;
3. read the registered current-authority source;
4. recompute capability resolution and required-context consumption; and
5. invoke the consumer only if the new assessment is `Ready`.

If a consumer did not begin, an ordinary retry may start a new resolution.

If a read-only consumer began but returned an ambiguous outcome, the runtime
must disclose uncertainty before repeating it. A future write-capable consumer
requires the existing SideEffect/idempotency/reconciliation boundaries and is
outside this plan.

## 11. Approval Resume Semantics

Approval grants permission only for the bounded approved action and context.
It does not preserve a prior current-authority assessment.

On approval resume:

- reload durable approval and approval-presentation proof from their owning
  boundaries;
- rehydrate the exact immutable run;
- reject workflow/spec/context substitution;
- read current authority again at the decision-time/use-time boundary;
- re-evaluate grant lifecycle, availability, sensitivity, and all independent
  prerequisites;
- rerun required-context consumption; and
- block when authority changed after approval.

An approval granted before revocation, expiry, policy denial, evidence
invalidation, check staleness, or context unavailability cannot override the
new current posture.

## 12. Worker Restart And Recovery

No private assessment or use capability may be serialized into workflow state.

After worker or process restart:

- load only durable run identity and accepted owning records;
- reconstruct the immutable execution binding from validated sources;
- perform a new registered-source resolution;
- never restore `Ready` from an assessment commitment; and
- never infer permission from a prior event, log, report, or successful use.

If future runtime recovery needs to know whether a consumer already ran, it
requires a durable use record and idempotency/reconciliation design. Until
that exists, ambiguous prior use must block automatic replay.

## 13. Duplicate Calls And Concurrency

The private same-call boundary prevents duplicate invocation within one call.
It does not prevent two workers from resolving and invoking concurrently.

Before any distributed or persistent consumer, Workflow OS must define:

- a stable Core-owned use-operation ID;
- authoritative create-only or compare-and-set registration;
- exact run/step/target/attempt binding;
- pending, completed, failed, and ambiguous lifecycle;
- atomic claim/consume semantics;
- stale-claim recovery;
- idempotency-key relationship;
- reconciliation after uncertain completion; and
- retention and privacy posture.

Caller-supplied IDs or in-memory locks cannot establish this guarantee.

## 14. Candidate Outcome Vocabulary

The private same-call outcome may expose only bounded posture such as:

- `BlockedBeforeUse`;
- `ConsumerNotInvoked`;
- `ConsumerSucceeded`;
- `ConsumerFailed`;
- `ConsumerOutcomeAmbiguous`; and
- `SourceFailure`.

Stable reasons should include:

- source unavailable, incomplete, stale, future-dated, or changed;
- binding or contract mismatch;
- current authority blocked;
- independent prerequisite required;
- required context gap;
- capability use already attempted inside the call;
- consumer rejected the exact context;
- consumer failure; and
- consumer outcome ambiguity.

The outcome must not contain context payloads, source records, raw consumer
errors, command output, provider responses, credentials, paths, or tokens.

## 15. Relationship To Proportional Governance

Proportional governance may select quiet execution only after current authority
is resolved at the consuming boundary.

Quiet success may reduce presentation friction. It cannot:

- cache authority;
- skip source resolution;
- reuse a prior approval or check as current;
- suppress blocked or ambiguous posture;
- convert missing facts into permission; or
- omit evidence, audit, disclosure, or reporting obligations.

Visible disclosure remains presentation posture rather than authority.
Blocking approval remains an execution requirement rather than a substitute
for capability authority.

## 16. Relationship To Scoped Runtime Authority

The future scoped-runtime-authority lane may consume this boundary but must not
replace it.

A scoped capability grant states bounded authority vocabulary. The
registered-source resolver determines whether that grant and its prerequisites
are current for one exact use. Step-scoped tool visibility must be rebuilt from
that current resolution and must not survive beyond the exact invocation.

Composable Harness Contracts must receive only the capabilities and context
references authorized for their exact harness boundary.

## 17. Relationship To OpenShell And Other Execution Providers

OpenShell or another optional execution provider may later enforce filesystem,
network, process, inference, and credential boundaries. It is not the current
authority source and cannot make a blocked Workflow OS assessment ready.

For every sandbox invocation, Workflow OS must re-resolve current authority,
then bind the exact sandbox identity, effective policy revision/hash, runtime
identity, and invocation context to that use. A prior sandbox or loaded policy
must not make a later Workflow OS assessment reusable.

No OpenShell adapter or fork is authorized by this plan.

## 18. Privacy And Redaction

The model and errors must not store or expose:

- target payloads or source contents;
- provider, command, process, parser, CI, Jira, GitHub, or sandbox payloads;
- credentials, authorization headers, private keys, tokens, or cookies;
- environment values;
- raw paths, mounts, endpoints, or policies;
- raw approval, evidence, check, or audit payloads;
- unbounded errors; or
- caller values in validation failures.

Debug output should reveal only bounded posture, reason categories, and counts.
Commitments, timestamps, IDs, and exact targets remain redacted unless a
separately reviewed report-safe reference requires them.

## 19. Error Handling

Failures must use stable non-leaking codes and preserve the stage that failed:

- source read;
- fact-set construction;
- capability resolution;
- projection;
- required-context consumption;
- private use-capability construction;
- consumer invocation; or
- ambiguous consumer completion.

No error may fall back to permission, silently retry, fabricate evidence, or
become a misleading user project diagnostic.

## 20. First Implementation

The next implementation should be the private same-call use boundary only:

1. add the private borrowed use capability;
2. add the Core-owned `FnOnce` resolve-and-use helper;
3. call a test-only bounded read-only consumer;
4. prove the consumer is never invoked for source failure or `Blocked`;
5. prove the capability cannot be returned, cloned, or serialized through the
   public API;
6. return bounded payload-free outcomes;
7. add focused determinism, privacy, and failure tests; and
8. perform focused maintainer review.

Do not add persistence, executor wiring, dereference, a real provider,
OpenShell, sandbox execution, SideEffects, writes, schemas, or CLI behavior.

## 21. Test Plan

Future tests should prove:

- `Ready` invokes exactly one `FnOnce` consumer;
- `Blocked` never invokes the consumer;
- source failure never invokes the consumer;
- consumer success returns bounded success;
- consumer failure returns a stable non-leaking failure;
- ambiguous consumer completion remains explicit and blocks automatic retry;
- the capability is private, non-cloneable, and non-serializable;
- the capability cannot escape its call lifetime;
- changed binding, contract, actor, run, step, harness, sensitivity, source,
  grant, availability, reference, or prerequisite posture forces a new
  resolution or blocks;
- expired and revoked grants block later use;
- retry obtains a new assessment;
- approval resume obtains a new assessment;
- worker restart cannot restore prior readiness;
- duplicate in-call invocation is structurally impossible;
- fixed commitment vectors remain stable where compatibility requires them;
- Debug and errors do not leak identities, hashes, targets, timestamps, paths,
  or secret-like values; and
- existing current-authority, capability, context, approval, local-check,
  proportional-governance, runtime, provider, and workspace tests pass.

Compile-fail tests may be justified later if Rust privacy and lifetime
constraints cannot be demonstrated adequately through public API tests. Do not
add a new compile-test dependency without separate justification.

## 22. Proposed Implementation Sequence

1. Focused plan review.
2. Private same-call use-capability and `FnOnce` boundary.
3. Focused implementation review.
4. Direct negative-path and fixed-vector hardening.
5. Plan one opt-in read-only consumer.
6. Implement and review that consumer with no persistence or provider.
7. Separately plan durable use records and replay prevention.
8. Only after those reviews, consider an optional execution provider such as
   OpenShell.

## 23. Open Questions

- Should the first helper invoke the consumer only for fully required-context
  `Ready`, or should it support an explicit observation-only blocked callback?
- Which bounded consumer outcome categories are needed before a real read-only
  consumer exists?
- Should consumer ambiguity be modeled now or only when a real consumer can
  begin work?
- What stable use-operation identity belongs in a future durable store?
- Which source generation/watermark changes require explicit operator
  disclosure even when reassessment remains `Ready`?
- How should accepted approval, evidence, and check records enter the current
  source without caller-authored booleans?
- Which runtime event should record a future same-call use without exposing
  the authority material?
- When should fixed assessment and use-outcome commitment vectors become
  compatibility requirements?

## 24. Final Recommendation

Proceed to focused maintainer review of this plan.

If accepted, implement the private same-call `FnOnce` use boundary. Do not
create a reusable authority token, TTL lease, public readiness result,
persistent replay record, executor integration, dereference path, provider,
OpenShell adapter, SideEffect execution, schema, CLI behavior, or write.
