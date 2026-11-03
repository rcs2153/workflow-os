# Hacker News Launch Admission Plan

Status: Proposed product-admission lane. Focused maintainer/product/security
review is required before implementation. This plan does not authorize release
publication, a Hacker News submission, or expansion of the product boundary.

Related foundations:

- [Current Product Contract](../user-guide/current-product-contract.md)
- [Workflow OS 0.3.0-preview.1 Release Notes](../release/0.3.0-preview.1_RELEASE_NOTES.md)
- [Existing Repo Governance Onboarding Plan](existing-repo-governance-onboarding-plan.md)
- [First-Run Quiet-Success Artifact Composition Plan](first-run-quiet-success-artifact-composition-plan.md)
- [Capability-Gated GitHub Read-Only Public Adoption And Recovery Plan](capability-gated-github-read-only-public-adoption-recovery-plan.md)
- [Delegated Authority Projection Into External Connectors Plan](delegated-authority-external-connector-projection-plan.md)

## 1. Decision

Insert an explicit Hacker News launch-admission milestone immediately after the
current delegated-authority connector lane and before Governed Obligation Plan
reconciliation or additional architectural expansion.

The milestone answers one product question:

> Can a technically sophisticated new user install Workflow OS, introduce it
> safely into an unrelated repository, receive immediate governance value,
> execute one real bounded operation, and inspect truthful durable evidence in
> ten minutes without understanding Workflow OS internals?

The launch threshold is not enterprise readiness. It is one compelling,
repeatable, honest local-first product loop built from capabilities the kernel
already owns.

## 2. Why This Lane Is Next

Workflow OS has crossed the credible-kernel threshold. It now has immutable run
identity, deterministic sequential execution, policy and approval enforcement,
current-fact reassessment, proportional governance, durable events, continuation,
evidence and SideEffect vocabulary, WorkReports, existing-repository onboarding,
and narrow real integration proofs.

The remaining launch risk is composition, not missing vocabulary. Continuing to
add model families before proving the first-user loop would increase internal
depth without materially improving the experience an external evaluator sees.

The current delegated-authority connector lane remains ahead of this milestone
because it is already in flight and closes a real continuity and authority gap.
This plan must not weaken, bypass, or partially accept that work.

## 3. Launch Claim

The admitted launch claim is narrow:

> Point Workflow OS at an existing repository and it establishes a conservative
> local governance envelope, explains the repository's bounded governance
> posture, executes one exact governed operation, and leaves an inspectable
> evidence and reporting trail.

The launch must not claim:

- production enterprise readiness;
- a general agent runtime;
- arbitrary repository command execution;
- broad provider-write support;
- production autonomous scheduling;
- recursive agents, agent swarms, or nested harness execution;
- production multi-tenant hosting, identity, RBAC, or high availability; or
- OpenShell execution-provider readiness.

## 4. Golden Path

The acceptance path must use a packaged release artifact rather than a developer
checkout and must not require provider credentials.

1. Install a signed or checksummed Workflow OS release artifact through one
   documented command sequence.
2. Run `workflow-os init-repo-governance --agent <agent> --dry-run` in an
   unrelated existing repository.
3. Initialize governance without replacing unmanaged `AGENTS.md` content or
   silently overwriting customized files.
4. Run `workflow-os validate` and receive one concise actionable result.
5. Run `workflow-os first-run` and receive bounded repository posture,
   implemented-versus-deferred disclosures, and concrete review-only workflow
   recommendations.
6. Materialize the existing explicit first-run report artifact path and prove
   real authoritative project validation, proportional-governance selection,
   evidence attachment, and durable WorkReport persistence.
7. Execute one exact reviewed non-mock governed operation through the ordinary
   `run`, `status`, `next-action`, and `inspect` surfaces. Prefer a credential-free
   fixed local operation. A live provider read may be an optional supplemental
   proof but cannot be required for baseline admission.
8. Show that exact replay does not duplicate work or evidence and that a
   recoverable interruption resumes from durable state.

The existing `--mock-all-local-skills` demonstration may remain documented as a
sequencing demo, but it cannot satisfy any launch-admission evidence requirement.

## 5. Exact Operation Selection

The implementation plan review must select one existing or minimally composed
operation using these criteria:

- credential-free and network-free for the baseline path;
- useful in at least two unrelated repository types;
- fixed and reviewed rather than arbitrary shell execution;
- admitted through ordinary immutable run and policy contracts;
- real output that can be independently checked;
- evidence and WorkReport references produced by the actual execution path;
- proportional-governance behavior derived by Core rather than chosen by the
  caller; and
- truthful failure, replay, and interruption behavior.

The current authoritative project-validation check is the preferred starting
candidate. If it cannot produce a meaningful ordinary governed operation without
creating a parallel execution path, focused review must identify the smallest
existing composition that can. It must not respond by adding a generic command
runner.

## 6. Installability

Launch admission requires release artifacts that do not force evaluators to
compile the repository. At minimum:

- publish versioned archives for macOS arm64 and Linux x86_64;
- publish SHA-256 checksums and a verification command;
- make `workflow-os --version` identify the exact release;
- document an uninstall path and local state location;
- test installation in clean environments; and
- retain a source-build path as a contributor option, not the primary user path.

Signing or provenance attestation should be used when the existing release
infrastructure can support it without introducing an unreviewed release system.
Checksums are the minimum admission boundary, not a claim of complete software
supply-chain security.

## 7. Default User Experience

The default path must lead with decisions and next actions rather than an
internal posture matrix.

The first screen should answer:

- what Workflow OS detected;
- what is ready now;
- what remains review-only or unimplemented;
- whether human action is required;
- what evidence was gathered; and
- the exact next command.

Verbose and JSON modes may preserve complete diagnostic and machine-readable
detail. The default path must not hide governance failures or omit limitations
to appear simpler.

## 8. Clean-Room Evaluation

Run the exact packaged artifact against at least two public or disposable
repositories that were not used to design the path:

- one JavaScript or TypeScript repository; and
- one Rust, Python, Go, or other non-Node repository.

The evaluator starts only with the release installation and user guide. The
evaluation record must include:

- elapsed time to first useful WorkReport;
- commands executed;
- approvals and interruptions encountered;
- whether existing repository instructions were preserved;
- every mock, fixture, skipped check, and out-of-kernel action;
- whether the operation produced independently checkable real output;
- event, evidence, and report references;
- replay and interruption-recovery results;
- confusing terminology or unsupported assumptions; and
- any manual state repair or Workflow OS source knowledge required.

Any hidden manual state repair, mock presented as execution, lost repository
guidance, ambiguous terminal state, or unsupported success claim blocks launch
admission.

## 9. Admission Metrics

Both clean-room runs must meet all of these thresholds:

- first useful result in ten minutes or less after installation;
- no Workflow OS source checkout or local compilation;
- no provider credential required for the baseline path;
- zero mock handlers in admission evidence;
- zero silent repository-file replacements;
- zero manual runtime-state edits;
- no unexplained approval prompt;
- one persisted WorkReport citing actual validation or execution evidence;
- one complete inspectable event trail;
- exact replay without duplicated work; and
- truthful recovery or explicit fail-closed posture after interruption.

Usability findings that do not threaten truth, safety, or completion may be
recorded as post-launch follow-ups. Correctness, evidence, recovery, installation,
and product-truth failures are blockers.

## 10. Phased Delivery

### Phase 0: Focused Plan Review

Reconcile this plan against current release, onboarding, first-run, execution,
report, and connector behavior. Select the exact non-mock operation and identify
the smallest missing composition. Reject any hidden parallel runtime path.

### Phase 1: Release Artifact Rehearsal

Produce and verify local candidate archives and checksums for the selected
platforms. Do not publish them. Validate version identity, install, uninstall,
and clean-state behavior.

### Phase 2: Golden-Path Composition

Implement only the missing composition required for the selected operation and
default UX. Reuse existing authority, policy, proportional-governance,
continuity, evidence, and report contracts.

### Phase 3: Clean-Room Evaluation

Execute the exact packaged path against two unrelated repositories, preserve
the evidence, and fix only admission-blocking findings.

### Phase 4: Release-Candidate Review

Run the full required test matrix, dependency and documentation checks,
security review, release checklist, and exact-commit CI. Confirm announcement
language matches implemented behavior.

### Phase 5: Separate Publication Decision

Tagging, publishing release artifacts, making a GitHub Release, and submitting
to Hacker News require a separate governed release action. Admission does not
automatically authorize publication.

## 11. Strict Non-Goals

This lane does not authorize:

- a new primitive or policy language;
- Salsa or another incremental computation dependency;
- a generic shell, build, package-manager, or arbitrary-command executor;
- broad GitHub, Jira, CI, cloud, SaaS, or generic HTTP mutation;
- OpenShell activation or a Workflow OS runtime fork;
- parallel execution, DAG scheduling, or nested harness runtime;
- production background scheduling or unattended Level 3/4 autonomy;
- enterprise identity, stewardship administration, RBAC, or IdP integration;
- a complete operator UI;
- production hosting or multi-tenancy; or
- launch publication without separate approval.

If clean-room evidence reveals that one deferred capability is required for
truthful completion, stop and document the blocker. Do not expand scope by
implication.

## 12. Exit Criteria

The lane is complete only when:

- focused plan review accepts the exact golden path and operation;
- installable candidate artifacts and checksums pass clean-environment rehearsal;
- the golden path contains no mock execution or hidden manual repair;
- two unrelated clean-room repositories satisfy every admission metric;
- complete evidence, events, WorkReports, limitations, and recovery results are
  preserved;
- required CI is terminal-green for the exact reviewed commit;
- release and security review accept the product claim; and
- the roadmap records a separate next action for the publication decision.

Failure to meet these criteria means Workflow OS remains a valid technical
preview but is not admitted for the intended Hacker News launch posture.
