# Expansion Readiness Review

Review date: 2026-07-31

## 1. Executive Verdict

**Ready to plan one optional OpenShell no-write execution-provider vertical
slice; not ready to broaden provider mutations.**

Workflow OS has enough accepted governance, immutable-input, authority,
durable-state, hosted-dispatch, evidence, and report foundations to evaluate a
real sandboxed execution provider. The next useful expansion is not another
write adapter and not an OpenShell fork. It is one explicit, local or
single-tenant, no-write OpenShell adapter behind the existing
`HostedExecutionProvider` boundary.

The current provider-neutral receipt is close to the required boundary, but it
does not yet prove the exact effective sandbox policy, enforcement/degradation
posture, runtime image identity, or cleanup outcome. Those facts must be added
as part of the same bounded vertical slice before a sandbox result can count as
governed execution evidence.

## 2. Scope Verification

This review inspected accepted roadmap evidence, provider-neutral hosted
contracts, authority and capability projection, proportional governance,
immutable run binding, the first provider-write proof, and current OpenShell
documentation.

It adds no runtime code, dependency, provider call, sandbox, fork, SideEffect,
write, schema, SDK, CLI command, example, credential handling, hosted claim,
multi-tenancy, reasoning lineage, or release change.

## 3. Foundations That Are Ready

The repository now has the main governance-side inputs for a bounded execution
provider:

- immutable run-bundle identity and integrity roots;
- exact run, workflow, step, correlation, and idempotency identity;
- proof-enforced approval presentation and decision paths;
- scoped capability and current-authority foundations;
- typed policy bindings and bounded execution budgets;
- durable hosted work, attempt, lease, reconciliation, and terminal projection;
- payload-free artifact, log, denied-action, telemetry, and reconciliation
  references;
- terminal WorkReport and report-artifact foundations;
- proportional-governance assessment and quiet-success composition;
- local and shared durable-state conformance foundations.

The accepted GitHub comment sandbox proves one narrow authority-to-effect path.
It does not authorize another mutation family. The accepted hosted alpha proves
provider-neutral no-write dispatch, durable attempts, terminal projection, and
recovery posture. Those are the relevant foundations for an OpenShell proof.

## 4. Why OpenShell Fits

Workflow OS and OpenShell own different boundaries:

- Workflow OS decides whether work may proceed, what authority and approvals
  are required, which evidence and checks are owed, and how the result is
  recorded and reported.
- OpenShell owns the execution environment, process identity, filesystem
  restrictions, network mediation, credential routing, sandbox lifecycle, and
  local security logs.

That separation preserves the product posture: **Agent executes. Workflow OS
governs.** OpenShell is an execution substrate, not a second governance source.
It must not decide Workflow OS policy, grant authority, append Core events, or
declare a workflow successful.

Current upstream behavior is promising for this boundary. OpenShell exposes
sandbox lifecycle, effective and revisioned policy inspection, static
filesystem/process policy, dynamic network policy, default-deny behavior,
enforce and audit modes, denied-action logging, and local sandbox logs. It is
also explicitly alpha software, so Workflow OS must pin and test an exact
version rather than depend on an unstable latest interface.

## 5. Required Integration Contract

The first adapter must consume one already-authorized, immutable,
payload-free `HostedExecutionRequest`. It must return a bounded receipt that is
validated against the exact request and durable attempt.

Before invocation, Workflow OS must bind:

- immutable bundle identity and integrity root;
- run, workflow, step, correlation, and idempotency identity;
- authorized capabilities and the no-write SideEffect posture;
- requested sandbox policy identity and canonical hash;
- execution budget;
- pinned OpenShell and runtime image identity;
- explicit input and output artifact references;
- zero access-material references for the first slice.

The provider receipt must add or prove:

- stable OpenShell gateway, sandbox, and provider invocation references;
- requested policy hash and the effective loaded policy revision/digest;
- enforcement mode and any degraded, skipped, or unsupported control posture;
- runtime image or environment identity;
- start, terminal, and cleanup timestamps or stable lifecycle references;
- exit status and terminal classification;
- bounded log and security-event references;
- denied-egress or denied-action references;
- produced artifact references and their integrity metadata;
- cleanup outcome and any reconciliation-required posture.

A request-policy echo is not sufficient attestation. If the effective policy
cannot be inspected, bound to the sandbox, and compared with the request, the
provider must fail closed. `audit` mode, best-effort filesystem degradation,
unsupported kernel controls, stale policy revisions, or cleanup uncertainty
must be explicit facts, never silently treated as enforced containment.

## 6. Minimal Compelling Prototype

The first prototype should be one opt-in, no-write, deterministic execution:

1. Start from an immutable run bundle and proof-enforced governed step.
2. Create one OpenShell sandbox from a pinned version and image.
3. Mount or copy one read-only input workspace and one bounded writable output
   directory.
4. Apply a hard-requirement filesystem/process posture and default-deny
   network policy.
5. Run one deterministic no-write validation command through the provider.
6. Prove one denied network attempt without making it a required success path.
7. Retrieve bounded artifact, log, security-event, and policy-revision
   references.
8. Destroy the sandbox and record cleanup posture.
9. Project the exactly bound receipt into existing hosted terminal events and a
   report artifact.
10. Prove restart/reconciliation behavior without blindly repeating an
    ambiguous invocation.

