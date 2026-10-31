# Post-0.3 Roadmap Queue Selection Report

## 1. Decision

Select **capability-gated GitHub read-only public adoption and recovery
planning** as the next Workflow OS roadmap lane.

The first phase is planning-only. It should define how the already accepted,
crate-private executor operation for one GitHub repository-metadata read can
become an explicit, opt-in, user-selectable local path without weakening its
immutable-run, policy, approval, current-authority, continuation, continuity,
telemetry, privacy, or no-write boundaries.

This decision opens a roadmap lane. It does not authorize implementation,
schema or CLI changes, live-provider use, credentials, a second operation,
provider mutation, release publication, or package publication.

## 2. Why This Lane Is Next

Workflow OS already has an accepted end-to-end private composition for one
read-only operation. Core:

- rehydrates the exact durable run and immutable bundle;
- selects one exact current step and repository target;
- evaluates the immutable policy set;
- resolves current source-backed capability authority;
- reconciles durable source continuity;
- consumes a cursor-bound one-time continuation claim;
- invokes an injected GitHub read-only adapter at most once;
- records bounded adapter telemetry and terminal workflow events; and
- returns a payload-free capability-use receipt.

The path remains crate-private. A user cannot author or select it through an
accepted public runtime contract. Configuration and credential posture are not
defined for adoption. A consumed continuation claim without a start event and
a started invocation without a terminal event are truthful but operationally
ambiguous states without an accepted recovery contract.

The project therefore has more value to unlock by composing and hardening
accepted runtime behavior than by adding another semantic primitive. This is
the same correction the roadmap has repeatedly adopted: reduce the gap between
documented governance and enforceable, usable runtime paths.

## 3. Required Planning Scope

The next plan must define one coherent boundary for:

1. **Exact authored operation selection.** A validated immutable workflow step
   must select only the existing repository-metadata operation. Natural
   language, adapter names alone, or caller-selected function pointers must
   not choose runtime behavior.
2. **Explicit local activation.** The path must remain disabled by default and
   require a deliberate project or invocation-level opt-in. Activation is not
   authority and must not bypass policy, approval, or current facts.
3. **Configuration and credential injection.** Repository identity, adapter
   mode, and credential references must be bounded and explicit. Core must not
   discover ambient credentials, store secret material, or copy credentials
   into events, receipts, reports, diagnostics, or debug output.
4. **Current authority and immutable binding.** Adoption must preserve the
   exact actor, workflow, run, step, harness, repository, policy, capability,
   approval, sensitivity, bundle, cursor, source-registration, and continuity
   bindings already accepted by the private composition.
5. **Ambiguous-state recovery.** The plan must distinguish at least:
   - a consumed continuation claim with no durable invocation-start event; and
   - a durable invocation-start event without a terminal provider or workflow
     outcome.
   Recovery must not fabricate success, repeat an uncertain provider call,
   reset continuity by hand, or let a caller choose a more favorable state.
6. **Evidence and disclosure.** The public path must expose bounded operation,
   authority, continuity, provider-outcome, ambiguity, limitation, and recovery
   posture suitable for audit and a future WorkReport without copying raw
   provider payloads or inventing an inaccurate `EvidenceReference` kind.
7. **Compatibility.** Existing workflows, private tests, read-only adapters,
   ordinary executor paths, first-run behavior, and `0.3.0-preview.1` contracts
   must remain unchanged unless a later implementation phase explicitly
   documents and tests an additive public surface.
8. **Acceptance sequencing.** The plan must split implementation only where
   independent security or compatibility review is needed, while keeping the
   lane oriented toward one usable vertical slice rather than an indefinite
   chain of disconnected models.

## 4. Strict Non-Goals

The selected lane must not imply:

- a second GitHub operation or another provider family;
- GitHub comments, pull requests, Jira writes, CI writes, or other mutation;
- automatic or default provider execution;
- ambient credential discovery or hidden authentication loading;
- generic connector marketplaces or automatic handler registration;
- arbitrary source, prompt, transcript, command-output, or provider-payload
  persistence;
