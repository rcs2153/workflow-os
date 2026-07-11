# Provider Write Sandbox Readiness Helper Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The provider-write sandbox readiness helper is a narrow, pure checkpoint before
any live sandbox provider mutation. It returns bounded allow, deny, and defer
postures from explicit caller-supplied inputs, and it does not call providers,
load credentials, append workflow events, mutate stores, write report artifacts,
expose CLI behavior, add schemas, add examples, broaden adapter writes, or
change default executor write behavior.

Do not proceed directly to broader/default writes from this review. The next
roadmap work should either review a future sandbox-auth/source plan before live
provider mutation, or, based on recent external evaluator feedback, prioritize
current-product contract hardening for CLI/docs/onboarding before more write
expansion.

## 2. Scope Verification

The phase stayed within approved helper-only scope.

Implemented:

- `ProviderWriteSandboxReadinessInput`;
- `ProviderWriteSandboxReadinessResult`;
- `ProviderWriteSandboxReadinessDecision`;
- bounded posture enums for target, auth, approval, side effect, event proof,
  and provider/local reconciliation;
- bounded issue vocabulary;
- `assess_provider_write_sandbox_readiness(...)`;
- focused tests;
- roadmap, plan, and phase report updates.

No accidental implementation was found for:

- provider calls;
- hidden auth loading from environment, keychain, GitHub CLI, git config, OAuth,
  or secret managers;
- workflow event append;
- side-effect store mutation;
- report artifact writing;
- CLI mutation behavior;
- workflow schema changes;
- examples;
- hosted or distributed runtime behavior;
- broad GitHub, Jira, CI, filesystem, HTTP, or arbitrary provider writes;
- automatic retry or repair;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 3. Helper API Assessment

The helper API is appropriately explicit and minimal for this phase.

The input carries only posture and bounded model values:

- write capability;
- bounded `AdapterWriteTarget`;
- explicit sandbox target classification;
- explicit auth-source posture;
- approval requirement and approval posture;
- SideEffect attempted posture;
- event-proof requirement and event-proof posture;
- provider/local reconciliation posture;
- sensitivity;
- redaction metadata.

The result exposes:

- readiness decision;
- bounded issue list;
- retry-blocked posture;
- operator-action-required posture;
- sensitivity and redaction metadata accessors;
- explicit false authority flags for provider calls, workflow event appends,
  side-effect record writes, and report artifact writes.

This shape is the right bridge between existing write-readiness primitives and
future live sandbox testing. It does not infer runtime state, load stores, load
credentials, or pretend to authorize mutation.

## 4. Readiness Gate Assessment

The implemented gate logic is conservative and deterministic.

The helper allows only when:

- capability is `GitHubPullRequestComment`;
- target posture is `ExplicitSandbox`;
- auth posture is `ExplicitCallerSupplied`;
- required approval is linked and approved;
- SideEffect posture is `Attempted`;
- required event proof is present;
- provider/local posture is not ambiguous or unknown.

It denies unsupported capability, non-sandbox target posture, missing or hidden
auth posture, missing required approval, denied approval, missing attempted
SideEffect posture, and missing required event proof.

It defers ambiguous or unknown provider/local posture, blocks retry, and
requires operator action. That is the correct posture before any retry or
recovery mutation exists.

## 5. Runtime Boundary Assessment

The helper does not mutate runtime state.

Verified boundaries:

- no `WorkflowRun` mutation;
- no `WorkflowRunSnapshot` mutation;
- no event-history append;
- no audit emission;
- no observability emission;
- no `SideEffectRecordStore` write;
- no `WorkReport` or artifact write;
- no provider invocation;
- no auth loading;
- no CLI output surface;
- no default executor behavior change.

The explicit false authority accessors are useful because they make the boundary
inspectable for future composition code and tests.

## 6. Privacy And Redaction Assessment

The helper remains reference/posture-only.

It does not carry or copy:

- provider payloads;
- provider response bodies;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- command output;
- raw CI logs;
- raw GitHub issue, pull request, or file contents;
- raw spec contents;
- report artifact text.

`ProviderWriteSandboxReadinessInput` has redaction-safe `Debug`. The result has
redaction-safe `Debug` and custom serialization that emits redacted redaction
metadata rather than caller-supplied redaction field names or reasons.