Unit and integration tests should use an injected transport or API client. One
real OpenShell smoke test may remain explicit and ignored by default until CI
has a reviewed runtime installation boundary.

## 7. APIs And Data Surfaces Required From OpenShell

An integration should require stable or machine-readable access to:

- create, inspect, wait, stop, and delete sandbox lifecycle operations;
- sandbox and gateway identity;
- policy set/update status, current effective policy, and policy revision;
- explicit enforcement and degradation status for filesystem, process, and
  network controls;
- command or child-process start, exit, timeout, and cancellation status;
- structured security events for network, process, filesystem, policy, and
  configuration decisions;
- artifact import/export with integrity information;
- bounded logs or durable log references;
- provider/credential references without credential material;
- idempotent lookup or reconciliation after client/worker interruption.

CLI text scraping is acceptable only for an exploratory spike. The reviewed
adapter must use a documented SDK/API or a version-pinned machine-readable
contract with compatibility tests.

## 8. Fork Decision

Do not fork OpenShell now.

A fork would make Workflow OS responsible for container and VM lifecycle,
filesystem and kernel controls, network proxying, credential injection,
platform support, vulnerability response, and enterprise runtime hardening.
That is a separate product and security program.

Reconsider a fork only if all of these become true:

- upstream cannot expose effective-policy identity or enforcement/degradation
  posture;
- upstream cannot provide stable lifecycle, security-event, artifact, and
  reconciliation surfaces;
- the missing behavior is central to Workflow OS governance rather than a
  product preference;
- sustained upstream contribution has failed;
- Workflow OS has maintainers, threat-model ownership, release engineering,
  CVE response, and cross-platform test capacity for the runtime surface.

An upstream plugin, adapter, or contributed API should precede a fork.

## 9. Risks

The main underestimated risks are:

- **Attestation confusion:** a requested policy hash can be mistaken for proof
  that the exact policy loaded and remained effective.
- **Policy TOCTOU:** OpenShell network policy can change on a running sandbox;
  the receipt must bind the effective revision observed for execution.
- **Degraded controls:** best-effort or platform-specific enforcement can leave
  a sandbox less restricted than the workflow assumes.
- **Credential boundary expansion:** OpenShell providers and inference routing
  introduce a new secret and identity control plane.
- **Ambiguous completion:** gateway, worker, or network failure can leave an
  invocation or cleanup outcome uncertain.
- **Artifact trust:** sandbox-produced artifacts require integrity, sensitivity,
  provenance, and size validation before they become evidence.
- **Log sensitivity:** security and process logs can contain paths, arguments,
  destinations, or other sensitive metadata.
- **Alpha churn:** upstream APIs and behavior may change without notice.
- **Double-policy drift:** Workflow OS and OpenShell policy vocabularies can
  diverge unless the adapter translates a small explicit contract and records
  both identities.
- **False product claims:** sandboxing one provider must not be described as
  making every local handler, adapter, or workflow sandboxed.

## 10. Expansion Decision

The next lane should strengthen execution, not add another primitive family or
provider mutation. Plan one **optional OpenShell no-write execution-provider
vertical slice** that includes the provider-neutral attestation additions,
adapter, durable attempt/receipt integration, report evidence, cleanup and
reconciliation posture, and one bounded real smoke test.

Another GitHub mutation, Jira creation, write-capable adapter family,
production credential flow, automatic sandboxing default, or hosted production
claim remains blocked. OpenShell must remain optional; native local handlers
remain supported and continue to fail closed when missing.

## 11. Non-Blocking Follow-Ups

- Define retention and sensitivity posture for OpenShell security logs.
- Decide whether structured OpenShell events map directly to existing hosted
  reference kinds or need one bounded provider-observation vocabulary.
- Measure startup and teardown overhead before considering quiet-success
  defaults.
- Keep low-risk native local checks available where a sandbox would add cost
  without satisfying a declared policy requirement.
- Continue quiet-success UX work independently: containment should not imply a
  human approval for every low-risk run.

## 12. Recommended Next Phase

Create and review an **OpenShell Optional No-Write Execution Provider Vertical
Slice Plan**. The plan should cover the attestation contract, exact upstream
version, API boundary, threat model, prototype command, test matrix, lifecycle
and cleanup, failure/reconciliation semantics, evidence/report mapping, and
explicit non-goals.

Do not implement the adapter until that focused plan is accepted. Do not fork
OpenShell or broaden provider writes.

## 13. Governed Review Evidence

- Dogfood workflow: `dg/review`.
- Run ID: `run-1785557050712879000-2`.
- Approval ID:
  `approval/run-1785557050712879000-2/review-scope-approved`.
- Approval presentation ID: `presentation/635ccc210b02b326`.
- Approval outcome: granted by the delegated maintainer through the
  proof-enforced path.
- Review scope: expansion readiness only; no runtime or provider behavior.
- Event summary: 39 ordered events, including one proof-enforced approval
  request and grant, eight policy decisions, six scheduled steps, six
  successful skill invocations, and one completed run; no retries or
  escalations.
- Validation: `npm run check:docs` passed; `git diff --check` passed. Runtime
  tests were not run because this review changed documentation only.
- Out-of-kernel work: Codex inspected repository and upstream OpenShell
  evidence, authored this review, and will perform git/PR operations. The
  kernel governed scope and approval but did not browse, edit files, execute
  validation, or perform git/PR actions.
