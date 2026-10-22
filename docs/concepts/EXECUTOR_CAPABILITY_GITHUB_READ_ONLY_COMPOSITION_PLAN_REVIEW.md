# Executor Capability-Gated GitHub Read-Only Composition Plan Review

## 1. Executive Verdict

**Plan accepted with mandatory implementation constraints.**

The plan selects the right next runtime-composition boundary: one crate-private,
explicit, read-only executor path that composes existing immutable-run,
policy, current-authority, source-continuity, continuation, adapter, telemetry,
and receipt foundations. The generic `SkillHandler` remains the wrong authority
boundary because its mapped values do not prove the complete governed context.

The review resolves the plan's open event, operation-selection, evidence, and
transaction questions below. Implementation may proceed only if those
resolutions are preserved. This acceptance does not authorize a second
provider operation, provider mutation, public/default behavior, CLI or schema
exposure, runtime configuration, hosted identity, OpenShell, nested harnesses,
or release changes.

## 2. Scope Assessment

The proposed slice is appropriately narrow. It composes one already accepted
GitHub repository-metadata read and leaves ordinary executor and adapter APIs
unchanged unless the private path is selected explicitly.

The plan does not authorize:

- another GitHub read operation or another provider;
- comments, issues, pull requests, merges, or any provider write;
- ambient credentials or automatic grants;
- automatic handler registration or default capability gating;
- workflow, skill, policy, harness, schema, SDK, example, CLI, or UI changes;
- report artifact persistence;
- hosted or enterprise identity behavior;
- OpenShell or nested harness execution; or
- reasoning-lineage or release-posture changes.

## 3. Runtime Boundary Assessment

A closed Core operation adjacent to `LocalExecutor` is the correct first
boundary. The operation must act on an existing immutable-bundle-backed run and
the exact current actionable step. It must not create a run, execute unrelated
steps, or accept serialized capability projections, receipts, source history,
or continuation state as authority.

The operation should participate in the existing step invocation lifecycle,
not operate as an untracked side channel. Its private API may accept injected
dependencies and expected identities, but Core must rehydrate and compare every
authoritative run, bundle, step, policy, source, and capability fact.

## 4. Generic Handler Assessment

The plan correctly rejects the generic `SkillHandler` as the authority
boundary. `SkillInput` contains useful immutable workflow identity and mapped
string values, but it does not carry or independently prove:

- the immutable bundle root;
- the current event cursor;
- the harness contract;
- the registered authority source;
- source generation and watermark continuity;
- the exact selected grant and projection; or
- authenticated policy-precheck provenance.

Adding these values as strings would make caller-authored data look like
authority. The dedicated path must resolve them inside Core.

## 5. Operation Selection Resolution

The private method name or a caller-supplied enum is not sufficient proof that
the immutable current step selected repository metadata. The first
implementation must derive a private operation binding from the immutable
bundle's exact step and resolved skill definition.

Without a schema change, the accepted minimum is a crate-private constructor
that requires all of the following:

- the exact immutable step and skill identity;
- the canonical symbolic GitHub read-only adapter requirement;
- the expected bounded owner and repository input contract;
- the required read-only capability posture; and
- a domain-separated commitment that includes the constant
  `github.repository.metadata` operation code.

The constructor must be private and must reject missing, additional,
incompatible, or substituted operation-contract inputs. The caller may request
the dedicated API, but it may not supply a prevalidated binding.

This is acceptable only for the private opt-in slice. Any public, automatic,
or multi-operation adoption needs explicit authored operation vocabulary and a
separate schema review rather than additional hidden conventions.

## 6. Admission Ordering Resolution

The implementation must preserve this ordering:

1. rehydrate the run from managed SQLite state;
2. validate the immutable bundle and exact current step;
3. derive the private operation binding from the immutable step and skill;
4. re-evaluate policy and construct the allowed precheck inside Core;
5. resolve the registered current-authority exact-query fact set;
6. validate freshness, completeness, grant scope, availability, prerequisites,
   sensitivity, and step-scoped projection;