Validation errors for invalid target and redaction metadata use stable,
non-leaking codes:

- `provider_write_sandbox_readiness.target.invalid`;
- `provider_write_sandbox_readiness.redaction.invalid`.

## 7. Serialization And Compatibility Assessment

The new posture enums use stable snake-case serialization vocabulary. The
result serialization is intentionally bounded and redacts redaction metadata.

The input does not introduce a public serialized request shape. That is
appropriate because this phase is a pure in-process helper and not a schema or
CLI surface.

No workflow spec schema changes were introduced.

## 8. Test Quality Assessment

Focused tests cover the important first slice:

- all gates satisfied returns `AllowedForSandbox`;
- missing explicit auth returns `Denied`;
- missing required approval returns `Denied`;
- missing attempted SideEffect posture returns `Denied`;
- missing required event proof returns `Denied`;
- ambiguous provider/local posture returns `Deferred`;
- production-like target returns `Denied`;
- unsupported capability returns `Denied`;
- result exposes no provider-call, event-append, side-effect-store, or
  artifact-write authority;
- Debug and serialization do not leak target strings, redaction values,
  token-like strings, or provider payload markers.

Full workspace validation passed during implementation.

Non-blocking test follow-ups:

- add focused coverage for `HiddenOrAmbient` auth posture specifically;
- add focused coverage for `Unknown` target/event-proof/provider-local posture
  combinations;
- add focused validation-error tests for invalid target and invalid redaction
  metadata once a serialized or external caller path is planned.

These are not blockers because the current helper logic handles those variants
conservatively and the public surface is not yet schema or CLI exposed.

## 9. Documentation Review

Documentation is honest about implemented and unimplemented scope.

The roadmap, runtime write-readiness checkpoint plan, and phase report state
that:

- the helper is implemented;
- provider calls are not implemented by this helper;
- hidden auth loading is not implemented;
- event append is not implemented by this helper;
- side-effect store mutation is not implemented by this helper;
- report artifact writing is not implemented by this helper;
- CLI mutation behavior is not implemented;
- schemas and examples are not added;
- hosted behavior, broader writes, automatic retry/repair, reasoning lineage,
  recursive agents, agent swarms, Level 3/4 autonomy, and release posture
  changes remain unsupported.

No dangerous false claims were found.

## 10. External Feedback Alignment

Recent external evaluator feedback emphasized that Workflow OS is credible as a
local governance kernel, but the next product risk is the gap between modeled
governance and first-use operator experience.

This review does not block the provider-write helper. It does, however, affect
roadmap sequencing. After this review, the project should avoid drifting into
more write-adjacent primitives without also tightening:

- CLI identity and `--version` behavior;
- docs drift around implemented initialization commands;
- generated scaffold documentation;
- clearer separation between real first-run posture and mock runtime demos;
- more concrete bridge from `first-run` recommendations to reviewed workflow
  authoring.

## 11. Blockers

No blockers.

## 12. Non-Blocking Follow-Ups

- Add the targeted posture-variant tests listed above.
- Keep the first live sandbox provider write behind a separate reviewed plan
  that defines explicit auth-source handling and sandbox target policy.
- Prioritize current-product contract hardening before expanding broader write
  capability unless a blocker requires otherwise.
- Continue disclosing that GitHub PR comments are the only write candidate lane
  modeled this deeply.

## 13. Recommended Next Phase

Recommended next phase: current-product contract hardening planning.

Reason: the provider-write sandbox readiness helper is accepted, but external
testing shows the more urgent preview-readiness gap is not another primitive.
It is tightening the first-use contract so users can trust the current kernel:
version/build identity, docs consistency, scaffold docs, and the bridge from
first-run posture to governed workflow authoring.

Do not implement provider writes, hidden auth loading, CLI mutation writes,
schemas, examples, hosted behavior, broad write-capable adapters, reasoning
lineage, or release posture changes as part of that next phase.

## 14. Governed Review Run

- workflow: `dg/review`;
- run ID: `run-1783747749020962000-2`;
- approval ID: `approval/run-1783747749020962000-2/review-scope-approved`;
- presentation ID: `presentation/e454814205e67ad5`;
- approval outcome: delegated maintainer approved;
- approval presentation enforcement: proof-enforced.

## 15. Validation

Validation commands for this review:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run check:docs
git diff --check
```

Result: passed.
