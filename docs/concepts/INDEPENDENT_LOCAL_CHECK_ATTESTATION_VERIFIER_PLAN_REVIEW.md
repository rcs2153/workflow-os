# Independent Local Check Attestation Verifier Plan Review

## 1. Executive Verdict

**Needs planning blocker fixes.**

The plan correctly protects accepted-proof construction, freshness, payload
privacy, and runtime non-scope. It is not implementation-ready because it
assumes the current immutable run bundle freezes the local check command and
handler identity. It does not.

## 2. Scope Verification

The phase stayed within planning-only scope. It did not add verifier code,
process execution, persistence, events, runtime wiring, schemas, CLI behavior,
providers, SideEffects, writes, hosted behavior, or release changes.

## 3. Findings

### Blocker 1: Command Contract Is Not In The Immutable Run Bundle

The plan tells the verifier to resolve exactly one command definition from the
stored immutable run bundle. Current
`ImmutableRunBundleDefinitionKind` supports only `Workflow`, `Skill`, and
`Policy`. A `LocalCheckCommandContract` is neither a bundle definition record
nor a separately bound immutable execution input.

Passing a current command contract into the verifier and comparing it with a
caller-supplied candidate fingerprint proves internal consistency at evaluation
time. It does not prove that this was the contract frozen for the run before
execution.

Action required: define a deterministic immutable local-check execution binding
created before execution. It must commit the canonical command-contract
fingerprint to the exact run, step, skill/handler selection, and immutable bundle
binding. Decide whether this is a new bundle definition kind or a separate
content-addressed binding referenced by the resolved execution context. Do not
claim current bundle membership until that prerequisite exists.

### Blocker 2: Handler Fingerprint Has No Trusted Immutable Source

The candidate carries a caller-supplied `handler_fingerprint`. The current bundle
handler reference binds only skill ID/version and one honest posture:
`DeclaredOnly`, `RegisteredUnattested`, `MockSelected`, or `Unavailable`. It does
not bind a handler implementation identity or prove that a registered handler is
attested.

A crate-private observation can prove which Core path reported an invocation,
but without a pre-execution immutable handler commitment the verifier cannot
prove that the implementation was the one selected for the run.

Action required: the same immutable local-check execution binding must commit a
Core-derived handler identity/fingerprint and honest registration posture.
`MockSelected`, `DeclaredOnly`, `Unavailable`, and unbound handler identity must
remain insufficient. If v0 intentionally proves only kernel-observed process
execution under a registered-but-unattested handler, name that assurance
honestly and do not imply implementation attestation.

## 4. Accepted Design Elements

- Crate-private observation and verifier visibility is the correct first boundary.
- A public read-only accepted type with no public constructor is appropriate.
- Deferring `Deserialize` prevents serialized data from minting accepted authority.
- Requirement and candidate fingerprints must be recomputed.
- Freshness must be evaluated at the supplied evaluation time and again at use.
- Stable non-leaking rejection codes are appropriate.
- An accepted check remains evidence/check posture, not approval, capability,
  policy, SideEffect, or execution authority.
- Proportional-governance presentation remains separate from execution disposition.

## 5. Test Plan Assessment

The planned mismatch, freshness, privacy, status, policy, and non-leakage tests
are strong. Add tests proving:

- the immutable local-check execution binding is created before observation;
- command and handler commitments are included in its root/fingerprint;
- a current but unbound command contract is rejected;
- a current but unbound handler fingerprint is rejected;
- bundle/run/step/skill/check-binding cross-combinations fail closed;
- mock and registered-unattested postures cannot be mislabeled as implementation attestation;
- changing the execution binding invalidates the candidate even when the
  command ID and result status remain identical.

## 6. Privacy And Error Assessment

No privacy blocker was found. The plan excludes raw output, arguments, paths,
source contents, environment values, credentials, and provider payloads. The
blocker fix must preserve reference-only/fingerprint-only immutable bindings and
stable errors that do not echo identifiers or payloads.

## 7. Relationship To External Feedback

The plan aligns with the external feedback that mocks are not execution evidence
and real checks require stronger attestation. Fixing the immutable-input gap is
necessary before Workflow OS can honestly present accepted check proof as a
proportional-governance fact.

The constraint-first onboarding feedback does not authorize inference here.
Repository metadata may recommend checks, but only immutable exact runtime
bindings and kernel-owned observations may establish accepted proof.

## 8. Blockers

1. Select and define the immutable command-contract binding source.
2. Select and define the immutable handler identity/posture binding source and
   its honest v0 assurance claim.

## 9. Non-Blocking Follow-Ups

- Decide whether accepted records should serialize only when persistence is planned.
- Consider compile-fail tests for public construction boundaries without adding
  a dependency solely for this phase.
- Keep record-ID uniqueness separate from proof identity in future stores.
- Fix the dogfood phase-close presentation-record list cap separately.

## 10. Recommended Next Phase

Run a focused verifier planning blocker fix. Define an immutable local-check
execution binding and update verifier inputs, algorithm, tests, sequencing, and
non-goals around that prerequisite. Then repeat focused plan review.

Do not implement the verifier, executor integration, or broader provider writes.

## 11. Validation

- `npm run check:docs`
- `git diff --check`
- direct inspection of immutable bundle kinds, handler posture, local check
  contracts/results, attestation candidate fields, and related roadmap plans

## 12. Governed Review

- workflow: `dg/review`
- run: `run-1784512509438967000-2`
- approval: `approval/run-1784512509438967000-2/review-scope-approved`
- presentation: `presentation/9ca8727f082b0562`
- outcome: granted by delegated maintainer through proof enforcement
- work performed outside kernel: repository inspection, review writing, and
  documentation validation

## 13. Fix-Forward Status

The focused blocker phase defines a separate, content-addressed
`ImmutableLocalCheckExecutionBinding` created before observation. It references
the stored immutable run bundle and commits the canonical command contract,
Core-derived registered-handler selection metadata and honest `RegisteredUnattested`
posture, and effective execution policy. The corrected plan no longer claims
that current immutable bundle definition records contain local-check commands.

It also narrows the first assurance to `KernelObservedLocalProcess`: Core
observed a process under the exact pre-bound registered handler and policy. It
does not claim handler implementation integrity. This review's original blocker
finding remains part of the record; focused re-review is still required before
implementation.
