# Capability-Gated GitHub Read-Only Public Adoption And Recovery Plan

Status: Focused maintainer/security plan review accepted after required
boundary corrections. Implementation remains phased and opt-in.

Related foundations:

- [Post-0.3 Roadmap Queue Selection](../concepts/POST_0_3_ROADMAP_QUEUE_SELECTION_REPORT.md)
- [Executor Capability-Gated GitHub Read-Only Composition Plan](executor-capability-github-read-only-composition-plan.md)
- [Executor Capability-Gated GitHub Read-Only Composition Review](../concepts/EXECUTOR_CAPABILITY_GITHUB_READ_ONLY_COMPOSITION_REVIEW.md)
- [Scoped Capability GitHub Read-Only Integration Plan](scoped-capability-github-read-only-integration-plan.md)
- [Production Current-Authority Source Boundary Plan](production-current-authority-source-boundary-plan.md)
- [Authorized Execution Continuity Plan](authorized-execution-continuity-plan.md)
- [Authorized Execution Continuity Atomic State Plan](authorized-execution-continuity-atomic-state-plan.md)
- [Profile-Controlled Authoritative Governance Activation Plan](profile-controlled-authoritative-governance-activation-plan.md)
- [Immutable Run Bundle Boundary Plan](immutable-run-bundle-boundary-plan.md)

## 1. Executive Decision

Adopt the existing GitHub repository-metadata read through the ordinary
governed `run` path as one explicit, opt-in, no-write public preview operation.

The public path must not expose the current crate-private helper directly. It
must compose four reviewed boundaries:

1. an exact authored operation declaration in the immutable skill contract;
2. an explicit project-level activation, resource allowlist, and access
   reference that constrain but do not themselves authorize execution;
3. a Core-owned local current-authority source that derives one exact
   run-scoped grant and current availability from those validated inputs; and
4. the accepted atomic authorized-execution continuity and dispatch machinery,
   replacing the private helper's simple consumed idempotency claim.

The provider-read declaration is subordinate to the existing supported
`governance.authoritative_execution` activation. Declaring a provider read
without that authoritative execution posture is invalid and cannot enter the
adapter.

The result remains one GitHub.com repository-metadata `GET`. It does not become
a generic connector framework, a second provider operation, a mutation path,
or a default runtime behavior.

## 2. Current Accepted Baseline

The crate-private executor composition already:

- rehydrates an exact running `WorkflowRun`;
- validates the exact immutable run bundle and current step;
- recognizes one structural skill contract for
  `github.repository.metadata`;
- resolves immutable policy effects and granted step approval;
- evaluates and durably records policy before provider entry;
- reads complete current facts from a registered in-memory source;
- resolves exact actor, workflow, run, step, harness, resource, sensitivity,
  capability, availability, and prerequisite posture;
- reconciles source continuity in SQLite;
- consumes one cursor-bound governed-continuation claim;
- appends invocation request and start before provider entry;
- invokes `GitHubReadOnlyAdapter::read_repository_metadata` once;
- emits bounded adapter telemetry;
- appends truthful success, failure, and run-terminal events; and
- returns a payload-free `CapabilityUseAuthorityReceipt` after successful use.

Focused review accepted that private proof. It also identified the boundaries
that block public adoption:

- operation selection is inferred from an exact structural skill shape rather
  than an explicit authored operation code;
- the authority source is injected, crate-private, and in-memory;
- the continuation claim can be consumed before an invocation-start event is
  durably appended;
- an invocation may start without a terminal result;
- neither ambiguous state has a public recovery contract;
- no ordinary CLI path activates the operation;
- no public disclosure contract distinguishes blocked, admitted, ambiguous,
  and terminal postures; and
- `GitHubLiveReadOnlyClient` still sends the stale user agent
  `workflow-os/0.1.0-preview.1`.

## 3. Product Contract

The user experience should remain Workflow OS-shaped:

