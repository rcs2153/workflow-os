# OpenShell v0.1.5 Execution-Evidence Sufficiency Matrix Review

## 1. Executive Verdict

**Matrix accepted; provider wiring remains blocked. Proceed only to a bounded
upstream gap-record and discussion-draft refresh.**

The matrix accurately evaluates OpenShell `v0.1.5` at source commit
`bdb956f2cd65a635f1294fccf1d138d263bf650f` for the selected Docker-on-Apple-
Silicon evaluation boundary. It correctly distinguishes public schema
vocabulary from facts that the selected production driver actually projects.

Stable `v0.1.5` materially improves mutation identity, desired-state
operations, provisioning ownership, endpoint status, and deletion outcomes.
Those advances do not close the mandatory execution-evidence boundary. The
selected Docker driver leaves required workload-identity and fence evidence
empty, admitted exec retries cannot recover terminal outcome, owner-loss
reconciliation is unavailable, security observations remain incomplete, and
cleanup lacks one terminal generation-bound receipt.

This verdict accepts the matrix, not an execution provider. It does not
authorize a compatibility transport, live smoke, OpenShell installation,
Cargo invocation, provider wiring, or fork.

## 2. Scope Verification

The matrix stayed within its approved source-analysis scope. It pinned one
stable release and selected boundary, inspected official source and
documentation, classified exact facts, defined fail-closed Workflow OS
outcomes, preserved privacy boundaries, and updated roadmap status.

It did not add or execute Rust, OpenShell, Cargo, a transport, a sandbox, a
provider, admission, persistence, events, schemas, CLI behavior, authority,
writes, or release changes. No scope expansion was found.

## 3. Source And Version Assessment

The source pin is exact:

- OpenShell tag `v0.1.5` resolves to
  `bdb956f2cd65a635f1294fccf1d138d263bf650f`;
- inspected upstream `main` resolves to
  `c4688aa8715ea75d6c39fe468361dbbc16c691fa`; and
- the relevant public gateway and compute-driver protobufs are unchanged
  between those refs.

The matrix correctly limits that conclusion to reviewed source. It does not
treat source provenance as installed-binary, release-asset, image, driver,
platform, or live enforcement proof.

## 4. Selected Boundary Assessment

Selecting the OpenShell Docker driver on a local Apple Silicon macOS host is a
specific and reviewable evaluation boundary. The matrix does not infer
cross-driver portability from shared protobufs or version strings.

The candidate remains optional. Workflow OS owns authorization, evidence
acceptance, audit, and reporting; OpenShell would own containment and report
facts from its execution boundary. The matrix preserves that separation.

## 5. Identity And Capability Assessment

The matrix correctly accepts only narrow identity and capability facts:

- gateway-owned sandbox identity and metadata;
- driver and extension self-reported identity;
- selected CPU, memory, and GPU capability fields; and
- bounded request replay identity.

It correctly rejects those fields as a complete security-capability contract.
The Docker capability response does not negotiate mandatory filesystem,
process-tree, egress, protected-write, observation-completeness, and terminal-
cleanup capabilities. Unknown support must fail before execution.

## 6. Docker Workload And Fence Assessment

The decisive selected-driver finding is source-backed.

The public `DriverSandboxStatus` schema contains `resolved_identity` and
`fence_evidence`. The Docker driver resolves workload identity internally
during provisioning, but its production status constructors populate name,
instance ID, lifecycle conditions, and deletion posture, then use default
values for the remaining fields. Internal resolution and tests do not expose
those facts to a Workflow OS adapter.

The matrix therefore correctly classifies exact resolved workload identity,
generation-bound fence evidence, and exact drift comparison as unavailable.
Protobuf vocabulary is not execution evidence.

## 7. Invocation And Reconciliation Assessment

The matrix correctly separates durable launch admission from durable terminal
outcome.

An exec `request_id` prevents duplicate relaunch. Official documentation also
states that a retry after admitted completion returns
`REQUEST_STREAM_UNAVAILABLE` and does not replay output, exit code, stream
attachment, or stdin. A connected terminal frame can prove the outcome only
if Workflow OS durably projects it before loss; it cannot support restart
reconciliation by itself.

The required result remains `ReconciliationRequired`, never inferred success.

## 8. Driver-Operation Ownership Assessment

The persisted `driver_operation_pending` and `driver_operation_id` fields are
meaningful safety advances. They fence stale callbacks and block unsafe
start, stop, deletion, retry, and cleanup completion while the original driver
operation remains owned.

Official lifecycle guidance also states that a gateway crash before recording
the driver response can leave that state indefinitely and that no resolution
API exists. The matrix correctly treats this as safe fail-closed behavior but
not completion or recovery evidence.

## 9. Observation Assessment

The observation classifications are conservative and accurate.

Individual retained OCSF records can authoritatively prove their own bounded
contents. They cannot prove a complete operation interval. Official docs
describe collection as best-effort, document bounded queues and dropped
records, identify shutdown loss without a gap record, and require collector
metrics for partial rejection.

Endpoint status is useful revision- and session-bound network posture, but it
does not provide complete filesystem, process, protected-write, terminal, or
operation-interval coverage. No weak observations are combined to manufacture
authority.