- retries after a may-have-started provider boundary without authoritative
  reconciliation;
- OpenShell installation or execution-provider wiring;
- nested harness runtime, recursive agents, or agent swarms;
- hosted identity, multi-tenancy, enterprise administration, or production
  readiness;
- release tagging, GitHub Release publication, or package publication.

## 5. Alternatives Considered

### OpenShell provider wiring

Not selected. OpenShell remains the preferred optional containment-provider
candidate, but the accepted `v0.1.5` evidence matrix still lacks complete
operation replay, owner-loss resolution, operation-bound observation
completeness, selected-driver resolved identity and fence evidence, complete
capability negotiation, and a terminal cleanup receipt. Provider wiring would
weaken the accepted fail-closed boundary.

### Broader provider mutation

Not selected. The narrow GitHub comment and managed draft-pull-request sandbox
proofs are intentionally bounded. A new mutation family would broaden risk
before the existing no-write executor composition is safely adoptable.

### Nested or composable harness runtime

Not selected. Harness contracts remain valuable vocabulary, but nested runtime
behavior would add scheduling, authority, context, failure, and reporting
complexity without first proving a public single-operation adoption boundary.

### Another model-only primitive

Not selected. Capability grants, resolution, step-scoped capability
projection, governed context projection, required-context consumption,
immutable bindings, current-authority sources, continuation, telemetry, and
receipts already exist. The next value is composition and recovery.

### Immediate `0.3.0-preview.1` publication

Not selected as roadmap implementation. The release preparation is merged, but
tagging, GitHub Release publication, and package publication are separate
release-authority decisions and do not replace selection of the next runtime
lane.

## 6. Evidence Consulted

- `ROADMAP.md` active queue and recently closed evidence;
- `docs/ENGINEERING_STANDARD.md` product, safety, determinism, security, and
  compatibility requirements;
- `docs/implementation-plans/executor-capability-github-read-only-composition-plan.md`;
- `docs/concepts/EXECUTOR_CAPABILITY_GITHUB_READ_ONLY_COMPOSITION_REVIEW.md`;
- `docs/implementation-plans/scoped-capability-github-read-only-integration-plan.md`;
- `docs/concepts/SCOPED_CAPABILITY_GITHUB_READ_ONLY_INTEGRATION_BLOCKER_FIX_REVIEW.md`;
- `docs/implementation-plans/trusted-host-local-cargo-execution-provider-suitability-reevaluation.md`;
- `docs/implementation-plans/openshell-v0-1-5-execution-evidence-sufficiency-matrix.md`;
- `docs/implementation-plans/scoped-runtime-authority-capability-projection-plan.md`;
- `docs/implementation-plans/governed-context-access-projection-plan.md`; and
- `docs/implementation-plans/authoritative-agent-continuation-context-rehydration-plan.md`.

## 7. Required Next Phase

Write and review a **Capability-Gated GitHub Read-Only Public Adoption And
Recovery Plan**. The plan must cover every requirement in Section 3 and retain
every non-goal in Section 4. Implementation may begin only after focused
maintainer/security review accepts that plan.

## 8. Governed Phase Record

- workflow ID: `dg/d`
- phase: `planning`
- run ID: `run-1793444070028021000-2`
- approval ID:
  `approval/run-1793444070028021000-2/planning-approved`
- approval presentation ID: `presentation/a582757e285fe23f`
- approval presentation content hash:
  `a582757e285fe23fcd9d032d590819cfe8c572cd1a21c644c5fdf2f8f6dae7cb`
- approval outcome: granted under standing delegated-maintainer authority after
  the complete persisted handoff was presented
- kernel boundary: governance coordination only; source inspection, queue
  analysis, documentation authorship, validation, Git, pull-request, and merge
  work remain outside the kernel