```text
validated project declaration
  -> immutable workflow and skill selection
  -> Core-owned current authority and policy
  -> proportional-governance route
  -> atomic dispatch admission
  -> injected read-only adapter
  -> durable events, bounded evidence, and report posture
```

Users run the existing workflow command. They do not call a new
`github metadata` utility command. Workflow OS remains the governance kernel;
the adapter remains an execution dependency selected by an exact governed
contract.

Public adoption means the path is documented, explicitly activatable,
deterministically validated, restart-safe within its stated limits, and
inspectable. It does not mean enabled by default, production-ready, or broadly
compatible with all GitHub deployments.

## 4. Exact Authored Operation Selection

Add one optional exact operation field to `AdapterRequirement`:

```yaml
adapter_requirements:
  - adapter_id: symbolic/github-read-only
    operation: github.repository.metadata
    capabilities:
      - external.read
```

The field should be represented in Rust, JSON Schema, and the TypeScript SDK
as a bounded validated operation identifier. It is optional for compatibility,
but the new public live path requires it. Existing skills without the field
retain their current declaration-only or fixture-handler behavior and must not
silently become live provider operations.

For the first public operation, Core must still require the complete accepted
contract:

- exactly one non-`continue` step;
- exactly one skill selected by that step;
- exactly one `symbolic/github-read-only` adapter requirement;
- exact operation `github.repository.metadata`;
- exact `external.read` adapter capability;
- exact skill capabilities `external.read` and `adapter.invoke`;
- exact nonsensitive required string inputs `owner` and `repo`;
- immutable literal step mappings for both values;
- a policy set that permits external read and adapter invocation;
- no write, secret, unknown, or extra adapter capability; and
- a terminal behavior compatible with the one-step result.

An adapter ID without the operation, a free-form action string in ordinary
inputs, tags, descriptions, or model output must not select the operation.

## 5. Explicit Project Activation And Resource Constraints

Extend the existing optional project governance declaration with one closed
preview field. The exact final name may follow parser conventions, but the
accepted semantic shape is:

```yaml
governance:
  authoritative_execution:
    profile: observe_and_report
    local_check_profile: workflow_os_project_validation
  provider_reads:
    github_repository_metadata:
      enabled: true
      endpoint: github.com
      access_reference: env/WORKFLOW_OS_GITHUB_TOKEN
      resources:
        - owner: rcs2153
          repository: workflow-os
```

Required semantics:

- absence preserves all current behavior;
- the existing supported `authoritative_execution` declaration is required;
- `provider_reads` without supported authoritative execution fails validation;
- `enabled: true` is an explicit activation constraint, not authority;
- the first slice accepts only `github.com` and the fixed API base URL;
- `access_reference` accepts only the exact named environment reference
  `env/WORKFLOW_OS_GITHUB_TOKEN`;
- the public governed path must not fall back to `GITHUB_TOKEN`;
- the declaration stores no access material;
- resources are exact canonical owner/repository pairs with hard count and
  length limits;
- the immutable step target must match exactly one allowlisted resource;
- duplicate, overlapping, secret-like, malformed, or unknown entries fail
  validation;
- unknown fields fail schema parsing; and
- activation, allowlist, and access-reference commitments participate in the
  immutable run bundle and approval/resume resolved-context commitment.

The declaration is a reviewed maximum scope. It does not prove current access
availability, issue a reusable grant, bypass policy, satisfy approval, or
permit another operation.

## 6. Core-Owned Local Current-Authority Source

The current injected `RegisteredInMemoryCurrentAuthoritySource` is not a public
runtime source. The implementation should add one private Core-owned local
source constructor for this exact operation.

The constructor must derive its complete inventory from:

- the validated immutable project activation and resource declaration;
- the exact immutable workflow, step, skill, operation, and target;
- the current run actor and harness scope;
- current policy and approval prerequisites;
- the fixed read-only capability set;
- the current presence posture of the exact access reference; and
- the selected adapter mode and endpoint.

