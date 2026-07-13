# Proportional Governance Workload Assessment Report

## 1. Executive Summary

Workflow OS now has a model-only deterministic workload-assessment helper for
proportional governance. The helper derives a review-only recommendation from
bounded typed workload facts, composes that recommendation with explicit
validated governance minima, reports unresolved fact categories, and produces
a versioned payload-free input fingerprint.

This phase answers two important external-review findings. Visible disclosure
is not an execution mode: it remains an independent presentation obligation.
Ordinary callers also no longer need to manufacture an already-classified
`ProportionalGovernanceDecisionInput` when they have typed workload facts.

The helper is in-memory, assessed-not-enforced, and not persisted. It does not
inspect repositories, configure workflows, change schemas, integrate with the
executor, approve work, render UI, or invoke providers.

Initial maintainer review found that machine-width field framing made the
fingerprint architecture-dependent and that inferred action posture could be
mislabeled as a workflow-declared reason. The focused blocker fix uses `u64`
big-endian field lengths, pins a known v1 fingerprint vector, and adds a
distinct workload-assessment selector requirement and reason.

## 2. Scope Completed

This phase added:

- `ProportionalGovernanceWorkloadAssessmentInput`;
- `ProportionalGovernanceWorkloadAssessment`;
- versioned `GovernanceWorkloadAssessmentAlgorithm::V1`;
- `GovernanceDecisionReason::WorkloadAssessment` and a distinct selector input
  for deterministic assessment posture;
- typed action, authority, evidence/check, sensitivity, and SideEffect facts;
- deterministic unknown-fact and completeness posture;
- `assess_proportional_governance_workload`;
- a domain-separated fingerprint over the immutable definition root and every
  modeled decision-relevant input;
- focused mapping, monotonicity, invalidation, privacy, and non-regression
  tests.

## 3. Product Boundary

The accepted proportional-governance model has independent axes:

- execution: proceed, require approval, or deny;
- disclosure: quiet or visible.

The assessment helper does not include a local UI preference because an
operator may display quiet decisions without changing governance. A policy
requirement for visible disclosure remains authoritative regardless of whether
a presentation surface exists.

The first narrow GitHub PR-comment provider mutation remains separate. Pull
request creation, Jira issue creation, and other mutation families still
require their own capability, authority, idempotency, reconciliation,
SideEffect, evidence, approval, report, and sandbox boundaries.

## 4. Deterministic Mapping

The v1 assessment maps bounded facts conservatively:

| Fact | Derived posture |
| --- | --- |
| read-only action | quiet proceed |
| reversible local mutation | visible proceed |
| external mutation | blocking approval |
| unsupported action | denial |
| sufficient authority | quiet proceed |
| approval-required or unknown authority | blocking approval |
| unavailable authority | denial |
| satisfied evidence/checks | quiet proceed |
| optional evidence/check unavailable | visible proceed |
| required evidence/check unavailable or failed | denial |
| routine sensitivity | quiet proceed |
| elevated sensitivity | visible proceed |
| restricted or unknown sensitivity | blocking approval |
| no SideEffect | quiet proceed |
| reversible local SideEffect | visible proceed |
| external or unknown SideEffect | blocking approval |
| ambiguous or unsupported SideEffect | denial |

Unknown modeled facts are returned as stable categories and make completeness
`incomplete`. Unknown safety-relevant facts never produce optimistic quiet
execution.

## 5. Explicit Minima And Authority

Inference is recommendation, not policy authority. The helper feeds inferred
requirements into the accepted proportional-governance selector alongside:

- governance-profile minimum;
- workflow minimum;
- policy minimum;
- authority minimum;
- evidence/check minimum;
- sensitivity minimum;
- SideEffect minimum;
- runtime escalation;
- prior accepted execution and disclosure posture;
- steward minimum.

