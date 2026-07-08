# Workflow Catalog Persistence Plan Report

## 1. Executive Summary

This planning phase defines how future Workflow OS work should persist workflow
catalog, stewardship, and archive metadata locally without changing runtime
registration, schemas, examples, providers, writes, hosted behavior, or release
posture.

The plan keeps active workflow files as the loader-visible execution source of
truth and treats catalog records as bounded lifecycle metadata that cite files,
hashes, validation references, approvals, evidence, and reports by stable
reference.

## 2. Scope Completed

- Created [Workflow Catalog Persistence And Stewardship Integration Plan](../implementation-plans/workflow-catalog-persistence-plan.md).
- Defined source-of-truth boundaries.
- Recommended a local `.workflow-os/catalog/` storage layout.
- Defined record identity and safe file naming policy.
- Defined write timing for stewardship, promotion, and archive integration.
- Defined promotion, stewardship, archive, and conflict policies.
- Defined future store API shape.
- Defined privacy, redaction, atomicity, Git posture, test plan, open questions,
  and implementation sequence.
- Updated the earlier catalog/stewardship plan to point to this persistence
  plan.
- Updated the roadmap to reflect the implemented core model review and this
  persistence planning phase.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- catalog storage code;
- command integration;
- runtime workflow registration;
- automatic workflow generation;
- automatic promotion;
- automatic archive cleanup;
- draft deletion;
- workflow schema changes;
- examples;
- provider calls;
- command execution or local check execution;
- hosted/distributed behavior;
- RBAC, IdP integration, notifications, or admin UI;
- write-capable adapters or provider mutation;
- release posture changes.

## 4. Planning Summary

The plan recommends a first local store under `.workflow-os/catalog/` with
separate `workflows/`, `stewardship/`, and `archives/` record areas. The store
should use validated model ids, safe file-name derivation, atomic writes,
deterministic listing, and validated deserialization. Catalog files may be
committed by user/team choice, but Workflow OS must not automatically run Git
operations.

## 5. Promotion And Archive Posture

Future promotion integration should write or update catalog records only after
active workflow validation succeeds. It should optionally require persisted
approved stewardship before promotion in a separately reviewed phase.

Future archive integration should write archive metadata only after a successful
eligible draft move and must not claim approval unless it cites a real persisted
stewardship decision.

## 6. Privacy And Redaction Summary

The plan forbids storing or printing raw workflow YAML, draft YAML, source
contents, package script bodies, dependency values, lockfile contents, CI logs,
command output, provider payloads, parser payloads, absolute private paths,
environment variables, credentials, authorization headers, private keys,
token-like values, and existing agent instruction bodies.

Future store helpers should use the existing validated catalog model
constructors before writing records. Invalid serialized records should fail
closed during reads.

## 7. Test Coverage Planned

The plan calls for tests covering valid writes/reads, duplicate rejection,
invalid serialized records, corrupt JSON, unsafe paths, atomic writes,
deterministic listing, identity mismatches, stale hashes, promotion/archive
integration posture, dry-run non-mutation, conflict detection, non-leaking
errors, and existing authoring command regressions.

## 8. Commands Run And Results

- `npm run dogfood:benchmark -- phase-start --phase planning ...`
  - Completed with run `run-1783487988956668000-2`.
  - Approval requested: `approval/run-1783487988956668000-2/planning-approved`.
- `npm run dogfood:benchmark -- approve run-1783487988956668000-2 approval/run-1783487988956668000-2/planning-approved --actor user/delegated-maintainer --reason approved-workflow-catalog-persistence-planning`
  - Granted; run completed.
- `npm run check:docs`
  - Passed.
- `npm run dogfood:benchmark -- phase-close run-1783487988956668000-2 --phase planning`
  - Passed.
  - Workflow: `dg/d`.
  - Status: `Completed`.
  - Events: `39`.
  - Event summary:
    `ApprovalGranted:1, ApprovalRequested:1, PolicyDecisionRecorded:8, RunCompleted:1, RunCreated:1, RunResumed:1, RunStarted:1, RunValidated:1, SkillInvocationRequested:6, SkillInvocationStarted:6, SkillInvocationSucceeded:6, StepScheduled:6`.

Out-of-kernel work disclosed: documentation edits, docs validation, and git/PR
packaging are performed by Codex as executor. The kernel coordinated the phase
boundary and approval checkpoint only.

## 9. Remaining Known Limitations

- No local catalog store exists.
- No catalog records are written by authoring commands.
- No persisted stewardship decisions are consumed by promotion.
- No archive metadata is written by `archive-draft`.
- No catalog conflict checks are integrated into preflight or steward review.
- No schema, hosted, enterprise, or write-capable behavior exists.

## 10. Recommended Next Phase

Recommended next phase: local workflow catalog store helper implementation.

The next phase should implement only the file-backed helper and tests for
validated catalog, stewardship, and archive records under `.workflow-os/catalog/`.
It should not integrate commands, runtime registration, schemas, examples,
providers, writes, hosted behavior, or release posture changes.
