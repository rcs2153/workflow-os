# Immutable Run Bundle Executor Binding Review

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups.**

The explicit opt-in executor path establishes the intended ordering and
identity boundary without changing default execution. One review blocker was
found: existing bound runs initially accepted changed explicit retry posture
when bundle ID and version still matched. The fix compares the caller's
workflow, bundle metadata, actor, sensitivity, redaction, and execution posture
to the stored manifest before rehydration. The regression suite passes.

## 2. Scope Verification

The phase stayed within the approved explicit executor-binding scope.

It did not add automatic bundle generation, executable replay, execution from
stored definitions, handler/check attestation, approval changes, policy
broadening, hook execution, SideEffect execution, report generation, provider
mutations, CLI behavior, schemas, SDK changes, examples, hosted behavior,
scoped authority, or release changes.

## 3. Publication Ordering Assessment

The new path prepares execution, evaluates pre-run policy, constructs and
validates the bundle, then publishes or exactly verifies the complete bundle
before `RunCreated`. A bundle construction, integrity, or store failure leaves
the event backend without a created run.

The local bundle manifest remains the bundle-store commit marker. Canonical
records published before a later failure may remain as immutable orphans, but
they do not create a run-to-bundle binding or an executable run.

## 4. Durable Identity Assessment

`ImmutableRunBundleBinding` stores bundle ID, bundle format version, and root
hash. `RunCreated` carries the optional binding and rehydration retains it in
`WorkflowRunIdentity`.

Later events retain the established event identity fields and are checked
against those fields. They do not restate or overwrite the creation-time bundle
binding. A second `RunCreated` is already invalid under the event-state model.

The binding is constructed from a validated manifest rather than arbitrary
public fields. Its ID and hash components retain existing validation and safe
serde behavior.

## 5. Retry And Rebinding Assessment

Exact retries read the durable run and complete stored bundle, verify the run's
binding against the stored manifest, and return the existing run without
invoking the skill again.

Review found that the initial implementation did not compare every explicit
bundle retry input. A caller could change sensitivity, redaction, actor,
creation time, workflow identity, or request execution posture while retaining
the same run and bundle IDs. The implementation now compares those fields to
the stored manifest and fails with the stable non-leaking binding-mismatch code.

A different bundle ID or version cannot rebind an existing run. A legacy
unbundled run cannot be silently upgraded through this explicit path.

## 6. Compatibility Assessment

The bundle field is optional and serde-defaulted. Legacy `RunCreated` events
without the field deserialize and rehydrate unchanged. Existing executor APIs
continue to create and return unbundled runs.

The new helper is additive and currently targets the standard local executor
sinks. Broader sink-generic exposure is not required for this first boundary
and should be added only with a concrete caller.

## 7. Privacy And Redaction Assessment

- Request and result Debug output does not print execution inputs, IDs,
  timestamps, roots, or stored model content.
- Stable errors do not echo paths, IDs, hashes, definitions, or caller values.
- The bundle stores canonical validated definitions rather than raw YAML.
- No parser payload, command output, provider body, environment value,
  credential, authorization header, private key, or token is copied.
- Handler identity remains explicitly `RegisteredUnattested` or `Unavailable`.

## 8. Test Quality Assessment

Focused tests cover successful pre-run publication and binding, exact retry,
changed retry posture rejection, legacy-run rejection on the explicit path,
rebinding rejection, persistence failure before `RunCreated`, default executor
compatibility, legacy JSON compatibility, and bound event round-trip.

Bundle builder and store tests continue to cover deterministic construction,
canonical records, missing/corrupt records, atomic publication, restart reads,
and redaction. The full workspace suite covers approval, policy, hooks,
SideEffects, reports, provider paths, and runtime regression.

## 9. Validation Assessment

After the review blocker fix, all required checks pass:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`; and
- `git diff --check`.

Only explicit opt-in live integration tests remain ignored by default.

## 10. Blockers

None remain.

The changed-retry-posture blocker was fixed and covered by a regression test
during this review.

## 11. Non-Blocking Follow-Ups

- Define recovery behavior when a complete bundle is published but event-store
  startup is only partially appended.
- Add read-only historical bundle/current-project comparison when it becomes a
  concrete operator need.
- Plan handler and check attestation before claiming execution evidence.
- Consider custom-sink generic exposure only when a caller requires it.
- Plan immutable orphan cleanup without weakening create-only behavior.

## 12. Governed Review Evidence

- Workflow: `dg/review`.
- Run: `run-1784157369842207000-2`.
- Approval: `approval/run-1784157369842207000-2/review-scope-approved`.
- Presentation: `presentation/e7d41ea47f37e5b6`.
- Approval outcome: granted under delegated-maintainer authority after the
  complete proof-enforced handoff was relayed.
- Event summary: 39 events, one approval, zero retries, zero escalations, and
  one persisted approval-presentation proof record with event marker present.
- Out-of-kernel work: Codex inspected the implementation and tests, identified
  and fixed the retry-posture blocker, authored this review, and reran required
  repository checks. The kernel governed scope and approval; it did not edit
  files or execute checks.

## 13. Recommended Next Phase

Proceed to the **Scoped Runtime Authority and Capability Projection core model
only**, following the existing plan. Start with bounded, expiring, revocable
capability-grant and availability vocabulary plus validation and redaction-safe
serde.

Do not add tool execution, connectors, provider writes, agent teams, memory
infrastructure, hosted administration, enterprise identity, cryptographic
receipts, workflow schemas, or additional provider mutation families.
