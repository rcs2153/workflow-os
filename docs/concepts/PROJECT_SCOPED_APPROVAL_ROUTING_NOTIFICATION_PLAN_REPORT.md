# Project-Scoped Approval Routing And Bounded Notification Plan Report

## 1. Executive Summary

The next collaborative milestone is now phase-ready. The plan composes immutable
workflow ownership/escalation metadata with deployment-owned project capability
grants, then exposes the result through a bounded pull-based approval inbox.

The decisive invariant is that metadata routes work but never grants authority.
An effective recipient must be both named by immutable workflow metadata and
already authorized to decide approvals for the exact project.

## 2. Scope Completed

- inspected ownership, escalation, approval, project-scope, principal-grant,
  proportional-governance, disclosure-delivery, event, audit, and report
  boundaries;
- defined deterministic route resolution and unresolved posture;
- defined the metadata-versus-authority invariant;
- defined a payload-free notification posture and pull-based inbox boundary;
- defined proportional-governance composition;
- defined privacy, failure, sequencing, and test requirements;
- selected the next implementation slice.

Parallel read-only architecture reviews confirmed that Core already has the
required ownership, approval, escalation, project-grant, disclosure-receipt,
event, audit, and report vocabulary. They also identified the later durable
boundary: route persistence must be create-only before decision enforcement,
and a real external send must be modeled as a governed SideEffect rather than a
special notification bypass.

## 3. Scope Explicitly Not Completed

No Core model, resolver, hosted endpoint, persistence, notification delivery,
email, Slack, paging, tickets, dynamic identity, RBAC, IdP, schema change,
provider write, workflow execution change, or release-posture change was
implemented.

## 4. Architecture Decision

The plan reuses:

- immutable workflow `OwnershipMetadata` as the candidate source;
- `HostedPrincipalBinding` and exact-project `ApprovalDecide` grants as the
  authority source;
- validated `ApprovalRequest` and `EscalationRecord` subjects;
- the disclosure-delivery claim boundary that surface acceptance does not prove
  human observation or acknowledgement;
- project-scoped access audit posture.

It does not reinterpret ownership metadata as authorization or misuse the
existing visible-proceed disclosure contract as an approval-notification
record.

## 5. Recommended First Implementation

Implement a pure Core project approval-route model and deterministic resolver.
It should return a routed or bounded unresolved result from explicit immutable
metadata, approval subject, project scope, and authority-view inputs.

After focused review, integrate it into a principal-filtered collaborative
hosted approval inbox. This sequencing keeps the first change reviewable while
committing the roadmap to a runnable consumer immediately afterward.

## 6. Privacy And Security Summary

- exact project capability remains the only decision-authority source;
- no cross-project or arbitrary-principal fallback is allowed;
- inbox summaries are payload-free and reference-only;
- no approval reasons, source contents, evidence payloads, command output,
  provider payloads, paths, credentials, or contact details are copied;
- errors and Debug output disclose bounded posture only;
- route availability does not claim delivery, observation, acknowledgement, or
  approval.

## 7. Validation

Completed:

- `npm run check:docs`: passed;
- `git diff --check`: passed;
- governed phase-close event inspection: passed.

## 8. Remaining Limitations

- routing is not implemented;
- the hosted approval inbox is not implemented;
- no external notification delivery exists;
- principals remain pre-provisioned deployment configuration;
- unresolved-route operator handling and reassignment remain open design
  questions;
- enterprise directory and administration support remain deferred.

## 9. Recommended Next Phase

Project-scoped approval-route Core model and deterministic resolver
implementation, followed immediately by focused review and the hosted inbox
consumer.

## 10. Governed Phase Record

- dogfood workflow: `dg/d`;
- run ID: `run-1786669384200709000-2`;
- approval ID:
  `approval/run-1786669384200709000-2/planning-approved`;
- approval outcome: granted with persisted presentation proof
  `presentation/4da9eb7e0f2cb560`;
- terminal status: `Completed`;
- event summary: 39 events, one approval, zero retries, zero escalations;
- proof enforcement: approval event marker present and matched to the persisted
  presentation record.

Repository inspection, documentation edits, validation commands, and future git
or pull-request operations are execution work performed outside the kernel. The
kernel governed scope and approval and recorded the planning workflow; it did
not inspect files, edit documentation, run documentation checks, or mutate git.
