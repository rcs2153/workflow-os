# Workflow Catalog Repair Proposal Review And Approval Plan Review

## 1. Executive Verdict

Plan accepted; proceed to in-memory repair proposal review model/helper
implementation.

The plan creates the missing safety boundary between non-mutating repair
proposals and any future repair apply mode. It correctly treats repair proposal
review as a maintainer/steward decision record, not as mutation permission.

## 2. Scope Verification

The plan stayed within planning-only scope.

No accidental authorization was found for:

- repair apply mode;
- automatic catalog repair;
- record creation;
- record update or overwrite;
- record deletion;
- catalog cleanup;
- active workflow rewrites;
- draft or archive movement;
- runtime workflow registration;
- runtime state creation;
- event or audit append;
- report artifact generation;
- workflow schema changes;
- examples;
- hosted or team catalog backend behavior;
- provider calls;
- local check or command execution;
- write-capable adapters;
- release posture changes.

## 3. Boundary Assessment

The planned sequence is appropriate:

```text
catalog-status -> catalog-repair --dry-run -> review/approval record -> future apply planning
```

This preserves the reviewed dry-run command as proposal-only and prevents users
or maintainers from interpreting a review decision as an apply action.

## 4. Review Artifact Assessment

The proposed review artifact is appropriately minimal and domain-specific. It
captures the key future fields:

- review id;
- proposal id;
- proposal action kind;
- proposal conflict kind;
- workflow id when available;
- source category;
- bounded source reference;
- reviewer actor;
- bounded reason;
- decision kind;
- reviewed timestamp;
- optional approval, policy, evidence, and report references;
- sensitivity and redaction metadata.

The plan does not require raw proposal JSON, raw workflow YAML, or raw catalog
payload storage.

## 5. Decision Kind Assessment

The proposed decision kinds are safe and useful:

- `ApprovedForFutureApplyPlanning`;
- `Rejected`;
- `Deferred`;
- `RequiresManualCatalogReview`;
- `RequiresManualWorkflowReview`;
- `RequiresNewDryRun`.

The naming of `ApprovedForFutureApplyPlanning` is especially important because
it avoids implying that approval equals mutation. This should be preserved in
implementation unless a better equally explicit name is chosen.

## 6. Staleness Assessment

The plan correctly recognizes that repair proposals are point-in-time outputs.
Capturing proposal id, conflict kind, action kind, source category, source
reference, and workflow id gives a future apply planner enough structure to
require a fresh matching dry-run before mutation.

This is a required safety property for any later apply mode.

## 7. Citation And Approval Assessment

The plan appropriately allows review records to cite:

- approval references;
- policy decision events;
- evidence references;
- future WorkReport or report artifact references;
- catalog-status proposal/conflict identity.

It also correctly forbids fabricated approval ids or policy references. Missing
approval and policy references remain explicit and safe rather than implied.

## 8. Privacy And Redaction Assessment

The plan is privacy-aligned with the existing catalog and repair model posture.
It forbids:

- raw workflow YAML;
- raw catalog record payloads;
- source file contents;
- command output;
- provider payloads;
- parser payloads;
- CI logs;
- environment values;
- credentials;
- tokens;
- secret-like reviewer reasons.

It requires bounded reasons, validated constructors, safe Debug output,
safe serialization, and fail-closed deserialization. That is the right boundary
for a future model/helper implementation.

## 9. Test Plan Assessment

The planned tests cover the right behavior:

- valid approve/reject/defer decisions;
- invalid proposal id and reviewer handling;
- unbounded or secret-like reasons rejected without leakage;
- unsupported decision kinds rejected;
- proposal identity preserved;
- optional approval/policy/evidence references validated;
- Debug and serialization non-leakage;
- invalid serialized review fail-closed behavior;
- stale proposal detection representability;
- no files, catalog records, runtime state, or mutation in helper tests.

No blocking test gaps were found for the next in-memory implementation slice.

## 10. Documentation Review

Documentation now states:

- repair proposal review and approval is planned;
- the boundary is before any future apply mode;
- dry-run repair remains proposal-only;
- review approval does not apply repairs;
- automatic repair is not implemented;
- apply mode is not implemented;
- deletion, overwrite, cleanup, runtime registration, schemas, examples,
  provider calls, writes, hosted behavior, and release posture changes remain
  deferred.

## 11. Blockers

No blockers.

## 12. Non-Blocking Follow-Ups

- During implementation, keep decision-kind naming explicit enough that
  approval cannot be confused with apply.
- Consider whether the review id should include proposal id and timestamp
  components or remain caller-supplied through a validated constructor.
- Defer persistence until after the in-memory model/helper is reviewed.

## 13. Recommended Next Phase

Recommended next phase: in-memory repair proposal review model/helper
implementation.

Why: the review/approval plan is accepted and the next useful slice is a
validated, redaction-safe model/helper for repair proposal review decisions. It
should remain in-memory only and must not implement persistence, CLI writes,
apply mode, automatic repair, deletion, overwrite, runtime state, schemas,
examples, provider calls, writes, or release posture changes.

## 14. Validation

Review-phase validation:

```text
npm run check:docs
```

Result: passed.

## 15. Governed Phase Metadata

- dogfood workflow id: `dg/review`
- run id: `run-1783545919872212000-2`
- approval id:
  `approval/run-1783545919872212000-2/review-scope-approved`
- approval outcome: granted by delegated maintainer
- event summary: 39 events; 1 approval; 0 retries; 0 escalations
- out-of-kernel work: documentation edits and validation commands were executed
  by Codex/human execution layer; the kernel coordinated governance only
