# Capability-Gated GitHub Read-Only Public Adoption And Recovery Plan Review

## 1. Verdict

Accepted after required plan corrections.

The proposed lane is directionally correct: one exact opt-in GitHub repository
metadata read should become available through the ordinary governed workflow
path, not through a provider utility command. The review found three blocking
ambiguities in the initial draft and corrected them before implementation was
authorized.

## 2. Review Scope

The review compared the plan with:

- the crate-private GitHub repository-read executor composition;
- the registered in-memory current-authority source;
- project authoritative-execution activation and immutable-run binding;
- governed-continuation claim behavior;
- authorized-execution continuity and dispatch reservation;
- GitHub read-only configuration and live-client behavior;
- Rust, JSON Schema, TypeScript SDK, CLI, event, receipt, and report surfaces;
  and
- the previously accepted private composition review.

No runtime, schema, SDK, CLI, provider, credential, or package behavior changed
during this documentation-only review.

## 3. Blocking Finding: Parallel Activation Boundary

The draft introduced `governance.provider_reads` but did not explicitly require
the existing supported `governance.authoritative_execution` declaration. That
left room for a provider-read declaration to become a parallel execution
activation path.

The plan now requires both declarations. Provider-read configuration is a
subordinate constraint under authoritative execution. It cannot select a final
governance route, bypass policy, issue authority, or activate an ordinary
project by itself.

## 4. Blocking Finding: Authority And Credential Delivery Were Blurred

The draft correctly prohibited durable access material, but it did not define
how the exact token reaches the adapter after the current-authority source
reports availability.

The plan now separates the boundaries:

- Core-owned current authority derives only bounded availability and authority
  facts;
- a private transient-access resolver reads the exact configured reference
  only after governance and dispatch admission succeed; and
- a non-serializable same-call secret capability reaches only the injected
  adapter consumer.

The token value must never enter authority inventories, immutable bundles,
events, continuity state, receipts, reports, errors, or debug output.

## 5. Blocking Finding: Dispatch Reservation Is Not Standalone Admission

The existing dispatch reservation consumes an authorized execution attempt and
window binding created by the broader continuity lifecycle. The draft's
"atomic dispatch" wording could have been implemented as though reservation
alone could authorize an ordinary provider read.

The plan now requires the complete lifecycle: operational window, any required
resume directive, exact attempt creation, one-winner dispatch reservation,
same-call attempt capability, and terminal or ambiguity recording. Existing
state machinery must be reused. A parallel idempotency or recovery store is
still prohibited.

## 6. Recovery Assessment

The corrected recovery contract is conservative and acceptable:

- no committed admission permits a fresh reassessed attempt;
- started without terminal is ambiguous and never automatically retried;
- a missing event cannot override a durable started record;
- explicit abandonment closes the old attempt as ambiguous and requests a new
  governed read after fresh reassessment;
- terminal acknowledgement uncertainty is reconciled through a fresh backend
  connection; and
- exact terminal replay performs no provider call.

This is truthful for a read-only GitHub `GET`, whose prior result cannot be
reconstructed authoritatively after process loss.

## 7. Operation, Compatibility, And Privacy Assessment

The exact optional `AdapterRequirement.operation` field is an acceptable
selection boundary when combined with the existing closed structural contract.
It is additive for old definitions but mandatory for new live selection.

The explicit resource allowlist, fixed GitHub.com endpoint, exact Workflow OS
environment reference, no `GITHUB_TOKEN` fallback, payload-free receipts,
bounded telemetry, and fixture-first tests are appropriate. The stale live
client user-agent remains a blocker before live activation.

## 8. Implementation Sequencing Decision

The original integrated implementation shape was too large for one governed
phase. The lane is now divided into four reviewed phases:

1. dark contract, immutable binding, schema/SDK parity, and compatibility;
2. private authority, transient access, continuity, dispatch, and recovery;
3. fixture-backed public CLI vertical slice and disclosure;
4. live-client identity correction, exact live activation, and separately
   authorized smoke proof.

No phase may expose a partially governed provider path. Phase 3 is the first
public selection point; Phase 4 is the first live-network point.

## 9. Validation

Review validation:

- `npm run check:docs`
- `git diff --check`
- source-reference inspection against the current Rust, schema, SDK, and CLI
  implementation

Runtime tests were intentionally skipped because the review changed
documentation only.

## 10. Governed Review Record

- workflow ID: `dg/review`
- phase: `review`
- run ID: `run-1793444860964571000-2`
- approval ID:
  `approval/run-1793444860964571000-2/review-scope-approved`
- approval presentation ID: `presentation/7da08c38998d9201`
- approval presentation content hash:
  `7da08c38998d9201ada9a48ba09dad3a9e8c1a63b06ff67a6b81088b36d4bb3c`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted handoff was presented
- approval-presentation enforcement: proof enforced
- out-of-kernel work: source inspection, documentation edits, shell validation,
  Git, pull-request, and merge actions remain outside the kernel

## 11. Next Phase

Implement Phase 1 only: exact authored operation and subordinate project
activation contracts, immutable binding, schema/SDK parity, compatibility
behavior, and focused validation. Keep the path dark and non-executable.
