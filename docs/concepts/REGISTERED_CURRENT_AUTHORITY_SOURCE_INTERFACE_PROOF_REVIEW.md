# Registered Current-Authority Source Interface Proof Review

## 1. Executive Verdict

Phase accepted; proceed to private registered-source and same-call resolver
composition.

The proof establishes a credible Core-owned source boundary without exporting
source invocation or conferring readiness. No blocker was found.

## 2. Scope Verification

The phase stayed within the approved private interface-proof scope.

It added one crate-private in-memory aggregate source, Core-owned registration
construction, exact request derivation, canonical inventory selection,
payload-free snapshot commitments, bounded source failures, focused tests, and
documentation.

It did not add a public source trait, runtime consumer, readiness API, target
dereference, persistence, events, audit projection, providers, OpenShell,
sandbox execution, SideEffect execution, writes, schemas, SDKs, CLI behavior,
hosted behavior, reasoning lineage, or release changes.

## 3. Trust-Boundary Assessment

The trust boundary is appropriately narrow for this proof.

- The source module is private to `current_authority_source`.
- No source type or invocation method is exported from `workflow-core`.
- The source constructor creates `CurrentAuthoritySourceRegistration`
  internally.
- A caller cannot inject a separately constructed public registration into the
  private source.
- Registration fixes the source kind to `LocalAggregate`, consistency to
  `AtomicSnapshot`, all three supported fact families, and required
  redaction.

The private input still supplies bounded source identity, configuration
commitment, freshness, sensitivity, and inventory values. That is acceptable
inside this Core-owned proof, but future runtime registration must not expose
the same input as an untrusted public trust-establishment API.

## 4. Exact-Request Assessment

Each read derives `CurrentAuthoritySourceRequest` from:

- the accepted internal registration;
- the exact immutable execution binding;
- the exact required-context contract;
- all three supported fact families; and
- an injected evaluation timestamp.

The request constructor revalidates binding and contract identity, rejects
evaluation before binding, binds the canonical exact query set, enforces the
registered sensitivity ceiling, and commits all decision-relevant bounded
posture.

Callers cannot provide a prefiltered query, prior fact-set commitment, or
caller-authored source snapshot to the private read boundary.

## 5. Selection And Completeness Assessment

The source owns canonical complete inventories.

- Grant selection includes every candidate matching exact capability,
  resource, actor, workflow, run, step, and harness scope.
- Grant lifecycle, expiry, delegation, and prerequisites remain resolution
  concerns rather than source-selection filters.
- Every exact query requires one availability observation.
- Every contract target requires one payload-free governed context reference.
- Duplicate grant IDs, availability keys, and context-reference targets are
  rejected before reads.
- Zero matching grants remains a valid complete result.

The resulting snapshot reports exact family coverage and exact per-query
availability/reference counts. Missing exact records return `Incomplete`
rather than being misrepresented as negative authority.

## 6. Consistency And Freshness Assessment

The in-memory source commits one canonical inventory and represents the read as
one atomic snapshot. The inventory commitment supplies the opaque watermark,
while the selected-record commitment binds the exact request and returned
records.

Freshness is deterministic:

- source observation time cannot be after evaluation;
- source validity cannot predate observation;
- Core maximum observation age caps source validity;
- stale and future-dated snapshots become bounded failures; and
- no hidden clock read occurs.

The proof does not simulate concurrent mutation. That is an explicit
limitation, not an overclaim. A later mutable or external source must prove
atomicity or stable-watermark behavior rather than inheriting the in-memory
claim.

## 7. Failure Assessment

Read outcomes are explicit snapshots or bounded failures.

Missing exact records, stale data, future data, and query mismatch cannot
become permission. Failure records contain only registration and request
commitments, typed kind, and bounded retry posture.

Inventory-construction defects fail as stable validation errors before a read.
That is appropriate for an in-memory trusted-source constructor. A later
external source must map operational, corrupt, ambiguous, and concurrent
failures explicitly at its source boundary.

## 8. Privacy And Redaction Assessment

The source stores validated authority metadata and payload-free references. It
does not store target contents, provider payloads, command output, source
files, credentials, environment values, raw configuration, or endpoints.

Debug output redacts registration identity, timestamps, inventory commitments,
and target-bearing records. Errors do not echo rejected IDs or source values.
The public snapshot contains commitments and bounded counts rather than source
records.

The future resolver-composition phase must keep selected source records private
and must not treat a payload-free snapshot commitment by itself as consumable
authority.

## 9. Test Quality Assessment

Focused tests cover:

- Core-owned registration and a complete exact snapshot;
- deterministic commitment across inventory order;
- exact three-family coverage;
- incomplete failure for a missing exact record;
- stale and future-dated failure;
- duplicate inventory rejection; and
- redaction-safe source Debug output.

Workspace validation also covers the public source model, capability authority,
governed context, immutable bundles, proportional governance, runtime events,
SideEffect foundations, reports, providers, and catalog behavior.

Non-blocking test follow-ups for the composition phase:

- test missing governed-context-reference coverage separately;
- test invalid and expired source-validity bounds at the private source;
- test multiple matching grant candidates, including inactive candidates;
- test registration/request sensitivity boundaries through the private source;
- test query mismatch mapping at the private read outcome; and
- add fixed commitment vectors before persistence or cross-process
  compatibility depends on these hashes.

## 10. Documentation Assessment

The roadmap, source-boundary plan, and implementation report accurately
describe a private in-memory proof.

They do not claim a production source, public registration service, current
authority readiness, dereference, runtime consumption, provider execution,
OpenShell integration, SideEffect execution, writes, persistence, schemas,
hosted behavior, or reasoning lineage.

## 11. Blockers And Follow-Ups

Blockers: none.

Non-blocking follow-ups:

- keep trusted registration and source invocation private;
- preserve exact request derivation inside Core;
- carry selected records and their snapshot commitment together into the
  same-call resolver;
- fail closed on any source failure before resolution;
- prevent reusable readiness, replay, or later target dereference;
- broaden negative-path tests during composition;
- define one-time-use posture before a runtime consumer; and
- keep OpenShell as a later optional execution-provider boundary, not an
  authority source.

## 12. Recommended Next Phase

Compose the private registered source with the existing private same-call
resolver.

The composition should perform one source read and one resolution chain in the
same Core-owned call, bind the resulting assessment to the exact source
snapshot commitment, and return no reusable authorization handle.

Do not add a public source trait, executor integration, readiness API, target
dereference, persistence, providers, OpenShell, sandbox execution, SideEffect
execution, writes, schemas, CLI behavior, or hosted behavior.

## Governed Review Record

- workflow: `dg/review`
- run ID: `run-1785166638391524000-2`
- approval ID:
  `approval/run-1785166638391524000-2/review-scope-approved`
- approval presentation ID: `presentation/75a87eb2bf2a2559`
- approval presentation content hash:
  `75a87eb2bf2a2559890d8546de4192b1acd9f424b6f9092b7184252d13a45456`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted review handoff was presented
- review status: accepted and governed phase completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval-presentation enforcement: proof enforced with event marker
- out-of-kernel work: the delegated maintainer independently inspected the
  implementation, tests, documentation, and prior validation evidence and
  authored this review; the kernel governed scope and approval but did not
  inspect code, edit files, execute checks, or mutate git
