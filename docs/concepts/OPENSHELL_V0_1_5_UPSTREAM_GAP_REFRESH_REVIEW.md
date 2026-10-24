# OpenShell v0.1.5 Upstream Gap Refresh Review

## 1. Executive Verdict

**Refresh and follow-up draft accepted; proceed to one separately governed
comment on existing OpenShell Discussion #2661.**

The refreshed gap record accurately credits stable `v0.1.5` progress while
preserving the selected Docker boundary's remaining authoritative-fact gaps.
The follow-up is concise relative to the original proposal, source-backed,
provider-neutral, privacy-safe, and clear that Workflow OS is not authorized
to wire or execute the provider.

This verdict authorizes only one later governed comment containing the exact
accepted body. It does not itself post externally or authorize an issue, pull
request, second discussion, provider wiring, transport, smoke, execution,
access material, fork, or production claim.

## 2. Scope Verification

The refresh stayed within its approved documentation boundary. It updated the
existing proposal and discussion artifact, preserved the original submitted
body, added one separate follow-up draft, advanced the roadmap, and created a
phase report.

It did not mutate upstream state, install or execute OpenShell, invoke Cargo,
change Rust, add a transport/provider, add schemas or CLI behavior, expose
access material, enable writes, fork OpenShell, or change release posture.

## 3. Source Fidelity Assessment

The follow-up correctly identifies stable `v0.1.5` at
`bdb956f2cd65a635f1294fccf1d138d263bf650f` and the selected Docker-on-Apple-
Silicon evaluation boundary.

Focused inspection confirms the cited facts:

- create and exec request IDs provide bounded durable admission behavior;
- selected resource-version, desired-state-operation, endpoint-status,
  driver-operation, and deletion fields exist;
- `DriverSandboxStatus` defines `resolved_identity` and `fence_evidence`;
- Docker production status constructors leave those fields at defaults;
- admitted exec retries return `REQUEST_STREAM_UNAVAILABLE` without terminal
  replay;
- official lifecycle guidance documents unresolved owner loss;
- official OCSF guidance documents best-effort and loss postures; and
- deletion completion does not prove downstream platform garbage collection.

No source blocker was found.

## 4. Improvement Credit And Tone Assessment

The draft leads with concrete upstream improvements before discussing gaps.
It avoids suggesting that `v0.1.5` made no progress and asks whether the
remaining selected-boundary findings align with intended semantics.

The wording is collaborative rather than prescriptive. It asks whether
focused issues or patches would be useful only after maintainers advise on
preferred boundaries. This is appropriate for an existing design discussion.

## 5. Docker Projection Assessment

The selected-driver claim is precise. The Docker crate resolves workload
identity during provisioning but does not project it through the production
status objects that an adapter can consume. The draft does not overgeneralize
this behavior to every OpenShell driver or platform.

Calling out schema-versus-production projection is useful upstream feedback,
not a private vulnerability disclosure. The claim is supported entirely by
public stable source.

## 6. Reconciliation Assessment

The follow-up correctly separates:

- launch fencing from terminal-result recovery;
- pending-operation ownership from owner-loss resolution; and
- fail-closed blocking from successful reconciliation.

This preserves the Workflow OS invariant that uncertainty after possible
execution cannot be relabeled as not-started or successful.

## 7. Observation And Cleanup Assessment

The draft correctly treats structured OCSF output as useful but incomplete.
It names queue loss, shutdown loss, and collector partial rejection without
copying raw observations or alleging a vulnerability.

The cleanup request is also proportionate. Exact deletion target and typed
gateway outcome are credited, while exact-generation platform reclamation,
descendant termination, credential purge, and degradation remain the terminal
receipt requirements.

## 8. Non-Duplication Assessment

The draft does not repost the complete original resource model. It summarizes
only the stable delta, selected-driver findings, strongest next increments,
and two focused questions.

