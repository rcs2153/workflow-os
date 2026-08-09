# OpenShell Current-Upstream Attestation Re-verification Review

## 1. Executive Verdict

Current upstream re-verified; provider wiring remains blocked.

NVIDIA OpenShell remains the recommended optional execution-containment
provider boundary for Workflow OS. The reviewed release remains `v0.0.101` at
source commit `8ddd98c3dff62619a3963f99ba1e055b67650e72`. Official upstream `main`
was re-checked at commit `4cb77a900ebd6b789d2b68daaba4830866833b1c` on
2026-08-09.

An exact Git-tree comparison found only four differences between those refs:

- Podman Machine loopback-listener and TLS-authority handling;
- its focused tests;
- the corresponding compute-driver documentation; and
- maintainer debugging guidance.

The public protobuf schemas and attestation-relevant observability
documentation are identical. Current upstream therefore does not change the
accepted evidence-sufficiency matrix, close the provider-wiring blockers, or
justify a fork.

## 2. Governed Review Scope

This phase:

- resolved the latest official upstream release tag and source commit;
- resolved official upstream `main` to an exact commit;
- compared the two complete Git trees rather than inferring API changes from
  version numbers or documentation pages;
- re-read the public sandbox, policy, driver, operation, logging, and OCSF
  surfaces against every required Workflow OS attestation fact;
- refreshed the accepted matrix, discussion draft, and roadmap status; and
- preserved the optional-provider, no-fork, and no-live-execution boundaries.

It did not:

- install, build, start, or execute OpenShell;
- create a gateway, sandbox, provider, credential, or network connection;
- wire `OpenShellNoWriteClient` or change Workflow OS Rust code;
- submit an issue, discussion, pull request, or other upstream write;
- add schemas, SDK behavior, CLI behavior, examples, or defaults;
- authorize provider writes, arbitrary commands, or production use; or
- claim that source re-verification replaces the required live smoke proof.

## 3. Exact Source Boundary

The official refs inspected were:

| Boundary | Exact ref |
| --- | --- |
| Latest release tag | `v0.0.101` |
| Release source commit | `8ddd98c3dff62619a3963f99ba1e055b67650e72` |
| Official upstream `main` | `4cb77a900ebd6b789d2b68daaba4830866833b1c` |

The current-main and release trees differ only in:

- `.agents/skills/debug-openshell-cluster/SKILL.md`;
- `crates/openshell-core/src/forward.rs`;
- `docs/reference/sandbox-compute-drivers.mdx`; and
- `e2e/with-podman-gateway.sh`.

No files under `proto/`, `docs/observability/`, the policy service, sandbox
supervisor, or OCSF implementation differ between the compared trees. This is
a source-contract statement only. It does not prove release-asset integrity,
installed binary identity, a selected driver artifact, platform behavior, or
runtime image identity.

## 4. Re-verified Facts

The current public contract continues to expose useful authoritative facts
within narrow boundaries:

- gateway-owned sandbox ID, workspace, timestamps, and resource version;
- gateway and selected-driver name/version self-report;
- committed structured policy and composed effective-policy payload;
- deterministic policy hash, version, source, configuration revision, and
  failure mode;
- persisted sandbox policy revision and sandbox-reported load status;
- driver-observed runtime object identity and lifecycle conditions; and
- the terminal exit code for one connected exec stream.

These facts strengthen a future adapter. They do not combine into a durable,
restart-safe execution receipt. Requested policy and image values remain
intent unless the enforcing or observing component reports applied state.

## 5. Blocking Fact Assessment

| Required fact family | Current upstream result | Verdict |
| --- | --- | --- |
| Restart-safe create identity | `CreateSandboxRequest` has no idempotency key or provider-owned request commitment | Blocking |
| Driver-observed immutable image | Public and driver status expose requested image and runtime object identity, not the resolved image digest actually running | Blocking |
| Complete applied-control state | Policy revision/load facts exist, but there is no complete typed cross-driver snapshot of applied filesystem, process, network, degradation, and skipped-control posture | Blocking |
| Durable operation outcome | Exec remains a transient stream without operation ID, canonical request commitment, applied-state binding, restart lookup, or complete terminal reason | Blocking |
| Complete operation observations | OCSF JSONL records are structured, but there is no operation-bound manifest with watermarks, flush state, event/drop counts, digest, and completeness posture | Blocking |
| Exact cleanup | Public delete is name/workspace based and returns a boolean; it is not resource-version-bound and has no durable terminal cleanup receipt | Blocking |
| Typed attestation capabilities | Driver capability snapshots report name and version, not the required attestation and enforcement capabilities of the active gateway/driver/platform combination | Blocking |

