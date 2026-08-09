# OpenShell Upstream Attestation Discussion Draft Review

## 1. Executive Verdict

Draft accepted; approve one upstream OpenShell Design Discussion submission.

The draft is source-current, provider-neutral, privacy-bounded, and explicit
that Workflow OS is evaluating an optional execution-containment provider
rather than transferring governance semantics into OpenShell. Submission must
occur through a separately governed external-engagement phase.

## 2. Scope Verification

The review stayed within the approved documentation and submission-decision
scope. It did not submit upstream, change Rust code, wire a provider, execute
OpenShell, introduce access material, enable writes, create a fork, or make a
production claim.

## 3. Source Currency

Official upstream references were rechecked on 2026-08-09:

- OpenShell `main` resolved to
  `4cb77a900ebd6b789d2b68daaba4830866833b1c`;
- tag `v0.0.101` resolved to
  `8ddd98c3dff62619a3963f99ba1e055b67650e72`;
- the public protobuf and attestation-relevant observability boundaries remain
  as classified by the accepted evidence-sufficiency matrix; and
- the existing gateway interceptor extension point can validate or apply
  control-plane mutations but does not establish complete operation-bound
  runtime attestation, observations, or cleanup proof.

The draft's source claims therefore remain current.

## 4. Venue And Contribution Posture

The selected venue is the official OpenShell Discussions **Design Discussion**
category. OpenShell's contribution guidance permits design proposals and keeps
assessment, disposition, sequencing, and ownership with upstream maintainers.

One discussion is the correct first engagement. No issue or pull request is
authorized before maintainers respond.

## 5. Architectural Assessment

The draft preserves the correct boundary:

- Workflow OS decides whether work may execute and which evidence obligations
  apply;
- OpenShell would enforce the sandbox controls it owns;
- authoritative facts should come from the component that enforces or observes
  them; and
- stable operation and resource identities must bind requested state, applied
  state, terminal outcome, observations, and cleanup.

The proposal is useful to general OpenShell consumers and does not ask upstream
to adopt Workflow OS approvals, reports, evidence-ledger types, or workflow
models.

## 6. Privacy And Security Assessment

The draft requests bounded references, commitments, counts, and typed posture.
It explicitly keeps raw output, policy, environment data, and security logs out
of governance state. It contains no access material, private payload, host
path, unpublished vulnerability, or production-security assertion.

## 7. Fork Assessment

A fork is not justified. Workflow OS should remain responsible for governance
and evidence obligations while OpenShell owns containment and runtime-security
maintenance. A fork threshold should be reconsidered only if upstream rejects
or cannot support the general enforcer-owned attestation hooks required for a
trustworthy optional provider.

## 8. Submission Boundary

The next phase may submit exactly one discussion using the accepted title and
copy-ready body. It must record the stable external URL and exact submitted
content. It must not open additional issues or patches, install or execute
OpenShell, wire a provider, expose access material, enable writes, or infer
runtime authorization from the submission.

## 9. Validation

- `npm run check:docs`: passed;
- `git diff --check`: passed; and
- governed event-trail inspection: completed through `phase-close`.

## 10. Remaining Limitations

- No upstream response exists yet.
- OpenShell `v0.0.101` remains insufficient for trusted provider wiring.
- Live sandbox execution and integrated evidence proof remain blocked.
- Discussion acceptance would be planning input, not execution authorization.

## 11. Recommended Next Phase

Run one separately governed external-engagement phase to submit the accepted
Design Discussion and preserve the stable URL and exact submitted body.

## 12. Governed Review Record

- workflow ID: `dg/review`;
- run ID: `run-1786273350071863000-2`;
- approval ID:
  `approval/run-1786273350071863000-2/review-scope-approved`;
- approval presentation ID: `presentation/765fad4950198412`;
- approval presentation hash:
  `765fad49501984124250325a49be5db46cd68b408a0d5fc1fda088676553523c`;
- approval outcome: granted by delegated maintainer;
- phase status: `Completed`;
- event summary: 39 events, one approval, zero retries, and zero escalations;
- approval-presentation enforcement: proof enforced with one persisted record;
- external activity: none; and
- out-of-kernel work: upstream source and contribution-guidance inspection,
  documentation edits, validation commands, Git operations, and pull-request
  actions.