## 10. Cleanup Assessment

The matrix correctly recognizes improved deletion semantics without claiming
terminal cleanup proof.

The public response identifies the immutable targeted sandbox and
distinguishes completed, accepted, and already-absent outcomes. The schema
itself warns that completed gateway deletion may precede downstream platform
garbage collection. Later absence corroborates cleanup but does not provide
one immutable receipt for exact-generation reclamation, descendant
termination, credential purge, completion, and degradation.

Cleanup remains a provider-readiness blocker.

## 11. Fail-Closed And Privacy Assessment

The proposed Workflow OS mappings are correct:

- missing mandatory pre-start facts deny before start;
- uncertain admitted execution, owner loss, observation gaps, and incomplete
  cleanup require reconciliation;
- accepted deletion remains cleanup-pending; and
- no successful report or artifact may claim completion without exact
  terminal and cleanup facts.

The matrix also preserves the payload-free Core boundary. Raw command I/O,
OCSF payloads, policy bodies, source contents, environment values,
credentials, paths, endpoints, and provider errors remain outside Core.
Stable IDs, commitments, bounded counts, typed postures, sanitized reasons,
and evidence references are the appropriate future projection.

## 12. Matrix Blockers

No blocker was found in the matrix itself.

Provider readiness remains blocked by:

1. missing Docker projection of resolved workload identity and fence evidence;
2. missing durable exec terminal-outcome lookup after stream loss;
3. missing owner-loss resolution for pending driver work;
4. missing operation-bound complete observation manifest;
5. incomplete mandatory security-capability negotiation;
6. incomplete applied-control and degradation posture; and
7. missing exact terminal cleanup receipt.

These are contract and selected-driver blockers. A live smoke or log scraper
cannot close them.

## 13. Non-Blocking Follow-Ups

- Preserve the matrix as the exact `v0.1.5` selected-boundary record.
- Refresh the existing upstream gap record and discussion draft with the
  stable delta and Docker production-projection findings.
- Keep any actual upstream post or comment separately governed and compare it
  byte-for-byte with an accepted draft.
- Reassess only against a later stable release or an accepted upstream change
  that closes the mandatory public and selected-driver facts.
- Continue to require a separately planned live selected-platform proof after
  source contracts become sufficient.

## 14. Fork Decision

Do not fork OpenShell.

The remaining needs are general execution-provider attestation,
reconciliation, observation, and cleanup requirements. Upstream refusal or an
unavailable extension path has not been established. A fork would transfer a
broad runtime, platform, credential, network, release, and vulnerability
surface to Workflow OS without resolving the product boundary economically.

## 15. Recommended Next Phase

Update and review the **OpenShell upstream attestation gap record and
discussion draft** for the exact `v0.1.5` delta and selected Docker findings.

That phase is documentation and upstream-communication preparation only. It
must not implement a compatibility transport, install or execute OpenShell,
run Cargo, wire a provider, add access material, broaden provider mutations,
or fork the runtime. Any actual upstream submission remains a separate
governed action.

## 16. Validation

- official stable tag and current-main commits: resolved;
- focused public protobuf comparison: no relevant difference found;
- Docker production status and capability implementation: inspected;
- official exec replay, owner-loss, OCSF, and deletion contracts: inspected;
- `npm run check:docs`: required at phase close;
- `git diff --check`: required at phase close; and
- Rust formatting, linting, and tests: not required because this review changes
  documentation only and no Rust or runtime behavior.

## 17. Governed Review Record

- Workflow ID: `dg/review`.
- Run ID: `run-1792850901864477000-2`.
- Approval ID:
  `approval/run-1792850901864477000-2/review-scope-approved`.
- Approval presentation ID: `presentation/9b61d96a40d99dab`.
- Approval presentation content hash:
  `9b61d96a40d99dabf0970f82163e0ade1c24547480e058a605aef70fbc0b977d`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact focused review scope.
- Out-of-kernel work: the delegated maintainer inspected pinned official
  source and documentation, reviewed the matrix and roadmap, authored this
  review, and ran validation. The kernel governed scope and approval but did
  not inspect upstream source, edit files, or run validation.

Phase-close result:

- phase status: `Completed`;
- events: 39 total;
- approvals: 1;
- retries: 0;
- escalations: 0;
- event kinds: 1 `RunCreated`, 1 `RunValidated`, 1 `RunStarted`, 6
  `StepScheduled`, 8 `PolicyDecisionRecorded`, 1 `ApprovalRequested`, 1
  `ApprovalGranted`, 1 `RunResumed`, 6 `SkillInvocationRequested`, 6
  `SkillInvocationStarted`, 6 `SkillInvocationSucceeded`, and 1
  `RunCompleted`; and
- approval-presentation enforcement: proof-enforced with one matching
  presentation record and event marker.

Validation result:

- `npm run check:docs`: passed under the repository-pinned Node 20 runtime;
- `git diff --check`: passed; and
- Rust formatting, linting, and tests: not run because this review changed
  documentation only and no Rust or runtime behavior.
