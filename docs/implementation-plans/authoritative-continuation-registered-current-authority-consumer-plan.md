# Authoritative Continuation Registered Current-Authority Consumer Plan

Status: Implemented and accepted after final focused maintainer/security
review. Direct composed duplicate and stale-claim outcomes plus all exact
binding substitutions pass at the executor boundary. The proof remains
crate-private and does not authorize operational source configuration.

Related foundations:

- [Authoritative Agent Continuation Context And Rehydration Plan](authoritative-agent-continuation-context-rehydration-plan.md)
- [Current-Authority One-Time-Use And Replay Posture Plan](current-authority-one-time-use-replay-posture-plan.md)
- [Production Current-Authority Source Boundary Plan](production-current-authority-source-boundary-plan.md)
- [Required Context Immutable-Run Binding And Time-Of-Use Plan](required-context-immutable-run-time-of-use-plan.md)
- [Authoritative Agent Continuation Preview Review](../concepts/AUTHORITATIVE_AGENT_CONTINUATION_PREVIEW_REVIEW.md)

## 1. Executive Summary

The first authoritative continuation slice proves one immutable-run local
skill invocation can be selected from durable state and consumed once through
a cursor-bound claim. The read-only `next-action` preview exposes the same
bounded orientation without granting authority.

That path does not yet resolve an independent registered current-authority
source or consume an exact required-context contract immediately before the
skill consumer. This phase should compose the existing private registered
source, same-call use boundary, authoritative continuation claim, and local
skill invocation into one narrow Core-owned operation.

The implementation must remain crate-private and opt-in. It proves the
composition without creating a public source trait, caller-asserted authority,
runtime source configuration, provider mutation, or nested harness runtime.

## 2. Governing Invariant

```text
The agent may remember or propose the next step.
Only the kernel may declare and authorize the next material action.
```

For the selected source-backed path, authority means all of the following in
one Core-owned call:

```text
exact immutable run and cursor
  + exact registered current-authority source resolution
  + exact required-context contract consumption
  + exact continuation action and idempotency claim
  -> one local current-step skill consumer
```

No individual brief, source snapshot, assessment, binding, or claim is
sufficient by itself.

## 3. Goals

- Add one crate-private source-backed continuation composition for
  `invoke_current_step_skill`.
- Re-resolve the registered source for every attempted use.
- Validate the required-context execution binding against the exact run,
  immutable bundle, step, actor, harness contract, and contract hash.
- Consume the exact required-context contract before continuation claim or
  handler invocation.
- Bind the source snapshot, fact set, assessment, and context-consumption
  commitments into the continuation governance commitment.
- Preserve the existing durable cursor-bound first-writer claim and post-claim
  cursor reread.
- Invoke the existing hook-plus-local-skill consumer only after both authority
  and continuation boundaries accept.
- Fail closed with stable non-leaking errors for blocked, unavailable, stale,
  mismatched, duplicate, or ambiguous posture.
- Keep existing public executor and preview behavior unchanged.

## 4. Non-Goals

This phase does not authorize:

- a public current-authority source trait or public source constructor;
- caller-assembled facts becoming trusted authority;
- workflow or runtime source configuration;
- a reusable grant, lease, session, or serialized permission object;
- a public readiness or authorization result;
- a new `next-action` execution command;
- provider calls, provider mutation broadening, OpenShell integration, or
  sandbox execution;
- typed child runtime or nested harness execution;
- SideEffect execution or new write behavior;
- generic context dereference or report-body access;
- automatic command, shell, editor, git, browser, or provider interception;
- new workflow events, audit records, report artifacts, schemas, SDKs, CLI
  behavior, examples, hosted behavior, or release changes.

## 5. Existing Boundaries To Reuse

The implementation should reuse rather than duplicate:

- `RegisteredInMemoryCurrentAuthoritySource::use_current_authority` for fresh
  source selection, fact-set construction, capability resolution, governed
  context projection, and required-context consumption;
- `RequiredContextExecutionBinding` and
  `RequiredContextContractBinding` for immutable identity and contract binding;
- `GovernedContinuationBrief` and `GovernedContinuationBinding` for the exact
  run cursor, step, action, invocation key, immutable root, and governance
  commitment;
- `consume_governed_continuation` for the durable first-writer claim and
  post-claim cursor reread; and
- `LocalExecutor::invoke_authorized_local_skill` for the existing hook,
  SideEffect disclosure, invocation-event, attempt-idempotency, and handler
  semantics.

No parallel authority, context, continuation, or invocation model should be
introduced.