The gateway log stream remains explicitly bounded, nonpersistent, and able to
drop events. Sandbox-resident OCSF JSONL may contain complete individual event
records, but individual record completeness is not interval completeness.
Neither surface may be promoted into an operation-bound proof by Workflow OS.

## 6. Draft Currency Assessment

The copy-ready upstream discussion remains accurate. Its seven requests map
directly to the blockers above:

1. idempotent creation and exact reconciliation;
2. driver-observed immutable image identity;
3. complete typed applied-control and degradation state;
4. durable operation identity and terminal semantics;
5. complete operation-bound observation manifests;
6. resource-version-bound deletion and cleanup receipts; and
7. typed capability negotiation.

The discussion remains a draft. This review does not submit it or authorize
automatic submission. Human review of tone, source links, requested scope, and
the external engagement decision is still required.

## 7. Fork Assessment

A fork is not justified.

The missing surfaces are generally useful sandbox lifecycle and attestation
primitives, not Workflow OS-specific governance concepts. Workflow OS should
first seek upstream alignment and keep its own provider contract neutral. A
fork would transfer container, VM, Kubernetes, filesystem, network, process,
credential, release, platform, and vulnerability-management responsibility to
this project without solving the product boundary more cleanly.

The accepted fork threshold remains unchanged: consider it only after a real
integration proves that essential authoritative hooks are structurally
incompatible with upstream direction and upstream contribution paths have
failed.

## 8. Privacy And Error Posture

Future provider projection must remain payload-minimizing. Workflow OS Core may
accept stable IDs, commitments, revisions, typed postures, bounded counts,
timestamps, and governed evidence references. It must not ingest raw policy,
OCSF records, stdout, stderr, environment values, credentials, provider
payloads, host paths, endpoints, or upstream error bodies.

Source inspection used only public upstream material. No access material or
runtime payload was introduced into the repository.

## 9. Validation And Evidence

The phase used:

- official `git ls-remote` release and branch refs;
- a shallow clone of official NVIDIA OpenShell `main`;
- an exact fetch of tag `v0.0.101`;
- Git-tree and focused source comparisons;
- direct inspection of public protobuf and observability contracts;
- `npm run check:docs`; and
- `git diff --check`.

Governed phase identity:

- workflow ID: `dg/review`;
- run ID: `run-1786270674095472000-2`;
- approval ID:
  `approval/run-1786270674095472000-2/review-scope-approved`;
- approval presentation ID: `presentation/34dbe2a9d18cf288`;
- approval outcome: granted by the delegated maintainer; and
- out-of-kernel work: public upstream source resolution and inspection, plus
  these documentation edits.

Phase-close result:

- phase status: `Completed`;
- events: 39 total;
- approvals: 1;
- retries: 0;
- escalations: 0;
- event kinds: 1 `RunCreated`, 1 `RunValidated`, 1 `RunStarted`, 6
  `StepScheduled`, 8 `PolicyDecisionRecorded`, 1 `ApprovalRequested`, 1
  `ApprovalGranted`, 1 `RunResumed`, 6 `SkillInvocationRequested`, 6
  `SkillInvocationStarted`, 6 `SkillInvocationSucceeded`, and 1
  `RunCompleted`; and
- approval-presentation enforcement: one proof-enforced presentation record
  with a matching event marker.

Validation result:

- `npm run check:docs`: passed;
- `git diff --check`: passed; and
- Rust format, lint, and test commands: not run because the approved phase
  changed documentation only and no Rust or runtime behavior.

## 10. Remaining Limitations

- No live gateway, driver, supervisor, or sandbox was exercised.
- No release binary or container digest was verified.
- No platform-specific enforcement was tested.
- No upstream maintainer feedback has been received.
- No authoritative API additions have been released.
- The existing CLI transport remains compatibility evidence only and must not
  be wired as an execution provider.

## 11. Recommended Next Phase

Human review and an explicit upstream discussion submission decision.

If submission is approved, use a separately governed external-engagement phase
that records the exact submitted title, body, destination, and resulting stable
reference. Do not implement provider wiring while the required facts remain
unavailable. If upstream feedback identifies an existing equivalent contract,
reassess it through a new bounded source and live-proof phase before changing
the provider-readiness verdict.