7. reconcile durable source continuity;
8. construct the operation-specific governed-continuation commitment;
9. consume one cursor-bound continuation claim;
10. revalidate the cursor immediately after claim consumption;
11. append the write-ahead invocation request and attempt-start events;
12. invoke the injected GitHub read-only adapter once; and
13. persist bounded telemetry and close the invocation according to the exact
    outcome.

Failures through cursor revalidation must make zero provider calls. Source
continuity may be durably observed before the continuation claim because it is
source history, not permission or proof that the provider ran.

## 7. Event Vocabulary Resolution

Existing invocation events are sufficient for this first slice only when the
dedicated operation is the current step's skill attempt:

- `SkillInvocationRequested` identifies the exact current invocation;
- `SkillInvocationStarted` is the durable write-ahead marker before provider
  entry;
- `SkillInvocationSucceeded` is appended only after the read, required
  telemetry persistence, and receipt construction succeed; and
- `SkillInvocationFailed` is appropriate only when the provider read itself
  fails under existing retry semantics.

`ReadSucceededReceiptFailed` must not be collapsed into provider failure or
success. The run must close or pause with a stable post-read receipt/evidence
failure while retaining the started invocation and successful adapter
telemetry posture. It must not append `SkillInvocationSucceeded`, because the
governed invocation did not complete its required receipt boundary, and it
must not retry the provider blindly.

If existing run-failure or pause semantics cannot represent that sequence
without contradiction, implementation must stop as a blocker rather than add
an unreviewed event type.

## 8. Transaction And Replay Resolution

The provider call cannot be part of an SQLite transaction. The correct local
boundary is at-most-once dispatch with explicit ambiguity, not fictional
atomicity.

The continuation claim and write-ahead invocation events must make these
restart states distinguishable:

- no claim: provider entry was not authorized;
- claim consumed without invocation-start event: dispatch did not reach the
  durable provider-entry marker and remains burned pending explicit recovery;
- invocation started without terminal invocation outcome: the provider may
  have run and automatic retry is forbidden;
- provider failure recorded: retry follows existing reviewed retry policy with
  a fresh operation attempt; and
- invocation success recorded: the bounded read, telemetry, and receipt
  completed.

An exact concurrent consumer must observe the consumed claim and perform zero
provider calls. An ambiguous restart must surface a stable non-leaking posture;
it must not infer success or retry from absence of a success event.

No V8 schema expansion is required if existing idempotency, event, and
telemetry records prove these states. If they do not, implementation must stop
and plan the smallest durable operation journal separately.

## 9. Policy And Authority Assessment

Policy success and capability authority must remain independent. The executor
must evaluate policy from the immutable current step and construct the adapter
precheck internally. It must not accept an arbitrary allowed
`AdapterPolicyPrecheck` from the adopting caller.

Current authority must be resolved for the exact actor, workflow, run, step,
harness, repository resource, sensitivity, and evaluation time. A broad,
expired, revoked, unavailable, incomplete, ambiguous, stale, or
prerequisite-bearing grant remains non-authorizing.

The accepted durable source-continuity store remains the source of first,
replayed, or advanced posture. Continuity never authorizes the read and never
permits a second provider call.

## 10. Evidence Resolution

The current `EvidenceReference` taxonomy has adapter invocation and response
summary kinds, but no capability-use authority-receipt kind. The first
implementation must therefore return the trusted
`CapabilityUseAuthorityReceipt` and bounded adapter telemetry separately. It
must not label a receipt as an audit event, policy decision, or another
nearby-but-false evidence kind.

An in-memory evidence reference may be included only if it truthfully cites an
existing adapter invocation identifier using the existing adapter evidence
taxonomy. Receipt-specific EvidenceReference or WorkReport citation vocabulary
requires separate planning. No report artifact is written in this slice.

## 11. Failure Semantics Assessment

The implementation must preserve four non-interchangeable outcomes:

- blocked before read;
- provider read failed;
- read and governed receipt boundary succeeded; and
- read succeeded but receipt or required post-read projection failed.

All admission errors must use stable fixed codes and messages. They must not
echo repository identity, source values, grants, commitments, paths,
credentials, headers, provider payloads, or command output.

The dedicated operation may affect only its current step according to existing
terminal behavior. It must not retroactively rewrite unrelated workflow
results or claim that a provider read failed when only receipt construction
failed.

## 12. Privacy And Redaction Assessment

The plan's privacy posture is accepted with these requirements:

- repository owner and name remain out of events, Debug, errors, evidence
  summaries, and report summaries by default;
- provider payloads remain inside the adapter result boundary and are not
  persisted by the composition;
- durable continuity stores commitments, generation, and watermark only;
- receipts remain payload-free and unverified after deserialization;
- telemetry uses the existing bounded redacted records; and
- SQLite main, WAL, and shared-memory files are scanned for forbidden fixture
  markers in focused tests.

## 13. Test Assessment

The proposed test plan is appropriately broad for a security-sensitive
composition. Implementation coverage must include, at minimum:

- exact immutable operation-binding derivation and substitution rejection;
- zero provider calls for every admission failure;
- at-most-one provider call under concurrent exact consumers;
- write-ahead invocation ordering;
- restart handling for unclaimed, claimed-only, started-ambiguous, failed, and
  successful postures;
- source first observation, replay, advancement, rollback, and substitution;
- independent policy denial and capability denial;
- truthful provider-failure versus post-read receipt-failure behavior;
- bounded telemetry and receipt output without fabricated evidence;
- repository, source, grant, credential, and payload non-leakage; and
- unchanged ordinary executor, adapter, report, and state behavior.

The full workspace format, Clippy, test, documentation, and diff checks remain
required.

## 14. Blockers

None at the planning boundary, provided implementation treats Sections 5
through 10 as mandatory constraints.

Implementation must stop and report a blocker if it cannot:

1. derive the exact private operation binding from immutable run content;
2. represent post-read receipt failure truthfully with existing event
   semantics; or
3. reconstruct an explicit ambiguous restart posture from existing durable
   claim and event records.

These are fail-closed implementation conditions, not permission to weaken the
contract.

## 15. Non-Blocking Follow-Ups

1. Add explicit authored adapter-operation vocabulary before any public,
   automatic, or multi-operation adoption.
2. Add receipt-specific evidence or report citation vocabulary only after a
   separate taxonomy review.
3. Add process-level concurrency coverage before multi-worker or hosted use.
4. Consider a dedicated durable operation journal only if existing event and
   idempotency state cannot represent restart ambiguity exactly.

## 16. Governed Review Record

- workflow ID: `dg/review`
- run ID: `run-1792702501455334000-2`
- approval ID:
  `approval/run-1792702501455334000-2/review-scope-approved`
- presentation ID: `presentation/77bc67717e10ab9d`
- presentation content hash:
  `77bc67717e10ab9d003d96915a695ae199774f4f9c23a013fa4ed0394d29e884`
- approval outcome: granted under delegated-maintainer authority after review
  of the complete persisted handoff
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- approval-presentation enforcement: proof enforced with one matching persisted
  presentation record and event marker
- validation summary: `npm run check:docs` and `git diff --check` passed; Rust
  checks were not run because this review changed documentation only
- kernel boundary: governance coordination only; source inspection, review
  authorship, validation, Git, pull-request, CI, and merge work remain outside
  the kernel
- out-of-kernel work: source inspection, review authorship, local documentation
  validation, Git operations, pull-request publication, CI observation, and
  merge handling

## 17. Recommended Next Phase

Implement the one crate-private executor capability-gated GitHub
repository-metadata read composition under the mandatory constraints in this
review. Keep the operation explicit and opt-in. Perform focused
maintainer/security review before public adoption, a second provider
operation, or any provider mutation.
