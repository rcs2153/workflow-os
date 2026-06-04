# RC1 Internal Evaluation Guide

This guide explains how to evaluate the current Workflow OS build safely.

RC1 internal evaluation is not production readiness. It is a controlled evaluation of the local kernel preview, the vertical-slice example, Phase 2 fixture-backed read-only adapters, and scoped adapter telemetry mapping.

## What RC1 Internal Evaluation Means

RC1 internal evaluation means:

- the local kernel can be built, validated, and exercised locally
- local workflow runs can be created, approved, inspected, and rehydrated
- local state health can be checked
- read-only adapter examples can run against fixtures without live credentials
- adapter telemetry mapping can be inspected for controlled fixture-backed examples
- known limitations remain explicit

It does not mean:

- production deployment readiness
- public read-only integration preview readiness
- live provider behavior is proven
- write-capable adapters exist
- generic live adapter execution exists
- distributed workers or production backends exist
- UI, hosted service, marketplace, OAuth, webhooks, or Level 3/4 autonomy are available

## Safe To Test

| Evaluation path | Safe in RC1? | Notes |
| --- | --- | --- |
| Docs checks | Yes | Run locally. |
| Rust tests and TypeScript checks | Yes | No live provider credentials required. |
| `workflow-os validate` | Yes | Does not execute workflows. |
| Vertical slice approval example | Yes | Uses explicit deterministic local mock handler. |
| CLI `run`, `approve`, `status`, `inspect`, `doctor state` | Yes within example scope | Uses local filesystem state. |
| GitHub/Jira/CI read-only examples | Yes with fixtures | Use `--mock-all-local-skills` and checked-in fixtures. |
| Adapter telemetry inspection | Yes with fixtures | Local runtime-visible telemetry only. |
| Live GitHub/Jira/CI smoke tests | Maintainer-only | Requires approved non-sensitive resources and read-only credentials. Not run by default. |

## Not Supported

Do not evaluate or claim:

- GitHub branch creation, commits, PR creation, comments, labels, reviews, merges, or closes
- Jira issue creation, issue updates, comments, status transitions, assignment, labels, or links
- CI reruns, workflow dispatch, cancellation, artifact mutation, check mutation, or auto-repair
- webhooks or trigger ingestion
- OAuth app flows
- production database backend
- distributed workers
- hosted service
- UI
- marketplace
- domain packs
- production pattern catalog
- work reports
- evidence references
- Reasoning Lineage / Claim Graph
- Level 3/4 autonomy enablement

## Prerequisites

Run all commands from the repository root unless a command explicitly says otherwise.

Before starting, confirm:

- Rust toolchain is installed and can run `cargo`.
- Node.js and npm are installed.
- The CLI has been built before using `target/debug/workflow-os` directly.
- Normal RC1 evaluation does not require live GitHub, Jira, or CI provider credentials.
- Temporary state directories are used for example runs so evaluator state does not pollute checked-in example directories.
- No sensitive data, live customer data, private issue bodies, private PR content, raw CI logs, or secrets are used in normal RC1 evaluation.

Live provider credentials are only for maintainer-owned live smoke tests with approved non-sensitive resources. Do not load those credentials for the normal paths in this guide.

## Baseline Checks

From the repository root:

```sh
npm run check:docs
npm run check
npm run check:integrations
```

Run Rust tests when changing code or when validating a full local build:

```sh
cargo test --workspace
```

`npm run check:integrations` uses fixtures and must not require live GitHub, Jira, or CI credentials.

## Local Kernel Test Path

Build the CLI:

```sh
cargo build -p workflow-cli --bin workflow-os
```

Validate the vertical slice:

```sh
target/debug/workflow-os \
  --project-dir examples/vertical-slice-approval \
  validate
```

Expected result:

```text
Project is valid.
```

## Vertical Slice Test Path

Use a temporary state directory so evaluation does not pollute the example tree:

```sh
target/debug/workflow-os \
  --project-dir examples/vertical-slice-approval \
  --state-dir /tmp/workflow-os-vertical-slice-state \
  --mock-all-local-skills \
  run ex/review
```

Expected result:

```text
status: WaitingForApproval
approval_id: approval/...
```

Copy the `run_id` from the run output. Copy the `approval_id` from the same output. Use those exact values in the following `status`, `approve`, and `inspect` commands wherever the examples show `<run-id>` and `<approval-id>`.