It must not accept caller-assembled grant or availability slices. Core derives
one run- and step-scoped `CapabilityGrant`, one exact resource availability
record, and any prerequisite records required by the immutable contract. The
grant expires with the run and cannot be serialized as a bearer credential or
reused by another actor, workflow, run, step, harness, resource, or operation.

The source registration commitment must cover every semantic input above while
excluding access material. Its generation and watermark must derive from
validated immutable configuration plus current availability posture, not from
caller-selected counters. SQLite source-continuity reconciliation remains the
durable monotonic boundary.

Missing access material means `declared_not_connected` or another existing
non-authorized availability posture. It must not become an authentication
attempt, fabricated grant, or generic source failure.

The authority source must never receive or return the access material. A
separate private transient-access resolver may inspect the exact named
environment reference only after authority, governance, and dispatch admission
have succeeded. It may return a non-serializable same-call secret capability
directly to the injected adapter consumer. That capability must not be stored
in the authority inventory, immutable bundle, events, continuity state,
receipts, reports, errors, or debug output. Presence posture may inform current
availability, but the secret value itself is not an authority fact.

## 7. Proportional Governance And Approval

Activation does not select the final governance route. Core must compose the
strictest current result from:

- the project governance profile;
- immutable workflow and policy declarations;
- current run-scoped authority and availability;
- approval prerequisites;
- evidence and check obligations;
- sensitivity and external-read posture;
- runtime escalation and prior accepted decision; and
- any ambiguity or recovery posture.

The ordinary local low-risk case may proceed quietly only when the accepted
selector returns `Proceed` with quiet disclosure allowed. A visible disclosure
obligation does not pause work. An approval route requires the existing
proof-enforced approval presentation and fresh decision-time reassessment.
Denial remains terminal for the attempted operation.

Neither the project declaration nor access-reference presence may downgrade an
explicit approval, denial, visible-disclosure, evidence, or check requirement.

## 8. Atomic Dispatch Migration

The public operation must not use the current sequence:

```text
record generic idempotency result
  -> recheck cursor
  -> append invocation request
  -> append invocation start
  -> call provider
```

That sequence leaves a durable consumed claim with no authoritative attempt
lifecycle if a process stops between writes.

The implementation should reuse the complete accepted authorized-execution
continuity contract, not dispatch reservation in isolation. The current
reservation API consumes an already-open authorized execution window and exact
attempt capability; it cannot manufacture those prerequisites for an ordinary
run. The provider-read integration must therefore define how Core opens the
exact operational window, derives or consumes the exact resume directive when
one is required, creates the attempt, and only then reserves dispatch.

Through those accepted atomic transitions, Core must:

1. rehydrate the exact run, immutable bundle, policy, approval, authority,
   source-continuity, and proportional-governance context;
2. bind an exact provider-read operation commitment;
3. open or rehydrate one exact authorized execution window;
4. admit one exact attempt at the current event cursor;
5. reserve dispatch and append the operation's invocation-start posture;
6. persist the attempt as started; and
7. return a private, non-serializable attempt-use capability to the same-call
   consumer.

Only that capability may enter the injected adapter and record a terminal
outcome. Concurrent or replayed callers receive the committed admission or a
bounded non-dispatchable posture; they never receive another provider-entry
capability.

If the existing trusted-host lifecycle vocabulary cannot truthfully describe a
provider read, add the smallest provider-neutral lifecycle adapter over the
same atomic state contract. Do not create a parallel idempotency or recovery
store.

## 9. Recovery Semantics

Recovery is classification plus explicit state transition. It is not blind
retry and cannot fabricate provider outcome.

### 9.1 No admitted attempt

If admission did not commit, the event cursor and continuity state remain
unchanged. A fresh ordinary invocation may reassess current facts and attempt
admission.

### 9.2 Admitted or consumed state without provider entry

The migrated public path must make this state either impossible or
authoritatively distinguishable. If a committed attempt is `started`, Core
must conservatively assume provider entry may have begun unless a durable
pre-entry state proves otherwise.

