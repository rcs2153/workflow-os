# OpenShell Trustworthy Sandbox Attestation Discussion Draft

Status: accepted by focused maintainer review for one upstream Design
Discussion; not submitted upstream.

Source currency was re-verified on 2026-08-09. Official upstream `main` at
commit `4cb77a900ebd6b789d2b68daaba4830866833b1c` has the same public protobuf and
attestation-relevant observability contracts as the reviewed `v0.0.101` tree.
The only four tree differences concern Podman loopback/TLS behavior and related
guidance. The source-backed claims and questions below remain current; this
re-verification does not authorize submission.

## 1. Purpose And Use

This document contains one bounded architectural discussion draft for NVIDIA
OpenShell maintainers. It translates the accepted Workflow OS source matrix and
provider-neutral proposal into a concise upstream question.

This artifact is not an upstream issue, pull request, commitment, or runtime
implementation. Do not submit it automatically. A human must review the title,
body, source references, tone, and requested scope and make an explicit
submission decision.

OpenShell's gateway interceptor extension point is useful for validating and
applying control-plane mutations. It does not, by itself, provide the
operation-bound runtime attestation, complete observation, or exact cleanup
facts requested below. The discussion therefore asks about authoritative
lifecycle records rather than proposing that interceptors be treated as proof.

## 2. Proposed Discussion Title

```text
Trustworthy sandbox attestation: durable operations, applied-state snapshots,
complete observations, and exact cleanup receipts
```

## 3. Copy-Ready Discussion Body

~~~markdown
Hello OpenShell maintainers,

We are evaluating NVIDIA OpenShell as an optional execution-containment
provider for a local-first workflow governance kernel. Our system decides
whether an operation may execute and which evidence must return; OpenShell
would remain responsible for sandbox lifecycle and the filesystem, process,
network, inference, and platform controls it actually enforces.

We reviewed OpenShell `v0.0.101` at exact source commit
`8ddd98c3dff62619a3963f99ba1e055b67650e72`. The current APIs already expose
useful typed facts, including gateway-owned sandbox identity and resource
version, committed structured policy, effective policy content and revision,
configuration revision, policy source and failure mode, sandbox-reported
policy-load status, driver runtime object identity, and the final exit code for
one connected exec stream.

For a restart-safe execution receipt, we could not identify authoritative API
facts for several other lifecycle boundaries:

1. idempotent sandbox creation and exact request reconciliation after an
   ambiguous transport result;
2. the immutable image digest observed by the compute driver after image
   resolution;
3. a complete typed snapshot of applied controls and degradation for the
   active gateway, driver, and platform;
4. a durable exec-operation identity with request commitment, applied-state
   binding, complete terminal semantics, and restart lookup;
5. an operation-bound observation export manifest with interval watermarks,
   event and drop counts, finalization posture, integrity commitment, and a
   stable reference;
6. resource-version-bound deletion with a durable terminal cleanup receipt;
   and
7. typed capability negotiation for those facts before sandbox creation.

We do not think callers should infer those facts from requested configuration,
human CLI text, a live stream alone, or partial log records. The component that
enforces or observes a fact should expose it authoritatively and bind it to
stable resource and operation identities.

Would a lifecycle model like the following align with OpenShell's direction?

```text
SandboxCreationRecord
  -> SandboxAppliedStateSnapshot
  -> SandboxExecOperation
  -> ObservationExportManifest
  -> SandboxDeletionOperation
```

The names are illustrative. Some of these semantics may fit better as fields
or operations on existing resources.

The properties we care about are:

- mutations accept an opaque idempotency key plus a canonical request
  commitment;
- matching retries return the original durable result and conflicting reuse
  fails closed;
- canonical structured input policy is related machine-readably to the final
  composed and loaded policy;
- the driver reports the immutable image and effective control posture it
  actually applied;
- exec is a durable operation rather than only a transient stream;
- observation completeness is reported by the observing subsystem, including
  known drops and finalization state;
- deletion targets an exact resource version and remains queryable after the
  sandbox disappears; and
- capability output reflects the active gateway, driver, and platform rather
  than a compile-time support claim.

For one narrow motivating use case, imagine a fixed no-write CI/governance
operation in a sandbox with no credentials and default-deny networking. A
caller wants to prove that the reviewed policy and immutable image were
applied, the fixed operation reached a typed terminal outcome, one deliberate
egress denial appears inside a complete observation interval, and the exact
sandbox reached terminal cleanup. Raw stdout, stderr, policy, environment, and
security logs should remain outside the caller's governance state; stable
references, commitments, bounded counts, and typed postures are sufficient.

We would value guidance on these questions:

1. Which of these semantics fit existing OpenShell resources, and which merit
   first-class durable resources?
2. Where should creation, operation, observation, and deletion records live so
   reconciliation survives gateway or client restart within a declared
   retention period?
3. Can the driver expose resolved immutable image identity and a portable
   applied-control/degradation snapshot across supported platforms?
4. Can the observation subsystem finalize an operation-bound manifest with
   watermarks, event and drop counts, integrity commitment, and retrieval
   posture?
5. How should canonical request commitments handle secret-bearing general exec
   inputs without exposing sensitive values as retrievable metadata?
6. Would you prefer one cohesive attestation API direction followed by smaller
   independently useful issues, or a different incremental sequence?

We are not asking OpenShell to adopt workflow approvals, evidence ledgers,
reports, or any Workflow OS-specific model. We are also not proposing a fork,
a custom runtime distribution, arbitrary provider writes, or a production
security claim. Our intent is to understand whether general-purpose,
enforcer-owned lifecycle attestation belongs upstream and where maintainers
would want those boundaries.

Source references for the reviewed pin:

- [OpenShell source at `v0.0.101` commit](https://github.com/NVIDIA/OpenShell/tree/8ddd98c3dff62619a3963f99ba1e055b67650e72)
- [public gateway API schema](https://github.com/NVIDIA/OpenShell/blob/8ddd98c3dff62619a3963f99ba1e055b67650e72/proto/openshell.proto)
- [sandbox policy API schema](https://github.com/NVIDIA/OpenShell/blob/8ddd98c3dff62619a3963f99ba1e055b67650e72/proto/sandbox.proto)
- [compute-driver API schema](https://github.com/NVIDIA/OpenShell/blob/8ddd98c3dff62619a3963f99ba1e055b67650e72/proto/compute_driver.proto)
- [OCSF JSON export documentation](https://docs.nvidia.com/openshell/latest/observability/ocsf-json-export)

Thank you for any direction on fit, preferred API shape, or work already in
progress that addresses these gaps.
~~~

## 4. Maintainer Review Checklist

Before any submission, a human reviewer must confirm:

- the selected upstream venue and its contribution/discussion rules;
- the `v0.0.101` source claims and links remain accurate;
- the draft does not imply that OpenShell or Workflow OS is production-ready;
- candidate names are clearly non-final;
- the request is useful to general OpenShell consumers;
- no Workflow OS-specific model is pushed into OpenShell;
- no access material, private payload, host path, or unpublished security
  finding is disclosed;
- the five-record lifecycle is presented as a discussion, not a demand;
- maintainers are invited to choose resource boundaries and sequencing; and
- external submission is explicitly approved.

This checklist was completed by the focused maintainer review. The selected
venue is OpenShell's Design Discussion category. The review authorizes one
separately governed submission of the copy-ready body and does not authorize a
second discussion, issue, pull request, implementation, or provider wiring.

## 5. Submission And Follow-Up Boundary

If a human approves submission:

1. confirm the preferred OpenShell discussion or issue surface;
2. recheck the current upstream release and contribution guidance;
3. submit exactly one architectural discussion;
4. record the external URL and immutable submitted body in a governed evidence
   record;
5. do not open follow-up issues or patches until upstream responds; and
6. treat upstream feedback as planning input, not runtime authorization.

Submission would not authorize Workflow OS provider wiring, OpenShell
installation or execution, access material, writes, schemas, examples, a fork,
or a production claim.

## 6. Governed Planning Evidence

- workflow ID: `dg/d`;
- run ID: `run-1786269770989982000-2`;
- approval ID: `approval/run-1786269770989982000-2/planning-approved`;
- approval presentation ID: `presentation/e7f0457a5438ecce`;
- approval outcome: granted by delegated maintainer with persisted presentation
  proof;
- event summary: 39 events, one approval, zero retries, and zero escalations;
- validation summary: `npm run check:docs` passed and `git diff --check`
  passed;
- provider/upstream activity: none;
- out-of-kernel work: source-backed drafting, documentation edits,
  documentation validation, Git operations, and GitHub pull-request actions.

Required phase-close validation:

- `npm run check:docs`: passed;
- `git diff --check`: passed;
- governed event-trail inspection: completed through `phase-close`; and
- changed-surface inspection: only documentation changed, excluding local
  untracked `.workflow-os/` state.

## 7. Recommended Next Phase

One separately governed external-engagement phase that submits the accepted
body to the OpenShell Design Discussion category and records the stable URL and
exact submitted content.

Do not submit automatically from this review phase. Do not open a related issue
or pull request, and do not begin provider wiring while waiting for an upstream
response.
