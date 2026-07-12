# Approval Resume Resolved-Context Integrity Blocker Fix Report

Report date: 2026-07-12

## 1. Executive Summary

The confirmed approval/resume time-of-check/time-of-use blocker is fixed in the
local executor. New approval requests bind the approved work to a deterministic,
payload-free resolved execution-context commitment. Every granted approval path
now rebuilds and validates the candidate context before appending any
`ApprovalGranted`, resume-policy, `RunResumed`, skill, hook, or SideEffect event.

This is a fail-closed integrity boundary. It is not a durable immutable run
bundle, handler attestation system, general replay engine, provider-write
expansion, or production authorization system.

## 2. Blocker Fixed

Previously, grant paths appended durable approval and resume events before
reloading current project definitions. The rebuilt plan selected the approved
step by ID but did not prove that the workflow, resolved skills, referenced
policies, or transient request posture matched the context that originally
paused.

The implementation now:

- creates the candidate resume plan before any grant-side append;
- compares current workflow identity to immutable run identity;
- requires a resolved execution-context commitment on the approval request;
- compares the candidate commitment to the approved commitment;
- validates approved step, skill, and skill-version identity;
- mutates durable state only after all comparisons pass.

## 3. Commitment Boundary

The domain-separated `workflow-os/resolved-execution-context/v1` SHA-256
commitment includes:

- workflow ID, version, schema version, and canonical content hash;
- ordered step IDs;
- each resolved skill ID, version, and canonical content hash;
- each referenced policy ID and canonical content hash, deduplicated and sorted;
- sorted required before-skill checkpoint step IDs;
- before-skill hook-input presence;
- SideEffect event and lifecycle-input counts;
- derived report-artifact high-assurance and proof-marker policy posture.

It excludes raw YAML, source contents, paths, prompts, provider payloads,
command output, environment values, credentials, timestamps, generated
invocation IDs, and approval reasons.

## 4. Compatibility And Failure Behavior

`ApprovalRequest.resolved_execution_context_hash` is optional only for backward
deserialization. New approvals always populate it.

- Legacy pending approvals without the commitment fail closed on grant with
  `executor.approval.resume_context_missing` before mutation.
- Legacy approvals remain deniable because denial cannot authorize work.
- Workflow identity drift fails with
  `executor.approval.workflow_identity_mismatch`.
- Skill, referenced-policy, or request-posture drift fails with
  `executor.approval.resume_context_mismatch`.
- Errors use fixed messages and do not expose paths, IDs, hashes, payloads, or
  secret-like values.

Non-default transient request posture cannot yet be reconstructed on resume.
Such runs now fail closed instead of silently discarding that posture.

## 5. Scope Explicitly Not Completed

This phase did not add raw or durable run bundles, handler binary digests,
execution attestations, provider calls or writes, CLI behavior, schemas,
examples, hosted or distributed runtime, RBAC or IdP integration, quorum
approval, automatic approval, reasoning lineage, or release changes.

## 6. Test Coverage

Focused regressions cover:

- unchanged approvals continuing to resume and complete;
- same-ID/version workflow changes failing before grant mutation;
- same-ID/version skill changes failing before grant mutation;
- referenced-policy changes failing before grant mutation;
- unreferenced-policy changes remaining non-blocking;
- required checkpoint posture not being silently dropped;
- deterministic commitments across equivalent runs;
- legacy missing commitments failing grant before mutation;
- legacy missing commitments remaining deniable;
- waiting status, event count, undecided approval state, and zero handler calls
  being preserved after mismatch;
- existing presentation, high-assurance, report-artifact, SideEffect, provider,
  state, and runtime paths through the workspace suite.

## 7. Governed Phase Evidence

- Workflow: `dg/blocker`.
- Run: `run-1783871933653261000-2`.
- Approval: `approval/run-1783871933653261000-2/fix-approved`.
- Presentation: `presentation/f91244a194385e66`.
- Approval outcome: granted through the proof-enforced presentation path.
- Kernel boundary: governance coordination only; Codex performed code and doc
  edits and executed checks outside the kernel.

## 8. Validation Commands

| Command | Result |
| --- | --- |
| `cargo fmt --all --check` | Passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed |
| `cargo test --workspace` | Passed |
| `cargo test -p workflow-core --test local_executor` | Passed, 250 tests |
| focused approval/resume and legacy regressions | Passed |
| `npm run check:docs` | Passed |
| `git diff --check` | Passed |

## 9. Remaining Limitations

- The commitment proves equality with currently available definitions but does
  not preserve a self-contained historical bundle.
- Handler implementations and check executables are not attested.
- Non-default transient resume inputs block rather than resume because durable
  reconstruction is not implemented.
- Distributed replay, remote policy synchronization, production actor/RBAC
  enforcement, and provider mutation expansion remain unsupported.

## 10. Recommended Next Phase

Perform a focused maintainer review of this blocker fix. If accepted, proceed to
the immutable run-bundle boundary already queued in the roadmap. Do not expand
provider mutations before both boundaries are accepted.
