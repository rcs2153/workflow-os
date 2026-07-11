# GitHub PR Comment End-To-End Live Sandbox Proof Report

## 1. Executive Summary

Workflow OS completed its first real, explicitly authorized GitHub pull request
comment sandbox proof through the accepted local composition path.

The proof used draft pull request
[`rcs2153/workflow-os#318`](https://github.com/rcs2153/workflow-os/pull/318)
as a confirmed non-production target. One ignored-by-default integration test
invoked the concrete GitHub HTTP provider with caller-supplied auth, classified
the response, transitioned the persisted SideEffect to `Completed`, and
appended one durable `SideEffectCompleted` workflow event.

No default write behavior, hidden auth loading, retry, repair, provider recall,
report artifact write, CLI mutation command, schema change, broader adapter,
hosted behavior, or release-posture change was added.

## 2. Scope Completed

- Added an ignored-by-default live integration test for the accepted sandbox
  path.
- Added a test-local `ureq` transport implementing the existing injected HTTP
  transport trait.
- Required explicit environment inputs for owner, repository, pull request,
  and token.
- Reused existing provider, sandbox validation, SideEffect transition, and
  event-proof helpers.
- Executed the live test once against draft PR #318.
- Verified the external comment through the GitHub connector.
- Verified local completed lifecycle and durable event-proof assertions.

## 3. Scope Explicitly Not Completed

This phase did not add automatic or default provider writes, a production
target, hidden credential discovery, runtime provider configuration, CLI write
commands, automatic retry/repair/lookup/recovery, provider recall during event
proof, report artifacts, another adapter mutation, schemas, examples, hosted
behavior, reasoning lineage, higher autonomy, or a release-posture change.

## 4. Opt-In Harness

The integration test is:

```text
live_github_pr_comment_sandbox_composes_provider_outcome_and_durable_event_proof
```

It is ignored in ordinary workspace and CI tests. Running it requires four
explicit inputs:

```text
WORKFLOW_OS_GITHUB_SANDBOX_OWNER
WORKFLOW_OS_GITHUB_SANDBOX_REPOSITORY
WORKFLOW_OS_GITHUB_SANDBOX_PULL_REQUEST
WORKFLOW_OS_GITHUB_SANDBOX_TOKEN
```

The transport exists only in integration-test code. Production library and CLI
code remain caller-injected, network-free by default, and unable to discover
credentials.

## 5. Live Target And External Effect

- Target classification: maintainer sandbox.
- Repository: `rcs2153/workflow-os`.
- Pull request: draft PR #318.
- External effect: one issue comment.
- Comment reference:
  [`issuecomment-4948387421`](https://github.com/rcs2153/workflow-os/pull/318#issuecomment-4948387421).
- Bounded body: `Workflow OS governed live-sandbox proof. No production action authorized.`

The token was passed only to the test process. It was not printed, serialized,
persisted by Workflow OS, or added to repository files.

## 6. Runtime Proof Chain

The test proved this accepted sequence:

1. Complete one local workflow run.
2. Persist one approved GitHub-write SideEffect and transition it to
   `Attempted`.
3. Validate explicit non-production target proof and sandbox readiness.
4. Construct and send one request through the concrete provider client and
   injected test transport.
5. Classify the GitHub success response with a bounded comment reference.
6. Transition the persisted SideEffect to `Completed`.
7. Compose the accepted live-sandbox result into durable workflow event proof.
8. Rehydrate the run and verify exactly one `SideEffectCompleted` event.

The event-proof helper did not call the provider again. No report artifact was
written.

## 7. Report And Disclosure Posture

This document is the bounded operator disclosure for the live proof. The
integration result exposes provider-call, provider-response, local-transition,
event-proof, and artifact-write posture through existing validated types.

No runtime WorkReport artifact was generated or persisted. Automatic report
generation and artifact writing remain separately gated and out of scope.

## 8. Privacy And Redaction

- The token is an explicit test-process input and is never logged by the test.
- HTTP request Debug output remains redacted by the production model.
- Raw GitHub response payloads are reduced inside the injected test transport
  to status plus comment ID before entering Workflow OS models.
- No repository contents, CI logs, command output, auth headers, browser state,
  or raw provider payloads are stored in the workflow event.
- Transport errors use stable non-leaking test codes.

## 9. Test Coverage

The live test asserts provider execution, classified success, completed
SideEffect state, appended event-proof posture, exactly one durable
`SideEffectCompleted` event, and no report artifact write.

Existing focused regressions cover disabled append policy, classified provider
failure, matching replay, canonical idempotency, correlation mismatch,
non-terminal blocking, provider non-recall, and non-leakage.

## 10. Commands And Results

The explicit live test passed:

```sh
cargo test -p workflow-core --test local_executor \
  live_github_pr_comment_sandbox_composes_provider_outcome_and_durable_event_proof \
  -- --ignored --exact --nocapture
```

Pre-live validation passed:

```sh
cargo test -p workflow-core --test local_executor live_sandbox_event_proof
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
git diff --check
```

Full end-of-phase validation passed:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run check:docs
git diff --check
```

The live proof test remains ignored in the ordinary workspace suite and was run
separately with explicit inputs. Other opt-in live provider tests remained
skipped behind their existing environment gates.

## 11. Governed Phase Evidence

- Dogfood workflow: `dg/runtime-composition`.
- Run ID: `run-1783795906742694000-2`.
- Approval ID:
  `approval/run-1783795906742694000-2/composition-approved`.
- Approval presentation ID: `presentation/c586aa12b649d5c7`.
- Approval outcome: granted by the delegated maintainer through the
  proof-enforced path.
- Event summary: 39 ordered governance events, including one approval request,
  one approval grant, eight policy decisions, six scheduled steps, six
  successful skill invocations, and one completed run; no retries or
  escalations.
- Approval-presentation enforcement: `proof_enforced`; the approval event trail
  exposes the matching presentation proof marker.
- Out-of-kernel work: Codex authored and ran the integration test, created the
  draft PR target, supplied the keychain credential to the test process,
  inspected the external comment, ran checks, edited docs, and performed
  git/PR operations. The kernel governed scope and approval but did not execute
  commands, edit files, load credentials, call GitHub, or perform git/PR
  actions.
- Report posture: this document is the phase report; no runtime WorkReport
  artifact was generated or persisted.

## 12. Observed Gaps

No blocker was exposed by the successful proof.

The documented append-success/rehydration-failure ambiguity remains a
non-blocking recovery concern, but it was not triggered. The live path also
remains intentionally test-driven and caller-configured rather than a public
CLI or default executor capability.

## 13. Recommended Next Phase

Recommended next phase: first provider-write sandbox expansion-readiness
review.

The review should assess whether this proof is deterministic, auditable,
restart-safe, sufficiently redacted, and narrow enough to justify planning one
additional mutation or adapter. It must not authorize broader default writes,
hidden auth, automatic retry or repair, production targets, or hosted behavior.
