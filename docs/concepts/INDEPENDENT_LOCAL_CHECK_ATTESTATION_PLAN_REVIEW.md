# Independent Local Check Attestation Plan Review

## 1. Executive Verdict

Plan accepted; proceed to independent local check attestation core model only.

The plan closes a real proof gap without pretending that a typed record, a
mock result, or a caller assertion is independently authoritative. Its source
of truth boundaries, immutable-run binding, freshness posture, privacy limits,
and incremental sequence are appropriate for the current kernel.

## 2. Scope Verification

The phase stayed planning-only. It did not add a model, verifier, handler,
process execution, registration, persistence, event, executor gate, schema,
CLI, UI, provider behavior, write, or release change.

## 3. Source-Of-Truth Assessment

The plan correctly keeps these concepts separate:

- a command contract declares an allowed check;
- a result describes a bounded outcome;
- a result reference cites that outcome;
- an evidence or report reference does not upgrade its assurance;
- a requirement declares the minimum proof a gate needs;
- an attestation binding describes payload-free proof context;
- a separately reviewed verifier is the only future boundary allowed to accept
  that context as satisfying an independent-check requirement.

Constructing or deserializing an attestation-shaped value must remain
non-authoritative. The proposed model-only phase must preserve that invariant in
type names, APIs, documentation, and tests.

## 4. Assurance And Source Assessment

`caller_asserted` and `mock_observed` are useful disclosure vocabulary and are
correctly barred from satisfying independent assurance. The proposed
`kernel_observed_local_process` source is appropriate future vocabulary, but
must not become trusted merely because a caller selects that enum variant.
Acceptance requires the later verifier over kernel-owned invocation facts.

`external_verifier` is appropriately deferred as vocabulary only. The plan
makes no cryptographic, hardware-backed, third-party, or hosted-runner claim.

## 5. Binding And Freshness Assessment

The candidate binding covers the decision-relevant identities: command
contract, immutable bundle, workflow/run/step, invocation and idempotency,
handler posture, execution policies, result posture, observation time, and
freshness requirement. This is sufficient for a model phase and correctly
follows the invariant that a decision-relevant input change invalidates reuse.

Freshness is evaluated at time of use rather than only at record construction.
Cross-run reuse, cache behavior, and automatic retry remain deferred. Before a
fingerprint contract is frozen, implementation should resolve whether duration
is decision identity or bounded disclosure; elapsed time alone should not make
otherwise identical proof nondeterministic without a concrete enforcement
reason.

## 6. Privacy And Failure Assessment

The payload-free posture is appropriate. Raw output, summaries, arguments,
paths, environment values, source contents, credentials, and provider payloads
are excluded. IDs, hashes, actor or handler references, and timestamps remain
potentially sensitive and require safe `Debug` and fixed non-leaking errors.

Failure behavior is conservative: mock, caller-created, stale, mismatched,
failed, skipped, unavailable, or unverifiable input cannot be converted into a
passing attestation. Persistence and event publication are separately deferred,
so the plan does not create partial accepted proof.

## 7. Testability Assessment

The test plan covers stable framing, field sensitivity, serde failure,
non-leakage, assurance insufficiency, immutable identity alignment, freshness,
result status, retry idempotency, and existing subsystem regressions. The
model-only implementation should not claim to test verifier, process, event, or
persistence behavior before those phases exist.

## 8. Planning Blockers

None.

## 9. Non-Blocking Follow-Ups

- Ensure `kernel_observed_local_process` cannot satisfy a requirement through
  public construction or deserialization before the verifier phase exists.
- Decide whether result duration belongs in the canonical identity before
  publishing a stable fingerprint vector.
- Select a canonical command-contract fingerprint only from validated command
  posture; do not copy command arguments or paths into attestation records.

## 10. Recommended Next Phase

Implement the independent local check attestation core model only:
requirement, assurance/source, freshness, and payload-free binding vocabulary
with validated constructors, safe serde, safe `Debug`, and focused tests.

Do not add the verifier, execute checks, register handlers, persist records,
emit events, enforce executor gates, expose schemas or CLI behavior, invoke
providers, or add writes.

## 11. Governed Review Record

- Workflow: `dg/review`
- Run ID: `run-1784508293122920000-2`
- Approval ID: `approval/run-1784508293122920000-2/review-scope-approved`
- Presentation ID: `presentation/d1ea6c57f7d3d809`
- Approval outcome: granted with persisted presentation proof
- Out-of-kernel work: repository inspection, maintainer reasoning,
  documentation edits, and validation commands
