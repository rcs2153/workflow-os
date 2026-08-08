# OpenShell Optional No-Write Execution Provider Plan Review

Review date: 2026-08-08

## 1. Executive Verdict

**Plan accepted; proceed to the integrated optional OpenShell no-write
execution-provider implementation milestone.**

The plan preserves Workflow OS as the governance authority, uses OpenShell only
as an optional containment provider, requires effective-policy and degraded-
control attestation, and defines a useful no-write proof with restart, cleanup,
evidence, and report closure. It is sufficiently bounded and implementation-
ready.

## 2. Scope Verification

The plan stays within planning scope. It does not implement or authorize a
dependency, OpenShell installation, adapter, fork, sandbox execution, provider
call, external mutation, credential flow, schema, SDK, CLI behavior, example,
multi-tenancy, lineage, production claim, or release change.

The future implementation remains explicit, opt-in, no-write, and
access-material-free.

## 3. Architecture Assessment

The ownership split is correct:

- Workflow OS owns immutable work identity, policy, authority, approval,
  proportional governance, durable attempts, events, evidence, and reports.
- OpenShell owns sandbox lifecycle, process/filesystem/network enforcement,
  local runtime identity, and provider security observations.

The plan prevents OpenShell from granting authority, appending Core events, or
declaring workflow success. It also preserves native local handlers and avoids
making containment a universal runtime claim.

The existing `HostedExecutionProvider` boundary is the right seam. The plan
strengthens its provider-neutral request/receipt contract instead of adding
OpenShell-specific concepts to workflow semantics.

## 4. Attestation Assessment

The plan correctly identifies the key missing invariant: echoing the requested
policy hash is not proof that the exact policy loaded and remained enforced.

Required effective-policy revision/digest, runtime image identity, filesystem/
process/network control posture, degradation posture, OCSF references,
cleanup, and reconciliation facts are appropriate. Every fact is bound to the
exact request, execution, durable attempt, provider, version, and configuration.

Failing closed on audit-only policy, best-effort degradation, skipped paths,
unsupported controls, unexpected endpoints/providers, revision changes, or
non-canonical effective policy is the right first-slice posture.

## 5. OpenShell Boundary Assessment

The current upstream surface appears sufficient for a bounded prototype:

- create, exec/connect, inspect, delete, and lifecycle operations;
- full effective policy and stored revision inspection;
- static filesystem/process and dynamic network policy;
- enforce/audit distinction;
- hard-requirement vs best-effort filesystem posture;
- process, network, policy, finding, and lifecycle OCSF events;
- machine-readable OCSF JSON export;
- file transfer and sandbox cleanup.

The plan correctly requires a pinned release/commit and image plus a documented
SDK/API or pinned machine-readable contract. Human CLI text parsing is excluded
from the reviewed adapter.

## 6. Prototype Assessment

One immutable input workspace, one bounded output directory, one fixed
deterministic no-write check, default-deny networking, a separate denied-egress
probe, structured evidence collection, and explicit sandbox deletion form a
small but compelling proof.

The denied-egress probe is correctly separated from primary command success.
The command is not arbitrary caller shell text. Access material, inference
routing, provider writes, and hot policy broadening remain excluded.

## 7. Lifecycle And Recovery Assessment

Persisting the durable attempt before sandbox creation and reconciling by exact
sandbox/invocation identity match existing hosted invariants.

The plan explicitly rejects duplicate sandboxes or command reruns while an
invocation may exist. Policy changes, artifact/log collection interruption,
cleanup uncertainty, and gateway/worker interruption remain visible as failure
or reconciliation-required posture rather than fabricated success.

Cleanup is correctly treated as part of the governed outcome. A successful
command with an unproven teardown is not a completely successful containment
record.

## 8. Evidence And Privacy Assessment

OpenShell OCSF records remain provider observations referenced by Workflow OS,
not replacements for the authoritative workflow event stream.

