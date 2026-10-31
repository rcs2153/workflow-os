# OpenShell v0.1.5 Follow-Up Submission Report

## 1. Executive Summary

The accepted OpenShell `v0.1.5` follow-up was posted exactly once as a comment
on existing Discussion #2661. The stable comment identity is:

- URL:
  <https://github.com/NVIDIA/OpenShell/discussions/2661#discussioncomment-19310244>;
- GitHub GraphQL ID: `DC_kwDORYG5fs4BJqak`;
- author: `rcs2153`; and
- created at: `2026-10-31T07:15:39Z`.

The first browser-editor submission transformed Markdown structure. Exact
verification caught the mismatch before the phase could be reported as
complete. The same comment was corrected in place using plain-text paste; no
second comment was created.

GitHub's returned raw body uses CRLF line endings. After canonical LF
normalization, its content is byte-for-byte equal to the exact body accepted by
focused review. This report preserves both the raw transport commitment and
the canonical accepted-content commitment rather than mislabeling the raw
response as identical.

## 2. Scope Completed

- Re-verified the target as existing OpenShell Discussion #2661.
- Confirmed that no identical follow-up comment already existed.
- Recomputed the accepted source body's byte length and SHA-256 commitment.
- Attempted one repository-scoped API mutation, which failed before comment
  creation because the token could not mutate the upstream discussion.
- Submitted one comment through the authenticated browser boundary.
- Detected that rich-Markdown paste had transformed the accepted body.
- Corrected the same comment in place through plain-text paste.
- Compared the returned body with the accepted source after explicit LF
  canonicalization.
- Recorded stable external identity, raw transport commitment, canonical
  content commitment, and governance evidence.

## 3. Scope Explicitly Not Completed

This phase did not:

- create a second comment, discussion, issue, or pull request;
- add a compatibility transport or run an OpenShell live smoke;
- install, build, start, or execute OpenShell or Cargo;
- wire an execution provider;
- add access material, a fork, runtime code, schemas, SDK behavior, CLI
  behavior, SideEffects, or another provider mutation;
- claim that OpenShell now satisfies the selected provider boundary; or
- treat third-party comments or the absence of a maintainer response as
  upstream acceptance.

## 4. Accepted Source Commitment

The reviewed source is the UTF-8 body inside section 4.1 of
[OpenShell Trustworthy Sandbox Attestation Discussion Draft](../implementation-plans/openshell-upstream-attestation-discussion-draft.md),
excluding the Markdown fence delimiters.

- accepted source size: 4,503 UTF-8 bytes;
- accepted source SHA-256:
  `4541ea7d05380112d86e45f78c225fd98ddf278531031452eb928289a6f700a8`;
  and
- target: existing OpenShell Discussion #2661.

These values match the focused review's accepted content commitment.

## 5. Submission And Correction Evidence

Before mutation, read-only inspection found two existing comments and no exact
duplicate of the accepted body.

The repository-scoped GitHub token could read the public discussion but the
GraphQL mutation failed with a provider authorization error. Follow-up
inspection confirmed that no comment was created by that failed attempt.

The authenticated browser then created one comment. Rich-Markdown paste
changed list and code formatting, so exact comparison failed. The phase did
not accept that transformed body. The same stable comment was edited in place
using plain-text paste, preserving the single-comment boundary.

After correction, GitHub's API returned:

- raw returned size: 4,577 UTF-8 bytes;
- raw returned SHA-256:
  `762ed0982d0fd2b822c905edd6bdb96c53fbcc643b65cfdd473dc1ec233a4c43`;
- carriage returns: 74, one for each line ending; and
- canonicalization rule: replace every CRLF line ending with LF without any
  other transformation.

The canonical LF result is:

- canonical size: 4,503 UTF-8 bytes;
- canonical SHA-256:
  `4541ea7d05380112d86e45f78c225fd98ddf278531031452eb928289a6f700a8`;
  and
- exact equality with the accepted source body: passed.

The returned content therefore differs only in transport line endings. No
word, Markdown token, link, ordering, or other content changed after the
in-place correction.

## 6. Review Verdict

The focused follow-up review accepts the posted comment.

Canonical LF normalization is appropriate for this text-only GitHub boundary
because the provider deterministically returned CRLF line endings while
preserving every accepted content byte after normalization. The raw response
commitment remains recorded so this normalization cannot conceal another
provider transformation.

