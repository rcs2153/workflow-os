# Production Current-Authority Source Boundary Plan Review

## 1. Executive Verdict

**Plan accepted; proceed to production current-authority source-boundary core
model implementation only.**

The plan defines the missing trust boundary between caller-supplied facts and
future Core-owned current-authority reads. Two architecture claims were
corrected during review: opaque watermark equality cannot prove monotonic
ordering, and source-supplied validity cannot override a stricter Core-owned
freshness policy.

## 2. Scope Verification

The plan stayed within planning-only scope.

It does not authorize a source implementation, trait, registry, runtime
consumer, readiness API, dereference, persistence, provider, OpenShell
integration, sandbox execution, SideEffect execution, write, schema, SDK,
CLI, UI, hosted service, reasoning lineage, or release change.

## 3. Trust-Root Assessment

The trust root is correctly assigned to a future Core-owned registration and
source invocation boundary.

A public constructor, serialized source registration, source response, or
caller-built fact set cannot establish trusted current authority. Registration
vocabulary in the first model phase remains descriptive and incapable of
authentication or readiness.

## 4. Exact-Request Assessment

The source request is bound to the immutable execution-binding commitment,
required-context contract commitment, canonical exact query-set commitment,
accepted registration commitment, requested fact families, sensitivity bounds,
and evaluation time.

Typed actor, workflow, run, step, and harness identities must be derived
internally from the already validated immutable binding. The model must not
accept a second caller-controlled identity set that can diverge from that
binding.

## 5. Completeness Assessment

`CompleteForExactQuery` has an appropriately strict meaning:

- every requested fact family was evaluated;
- every exact capability/resource query has an availability observation;
- every matching grant candidate in the coherent source view was returned;
- every exact context target has a current reference posture; and
- the claim is committed to the exact query set and source snapshot.

Unsupported, unavailable, incomplete, and unknown states remain distinct from
valid negative facts. An empty grant result may be complete; an omitted
required availability or context-target posture may not.

## 6. Snapshot And Concurrency Assessment

One aggregate snapshot for grants, availability, and governed context
references is the correct first production-shaped boundary. It avoids silently
combining facts observed at different times.

The plan now distinguishes:

- an opaque snapshot-watermark commitment, which proves equality or change;
  and
- an optional comparable source generation, whose ordering semantics must be
  defined by the accepted source contract.

An opaque hash is not evidence of monotonic ordering. A changed watermark
without a coherent snapshot fails closed as concurrent change.

## 7. Freshness Assessment

Freshness is explicit and deterministic. Observation, read-window, evaluation,
and optional validity times are validated with an injected clock boundary.

A source may bound its own observation validity, but it cannot decide how long
Workflow OS trusts authority. Future effective freshness must use the stricter
of source validity and a Core-owned maximum-age policy. Unknown or stale facts
may increase friction or deny work; they may never select a quieter governance
mode.

## 8. Failure And Retry Assessment

The failure taxonomy separates legitimate negative authority facts from
operational source failures. Stable bounded categories cover unsupported,
incomplete, unavailable, stale, future-dated, concurrent-change, ambiguous,
corrupt, registration-mismatch, query-mismatch, transport, and internal
posture.

Retry remains deferred. The future posture preserves the exact immutable
request, obtains a new complete snapshot, bounds attempts, and never reuses
partial facts.

## 9. Privacy And Compatibility Assessment

The planned model is payload-free and excludes credentials, provider payloads,
target contents, source code, command output, paths by default, raw
configuration, and raw database or queue cursors.

Debug, serialization, deserialization, and validation errors must remain
non-leaking. Public model types will become compatibility surface, so the
model-only phase must remain smaller than the broader future source
implementation.

## 10. Product Feedback Assessment

Fresh-pull evaluation describes Workflow OS as a credible local governance
kernel and recommends proportional governance and quiet success as the next
product priority.

This plan supports that direction rather than adding ceremony. A quiet decision
is only safe when Core can prove that the authority facts are current,
complete, coherent, and bound to the exact decision. The source does not choose
governance mode, and visible disclosure remains a delivery concern rather than
a separate authority fact.

## 11. OpenShell Assessment

OpenShell remains correctly positioned as a future optional execution
containment provider after a Workflow OS governance decision.

It is not a current-authority source, does not establish approval or policy
satisfaction, and is outside this phase. Forking or integrating OpenShell is
not required to implement the source-boundary model.

## 12. Blockers

None after the watermark and freshness corrections recorded in the plan.

## 13. Non-Blocking Follow-Ups

- Keep source registration incapable of authentication until a Core-owned
  registry or construction boundary exists.
- Select the minimum accepted consistency posture for the first private source
  proof.
- Define the first Core-owned maximum-age policy before runtime consumption.
- Define independent current sources for policy, approval, evidence, and check
  prerequisites.
- Decide one-time-use or replay posture before dereference.

## 14. Recommended Next Phase

Implement the production current-authority source-boundary core model only.

Do not add a source trait, registry, concrete source, runtime resolver,
readiness API, target dereference, provider, OpenShell adapter, SideEffect
execution, write, schema, CLI, hosted behavior, or release change.

## 15. Validation

- `npm run check:docs`: passed with the repository Node 20 toolchain.
- `git diff --check`: passed.

## 16. Dogfood Governance

- workflow: `dg/review`
- run ID: `run-1785157517096356000-2`
- approval ID:
  `approval/run-1785157517096356000-2/review-scope-approved`
- presentation ID: `presentation/bb308030024c0287`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted review handoff was presented
- phase status: completed
- event summary: 39 events, one approval, zero retries, zero escalations;
  approval-presentation proof was persisted and enforced
- validation summary: documentation and diff integrity checks passed
- out-of-kernel work: architecture inspection, review judgment,
  documentation edits, and validation were performed by the delegated
  maintainer; the kernel governed scope and approval but did not inspect docs,
  edit files, execute checks, or mutate git
