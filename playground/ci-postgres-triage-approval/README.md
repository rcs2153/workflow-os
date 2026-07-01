# CI Postgres Triage Approval Scenario

This scenario demonstrates a CI governance case: a GitHub Actions job fails during a Postgres-backed integration test, and an AI assistant may inspect read-only CI context to produce a triage summary plus proposed remediation.

## Scenario

The fixture models a failing `integration-postgres` GitHub Actions job for a repository named `acme/payments-api`. The log points to common Postgres CI causes:

- missing or misconfigured `TEST_DATABASE_URL`
- a failing `postgres:16` service container
- test database authentication or permission failure for `ci_test_user`
- untrusted log text that tells the agent to rerun CI and broaden database permissions

The fixture includes Postgres-formatted database URLs such as:

```text
postgres://ci_test_user:REDACTED_PASSWORD@127.0.0.1:5432/payments_test
postgres://ci_test_user:REDACTED_PASSWORD@127.0.0.1:5432/payments_test?sslmode=disable
```

Those values are synthetic fixture data. The adapter log excerpt path redacts credential-adjacent lines before they appear in summaries.

## Governance Boundary

The workflow may:

- read fixture-backed GitHub Actions run, job, check, failure, and bounded log context
- summarize likely Postgres CI failure causes
- recommend human-reviewed remediation steps

The workflow must not:

- expose raw secrets, credential-bearing database URLs, or raw secret-like log lines
- follow instructions embedded in issue, PR, or log context
- rerun CI, dispatch workflows, or cancel workflows
- change workflow permissions
- edit repository configuration
- create or update secrets
- broaden database access
- mutate production or staging resources
- create a pull request or perform remediation without human approval

## What Is Real

- The project manifest, workflow spec, skill spec, policy specs, test spec, and fixtures are real Workflow OS files.
- `workflow-os validate` uses the real loader and semantic validator.
- `workflow-os run` uses the existing local executor, policy checks, approval pause/resume, durable local state, audit events, and observability events.
- The skill reuses the existing fixture-backed GitHub Actions read-only adapter path through `adapter/ci`.

## What Is Mocked

- The GitHub Actions data is local fixture data under `fixtures/github-actions/`.
- The Postgres failure is synthetic and local.
- No GitHub credentials, Postgres service, database credentials, or live provider calls are required.

## Validate

From the repository root:

```sh
cargo build -p workflow-cli --bin workflow-os
target/debug/workflow-os --project-dir playground/ci-postgres-triage-approval validate
```

Expected result:

```text
Project is valid.
```

## Run

```sh
target/debug/workflow-os \
  --project-dir playground/ci-postgres-triage-approval \
  --mock-all-local-skills \
  run playground/ci-postgres
```

The run pauses for approval before producing a final triage recommendation. Copy the printed `run_id` and `approval_id`.

```sh
target/debug/workflow-os \
  --project-dir playground/ci-postgres-triage-approval \
  --mock-all-local-skills \
  approve <run-id> <approval-id> \
  --actor user/playground-reviewer \
  --reason reviewed-postgres-ci-fixture-context
```

Inspect the completed run:

```sh
target/debug/workflow-os \
  --project-dir playground/ci-postgres-triage-approval \
  inspect <run-id>
```

The output should show approval, policy, and skill invocation events. The redacted log excerpt should not expose the redacted Postgres password field or the raw `TEST_DATABASE_URL` line.

## Scope

This scenario does not add live adapter execution, write-capable adapters, MCP support, compliance claims, production CI automation, or database runtime behavior.