## 6. Proposed Internal API Boundary

Add a crate-private `RegisteredCurrentAuthorityContinuationUseInput<'a>`
adjacent to the registered source or executor. It should borrow:

- one Core-registered `RegisteredInMemoryCurrentAuthoritySource`;
- the exact `RequiredContextExecutionBinding`;
- the exact `RequiredContextContractBinding`;
- a Core-selected evaluation timestamp; and
- validated redaction metadata.

The executor should expose no new public builder in this phase. A crate-private
`with_registered_current_authority_continuation(...)` builder may attach this
input only to the selected local execution test/composition path and must also
enable the existing authoritative-continuation guard. Public
`LocalExecutor::new`, existing opt-in `with_authoritative_continuation`,
`execute`, approval, cancellation, and preview APIs remain unchanged.

The private generic source callback must not be widened or exported. The only
consumer reachable through the new composition is the current local step's
existing skill path.

## 7. Exact Binding Validation

Before source use, Core must prove that the supplied execution binding and
contract identify the same operation as the current executor plan and durable
run:

- workflow ID and run ID match the durable run;
- step ID matches the selected current step;
- actor matches the immutable execution actor;
- harness contract ID/version match the exact contract;
- contract content hash matches the execution binding;
- immutable bundle identity/root match the durable run binding;
- current run status is `Running`; and
- invocation idempotency identity matches the selected skill invocation.

Static identity mismatch must fail before the selected path records a new
invoke-policy decision. A cursor mismatch can be detected only after durable
rehydration and therefore preserves prior run events while still failing before
continuation claim, hooks, attempts, or handler invocation.

## 8. Source-Backed Use Sequence

The selected path should perform this order:

1. validate static plan, execution-binding, contract, and immutable-bundle
   identity before recording new invocation events;
2. evaluate and record the existing invoke policy;
3. resolve the exact registered local handler;
4. rehydrate the current durable run and validate its current cursor and
   identity;
5. call `use_current_authority` with the Core-selected evaluation time;
6. require `Ready` source-backed resolution and satisfied required context;
7. ask the borrowed private capability for one domain-separated continuation
   commitment rather than exposing its assessment internals;
8. derive the full bounded source-backed governance commitment from that
   private commitment plus existing executor governance material;
9. project the exact continuation brief at the current durable cursor;
10. atomically claim that exact continuation binding;
11. reread and verify the durable cursor;
12. invoke the existing authorized local skill path once; and
13. map the known consumer result without fabricating success or evidence.

The private capability commitment should cover the accepted source snapshot,
fact-set, assessment, and consumption commitments. It must not expose those
values individually to a new public surface.

Fresh authority resolution must happen before every attempted continuation,
including attempts that later encounter a duplicate or stale continuation
claim.

## 9. Commitment Composition

The existing continuation governance commitment should be extended only for
the selected source-backed path. Its canonical material should include:

- existing resolved execution-context and immutable-bundle commitments;
- existing proportional-governance assessment binding when present;
- approval, sensitivity, policy-effect, capability, and required-hook posture;
- registered source snapshot commitment;
- current-authority fact-set commitment;
- source-backed assessment commitment; and
- exact required-context consumption result commitment or an existing stable
  equivalent.

Raw grants, availability records, context references, source IDs, timestamps,
and target values must not enter events, errors, Debug output, or public
serialization.

## 10. Failure Semantics

- Invalid execution binding or contract: fail before source use.
- Registered source failure: fail before continuation claim and handler use;
  an already-recorded invoke-policy event may remain in the durable trail.
- Blocked authority or required-context gap: fail before claim and handler use.
- Expired or revoked grant: fail before claim and handler use.
- Unresolved independent policy, approval, evidence, or check prerequisite:
  fail before claim and handler use.
- Duplicate claim: return the existing bounded duplicate posture after fresh
  authority resolution; do not invoke the handler.
- Cursor mismatch after claim: burn the stale claim and require a new
  rehydration; do not invoke the handler.
- Known local consumer failure: preserve the existing executor failure.
- Ambiguous consumer outcome: remain explicit and block automatic retry.

Errors must use stable Core-owned codes and must not echo IDs, paths, source
records, context targets, policy text, command output, provider payloads,
credentials, environment values, or secret-like caller input.

## 11. Atomicity And Replay Posture

The current registered-source use boundary proves same-call freshness but not
cross-process replay prevention. The authoritative continuation boundary
already provides a durable cursor-bound first-writer claim for the selected
operation.

This phase composes those guarantees:

- source authority is freshly resolved in the same call;
- its bounded commitments become part of the exact continuation claim;
- only one worker may consume that cursor/action/commitment binding; and
- a stale post-claim cursor burns the claim.

This is not a reusable authority ledger and does not prove transactional
atomicity between an external future authority source and the local state
backend. A process crash after claim creation but before a durable consumer
outcome also remains a conservative ambiguous-recovery case. Those broader
claims remain deferred.

## 12. Workflow Semantics

Default executor behavior remains unchanged. The existing immutable-only
continuation path remains available exactly as reviewed for operations that do
not declare independent source-backed requirements.

The source-backed path is additive and crate-private. It must not alter run
pass/fail semantics, approval semantics, policy ordering, retry behavior,
event ordering, handler registration, or preview output. A blocked source or
context path may fail the selected step through existing executor failure
semantics, but it may not silently fall back to immutable-only continuation.

## 13. Privacy And Redaction

The composition may retain only stable payload-free commitments and bounded
posture. It must not copy or expose:

- source inventories, grants, availability records, or context contents;
- prompts, model reasoning, source code, command output, parser output, logs,
  provider payloads, or sandbox output;
- paths, endpoints, environment values, credentials, authorization headers,
  private keys, cookies, or secret-like values; or
- raw approval, evidence, check, policy, or SideEffect payloads.

Debug output should expose posture and counts only. Source-local errors must be
mapped to stable continuation error codes before crossing the composition
boundary.

## 14. Test Plan

Focused tests should prove:

1. one ready registered source reaches the current-step local handler once;
2. source resolution occurs before continuation claim and handler invocation;
3. exact source, fact-set, assessment, and context commitments bind the claim;
4. duplicate consumers still produce one durable first writer;
5. stale cursor after claim blocks before handler invocation;
6. expired and revoked grants block with zero continuation claim and handler
   calls;
7. missing, unavailable, stale, or changed source posture blocks;
8. required-context gaps block while optional gaps preserve the accepted
   registered-source semantics;
9. unresolved policy, approval, evidence, and check prerequisites block;
10. substituted actor, run, step, harness, contract, immutable bundle, or
    invocation identity blocks;
11. a later call performs fresh source resolution rather than reusing a prior
    accepted assessment;
12. source-backed execution preserves existing hook and invocation event
    ordering;
13. immutable-only and default executor paths remain unchanged;
14. the public preview remains non-authoritative and non-consuming;
15. errors and Debug output do not leak forbidden material; and
16. existing current-authority, continuation, required-context, executor,
    approval, policy, hook, SideEffect, WorkReport, provider, and backend tests
    remain green.

## 15. Documentation And Compatibility

The implementation must update:

- this plan;
- the authoritative continuation parent plan;
- `ROADMAP.md`;
- one end-of-phase implementation report; and
- one focused maintainer/security review.

Because the first composition remains crate-private, it adds no public API or
schema compatibility promise. Documentation must state that trusted runtime
source configuration, public consumption, and broad operational enforcement
remain unimplemented.

## 16. Validation

Run:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- focused current-authority and local-executor tests
- `cargo test --workspace`
- `npm run check`
- `npm run check:integrations`
- `npm run check:docs`
- `git diff --check`

## 17. Implementation Sequence

1. Add exact private binding validation and source-backed commitment
   composition.
2. Compose the private registered-source `FnOnce` use boundary around the
   existing authoritative continuation consumer.
3. Add focused positive, blocked, stale, substitution, duplicate, ordering,
   and non-leakage tests.
4. Run complete repository validation.
5. Create the implementation report.
6. Perform focused maintainer/security review before declaring the P0 source-
   backed boundary complete.

## 18. Acceptance Criteria

- One exact local skill invocation is authorized by fresh registered-source
  resolution and one durable cursor-bound continuation claim.
- No blocked or failed source path reaches a continuation claim or handler.
- Source-backed commitments are bound without exposing source records.
- Duplicate and stale consumers fail before handler invocation.
- The public preview remains orientation only.
- Default and immutable-only executor behavior remain unchanged.
- No public source/configuration API, provider write, nested harness runtime,
  schema, hosted behavior, or release change is introduced.
- Focused maintainer/security review accepts the implementation.

## 19. Recommended Follow-Up

Begin the separately scoped P0 authorized-execution continuity lane for
execution windows, executor yield, typed waits, scoped delegated grants, and
authoritative resume directives. Do not resume provider mutation broadening or
nested harness runtime merely because this private proof is accepted.
