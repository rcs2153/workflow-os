# External Developer Adoption Review

## Reported Downstream Dogfooding

An external developer reported using the Workflow OS CLI from a custom Python
harness while exploring an SDK contribution. The developer followed the
repository's `AGENTS.md`, reviewed the scoped approval handoff, approved the
gate, and inspected durable history. They identified shared state and authority
handoff as useful to their own harness and noted the PostgreSQL state option.
This is reported onboarding and approval-loop evidence, not independently
replayed evidence of cross-agent continuation, PostgreSQL failover, production
writes, or recovery from an ambiguous effect. No personal details or verbatim
testimony are needed to preserve the finding.

## Python SDK Contribution Review

An external Python SDK contribution is welcome. No open contribution PR was
found during this review. Review the actual PR and exact head when available;
this checklist is not an acceptance decision or a shipped quickstart.

- Accept a bounded subprocess/JSON client over the public Rust CLI. An FFI
  rewrite is not a prerequisite. Rust remains authoritative; Python must not
  implement a second governance engine or access state stores directly.
- Require typed inputs and outputs for supported public operations, explicit
  executable, project and state selection, deterministic argv and JSON
  decoding, and no shell interpretation of inputs.
- Require structured failures for nonzero exit, missing executable, invalid
  JSON, timeout and unsupported version. Document CLI compatibility and keep
  timeout or transport uncertainty distinct from a failed workflow decision.
- Do not implement automatic approvals, unsafe retries, hidden provider calls
  or credential handling. Preserve exact run identity and expose approval,
  refusal and continuation results without widening authority.
- Require offline tests against a real compatible CLI and fixture project,
  including an approval wait and durable inspection. Also cover a missing or
  incompatible binary, an error response, and timeout. Provide package install
  instructions and a tested quickstart for the actual reviewed API.

## Public Integration Wording

The README, user-guide index, package descriptions, GitHub About summary, and
the GitHub metadata, GitHub review-context and Jira intake examples were
checked against the [Current Product Contract](../user-guide/current-product-contract.md)
and the [read-only adapter guide](../integrations/read-only-adapters.md).
The examples already identify fixture or mock handling. Credential-backed
reads are opt-in for documented operations; fixture coverage does not establish
live coverage. External writes are narrow GitHub sandbox paths, not general
GitHub or Jira automation. The offline recovery and protected-host results
do not establish deployed or packaged cross-harness operation.

Proposed GitHub About wording, not applied by this phase:

> Open-source continuity-of-authority kernel for AI agents: shared governance that preserves scoped approvals and execution evidence so an authorized executor can determine what happened and what it may do next.

## Governance Record

Docs run `run-1794377970592711000-2` (`dg/d`) used the proof-backed approval
`approval/run-1794377970592711000-2/planning-approved`. Repository inspection,
public metadata reads, edits and validation are executor work outside the
preview handlers. This record does not authorize an SDK implementation, an
About edit, provider operations, or a roadmap sequence change.