Future browser-based submissions of committed Markdown should use plain-text
paste, compare the returned provider body after an explicitly documented line-
ending normalization, and preserve both raw and canonical commitments.

## 7. Security And Privacy Summary

The submitted comment contains public source references and architectural
facts only. It includes no token, credential, access material, private
payload, host path, raw command output, environment value, unpublished
vulnerability, or sensitive runtime observation.

The failed token mutation and browser session produced no access material in
repository files or this report.

## 8. Validation Summary

Completed at phase close:

- `npm run check:docs` under the repository-pinned Node 20 runtime: passed;
- `git diff --check`: passed;
- changed-surface inspection: passed; and
- governed release, focused-review, and status-correction event/proof
  inspection: passed.

Rust formatting, linting, and tests are intentionally not required because
this phase records documentation and external-action evidence only; it changes
no Rust or runtime behavior.

## 9. Governed Phase Record

External submission phase:

- workflow ID: `dg/release`;
- run ID: `run-1792852800889620000-2`;
- approval ID:
  `approval/run-1792852800889620000-2/release-approved`;
- approval presentation ID: `presentation/e2fd52b166e21bca`;
- approval presentation content hash:
  `e2fd52b166e21bcadc9824f6ae4aef9152f7627a9442d3fd3ec85b65e442aab2`;
- approval outcome: granted under standing delegated-maintainer authority for
  the exact one-comment external-action scope;
- phase status: `Completed`;
- events: 39 total;
- approvals: 1;
- retries: 0;
- escalations: 0; and
- approval-presentation enforcement: proof-enforced with one matching
  presentation record and event marker.

Line-ending focused review:

- workflow ID: `dg/review`;
- run ID: `run-1793431008931221000-2`;
- approval ID:
  `approval/run-1793431008931221000-2/review-scope-approved`;
- approval presentation ID: `presentation/5219ff61773b76ac`;
- approval presentation content hash:
  `5219ff61773b76acc5d761839bb94f90e4b2eb05b73996c6e5168e13f7554aff`;
- approval outcome: granted under standing delegated-maintainer authority for
  the exact returned-content review scope;
- phase status: `Completed`;
- events: 39 total;
- approvals: 1;
- retries: 0;
- escalations: 0; and
- approval-presentation enforcement: proof-enforced with one matching
  presentation record and event marker.

Both runs contained one `RunCreated`, one `RunValidated`, one `RunStarted`, six
`StepScheduled`, eight `PolicyDecisionRecorded`, one `ApprovalRequested`, one
`ApprovalGranted`, one `RunResumed`, six each of
`SkillInvocationRequested`, `SkillInvocationStarted`, and
`SkillInvocationSucceeded`, and one `RunCompleted` event.

Status-correction documentation phase:

- workflow ID: `dg/d`;
- run ID: `run-1793431126896478000-2`;
- approval ID:
  `approval/run-1793431126896478000-2/planning-approved`;
- approval presentation ID: `presentation/ab03796501d686b6`;
- approval presentation content hash:
  `ab03796501d686b61abe7fdcffba96e467ec0a0421d11bad426911db0d611cf2`;
- approval outcome: granted under standing delegated-maintainer authority for
  the exact post-submission status correction;
- phase status: `Completed`;
- events: 39 total;
- approvals: 1;
- retries: 0;
- escalations: 0; and
- approval-presentation enforcement: proof-enforced with one matching
  presentation record and event marker.

## 10. Out-Of-Kernel Work Disclosure

The delegated maintainer inspected the public discussion, computed content
commitments, attempted the API mutation, used the authenticated browser,
corrected the single comment, queried the returned body, compared raw and
canonical content, authored this report and roadmap update, and ran validation.

The kernel governed phase scope, approval, and event history. It did not
perform the external mutation, edit repository files, compute provider-body
commitments, or run validation.

## 11. Remaining Known Limitations

- No OpenShell maintainer response has been evaluated.
- The accepted comment does not close any provider integration blocker.
- Stable `v0.1.5` remains insufficient for the selected Docker provider
  boundary.
- A later source reassessment and selected-platform live proof remain required
  after upstream contracts expose the mandatory authoritative facts.

## 12. Recommended Next Phase

Run one governed roadmap queue-selection phase. OpenShell provider wiring must
remain blocked while awaiting authoritative upstream changes or maintainer
guidance, so the next executable phase should be selected from another mature
runtime lane using current accepted evidence rather than stale historical
`next phase` text.
