# Authoritative Local-Check Reassessment Binding Review

## 1. Executive Verdict

**Phase accepted; proceed to one opt-in executor-consumer planning phase.**

The private implementation closes the intended fact-to-reassessment identity
gap. It preflights deterministic immutable context before clock or process use,
creates the authoritative local-check fact in the same call, injects only the
selected step's evidence/check posture, reassesses the complete immutable
workflow, and returns one private value that keeps fact and assessment
authority inseparable.

## 2. Scope Verification

The phase stayed within the approved private, in-memory, unwired scope.

It did not add:

- executor integration or automatic local-check execution;
- quiet-success activation or operator presentation behavior;
- workflow-state mutation, persistence, events, evidence, reports, or
  artifacts;
- public APIs, serialization, schemas, SDK, CLI, UI, onboarding, or examples;
- providers, OpenShell, SideEffects, external writes, or network access;
- hosted or distributed behavior;
- reasoning lineage; or
- release-posture changes.

## 3. Preflight Assessment

`preflight_authoritative_local_check_reassessment(...)` first reuses the pure
immutable-bundle resolution boundary. That establishes:

- exact immutable workflow, skill, and referenced-policy resolution;
- exact runtime-fact count and step membership;
- selected-step membership;
- one unambiguous selected runtime fact; and
- absence of caller-selected evidence/check posture for the selected step.

It then derives canonical selected-step declarations and invokes the existing
complete local-check composition preflight. A runtime-fact mismatch, caller
posture, declaration mismatch, or command mismatch therefore fails before
clock or process use.

Final workload assessment necessarily remains after the current check
observation exists. A deterministic assessment error at that later boundary
returns no bound value and does not claim rollback of a completed
non-source-writing local check.

## 4. Authority And Ownership Assessment

The wrapper accepts the existing same-call local-check composition input. It
does not accept a detached aggregate fact, aggregate posture, leaf
contribution, coverage candidate, prior binding, or caller-selected
evidence/check posture for the selected step.

After composition, it verifies that the produced candidate-set fingerprint
matches the candidate derived during exact preflight. Only then does Core clone
the runtime-fact set and replace the selected step's absent evidence/check axis
with the authoritative fact posture.

The returned `AuthoritativeLocalCheckBoundAssessment` privately owns:

- the complete authoritative aggregate local-check fact;
- the complete immutable-bundle assessment set; and
- the versioned fact-to-assessment binding fingerprint.

Its accessors expose only the algorithm, binding identity, bounded local-check
posture, and assessment count. No accessor or conversion returns the raw fact
or raw assessment set as independently reusable authority. The surrounding
module is private and the new types remain crate-private.

This boundary is accepted.

## 5. Binding Identity Assessment

The v1 length-framed binding commits to:

- its algorithm domain;
- immutable bundle ID, version, and root;
- workflow, run, and selected-step identity;
- aggregate local-check fact algorithm and fingerprint;
- candidate-set and structural-coverage fingerprints;
- selected assessment algorithm and input fingerprint; and
- complete assessment-set algorithm and aggregate fingerprint.

The stable known vector and ambiguous-field framing regression are present.
Tests also prove that equal posture with different authoritative fact identity
changes the binding and that a changed non-check authority axis changes the
binding.

## 6. Governance Semantics Assessment

The authoritative fact changes one selected evidence/check axis only.
Authority, SideEffect, prior-decision, runtime-escalation, steward-minimum, and
profile inputs pass through the existing immutable-bundle assessment path.

The combined focused and existing component tests prove:

- satisfied checks can preserve eligible proceed posture;
- optional unavailable checks retain visible disclosure;
- required unavailable checks retain denial;
- executed failed checks remain failed and therefore denied;
- satisfied checks cannot weaken unavailable authority; and
- non-check governance changes invalidate binding identity.

The helper produces assessed authority but does not enforce, persist, or
present it.

## 7. Failure And Privacy Assessment

New failures use stable
`local_check_attestation.reassessment_binding.*` codes with static messages.
Candidate mismatch and missing selected-assessment errors do not echo
identifiers or fingerprints.

`Debug` exposes only bounded algorithm, posture, and count metadata. It redacts
results, fact contents, assessment contents, and binding fingerprints.
Commands, arguments, paths, source or spec contents, environment values,
process output, provider payloads, credentials, and report text are not
exposed.

If check execution fails, no assessment or binding is returned. If
reassessment fails after check execution, no bound value is returned.

## 8. Test Quality Assessment

Focused coverage includes:

- successful fact-bound reassessment;
- selected caller posture rejection before clock and process use;
- malformed runtime-fact shape before clock and process use;
- candidate identity preservation through the existing composition preflight;
- optional and required unavailable semantics;
- monotonic unavailable-authority behavior;
- equal posture with distinct fact identity;
- non-check-axis binding invalidation;
- post-check reassessment failure without a bound value;
- length-framing ambiguity resistance;
- a stable v1 fingerprint vector; and
- bounded `Debug` behavior.

The full workspace suite passed, including existing immutable-bundle,
proportional-governance, local-check attestation, executor, WorkReport,
SideEffect, adapter, and runtime-event tests.

## 9. Documentation Assessment

The roadmap, implementation plan, and implementation report accurately state
that:

- the private binding is implemented;
- executor consumption is not implemented;
- automatic checks and quiet success are not active;
- the result is in memory and not durable authority;
- persistence, events, providers, OpenShell, SideEffects, writes, schemas,
  hosted behavior, and release changes remain out of scope; and
- one separately governed executor-consumer planning phase is next.

One report sentence was narrowed during review so it attributes the complete
semantic proof to focused binding tests plus the existing accepted component
tests rather than to the new tests alone.

## 10. Validation

Completed successfully:

- `cargo check -p workflow-core`;
- focused `workflow-core` local-check runtime tests: 34 passed;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`; and
- `git diff --check`.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Keep the binding crate-private until a separately reviewed consumer can
  accept the inseparable bound value.
- When planning the first multi-step executor consumer, add a regression for
  deterministic public error precedence across an early assessment error and a
  later immutable-context error.
- Do not use this helper to broaden automatic checks, provider execution,
  OpenShell integration, SideEffects, or write authority.

## 13. Governed Review Record

- workflow: `dg/review`
- run: `run-1785023273650284000-2`
- approval:
  `approval/run-1785023273650284000-2/review-scope-approved`
- presentation: `presentation/01db476b760f8a0b`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- validation summary: focused tests, formatting, strict clippy, full workspace
  tests, docs check, and diff check passed
- out-of-kernel work: source inspection, test inspection, review authoring,
  validation commands, and documentation updates
- missing coverage: the kernel coordinated governance only; it did not execute
  engineering checks, edit files, or create a persisted WorkReport artifact

## 14. Recommended Next Phase

Plan one opt-in executor consumer of the private fact-bound reassessment value.

The plan should define exactly where the executor may run current declared
local checks, how the bound assessment is consumed without detaching its fact,
how workflow semantics remain monotonic, and what failure or disclosure is
returned when checks or reassessment fail.

Automatic/default checks, broad quiet-success activation, persistence, events,
providers, OpenShell, SideEffects, writes, schemas, hosted behavior, and
release changes must remain separately governed.
