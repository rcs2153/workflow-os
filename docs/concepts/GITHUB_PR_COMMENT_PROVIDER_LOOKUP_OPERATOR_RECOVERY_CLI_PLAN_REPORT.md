# GitHub PR Comment Provider Lookup Operator Recovery CLI Plan Report

## 1. Executive Summary

This planning phase defines the future local CLI boundary for inspecting GitHub
pull request comment provider lookup operator recovery posture.

The plan keeps the existing safety invariant intact:

```text
Provider lookup can inform recovery, but it is not durable workflow event proof.
```

No CLI implementation was added.

## 2. Scope Completed

- Created the provider lookup operator recovery CLI planning document.
- Defined a conservative local operator-facing command boundary.
- Defined explicit input and output policies.
- Defined event-proof, retry, repair, artifact, privacy, and error-handling
  policies.
- Defined future tests for rendering, non-leakage, no provider calls, no event
  append, no mutation, and no artifact writes.
- Updated roadmap/planning links.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- CLI commands or rendering;
- hidden auth loading;
- automatic provider lookup;
- provider writes;
- retries;
- manual repair;
- workflow event append from recovery;
- side-effect record mutation;
- report artifact writes;
- schemas;
- examples;
- hosted/distributed behavior;
- reasoning lineage;
- approval-presentation enforcement;
- release posture changes.

## 4. Plan Summary

The plan recommends a future local CLI surface that consumes existing validated
operator recovery summary posture and renders a concise recovery card.

The CLI should show:

- remote lookup posture;
- local event-proof posture;
- retry gate;
- artifact-write gate;
- operator-action posture;
- bounded next action;
- what the command did not do.

## 5. Privacy And Redaction Summary

The plan forbids raw provider payloads, comments, pull request bodies, diffs,
review threads, CI logs, command output, source contents, ambient credentials,
tokens, private keys, unbounded paths, and unsafe redaction metadata.

Future errors must use stable codes and must not leak raw input values.

## 6. Governed Dogfood Run

- workflow_id: `dg/d`
- run_id: `run-1783573544760607000-2`
- approval_id: `approval/run-1783573544760607000-2/planning-approved`
- approval outcome: granted by delegated maintainer
- approval reason: `approved-provider-lookup-operator-recovery-cli-planning-scope`

## 7. Validation Commands

- `npm run check:docs`

## 8. Remaining Known Limitations

- No operator recovery CLI command exists yet.
- No hidden auth loading exists.
- No automatic provider lookup exists.
- No manual repair or retry path exists.
- No workflow event append from recovery exists.
- Report artifact writes remain blocked without durable event proof.

## 9. Recommended Next Phase

Recommended next phase: **provider lookup operator recovery CLI plan review**.

The review should verify that the plan preserves the lookup-not-event-proof
boundary and does not authorize hidden auth, automatic lookup, writes, retries,
repair, event append, mutation, artifacts, schemas, examples, hosted behavior,
reasoning lineage, approval-presentation enforcement, or release posture changes.
