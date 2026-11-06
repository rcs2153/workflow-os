# Governed Obligation Plan Architecture Reconciliation Review

Status: **accepted after blocker correction**

## Scope

This owner-conducted focused maintainer and security review evaluates the
[Phase 1 architecture
reconciliation](GOVERNED_OBLIGATION_PLAN_ARCHITECTURE_RECONCILIATION.md)
against current Workflow OS Core. It reviews canonical source mappings, the
claimed inspectability gap, non-authorizing model boundaries, immutable source
closure, SideEffect non-inference, proportional-governance reuse, and future
differential validation.

This is repository assurance, not independent external security review. No
runtime, schema, SDK, CLI, dependency, provider, persistence, release, or
public behavior changed.

## Findings

### Blocker: per-entry commitments did not prove inventory completeness

The initial matrix required canonical source references and commitments for
each entry, but its minimal model did not require an aggregate commitment over
the complete canonical source closure and complete sorted inventory. A buggy
derivation could therefore omit one mandatory obligation while every retained
entry still validated correctly.

Correction: any future model must carry both an aggregate canonical source-
closure commitment and an aggregate inventory commitment over the algorithm,
closure, and complete sorted entry set. Per-entry integrity is necessary but
not sufficient.

### Blocker: open identities could become an authored authority channel

The initial minimal model named obligation identity, kind, and expected
evidence class without making their construction boundary explicit. If callers
could author those as strings, the supposedly derived index would become a new
language for inventing obligations or evidence expectations.

Correction: obligation identity is deterministically derived from canonical
source identity, source commitment, and a closed Core-owned kind. Evidence
class is a closed reference to an existing authoritative Core class only.
Callers cannot author identities, kinds, evidence classes, or aggregate roots.

### Important: unsupported posture must not manufacture a requirement

An authored requirement that Core cannot yet enforce is different from no
authored requirement at all. Labeling an inferred SideEffect as `unsupported`
would still manufacture an obligation from runtime observation.

Correction: the matrix now requires this distinction explicitly. Absent a
canonical authored source, no obligation entry may be created. SideEffect
records remain runtime facts and evidence only.

### Source-reference defect: `ProjectBundle` was attributed to the wrong module

`ProjectBundle`, `LoadedSpec`, and `load_project` are defined in `loader.rs`,
while the initial matrix attributed `ProjectBundle` to `project.rs`.

Correction: the matrix now points to the loader-owned canonical bundle and
separately identifies `ProjectManifest` and policy documents in `project.rs`.

### Accepted: the inspectability gap is real and bounded

Current Core has canonical authored contracts, typed policy effects, per-step
proportional-governance derivation, canonical local-check declarations,
required-context consumption, approval proof, SideEffect lifecycle records,
immutable run bundles, durable events, WorkReports, and terminal projection.
It does not expose one deterministic pre-execution inventory spanning all
supported static requirements and later reconcilable with terminal evidence.

That gap is an explanation and traceability gap, not an authority gap. A small
private model can be evaluated without replacing any existing runtime path.

### Accepted: proportional-governance derivation remains canonical prior art

`derive_workflow_step_governance_assessment_input` already validates the
project and resolves workflow, step, skill, and policy closure while keeping
runtime authority, checks, and SideEffect posture explicit. The future model
must compose with this semantics or shared primitives. Independently deriving
action class, sensitivity, policy minima, or definition roots is prohibited.

### Accepted: differential validation has an independent oracle boundary

The reconciliation requires independently specified expected obligations and
canonical runtime outcomes, including negative fixtures for omitted gates,
unsupported checks, inferred SideEffects, stale closures, and unstable order.
Reusing the implementation helper as its expected oracle does not count as
evidence.

## Accepted Model Boundary

A separately governed Phase 2 may implement one private model-only requirement
index with:

- deterministic source-derived identity;
- a closed Core-owned obligation kind;
- mandatory, optional, unsupported, or unknown static posture;
- canonical source references and commitments;
- deterministic ordering;
- closed references to existing evidence classes where available;
- derivation algorithm identity;
- aggregate source-closure commitment; and
- aggregate inventory commitment.

The model remains non-authorizing. It cannot carry policy, approval, evidence,
check, SideEffect, authority, or completion outcomes. It cannot create an
obligation without a canonical authored source.

## Explicitly Rejected

- caller-authored obligation identity, kind, or evidence class;
- a `WorkContract` language or public program ABI;
- a second evaluator, policy engine, evidence store, or completion model;
- inferred SideEffect obligations;
- mutable-checkout rederivation of a run-bound inventory;
- public schema, SDK, or CLI exposure;
- persistence or runtime composition in Phase 2;
- Salsa, another incremental query dependency, or an MSRV change;
- provider behavior, hosted compilation, or release publication.

## Validation

- source inspection of loader, project, definition, policy, required-context,
  local-check, proportional-governance, immutable-bundle, approval,
  SideEffect, runtime, executor, and WorkReport paths;
- complete matrix and fact-class review;
- `npm run check:docs`;
- `git diff --check`.

Runtime tests are intentionally not required because this review changes only
architecture documentation.

## Governed Review Record

- Workflow: `dg/review`
- Run: `run-1793951828215491000-2`
- Approval: `approval/run-1793951828215491000-2/review-scope-approved`
- Approval presentation: `presentation/305a3cb926ae5978`
- Presentation content hash:
  `305a3cb926ae5978ecd29ea70bdc8f641d4ba09a434aad424da1fed84d032596`
- Approval outcome: granted under delegated-maintainer authority after the
  complete proof-backed handoff was presented
- Governed scope: focused Phase 1 matrix review and documentation blocker
  correction only
- Phase result: completed
- Event summary: 39 ordered events, one approval request, one approval grant,
  eight policy decisions, six scheduled steps, six requested, started, and
  succeeded skill invocations, zero retries, and zero escalations
- Approval presentation enforcement: `proof_enforced`; one persisted record
  matched the approval and the event trail contains its proof marker

Workflow OS governed review scope and approval. Source inspection,
documentation edits, validation, Git, PR, and merge work occur outside the
kernel and must remain disclosed. No runtime enforcement, model implementation,
provider action, independent review, or release behavior was simulated.

## Verdict And Next Phase

Accepted after correction. The evidence now justifies a separately governed
Phase 2 model-only implementation of the smallest private requirement index
described above. Phase 2 must not add persistence, runtime composition, public
surfaces, a second evaluator, or incremental dependencies. A focused review of
that implementation remains required before Phase 3 explanation or
differential-validation work.
