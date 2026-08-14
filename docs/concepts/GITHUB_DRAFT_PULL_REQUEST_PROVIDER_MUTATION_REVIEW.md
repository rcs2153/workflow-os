# GitHub Draft Pull Request Provider Mutation Review

Review date: 2026-08-14

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups; proceed to concrete opt-in
GitHub HTTP sandbox wiring and live-smoke planning.**

The implementation remains an explicit local Core helper behind an injected
provider. Review initially found three gate gaps: a loose WorkReport ID in place
of the required artifact, acceptance of a durable assessment without a current
runtime-fact reassessment, and approval/policy checks that did not prove the
exact granted presentation or adapter-write action. The phase fixed all three
before acceptance and added focused regressions.

## 2. Scope Verification

The phase stayed within the accepted integrated Core-helper boundary. It added
no Git transport, concrete GitHub HTTP client, automatic executor path, CLI
mutation command, non-draft pull request, merge, branch or file mutation,
another provider family, workflow schema, SDK change, hosted default, hidden
credential discovery, example, or release-posture change.

The helper remains explicit, local, sandbox-oriented, and disabled from default
runtime paths.

## 3. Request And Identity Assessment

The request is narrow and typed. It requires:

- a coherent terminal `WorkflowRun` with immutable bundle binding;
- a validated `WorkReportArtifactRecord` matching workflow, run, schema,
  version, and spec hash;
- a durable accepted V3 proportional-governance binding and a matching current
  reassessment evaluated at provider-use time;
- an exact repository-scoped capability resolution for
  `github.pull_request.create`;
- a durable policy decision, approval request, approval presentation, and
  granted decision proof marker;
- explicit SideEffect, idempotency, adapter, integration, actor, step, target,
  content, timestamp, sensitivity, redaction, and auth inputs.

The request does not infer the current checkout, branch, repository, actor,
credential, policy, or authority from hidden process state.

## 4. Policy And Proportional Governance Assessment

Durable policy references must resolve to `PolicyDecisionRecorded` events in
the exact run. Each decision must be allowed, require approval, evaluate
`InvokeAdapter`, include `ExternalWrite` and `AdapterInvoke`, contain no unknown
capability, and match actor/workflow/run identity. A start-workflow decision or
generic caller-authored allowed posture cannot authorize the mutation.

The helper requires the terminal run's durable V3 current-runtime-fact binding.
A separate current V3 reassessment must reproduce the same accepted assessment
and authoritative runtime-fact commitment, and its evaluation time must equal
the attempted provider-use time. V1/V2 bindings and stale reassessments fail
closed.

The helper validates this accepted current reassessment; it does not query or
invent runtime facts. Callers must use the existing Core current-fact
assessment path.

## 5. Authority And Approval Assessment

Broad GitHub or external-write vocabulary is insufficient. Capability
resolution must be authorized for the exact capability reference, repository,
actor, workflow, run, step, and attempted timestamp.

The approval request must be durable in the run. The presentation must validate
against that request and bind the exact rendered-content commitment. The run
must also contain a granted decision whose proof marker requires approval
presentation, uses request-match validation, references the exact presentation
ID and content hash, and was validated at decision time. Marker-free or
mismatched approvals fail before provider observation.

This is a proof-enforced blocking-approval slice. It does not establish a quiet
or delegated provider-write default.

## 6. Provider And Git Boundary Assessment

The provider trait exposes only bounded ref observation, exact lookup, and one
create call. The helper never runs Git, creates or pushes a branch, changes a
ref, reads the current checkout, or mutates source.

Head and base SHAs are correctly treated as provider observations of mutable
refs. Known pre-create drift blocks. Post-create movement is surfaced as
concurrent-ref-change reconciliation posture rather than described as atomic
prevention. The implementation does not automatically retry or close an
externally visible draft.

## 7. SideEffect And Idempotency Assessment

The helper persists a `Proposed` SideEffect carrying policy, approval, and
terminal WorkReport references. It validates approval linkage from the actual
store, performs lookup before creation, and transitions to `Attempted` before
the provider create call.

Known creation or an exact existing managed draft can reconcile to completed.
Known rejection can fail. Ambiguous lookup or create, conflicting state, and
concurrent ref movement remain disclosed and operator-reconcilable without
automatic retry. This preserves the distinction between deterministic
idempotency identity and a provider API that lacks atomic conditional create.

