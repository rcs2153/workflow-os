# Authoritative Continuation Registered Current-Authority Consumer Report

## 1. Executive Summary

Workflow OS now has one crate-private source-backed continuation composition
that freshly resolves registered current authority, consumes the exact
required-context contract, binds that result into the existing durable
cursor-bound continuation claim, and only then invokes the current local skill
consumer.

The implementation preserves the governing invariant that conversation memory
may orient an agent but cannot authorize the next material action. It adds no
public authority API, runtime source configuration, provider mutation, nested
harness runtime, or reusable permission object.

## 2. Scope Completed

- Added a private registered-current-authority continuation-use input adjacent
  to the local executor.
- Added a crate-private opt-in executor builder for the selected composition.
- Added exact static and durable binding checks for workflow, run, step,
  original execution actor, harness contract, contract hash, and immutable run
  bundle.
- Added one private payload-free continuation commitment over the accepted
  source snapshot, fact set, assessment, and required-context consumption.
- Composed fresh source resolution and required-context consumption before the
  existing durable continuation claim and local handler.
- Preserved the existing hook, SideEffect disclosure, invocation-event,
  attempt-idempotency, and local-handler path.
- Corrected approval-resume reconstruction so it retains the immutable run
  bundle and original execution actor.

## 3. Scope Explicitly Not Completed

- No public current-authority source or consumer API.
- No workflow-declared or runtime-configured authority source.
- No provider call, provider mutation broadening, OpenShell integration,
  sandbox execution, SideEffect execution, or additional write behavior.
- No typed child runtime, nested harness runtime, schema, SDK, CLI, event,
  artifact, hosted, or release change.
- No reusable authority grant, lease, session, or serialized permission.

## 4. Composition Summary

The selected executor path validates immutable identity before recording a new
invoke-policy decision. After policy acceptance and local handler resolution,
it rehydrates the durable run, confirms that the run is still `Running`, and
validates the durable immutable bundle and execution identity.

It then calls the existing private registered-source use boundary. Only a
`Ready` capability can derive the domain-separated source-backed continuation
commitment. The executor combines that commitment with its existing bounded
governance material, projects the exact continuation brief, consumes the
existing durable first-writer claim, rereads the cursor, and enters the
existing authorized local skill path once.

## 5. Approval-Resume Integrity

The end-to-end test found two defects in the existing approval-resume plan
reconstruction: the immutable bundle was not retained and the approval actor
was being used where the original execution actor was required. Both are now
corrected. Approval remains a decision by the approver; it does not rewrite
the identity of the execution that requested approval.

## 6. Failure And Replay Posture

Invalid static bindings fail before source use. Durable binding drift, blocked
or unavailable authority, required-context gaps, source failures, and invalid
commitment composition fail before continuation claim or handler use. Existing
duplicate and stale cursor semantics remain authoritative.

The phase proves fresh same-call authority plus one durable local first writer.
It does not prove transactional atomicity with a future external authority
source. Crash-after-claim ambiguity remains deferred.

## 7. Privacy And Redaction

The composition retains only stable payload-free commitments and bounded
posture. Source inventories, grants, availability records, context contents,
IDs, paths, prompts, command output, provider payloads, environment values,
credentials, and secret-like caller input are not exposed through public
serialization, Debug output, or errors.

## 8. Test Coverage

Focused coverage proves:

- deterministic payload-free commitment derivation for ready authority;
- blocked authority cannot derive or use a continuation commitment;
- one real approval-gated immutable-run execution resumes through fresh
  registered authority and reaches the local handler exactly once;
- blocked authority and stale source posture create zero continuation claims
  and invoke zero handlers through the composed executor path;
- contract substitution fails without leaking either contract identifier;
- the durable continuation claim exists before handler entry and invocation
  events retain the expected policy, resume, request, attempt, success, and
  terminal ordering;
- replay of a completed approval decision does not invoke the handler or
  create another continuation claim;
- the same registered source is freshly reassessed at each executor use time;
- the durable run keeps its immutable bundle and original execution actor;
  and
- existing current-authority, executor, approval, provider, WorkReport,
  backend, and hosted tests remain green.

## 9. Validation

- `cargo fmt --all --check`: passed
- `cargo clippy --workspace --all-targets -- -D warnings`: passed
- focused source-backed approval-resume test: passed
- `cargo test --workspace`: passed
- `npm run check`: passed
- `npm run check:integrations`: passed
- `npm run check:docs`: passed through `npm run check`
- `git diff --check`: passed after the blocker-fix documentation pass

## 10. Governed Implementation Record

- workflow ID: `dg/implement`
- run ID: `run-1786776224280642000-2`
- approval ID:
  `approval/run-1786776224280642000-2/implementation-approved`
- approval presentation ID: `presentation/994b1b22b40b1f72`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted handoff was presented
- out-of-kernel work: code and documentation inspection, implementation,
  testing, and report authorship were performed by the delegated maintainer;
  the kernel governed scope and approval but did not edit files, execute
  checks, or mutate git

## 11. Remaining Limitations

- The source-backed executor path remains crate-private and test/composition
  only.
- Trusted operational source configuration is not implemented.
- Continuation-use event and report projection are not implemented.
- External operations are not intercepted by this path.
- Crash recovery after claim but before a durable consumer outcome remains
  conservative and ambiguous.

## 12. Recommended Next Phase

Repeat focused maintainer/security review of the composition-level blocker
fix. Only after acceptance should Workflow OS plan first-class authorized
execution windows, executor yield, typed wait conditions, and scoped delegated
authority before claiming uninterrupted autonomous continuation.