The strictest execution and disclosure requirements win. Inference cannot
downgrade any explicit minimum. Failed required checks and unavailable
authority deny rather than inviting an approval to override a failed invariant.

## 6. Fingerprint And Invalidation

The fingerprint uses the stable algorithm domain
`workflow-os/proportional-governance-workload-assessment/v1`. It includes:

- immutable definition root;
- governance profile;
- every explicit minimum;
- every typed workload fact;
- runtime escalation;
- prior accepted posture;
- steward minimum.

Identical validated inputs produce identical decisions and fingerprints. A
change to any modeled decision-relevant fact or the immutable definition root
changes the fingerprint and therefore provides a deterministic future
reassessment boundary.

Each label and value is framed with a fixed eight-byte big-endian length. The
focused tests pin a known v1 vector so host architecture and accidental format
changes cannot silently alter the contract.

The fingerprint is not an authorization grant, persisted decision, executable
replay artifact, or proof that underlying source and provider state remain
unchanged.

## 7. Privacy And Redaction

The assessment accepts no free-form prompts, source contents, command output,
provider payloads, paths, environment values, credentials, or tokens. Debug
output redacts the definition root and input fingerprint. Serialization exposes
only bounded enum posture, stable decision reasons, completeness, unresolved
fact categories, and the payload-free fingerprint.

## 8. Scope Explicitly Not Completed

This phase does not add:

- executor or runtime enforcement;
- automatic approvals or delegated self-approval;
- YAML, policy, workflow, or public schema fields;
- CLI or UI behavior;
- repository scanning or onboarding integration;
- persistence, events, audit records, or report artifacts;
- provider calls or new mutation families;
- hosted administration, enterprise identity, or RBAC;
- probabilistic model authority;
- release-posture changes.

## 9. Test Coverage

Focused tests cover:

- complete quiet read-only assessment;
- visible non-blocking local mutation;
- approval-gated external mutation;
- denial for failed or unavailable required checks;
- denial for unavailable authority and ambiguous SideEffects;
- explicit conservative handling of every unknown fact family;
- monotonic explicit workflow, policy, authority, evidence/check, sensitivity,
  SideEffect, runtime, profile, prior-decision, and steward minima;
- strict-enterprise fail-closed behavior;
- deterministic decisions and fingerprints;
- invalidation across every decision-relevant family and definition root;
- independence of disclosure from execution;
- truthful separation of inferred workload reasons from explicit workflow
  declaration reasons;
- a fixed known v1 fingerprint vector;
- payload-free serialization and redaction-safe Debug;
- existing proportional-governance selector behavior.

## 10. Validation

Validation completed during implementation:

- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed;
- `npm run check:docs`: passed;
- `git diff --check`: passed.

## 11. Governed Phase Evidence

- Dogfood workflow: `dg/implement`.
- Run ID: `run-1783926398072679000-2`.
- Approval ID:
  `approval/run-1783926398072679000-2/implementation-approved`.
- Approval presentation: `presentation/d5917bbc9f07929e`.
- Approval outcome: granted with persisted presentation proof.
- Phase status: completed.
- Out-of-kernel work: Codex implemented and tested the model; the kernel
  coordinated governance only.
- Report posture: this document is a repository report, not a generated or
  persisted runtime WorkReport artifact.

## 12. Remaining Limitations

- Callers must still supply typed workload facts explicitly.
- No safe repository-metadata adapter derives those facts yet.
- No runtime path consumes the recommendation or verifies fingerprint
  freshness.
- The assessment is not persisted or projected into events, audit, or reports.
- No presentation surface renders quiet or required-visible disclosures.
- No configuration source lets users accept or tighten onboarding defaults.

## 13. Recommended Next Phase

Perform a focused maintainer review of the workload-assessment and fingerprint
boundary. If accepted, implement one explicit read-only onboarding
recommendation path that derives typed facts from already accepted safe
metadata. Do not integrate executor enforcement or broaden provider mutations
in that phase.
