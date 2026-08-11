# Proportional-Governance Selected Local Project Consumer Plan

Status: planning accepted; the Core-owned selected-profile fact-source bridge
and additive selected-consumer composition API are implemented. The bridge is
accepted. Focused review found one blocker: selected-consumer evaluation time
must be Core-owned rather than caller-authored. CLI adoption is not implemented.

## 1. Executive Summary

Workflow OS now has an accepted explicit Core composition for proof-enforced
approval resume, fresh registered runtime-fact reassessment, trusted authority
receipt derivation, receipt-citing WorkReport generation, and local receipt and
report-artifact persistence. No product path owns the runtime-fact source and
consumes that complete composition yet.

The first selected consumer should be the existing project-declared
authoritative local project-validation path. It is the narrowest credible
consumer because it already has explicit project activation, an immutable run
bundle, a canonical `DocsCheck` requirement, same-call check execution,
proof-enforced approval, and a mandatory local WorkReport artifact.

Implementation began with a private Core-owned fact-source bridge and an
equivalence matrix. The bridge derives the selected check fact from the actual
same-call canonical `DocsCheck`, uses a fixed Core registration, returns a
payload-free current-fact snapshot and source-backed governance binding, and
fails closed if its assessment differs from the accepted private reassessment.
Existing CLI behavior and accepted authoritative APIs remain unchanged.

## 2. Problem

The generic registered-source composition remains caller-driven: the caller
provides source registration, source implementation, profile, evaluation time,
report inputs, and stores. That is useful as an explicit Core boundary but is
not a safe product default. A CLI caller must not be able to manufacture
authority-bearing facts or select an untrusted source and then describe the
result as project-declared governance.

In parallel, the existing authoritative local project-validation path already
derives a closed one-step posture from validated project declarations and an
actual same-call `DocsCheck` outcome. It uses a separate accepted composition
route and therefore does not yet produce the complete fresh-fact authority
receipt and report-artifact closure.

The gap is composition, not another primitive family.

## 3. Goals

- Select exactly one existing local product path as the first consumer.
- Keep runtime-fact source identity, registration, and fact derivation owned by
  Core for that path.
- Derive facts from validated immutable declarations, durable run context, and
  the actual same-call local-check result.
- Prove equivalence with the existing authoritative path before adoption.
- Preserve proof-enforced approval presentation and fresh reassessment.
- Reuse the accepted authority-receipt and report-artifact composition without
  weakening its ordering or failure semantics.
- Preserve current CLI output, executor semantics, local-only posture, and
  compatibility until a separate adoption review.

## 4. Non-Goals

The remaining phases do not authorize:

- a product consumer beyond the accepted private bridge;
- a second CLI mode or a new default executor path;
- caller-authored authoritative source registration;
- automatic approval, report generation, or persistence for other paths;
- arbitrary repository inference or source-code inspection;
- multi-step authoritative-governance expansion;
- provider execution, OpenShell, new SideEffects, or external writes;
- schemas, examples, hosted behavior, enterprise administration, or release
  posture changes;
- reusable, ambient, delegated, or persisted authority; or
- removal of the existing accepted authoritative APIs before equivalence and
  migration review.

## 5. Selected Consumer

The selected first consumer is the existing project-declared authoritative
local project-validation path exposed through the current authoritative local
run and approval commands.

It is selected because it already supplies the required closed boundary:

- project activation is explicit and validated;
- the supported profile is constrained to the reviewed one-step shape;
- the selected step declares the canonical
  `workflow_os_project_validation` requirement;
- immutable workflow, skill, policy, and execution declarations are stored;
- the actual `DocsCheck` runs in the same call after complete preflight;
- approval presentation proof is required before grant-side mutation;
- local WorkReport artifact generation and persistence are already mandatory;
  and
- the path remains local, explicit, payload-bounded, and provider-free.

Ordinary executor requests, undeclared projects, onboarding recommendations,
and caller-provided runtime-fact sources are not selected consumers.

## 6. Existing Path Inventory

The implementation phase must reuse rather than duplicate these accepted
boundaries:

- `LocalExecutionWithCoreOwnedAuthoritativeDocsCheckGovernanceRequest` for the
  closed project-declared request;
- immutable run-bundle persistence and exact durable binding;
- Core-owned authoritative `DocsCheck` preflight and same-call execution;
- the private fact-bound authoritative assessment;
- the current proof-enforced approval presentation boundary;
- the registered runtime-fact source and freshness validator;
- trusted decision-time authority-receipt derivation;
- receipt-bearing terminal WorkReport construction; and
- receipt and report-artifact persistence, referential integrity, and selected
  gates.

No implementation should reconstruct these rules in CLI code.

## 7. Core-Owned Source And Trust Boundary

The first implementation should add a private or narrowly scoped Core-owned
source adapter for the selected profile. Core defines its fixed source identity,
contract version, freshness rule, and supported workflow shape. The CLI may
request the selected product operation but may not provide or override the
source registration or fact values.

The source adapter may consume only facts established inside the same governed
call or read from exact durable state already bound to the run. Serialized
snapshots, report citations, persisted receipts, CLI flags, and natural-language
summaries cannot regain authority.

Unknown or unavailable facts remain explicit. Derivation may preserve or
increase strictness; it may never weaken workflow, policy, profile, authority,
evidence/check, sensitivity, SideEffect, or steward minima.

## 8. Fact Derivation