If a separate pre-entry posture is retained, recovery may classify it as
`not_started` only through an atomic exact-binding transition. It may then
create a new attempt after fresh authority and governance reassessment. A
missing invocation-start event by itself is not sufficient proof if another
durable record says the attempt started.

### 9.3 Invocation started without terminal outcome

This is `ambiguous_may_have_started`. The first public slice must:

- block automatic retry;
- preserve the started attempt and exact operation binding;
- record one explicit recovery-required posture;
- disclose that no provider result or receipt is available;
- avoid claiming adapter failure, success, evidence, or completion; and
- require an explicit recovery action before another attempt or terminal run
  decision.

Because the operation is read-only and GitHub does not expose a durable lookup
identity for a prior `GET`, the result cannot be reconciled from the provider.
An operator may explicitly abandon the ambiguous attempt and request a fresh
read. That action must close the old attempt as ambiguous, create a new
attempt identity only after fresh authority/governance reassessment, and carry
the prior ambiguity into final disclosure. It is a new governed read, not a
replay of the missing result.

### 9.4 Provider result known, terminal persistence uncertain

If the same-call consumer has a result but terminal persistence is ambiguous,
it must reconcile the exact durable attempt through a fresh backend connection.
If the terminal outcome committed, return its replay posture without another
provider call. If only started state exists, classify it as ambiguous and use
Section 9.3. Raw result payloads must not be written opportunistically during
reconciliation.

### 9.5 Exact terminal replay

Completed success or failure is replayable as bounded durable posture. It must
not invoke the provider again. A successful replay may expose only accepted
references and receipt identity, not reconstruct a lost raw provider payload.

## 10. Public CLI Surface

Public adoption should extend ordinary commands rather than create a provider
utility command.

### `run`

For `workflow-os run <workflow-id>`:

1. load and validate the project;
2. resolve project authoritative-governance and provider-read activation;
3. create or rehydrate the exact immutable run;
4. identify the exact declared adapter operation for the current step;
5. derive the Core-owned local authority source;
6. route through proportional governance and proof-enforced approval when
   required;
7. admit and execute through atomic continuity; and
8. print bounded status, next action, and disclosure posture.

Projects without the declaration retain current behavior. Declared but
incomplete or unsupported provider-read configuration fails before provider
entry and must never fall back to mock or ordinary local handlers.

### `approve`

Approval must rehydrate the exact immutable declaration and current provider
availability, then perform fresh resolved-context and proportional-governance
reassessment. Granting approval does not itself call GitHub; it makes the run
eligible for the separately admitted next attempt.

### `status` and `inspect`

Existing event inspection should expose bounded operation code, attempt
posture, authority-source reference, dispatch posture, recovery requirement,
and terminal disclosure. It must not expose access material, headers, raw
provider payloads, or unbounded response text.

### Recovery command

If a new command is required, it should be operation-neutral and exact-run
scoped, for example:

```text
workflow-os recover <run-id> --attempt <attempt-id> --abandon-ambiguous
```

The final name is subject to CLI review. It must only classify the exact
ambiguous attempt and request fresh reassessment. It must not directly retry a
provider, mutate state by hand, or accept caller-authored success/failure.

## 11. Evidence, Audit, And WorkReport Posture

The path must persist and disclose references for:

- immutable operation and resource binding;
- policy and proportional-governance decision;
- approval and presentation proof when required;
- current-authority source and continuity posture;
- dispatch admission and attempt identity;
- adapter invocation telemetry;
- provider outcome classification;
- capability-use receipt identity on success;
- ambiguity and recovery transitions; and
- final limitations, including fixture/live mode and unsupported recovery.

Do not reuse an inaccurate existing `EvidenceReference` kind for the authority
receipt. The first implementation may cite the receipt through bounded
report metadata or a dedicated typed reference added in the same reviewed
change. It must not copy the serialized receipt claim as trusted authority.

A successful provider read is not enough to complete a WorkReport if required
audit, check, approval, or recovery disclosures are missing. A failed or
ambiguous read must not fabricate result evidence.