## 8. Evidence, Reporting, And Event Assessment

Known reconciled completion returns a bounded provider-summary
`EvidenceReference` plus report-ready SideEffect and evidence citations. The
SideEffect record references the validated input report artifact identity.

The helper returns transition results for explicit later event composition. It
does not append workflow events, mutate the terminal run, create or persist a
new report artifact, claim workflow completion from provider success, or hide
ambiguous provider outcomes.

## 9. Privacy And Redaction Assessment

Provider auth and rendered title/body are non-serializable request material.
Debug output redacts auth, repository, branches, SHAs, content, idempotency,
report identity, actor, correlation, and redaction metadata. Errors use stable
codes without copying target values, provider payloads, logs, diffs, source,
command output, environment values, approval prose, policy payloads, or tokens.

Secret-shape checks include common GitHub, GitLab, and Slack token prefixes.
Provider rejection/ambiguity codes are bounded and Debug-redacted.

## 10. Test Quality Assessment

Focused tests use a real local SideEffect store, a rehydrated terminal run,
immutable bundle identity, V3 current-runtime-fact bindings, durable policy and
approval events, proof-enforced approval, exact capability resolution, a
validated terminal report artifact, and an injected provider.

They prove:

- one successful create and completed SideEffect/evidence closure;
- exact existing managed-draft reuse without create;
- ambiguous create is not retried and does not claim post-observation;
- pre-create ref drift blocks before create;
- exact approval content commitment;
- proof-marker-required approval;
- adapter-write policy action/capability enforcement;
- stale current-fact reassessment rejection;
- secret-like content rejection and non-leaking output.

The tests do not claim concrete GitHub transport or live provider semantics.
That proof remains a separate environment-gated sandbox phase.

## 11. Documentation Assessment

The plan, roadmap, implementation report, and this review consistently state
that the helper is implemented while GitHub HTTP transport, automatic runtime
integration, CLI mutation behavior, Git transport, broader writes, schemas,
examples, hosted defaults, and release changes are not implemented.

The documentation does not claim that branch SHAs are atomically frozen by
GitHub pull request creation.

## 12. Blockers

None after the review-driven gate fixes.

## 13. Non-Blocking Follow-Ups

- Add a concrete least-privilege GitHub transport only behind this exact
  injected boundary.
- Maintain one environment-gated sandbox smoke for provider semantics,
  duplicate reconciliation, and mutable-ref disclosure.
- Decide how the explicit current-fact assessment path is composed by the
  future executor/provider caller without broadening default writes.
- Compose returned event/evidence/report closure through an explicit later
  runtime path.
- Keep readiness, merge, labels, reviewers, branch updates, and another
  provider mutation family separately governed.

## 14. Recommended Next Phase

Plan and implement one **concrete opt-in GitHub HTTP sandbox wiring and live
smoke** behind the accepted provider trait.

The phase should use least-privilege explicit auth, preserve lookup-before-
create and no-retry semantics, remain disabled from default executor and CLI
paths, and prove one non-sensitive draft target. It must not add Git transport,
merge, non-draft creation, another write family, hidden auth, broad runtime
defaults, or production-hosted claims.

## 15. Validation

- `cargo fmt --all --check`: passed
- `cargo clippy --workspace --all-targets -- -D warnings`: passed
- `cargo test --workspace`: passed
- `npm run check:docs`: passed
- `git diff --check`: passed
- Focused draft-PR provider mutation tests: passed

## 16. Governed Review Evidence

- Workflow: `dg/review`
- Run ID: `run-1786700308522093000-2`
- Approval ID:
  `approval/run-1786700308522093000-2/review-scope-approved`
- Approval presentation ID: `presentation/b08a75d313f06e4f`
- Approval outcome: granted through the proof-enforced path
- Phase status: completed after validation and phase close
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations

Out-of-kernel work: Codex inspected the implementation and accepted plan,
identified and fixed the report, policy, approval-proof, and current-fact gate
gaps, authored this review, and ran validation. The kernel governed scope and
approval but did not edit files, call GitHub, execute provider mutation, perform
Git operations, or create a pull request.