Inspect status:

```sh
target/debug/workflow-os \
  --project-dir examples/vertical-slice-approval \
  --state-dir /tmp/workflow-os-vertical-slice-state \
  status <run-id>
```

Approve:

```sh
target/debug/workflow-os \
  --project-dir examples/vertical-slice-approval \
  --state-dir /tmp/workflow-os-vertical-slice-state \
  --mock-all-local-skills \
  approve <run-id> <approval-id> \
  --actor user/rc1-evaluator \
  --reason reviewed-local-evaluation
```

Inspect:

```sh
target/debug/workflow-os \
  --project-dir examples/vertical-slice-approval \
  --state-dir /tmp/workflow-os-vertical-slice-state \
  inspect <run-id>
```

Useful evidence:

- run ID
- schema version
- workflow version
- spec hash
- approval request and grant
- policy decisions
- skill invocation events after approval
- terminal state
- absence of external service calls

## Read-Only Fixture Adapter Test Path

These examples are Phase 2 read-only integration preview examples. They are fixture-backed by default and do not require live credentials.

Build the CLI first:

```sh
cargo build -p workflow-cli --bin workflow-os
```

### GitHub Read-Only Fixture Example

```sh
target/debug/workflow-os \
  --project-dir examples/github-read-only-review-context \
  validate
```

```sh
target/debug/workflow-os \
  --project-dir examples/github-read-only-review-context \
  --state-dir /tmp/workflow-os-gh-readonly-state \
  --mock-all-local-skills \
  run ex/gh
```

Approve and inspect:

```sh
target/debug/workflow-os \
  --project-dir examples/github-read-only-review-context \
  --state-dir /tmp/workflow-os-gh-readonly-state \
  --mock-all-local-skills \
  approve <run-id> <approval-id> \
  --actor user/rc1-evaluator \
  --reason reviewed-fixture-context
```

```sh
target/debug/workflow-os \
  --project-dir examples/github-read-only-review-context \
  --state-dir /tmp/workflow-os-gh-readonly-state \
  inspect <run-id>
```

### Jira Read-Only Fixture Example

```sh
target/debug/workflow-os \
  --project-dir examples/jira-read-only-intake-quality \
  validate
```

```sh
target/debug/workflow-os \
  --project-dir examples/jira-read-only-intake-quality \
  --state-dir /tmp/workflow-os-jira-readonly-state \
  --mock-all-local-skills \
  run ex/jira
```

Approve and inspect:

```sh
target/debug/workflow-os \
  --project-dir examples/jira-read-only-intake-quality \
  --state-dir /tmp/workflow-os-jira-readonly-state \
  --mock-all-local-skills \
  approve <run-id> <approval-id> \
  --actor user/rc1-evaluator \
  --reason reviewed-fixture-intake
```

```sh
target/debug/workflow-os \
  --project-dir examples/jira-read-only-intake-quality \
  --state-dir /tmp/workflow-os-jira-readonly-state \
  inspect <run-id>
```

### CI Read-Only Fixture Example

```sh
target/debug/workflow-os \
  --project-dir examples/ci-read-only-failure-summary \
  validate
```

```sh
target/debug/workflow-os \
  --project-dir examples/ci-read-only-failure-summary \
  --state-dir /tmp/workflow-os-ci-readonly-state \
  --mock-all-local-skills \
  run ex/ci
```

Approve and inspect:

```sh
target/debug/workflow-os \
  --project-dir examples/ci-read-only-failure-summary \
  --state-dir /tmp/workflow-os-ci-readonly-state \
  --mock-all-local-skills \
  approve <run-id> <approval-id> \
  --actor user/rc1-evaluator \
  --reason reviewed-fixture-ci-context
```

```sh
target/debug/workflow-os \
  --project-dir examples/ci-read-only-failure-summary \
  --state-dir /tmp/workflow-os-ci-readonly-state \
  inspect <run-id>
```

## Adapter Telemetry Inspection

For read-only fixture examples, `workflow-os inspect` should show concise adapter telemetry summaries. Look for:

- adapter audit telemetry count
- adapter observability telemetry count
- adapter kind
- action
- capability
- operation mode
- policy precheck provenance
- correlation ID
- redaction metadata