## 12. Privacy And Security

- The only accepted access reference is the named Workflow OS environment
  reference; no fallback or discovery is permitted.
- Environment values, authorization headers, raw GitHub responses, source
  inventories, grants, policy payloads, command output, and absolute paths must
  never enter events, receipts, reports, errors, or Debug output.
- Owner and repository are bounded resource identifiers. Errors should avoid
  echoing them where the failure class is sufficient.
- The live client must use fixed GitHub.com HTTPS, bounded timeouts, bounded
  response bodies, no redirects, no automatic retries, and the current release
  user-agent identity.
- The stale hard-coded `workflow-os/0.1.0-preview.1` user agent is a blocker for
  public live use and must be replaced by one package-version-derived constant
  with a regression test.
- A malformed response is provider failure, not evidence.
- Rate-limit and authorization failures remain bounded classifications and do
  not trigger automatic retry.
- Fixture mode remains the default for normal tests and CI. Live tests remain
  opt-in, ignored by default, and limited to approved non-sensitive targets.

## 13. Compatibility

The implementation must preserve:

- projects without `governance.provider_reads`;
- skills without an explicit adapter operation;
- fixture-backed read-only examples and `--mock-all-local-skills` behavior;
- ordinary local skill execution;
- existing immutable runs created before the new fields;
- current CLI output unless the new path is explicitly active;
- current public adapter APIs and fixture clients;
- `0.3.0-preview.1` JSON preview warnings; and
- exact replay and recovery of legacy runs through their stored bundle
  contract.

New fields are additive and optional in v0 schemas. A new run that activates
the provider path must fail closed if its stored bundle lacks the complete new
contract; it must not reinterpret an old structural skill as live-enabled.

## 14. Implementation Shape

After focused plan acceptance, implement the lane as one end-to-end roadmap
lane with four governed, independently reviewed phases. No partial phase may
make the provider operation publicly selectable before the fixture-backed
vertical slice is complete:

1. add exact operation and subordinate project activation contracts across
   Rust, schema, SDK, validation, docs, and immutable-bundle hashing;
2. add Core-owned authority derivation, the separate transient-access
   resolver boundary, and the complete provider-read operational-window,
   attempt, dispatch, terminal, and ambiguity lifecycle;
3. activate the fixture-backed ordinary CLI vertical slice with bounded
   status, inspect, recovery, audit, telemetry, receipt, and report disclosure;
4. correct the live read-only user-agent identity, then add explicit live-mode
   activation and the separately authorized smoke proof.

Each phase must retain the exact operation and no-write boundary. Phase 1 is
contract-only and dark. Phase 2 remains Core-private. Phase 3 is the first
publicly selectable path and must not fall back to live transport. Phase 4 is
the only phase that may authorize the exact live GitHub.com read.

If implementation discovers that the accepted atomic continuity contract
cannot represent provider-read lifecycle truthfully, stop and return to plan
review. Do not silently retain the generic consumed claim or invent a parallel
state machine.

## 15. Required Test Matrix

### Contract and compatibility

- operation and project activation schema/SDK parity;
- absence preserves legacy behavior;
- old stored bundles remain readable and non-live;
- explicit operation required for new live adoption;
- unsupported operation, endpoint, access reference, or extra capability
  fails before run creation or provider entry;
- allowlist and immutable target mismatch fail closed; and
- current release user-agent identity is exact.

### Authority and governance

- Core-owned source derives exact run/step/resource scope;
- caller cannot inject grants, availability, generation, or watermark;
- missing access material is non-authorized availability;
- stale, revoked, expired, unavailable, incomplete, or prerequisite-bearing
  authority blocks provider entry;
- policy denial, approval requirement, presentation proof, and reassessment
  remain authoritative;
- quiet, visible, approval, and denial routes retain their exact semantics; and
- declaration or availability drift invalidates prior assessment.

### Dispatch and recovery