Posting it as one comment on Discussion #2661 is preferable to opening a new
discussion, issue, or pull request before maintainers respond.

## 9. Privacy And Security Assessment

The accepted draft contains public source references and architectural facts
only. It includes no token, credential, access material, private payload, host
path, raw command output, raw observation payload, environment value, or
unpublished vulnerability.

The draft continues to request commitments, typed posture, bounded counts,
and references rather than sensitive runtime content.

## 10. Fork And Product-Boundary Assessment

The no-fork posture remains correct. The draft presents OpenShell as an
optional containment provider and Workflow OS as the governance and evidence-
acceptance layer. It does not ask OpenShell to implement Workflow OS approvals,
evidence-ledger types, reports, or workflow semantics.

Provider wiring remains blocked until a selected stable contract exposes all
mandatory facts and passes a separate live proof.

## 11. Accepted Content Commitment

The exact follow-up comment body accepted by this review is the UTF-8 content
inside section 4.1 of the discussion-draft document, excluding the Markdown
fence delimiters.

- body SHA-256:
  `4541ea7d05380112d86e45f78c225fd98ddf278531031452eb928289a6f700a8`;
- body size: 4,503 UTF-8 bytes; and
- target: existing
  [OpenShell Discussion #2661](https://github.com/NVIDIA/OpenShell/discussions/2661).

Any byte change requires another review. A submitter must reject an existing
identical comment, compare the returned body exactly, and preserve the stable
comment URL or ID.

## 12. Blockers

None for accepting the refresh and preparing one separately governed external
comment.

Provider integration blockers remain exactly those accepted in the `v0.1.5`
matrix. Upstream engagement does not close them.

## 13. Non-Blocking Follow-Ups

- Preserve the original submitted body and its historical commitments.
- If maintainers respond, review their exact statements as planning input.
- Do not translate external researcher comments into upstream commitments.
- Rerun the complete matrix against any later stable implementation change.
- Retain a separately governed live proof after source contracts become
  sufficient.

## 14. Recommended Next Phase

Run one governed external-engagement phase to post exactly the accepted
follow-up body as a comment on Discussion #2661.

The submitter must verify the target, detect an existing identical comment,
compare the returned body exactly, record the stable external identity and
content commitment, and fail closed on mismatch. It must not create a second
discussion, issue, pull request, code change, provider, transport, smoke,
execution, access material, fork, or release change.

## 15. Validation

- exact stable source and selected Docker implementation: inspected;
- current public Discussion #2661 thread: inspected read-only;
- follow-up body commitment and byte length: computed;
- `npm run check:docs`: required at phase close;
- `git diff --check`: required at phase close; and
- Rust formatting, linting, and tests: not required because this review changes
  documentation only and no Rust or runtime behavior.

## 16. Governed Review Record

- Workflow ID: `dg/review`.
- Run ID: `run-1792852068059571000-2`.
- Approval ID:
  `approval/run-1792852068059571000-2/review-scope-approved`.
- Approval presentation ID: `presentation/4cb404b15225407b`.
- Approval presentation content hash:
  `4cb404b15225407beb5a8e3e0b042185fa453390d10b274435b6e39bde6f029e`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact focused review scope.
- External activity: read-only inspection only; no comment was posted.
- Out-of-kernel work: the delegated maintainer inspected public source and the
  current discussion, reviewed documentation, computed the draft commitment,
  authored this review, and ran validation. The kernel governed scope and
  approval but did not inspect files, post externally, edit documentation, or
  run validation.

- Phase status: `Completed`.
- Event summary: 39 events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, six successful skill
  invocations, and one terminal run-completed event.
- Retries: 0.
- Escalations: 0.
- Approval presentation enforcement: `proof_enforced`; one presentation
  record and its event marker were present.
- Validation summary: `npm run check:docs` and `git diff --check` passed.
- Skipped checks: Rust formatting, linting, and tests were not run because the
  phase changed documentation only.