| Fact category | Authoritative source for the selected path | Required posture |
| --- | --- | --- |
| Action and reversibility | Frozen workflow and skill definition records | Exact declared value or unresolved; never inferred from prose |
| Authority | Validated project activation plus the closed Core-owned profile contract | Caller input cannot satisfy authority |
| Evidence and checks | Actual same-call canonical `DocsCheck` result and requirement coverage | Failed, missing, extra, or mismatched coverage fails closed |
| Sensitivity | Frozen validated declarations and accepted profile minimum | Unknown remains conservative |
| SideEffect | Frozen declarations and current durable run state | No provider or mutation inference |
| Runtime escalation | Current durable run and approval context | Escalation composes monotonically |
| Definition identity | Exact immutable run bundle and relevant-definition roots | Any relevant change invalidates the assessment |

The source snapshot remains payload-free and call-local evidence metadata. It
does not become reusable authority.

## 9. Required Runtime Ordering

The eventual selected consumer must preserve this order:

1. Load and validate the project and selected closed profile.
2. Build or validate the exact immutable run bundle.
3. Complete pure preflight for the selected local check batch.
4. Execute the canonical `DocsCheck` and establish its structured outcome.
5. Construct the Core-owned registered-source snapshot from exact declarations,
   durable context, and the same-call check outcome.
6. Assess, bind, and persist the initial governance commitment before run
   events or skill execution.
7. Present and durably prove the complete approval context when approval is
   required.
8. On grant, invoke the source again in the same decision call, validate
   freshness, and reproduce the exact durable governance binding before
   approval mutation.
9. Derive a trusted receipt only from the successful grant proof.
10. Build the receipt-citing terminal report.
11. Persist or reconcile the receipt, validate integrity and selected gates,
    then persist or reconcile the report artifact.

Denial remains available without decision-time source invocation after valid
presentation proof. Post-decision report or persistence failure must preserve
the truthful terminal workflow and approval result.

## 10. Equivalence-First Migration

Adoption should use three separately reviewable phases:

1. **Bridge and equivalence tests.** Implemented and accepted. The private
   Core-owned source uses a fixed registration, observes exact same-call facts,
   produces a source-backed binding, and proves required, optional, denied, and
   failed posture equivalence without changing product behavior.
2. **Explicit selected-consumer composition.** Add one additive Core API that
   owns the source bridge and accepted approval-to-artifact closure for this
   path. Implemented; focused review remains. The API preserves separate
   aggregate-governance and workflow step approvals when both are declared.
   Keep the existing public APIs available.
3. **CLI adoption.** Only after the Core-owned evaluation-time blocker fix and
   its focused review are accepted, route the existing explicitly
   activated authoritative project-validation command through the selected
   consumer. Preserve output and compatibility, and do not broaden activation.

The old path must remain until exact equivalence is demonstrated for success,
denial, approval wait/resume, failed check, stale facts, changed definitions,
missing proof, report failure, and persistence failure.

## 11. Failure And Compatibility Semantics

- Project, declaration, immutable-bundle, preflight, check, source, freshness,
  or assessment failure occurs before run events and execution where the
  current boundary requires it.
- Approval-presentation proof failure occurs before source access or mutation.
- Failed or incomplete checks cannot be converted into satisfied evidence.
- Existing authoritative CLI errors and human/JSON output remain stable until
  a separately approved compatibility change.
- Existing executor APIs remain available and unchanged.
- No source, registration, snapshot, identifier, path, command output, report
  text, environment value, or credential may appear in public Debug or errors.

## 12. Contract Enforcement

The selected consumer enforces only the already-reviewed closed project
contract. It does not accept an arbitrary `WorkReportContract`, arbitrary local
check commands, or workflow-declared source plugins. Missing required report
sections, receipt citations, integrity records, or selected gates fail through
existing constructors and helpers.

## 13. Test Plan

The implementation sequence should add focused tests for:

- exact fact equivalence with the existing authoritative path;
- fixed Core-owned source identity and registration;
- rejection of caller attempts to substitute authority or source facts;
- same-call check-result ownership and exact requirement coverage;
- failed, missing, duplicate, extra, and mismatched check posture;
- relevant-definition invalidation and unrelated-definition stability;
- no process use before complete preflight;
- proof-enforced grant and source-free denial;
- stale or changed decision-time facts blocking before mutation;
- trusted receipt and receipt-citing report-artifact success;
- truthful terminal results after report or persistence failure;
- no duplicate execution or writes on exact retry;
- stable human and JSON CLI output in the later adoption phase;
- redaction-safe Debug, errors, events, reports, and persisted records; and
- all existing executor, authoritative-governance, approval, report, receipt,
  SideEffect, adapter, runtime, and CLI tests.

## 14. Documentation And Validation

Each implementation phase must update the roadmap, its implementation report,
and focused review. The bridge implementation must run focused tests plus:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`; and
- `git diff --check`.

## 15. Open Questions

- Can the existing private fact-bound assessment be adapted directly without
  exposing its internal fact representation?
- Which existing error codes must remain byte-for-byte compatible in the later
  CLI adoption phase?
- Should source-snapshot identity be exposed in bounded machine output after
  adoption, or remain report evidence only?
- What exact equivalence vector is sufficient before retiring duplicate
  internal composition?

None of these questions authorizes caller-provided authority or wider runtime
activation.

## 16. Final Recommendation

The focused Core-owned evaluation-time blocker fix is implemented and accepted.
The selected public inputs no longer accept an evaluation timestamp; Core
chooses a fresh time at initial routing and each decision call. Plan CLI
adoption separately. Do not change CLI behavior or retire the existing
authoritative path without that separately governed compatibility phase.