- exact concurrent callers produce one admitted attempt and one provider call;
- crash before admission permits a fresh reassessed attempt;
- committed pre-entry posture is recovered only through exact atomic state;
- started/no-terminal becomes recovery-required and never auto-retries;
- terminal-commit acknowledgement ambiguity reconciles through a fresh
  connection;
- exact terminal replay produces no provider call;
- conflicting operation or target replay fails closed;
- explicit abandonment of an ambiguous read preserves disclosure and uses a
  new attempt after fresh reassessment; and
- backend interruption at every durable boundary leaves an inspectable,
  truthful state.

### Privacy and reporting

- forbidden markers in access material and provider payloads never appear in
  Debug, errors, SQLite bytes, events, receipts, reports, CLI JSON, or text;
- success cites accepted receipt/reference identity without copying payloads;
- failure and ambiguity fabricate no evidence;
- reports disclose prior ambiguous attempts when a fresh read later succeeds;
  and
- fixture and live mode are distinguishable in bounded output.

### Validation

- formatting and Clippy with warnings denied;
- focused Core, CLI, schema, SDK, adapter, continuity, recovery, report, and
  privacy suites;
- full Rust workspace tests;
- Rust documentation warnings denied;
- Node, contract, integration, documentation, and dependency checks;
- normal CI remains network-free; and
- one ignored opt-in live smoke only after focused review accepts fixture and
  recovery evidence.

## 16. Acceptance Criteria

The lane is complete only when:

1. one exact authored operation and one exact project activation select the
   path;
2. Core, not arbitrary caller facts, derives current run-scoped authority;
3. the public path uses atomic continuity rather than the simple consumed
   idempotency claim;
4. all pre-entry, started, terminal, and acknowledgement-ambiguous states have
   truthful durable posture;
5. no ambiguous attempt is automatically retried;
6. policy, approval, evidence, checks, sensitivity, and disclosure remain
   authoritative;
7. ordinary projects and legacy runs remain unchanged;
8. no access material or raw provider payload enters durable governance state;
9. the stale user-agent identity is fixed;
10. fixture-first and recovery tests pass without network access;
11. focused maintainer/security review accepts the implementation; and
12. live smoke, if separately authorized, proves only this exact read-only
    operation against an approved target.

## 17. Strict Non-Goals

This plan does not authorize:

- a second GitHub read operation or provider family;
- GitHub comments, pull requests, Jira writes, CI writes, or mutation;
- default provider activation;
- automatic access-material discovery or fallback to generic host variables;
- generic connector registration or arbitrary adapter dispatch;
- blind retry after a may-have-started boundary;
- provider-result persistence beyond bounded references and accepted report
  metadata;
- OpenShell integration;
- nested harness runtime, recursive agents, or agent swarms;
- hosted identity, multi-tenancy, enterprise administration, or production
  readiness;
- package publication or release-posture change; or
- Level 3 or Level 4 autonomy claims.

## 18. Review Outcome And Required Next Phase

Focused maintainer/security review accepted this plan after correcting three
blocking ambiguities:

- provider-read activation now requires the existing authoritative-execution
  declaration rather than creating a parallel ambient execution path;
- transient credential delivery is explicitly separate from current-authority
  facts and durable governance state; and
- provider execution must reuse the complete operational-window, attempt, and
  dispatch lifecycle rather than treating dispatch reservation as a standalone
  admission primitive.

The review also split implementation into four governed phases so the public
path cannot be exposed with only contracts or partial recovery semantics.

The next phase is Phase 1: implement the exact operation and subordinate
project activation contracts, immutable binding, schema/SDK parity,
compatibility behavior, and focused validation. It must remain dark: no public
runtime selection, provider call, credential read, or CLI activation is
authorized in that phase.

See [Capability-Gated GitHub Read-Only Public Adoption And Recovery Plan
Review](../concepts/CAPABILITY_GATED_GITHUB_READ_ONLY_PUBLIC_ADOPTION_RECOVERY_PLAN_REVIEW.md).
