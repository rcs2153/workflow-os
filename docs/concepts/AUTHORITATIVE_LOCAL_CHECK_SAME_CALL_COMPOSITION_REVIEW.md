# Authoritative Local-Check Same-Call Composition Review

## 1. Executive Verdict

**Phase accepted; proceed to authoritative aggregate-fact reassessment binding
planning.**

The crate-private helper closes the intended composition gap without changing
executor behavior. It derives obligations from canonical stored declarations,
preflights the complete supplied batch before process execution, derives
required-versus-optional posture inside Core, executes through the accepted
same-call attestation path, evaluates exact coverage, and returns the
provenance-bearing aggregate fact.

## 2. Scope Verification

The implementation stayed within the approved private, explicit, unwired
scope.

It did not add:

- executor checkpoints or automatic local-check execution;
- proportional-governance invocation or quiet-success enforcement;
- public API, schema, SDK, CLI, onboarding, or example changes;
- persistence, events, evidence records, reports, or artifacts;
- providers, OpenShell, SideEffects, or writes;
- hosted or distributed execution; or
- release-posture changes.

## 3. Authority Boundary Assessment

The helper accepts one `StoredImmutableRunBundle`, one step identity, and an
explicit borrowed batch of existing private execution inputs. It derives the
candidate obligation set and declaration record from that stored bundle.

Every supplied execution must match the canonical bundle, workflow, run, step,
attestation-requirement fingerprint, command identity, command kind, and
command-contract fingerprint. Unexpected and duplicate obligations fail
before execution.

The caller cannot supply requirement level, leaf contribution, structural
coverage, aggregate posture, or aggregate fact identity. This boundary is
accepted.

## 4. Preflight And Execution Assessment

The helper completes deterministic validation for the entire supplied batch
before entering its execution loop. A mismatch in a later input therefore
prevents an earlier valid process from starting.

Accepted inputs are mapped by canonical obligation fingerprint and then
reconstructed in canonical candidate order. Each accepted input executes once
through `execute_docs_check_governance_contribution(...)`, preserving the
existing command, observation, attestation, freshness, and bounded-result
boundary.

A runtime error returns no aggregate outcome. Earlier non-source-writing local
checks may already have run when a later runtime error occurs; the
implementation and report disclose that limitation and do not claim
transactional rollback.

## 5. Coverage And Aggregate Assessment

Core derives required-versus-optional posture from the canonical obligation:

- omitted required obligations become `RequiredUnavailable`;
- omitted optional obligations become `OptionalUnavailable`;
- an executed optional failure remains `Failed`;
- complete passing coverage becomes `Satisfied`; and
- a canonical empty declaration set executes nothing and may become
  `Satisfied`.

The helper evaluates exact structural coverage and converts it through the
previously reviewed authoritative aggregate-fact path. It returns the complete
fact with candidate, coverage, and fact commitments rather than a detached
posture enum.

One passing leaf therefore cannot claim aggregate satisfaction while another
declared obligation is failed, unavailable, or missing.

## 6. Failure And Privacy Assessment

Composition errors use stable
`local_check_attestation.composition.*` codes and static messages. Tests prove
that mismatched run identity and bounded process output do not appear in
errors or `Debug`.

The outcome's `Debug` exposes only result count and the aggregate fact's
already-redacted representation. It does not expose result payloads,
identities, paths, command text, fingerprints, source contents, environment
values, credentials, provider payloads, or natural-language report content.

## 7. Test Quality Assessment

Focused coverage proves:

- successful authoritative composition;
- omitted required and optional obligations;
- canonical empty declarations;
- an executed optional failure;
- duplicate and unexpected executions;
- a later batch mismatch preventing all process and clock use;
- command-contract mismatch before process and clock use;
- execution error returning no aggregate outcome;
- Core-owned requirement-level derivation; and
- bounded `Debug` and error behavior.

Existing tests continue to cover missing and legacy declaration sources,
unknown steps, authoritative identity validation, exact coverage, aggregate
fact invalidation, attestation, freshness, and immutable execution binding.

The current workflow schema admits one local-check requirement for a step.
Consequently, a direct multi-obligation caller-order permutation test would be
artificial today. Canonical ordering is implemented through the candidate
obligation sequence and should receive a direct multi-obligation regression
when the declaration model legitimately supports more than one requirement
per step.

## 8. Product And User-Feedback Assessment

Fresh-pull evaluation confirms that Workflow OS now presents a coherent,
honest local governance-kernel experience and that the next product pressure
is reducing ceremony for eligible low-risk work without weakening evidence.

The two bounded evaluator defects reported with that review are already fixed
and accepted on this phase's base:

- Node 24 integration-check output exhaustion now fails actionably; and
- `validate` renders the missing-manifest diagnostic once.

Those fixes are documented in
[Fresh-Pull Evaluator UX And Tooling Fix Report](FRESH_PULL_EVALUATOR_UX_AND_TOOLING_FIX_REPORT.md)
and its
[review](FRESH_PULL_EVALUATOR_UX_AND_TOOLING_FIX_REVIEW.md).
They do not reopen this phase or alter sequencing.

This implementation advances the evaluator's substantive recommendation:
quiet success can only be trustworthy when the kernel derives complete current
check posture from authoritative declarations and observed results.

## 9. Validation

Completed successfully:

- focused local-check runtime tests: 24 passed;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`; and
- `git diff --check`.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Add direct multi-obligation canonical-order permutation coverage when the
  declaration model supports multiple legitimate obligations per step.
- Keep the helper crate-private until one separately reviewed reassessment
  consumer binds both aggregate posture and fact fingerprint.
- Do not broaden to automatic executor checks, additional check families, or
  sandbox providers through this helper.

## 12. Governed Review Record

- workflow: `dg/review`
- run: `run-1785018957139803000-2`
- approval:
  `approval/run-1785018957139803000-2/review-scope-approved`
- presentation: `presentation/7c3b2463f9377778`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- validation summary: focused tests, formatting, strict clippy, full workspace
  tests, docs check, and diff check passed
- out-of-kernel work: source inspection, test inspection, review authoring,
  validation commands, and documentation updates
- missing coverage: the kernel coordinated governance only; it did not execute
  engineering checks or generate a persisted WorkReport artifact

## 13. Recommended Next Phase

Plan one private authoritative aggregate-fact reassessment binding.

That binding must consume the complete fact and its fingerprint, resolve fresh
runtime context, and preserve monotonic proportional-governance semantics. It
must remain explicit and unwired during its first phase.

Do not skip directly to executor-wide automatic checks, provider or OpenShell
execution, SideEffects, writes, hosted behavior, or default quiet-success
activation.