The telemetry is local/runtime-preview telemetry for controlled fixture-backed examples. It is not production telemetry export, SIEM integration, OpenTelemetry integration, or generic live adapter execution.

Telemetry must not contain:

- provider tokens
- raw CI logs
- raw Jira descriptions or comments
- raw large GitHub file contents
- raw provider payloads by default

## Local State Inspection

Use:

```sh
target/debug/workflow-os \
  --project-dir examples/vertical-slice-approval \
  --state-dir /tmp/workflow-os-vertical-slice-state \
  doctor state
```

The command is read-only. It reports backend health, event/index consistency, corrupt event files, rehydration failures, and approval projection issues where detectable. It does not repair or mutate state.

## Live Smoke Evidence Status

Public read-only integration preview remains blocked.

Live smoke evidence has not been recorded for:

- GitHub read-only
- Jira read-only
- GitHub Actions / CI read-only

Do not run live smoke tests unless all preconditions in [the live smoke checklist](../integrations/PHASE_2_LIVE_SMOKE_ENVIRONMENT_CHECKLIST.md) are met and maintainer authorization is explicit. Use [the evidence template](../integrations/PHASE_2_LIVE_SMOKE_EVIDENCE_TEMPLATE.md) to record results.

## Cleanup

If you used the temporary state directories shown in this guide and do not need to preserve evidence, remove them after evaluation:

```sh
rm -rf /tmp/workflow-os-vertical-slice-state
rm -rf /tmp/workflow-os-gh-readonly-state
rm -rf /tmp/workflow-os-jira-readonly-state
rm -rf /tmp/workflow-os-ci-readonly-state
```

Do not delete a state directory if it is needed for a bug report, security review, corruption investigation, or maintainer reproduction. Preserve the full state directory when reporting state or rehydration issues, and share only redacted summaries unless a maintainer gives a secure evidence-transfer path.

## Recommended Evidence Packet

For useful RC1 feedback, capture:

- commit SHA
- command transcript
- validation output
- run ID
- `inspect` output
- `doctor state` output
- whether fixture, mock, or live mode was used
- accepted limitations that apply to the report
- redacted logs or screenshots when useful

Evidence should be enough for maintainers to reproduce the issue without exposing credentials, private provider data, or sensitive business payloads.

## Reporting Issues

When reporting an RC1 issue, include:

- repository commit SHA
- command run
- operating system
- Rust and Node versions if relevant
- project path
- state directory path if non-sensitive
- expected result
- actual result
- relevant diagnostics
- run ID, event IDs, or correlation IDs where useful
- whether the example used fixture mode, mock mode, or live mode

Never paste secrets or raw sensitive provider data into issues, docs, chat, screenshots, logs, or review notes. Do not include:

- tokens
- authorization headers
- private keys
- raw credentials
- full private issue text
- full PR text
- raw CI logs
- full private repository file contents
- raw sensitive provider payloads

## Success Criteria

An RC1 internal evaluation is successful when:

- baseline docs and contract checks pass
- the vertical slice validates, pauses for approval, resumes, completes, and can be inspected
- local state health is inspectable
- read-only fixture examples validate and run if adapter evaluation is in scope
- adapter telemetry appears for fixture-backed read-only examples
- sensitive values remain redacted
- unsupported writes remain unavailable
- limitations are recorded rather than hidden

## Failure Criteria

Treat evaluation as failed or blocked when:

- validation errors are ignored
- a workflow executes a gated skill before approval
- a denied policy decision can be bypassed
- a write action occurs or appears available
- a live provider is called without explicit maintainer authorization
- secrets appear in specs, logs, diagnostics, audit, observability, telemetry, or docs
- local state corruption is hidden
- docs or reports claim production readiness or public read-only preview readiness without evidence

## Known Limitations

The authoritative limitations list is [V0 Known Limitations](../release/V0_KNOWN_LIMITATIONS.md).

Important RC1 reminders:

- local filesystem backend only
- no production database
- no distributed workers
- no hosted service
- no UI
- no real write-capable adapters
- no public read-only integration preview until live smoke evidence exists
- no active timeout scheduler
- no trigger ingestion service
- no Level 3/4 autonomy enablement
- no domain packs
- no production pattern catalog
- no work reports, evidence references, or reasoning lineage implementation
