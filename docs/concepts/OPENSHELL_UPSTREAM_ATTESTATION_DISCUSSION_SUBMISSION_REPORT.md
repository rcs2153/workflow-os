# OpenShell Upstream Attestation Discussion Submission Report

## 1. Executive Summary

Workflow OS submitted the accepted trustworthy sandbox attestation proposal as
exactly one upstream OpenShell Design Discussion:

- URL: <https://github.com/NVIDIA/OpenShell/discussions/2661>
- category: `Design Discussion`
- submission date: 2026-08-09

The submission asks for general enforcer-owned lifecycle attestation. It does
not ask OpenShell to adopt Workflow OS governance models and does not authorize
provider wiring, OpenShell execution, a fork, or production use.

## 2. Scope Completed

- Rechecked the accepted title, body, official venue, and selected category.
- Created one discussion in `NVIDIA/OpenShell`.
- Verified the create response returned the exact accepted title and body.
- Recorded the stable external URL and immutable content commitments.

## 3. Scope Explicitly Not Completed

- No issue or pull request was opened upstream.
- No second discussion or follow-up comment was created.
- No OpenShell installation, sandbox, or command was executed.
- No Workflow OS provider wiring or Rust behavior changed.
- No access material was stored in repository content.
- No provider mutation, fork, or production claim was introduced.

## 4. Exact Submitted Content

The exact submitted title and body are preserved in immutable Workflow OS
commit `dc8506d6bff589fca1b71f814f08fe3527fe00f9` at:

```text
docs/implementation-plans/openshell-upstream-attestation-discussion-draft.md
```

The title is the content of **Proposed Discussion Title** with wrapped
whitespace normalized to one space. The body is the complete content inside
the **Copy-Ready Discussion Body** Markdown fence, excluding the fence itself.

- title SHA-256:
  `a4df55cf697cfe5095344d23ee1ccf9c5a4114a862f63b5d8e85f1a0e98ec0da`
- body SHA-256:
  `07bb59853ae147a06c2eb0dfc9ccd28a50eccbb744ff80c54e77c9084ac5d925`
- submitted body size: 5,921 UTF-8 bytes

The bounded submitter extracted those values directly from the immutable
accepted document, checked for an existing exact-title discussion to avoid a
duplicate, selected the official `Design Discussion` category, and failed
closed on any title or body mismatch. GitHub's create response matched both
values exactly.

## 5. Architectural Posture

The submission preserves the intended split:

- Workflow OS remains the governance, authority, evidence-obligation, audit,
  and reporting layer.
- OpenShell remains a possible optional containment and execution provider.
- The subsystem that enforces or observes a fact must expose that fact
  authoritatively.
- Provider integration remains blocked until the required operation-bound
  facts can be proven through a supported upstream contract.

## 6. Privacy And Security Posture

No access material, raw output, environment data, host path, private payload,
or unpublished security finding was included. The external body requests
stable references, commitments, bounded counts, and typed posture instead of
copying sensitive runtime payloads into governance state.

## 7. Validation

- External URL inspection: passed through a read-only GraphQL query.
- Exact title/body response comparison: passed.
- External category comparison: passed as `Design Discussion`.
- `npm run check:docs`: passed.
- `git diff --check`: passed.
- Governed event-trail inspection: completed through `phase-close`.

An initial post-submit helper was rejected because it retained a conditional
create mutation when used for verification. It was not executed. A separate
read-only verifier replaced it and performed no mutation. This preserved the
one-submission authority boundary.

## 8. Governed Submission Record

- workflow ID: `dg/release`;
- run ID: `run-1786274058920264000-2`;
- approval ID: `approval/run-1786274058920264000-2/release-approved`;
- approval presentation ID: `presentation/41d7cb5bf3c2e985`;
- approval presentation hash:
  `41d7cb5bf3c2e985242420bd2cfb811da407eb710c9a111462f5df6cb2735136`;
- approval outcome: granted by delegated maintainer with persisted proof; and
- phase status: `Completed`;
- event summary: 39 events, one approval, zero retries, and zero escalations;
- approval-presentation enforcement: proof enforced with one persisted record;
- external action: one GitHub GraphQL `createDiscussion` mutation after an
  exact-title duplicate check; and
- out-of-kernel work: accepted-body extraction, one external GraphQL mutation,
  read-only external verification, documentation edits, validation commands,
  Git operations, and pull-request actions.

## 9. Remaining Limitations

- No upstream response has been received or evaluated.
- OpenShell `v0.0.101` remains insufficient for trusted provider wiring.
- Live sandbox smoke proof remains blocked.
- Upstream interest or acceptance would remain planning input, not runtime
  authorization.

## 10. Recommended Next Phase

Return to the active Workflow OS roadmap. When OpenShell maintainers respond,
run a focused governed response review before changing the provider plan.