The planned receipt/report data is bounded and useful: effective-policy,
environment, OCSF, denied-action, artifact, lifecycle, and cleanup references
plus bounded counts and integrity metadata.

Raw command lines, source, output, event messages, paths, URLs, environment
values, credentials, and provider payloads are excluded from Core errors,
Debug, events, evidence summaries, and reports by default. The plan also
accounts for the non-durable gateway log buffer by requiring durable referenced
OCSF files or a separately explicit durable sink.

## 9. Proportional-Governance Assessment

The plan preserves the important separation between containment and approval.
A low-risk no-write step can use OpenShell and still complete quietly when no
other requirement demands approval. A consequential step cannot bypass an
approval because it is sandboxed.

Quiet success still leaves durable policy, control, evidence, denial, artifact,
and cleanup posture. Visible disclosure remains a presentation obligation,
not a separate execution strictness axis.

## 10. Fork Assessment

The no-fork decision is correct. The reconsideration criteria are appropriately
high: a governance-critical upstream gap, failed contribution/plugin path, and
real Workflow OS capacity to own runtime security, CVE response, releases, and
cross-platform testing.

An adapter or upstream contribution must precede any fork discussion.

## 11. Test And Acceptance Assessment

The test matrix covers request/receipt substitution, policy mismatch and
TOCTOU, degraded controls, path and network posture, structured denied-egress
proof, OCSF/artifact reduction, lifecycle ambiguity, restart reconciliation,
Core state ownership, quiet success, terminal events, report artifacts, and
native-handler non-regression.

The acceptance criteria are end-to-end and appropriately stronger than unit
model validity. The explicit pinned real smoke test is necessary evidence, but
it may remain ignored by default until CI has a reviewed OpenShell installation
boundary.

## 12. Blockers

None for implementation start.

Implementation must not reach a real sandbox until the provider-neutral
attestation contract and injected client boundary pass focused tests. This is a
sequencing constraint inside the accepted milestone, not another planning
phase.

## 13. Non-Blocking Follow-Ups

- Select the exact OpenShell release/commit, image digest, compute driver, and
  machine-readable API during the compatibility spike at implementation start.
- Prefer domain-neutral existing reference kinds; add one bounded observation
  kind only if the implementation proves the existing taxonomy ambiguous.
- Keep command selection fixed for the first proof; a reviewed handler registry
  belongs to a later expansion.
- Document platform requirements where hard filesystem/process controls are not
  uniformly available.
- Measure sandbox startup/teardown cost before considering any default or
  quiet-success optimization.

## 14. Recommended Next Phase

Implement the **optional OpenShell no-write execution-provider vertical slice**
as one governed runtime-composition milestone following the accepted plan.

The milestone includes provider-neutral attestation/cleanup hardening, an
injected OpenShell client, policy rendering and effective-policy verification,
one deterministic no-write provider operation, OCSF/artifact reduction,
durable attempt/reconciliation, Core-owned terminal events and report artifact,
tests, runbook, and focused review.

Do not add credentials, provider writes, automatic sandboxing, schema/SDK/CLI
surface, multi-tenancy, production claims, another mutation family, or an
OpenShell fork.

## 15. Governed Review Evidence

- Dogfood workflow: `dg/review`.
- Run ID: `run-1786227173959073000-2`.
- Approval ID:
  `approval/run-1786227173959073000-2/review-scope-approved`.
- Approval presentation ID: `presentation/7ea13d6626058402`.
- Approval outcome: granted by the delegated maintainer through the
  proof-enforced path.
- Event summary: 39 events, including one approval request and grant, with no
  retries or escalations.
- Validation: `npm run check:docs` passed; `git diff --check` passed.
- Out-of-kernel work: Codex inspected the merged plan, repository contracts,
  and current upstream documentation; authored this review; and will perform
  git/PR operations. The kernel governed scope and approval but did not browse,
  edit files, execute validation, or perform git/PR actions.
