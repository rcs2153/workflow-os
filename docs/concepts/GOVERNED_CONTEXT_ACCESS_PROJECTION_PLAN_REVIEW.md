# Governed Context Access Projection Plan Review

## 1. Executive Verdict

**Plan accepted after focused blocker corrections; proceed to model-only
implementation.**

The corrected plan defines a bounded, deterministic context-authority layer
without turning Workflow OS into a content store, memory system, RAG platform,
connector gateway, or sandbox runtime.

## 2. Scope Verification

The plan stays within model/helper planning.

It does not authorize payload dereference, source reading, transcript or memory
access, tool loading, commands, connectors, provider calls, sandbox lifecycle,
OpenShell integration, SideEffect execution, writes, persistence, events,
receipts, schemas, SDKs, CLI behavior, hosted administration, enterprise
identity, or release changes.

## 3. Source-Of-Truth Assessment

The plan correctly distinguishes:

- a known stable reference from permission to read its target;
- capability availability from authority;
- a current capability resolution from durable or time-of-use authority;
- a context projection from a global catalog or payload;
- EvidenceReference and WorkReport citations from access grants;
- typed handoff declarations from inherited authority.

Context access reuses capability grants and resolutions rather than creating a
parallel permission system.

## 4. Initial Planning Blockers

The first draft had three blockers.

### 4.1 Access-Level Authority Was Underspecified

The draft required projected access not to exceed authority but left the
capability-to-access mapping as an open question. A pure helper could not prove
that invariant from arbitrary capability strings.

The correction fixes:

- `reference_only` to `context.reference.view`;
- `bounded_metadata` to `context.metadata.view`;
- `CapabilityResourceKind::ContextReference`;
- a Core-derived exact `<target-kind>/<stable-id>` resource reference;
- no aliases, wildcards, inheritance, or grouped resources in the first slice.

### 4.2 Gap Completeness Was Not Wire-Bound

The draft proposed serialized entries and gaps without retaining the complete
candidate set that derived them. A caller could omit an entry or gap while
presenting the remaining output as complete.

The correction requires every evaluated candidate and exact source resolution
to remain in the projection. Validation and deserialization recompute the exact
ordered entries and gaps and reject omission, substitution, or reordering.

### 4.3 First Target Scope Was Deferred

The draft left the exact stable-reference set to implementation time. That made
the proposed public model insufficiently bounded.

The correction limits the first set to existing typed Core identities:

- EvidenceReference;
- workflow event;
- audit event;
- validation diagnostic reference;
- approval decision;
- policy decision;
- SideEffect;
- typed handoff;
- WorkReport.

It explicitly defers generic strings and variants whose access semantics need
separate review.

## 5. Access And Metadata Assessment

The two initial positive access levels are appropriately narrow.

Reference-only exposes stable target identity. Bounded metadata adds only
target kind, declared sensitivity, and availability observation time. The plan
forbids summaries, snippets, paths, URLs, titles, diagnostic messages, report
text, event payloads, and arbitrary metadata maps.

Adding a future access-level enum variant cannot activate dereference by itself.

## 6. Authority Composition Assessment

The corrected algorithm requires exact actor, workflow, run, step, optional
harness, evaluation time, capability, resource, sensitivity, selected grant,
and prerequisite posture.

Only authorized resolutions produce entries. Non-authorized and
independent-evaluation results produce bounded gaps. Missing source resolution
is an input error rather than inferred authority posture.

The plan correctly defers grouped or inherited context authority.

## 7. Freshness And Runtime Assessment

The plan is honest that one evaluation timestamp proves batch consistency only.
A serialized projection is not a lease, receipt, or time-of-use authorization.

Any future dereference must re-resolve availability, grant lifecycle, policy,
approval, evidence, checks, immutable run context, target identity,
sensitivity, and redaction posture.

No runtime consumer is authorized by this plan.

## 8. Privacy And Redaction Assessment

The plan excludes raw source, evidence, report, event, transcript, prompt,
provider, command, parser, environment, and credential payloads. It avoids an
unrestricted metadata map and requires safe Debug, validated serde, stable
non-leaking errors, and sensitivity-aware handling of serialized projections.

No privacy blocker remains.

## 9. Relationship To Existing Architecture

- EvidenceReference remains a citation pointer.
- Typed handoff remains a transfer contract, not inherited authority.
- WorkReport remains a governed terminal disclosure.
- Proportional governance may escalate on missing or sensitive context but may
  not weaken explicit access minimums.
- SideEffect remains separate from read-only projection.
- Composable Harness Contracts may later declare requirements but cannot grant
  access.
- OpenShell or another sandbox may later receive an authorized projection but
  cannot become Workflow OS policy authority.

These relationships preserve the product boundary.

## 10. Test Plan Assessment

The corrected test plan covers the important positive, negative, wire,
determinism, freshness, scope, sensitivity, and non-leakage paths. It now also
covers:

- fixed capability mapping;
- exact canonical resource derivation;
- first-slice target taxonomy;
- omitted or substituted candidates;
- entries and gaps that do not exactly match retained candidates.

No blocking planned-test gap remains.

## 11. Documentation Assessment

The roadmap, parent authority plan, phase plan, and report accurately state
that planning is complete and implementation is not. They preserve all runtime,
payload, provider, sandbox, persistence, schema, and write non-goals.

## 12. Blockers

None after the three focused corrections above.

## 13. Non-Blocking Follow-Ups

- Required-context contract consumption needs separate planning.
- Time-of-use re-resolution and authority receipts need separate planning.
- Immutable run-bundle binding for context references remains undefined.
- A future audited dereference path must distinguish attempted, denied,
  succeeded, and ambiguous access.
- Later target variants need stable typed IDs and separate access semantics.
- Sandbox projection must prevent unprojected workspace exposure.

## 14. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.
- Governed review close: completed successfully.

Governed review:

- workflow: `dg/review`;
- run ID: `run-1785124425112096000-2`;
- approval ID:
  `approval/run-1785124425112096000-2/review-scope-approved`;
- presentation ID: `presentation/988f055fe8786771`;
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted handoff was reviewed and presented;
- approval-presentation posture: proof enforced;
- event summary: 39 events, one approval, zero retries, and zero escalations.

Review analysis, documentation corrections, validation commands, and Git
operations were performed outside the kernel. The kernel governed review scope,
approval, durable event history, and close disclosure; it did not edit files,
run checks, or perform Git operations.

## 15. Recommended Next Phase

Implement the governed context-access core model and pure step-scoped projection
helper with:

- the fixed first target set;
- `reference_only` and `bounded_metadata`;
- exact capability and context-resource mapping;
- complete evaluated-candidate retention;
- deterministically recomputed entries and gaps;
- validated serde, safe Debug, stable errors, and focused tests.

Do not implement target dereference, required-context enforcement, runtime
consumption, persistence, events, receipts, schemas, SDKs, CLI behavior,
providers, sandbox lifecycle, OpenShell integration, SideEffect execution,
writes, hosted administration, enterprise identity, or release changes.
