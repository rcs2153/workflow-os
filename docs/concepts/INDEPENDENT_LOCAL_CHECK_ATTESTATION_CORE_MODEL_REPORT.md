# Independent Local Check Attestation Core Model Report

## 1. Executive Summary

Workflow OS now has a domain-neutral model for declaring independent local
check proof requirements and constructing payload-free attestation candidates.
Every candidate is explicitly `unverified`. Public construction and
deserialization cannot create accepted proof, and caller or mock assurance
cannot define an independent-check requirement.

No check verifier, process execution, persistence, event, executor gate,
schema, or CLI behavior was added.

## 2. Scope Completed

- Added validated attestation candidate IDs and a versioned framed fingerprint
  algorithm.
- Added assurance and source vocabulary with deterministic alignment checks.
- Added bounded no-reuse and maximum-age freshness policies.
- Added independent-check requirements that require kernel-observed assurance,
  at least one accepted result status, and exact immutable-run binding.
- Added deterministic complete requirement fingerprints with canonical accepted
  status ordering.
- Added payload-free candidate bindings over command, immutable bundle,
  workflow/run/step, invocation, idempotency, handler, result, observation,
  truncation, and freshness posture.
- Added explicit `unverified` verification posture as the only representable
  candidate state.
- Kept caller-chosen attestation record IDs outside canonical proof identity.
- Added validated serde, safe `Debug`, fixed non-leaking errors, and focused
  model tests.

## 3. Scope Explicitly Not Completed

No verifier, accepted-attestation type, process observation, check execution,
handler registration, persistence, events, executor enforcement, cache,
automatic retry, evidence/report attachment, schema, CLI, UI, provider call,
write, remote attestation, signing, hosted runner, or release change was added.

## 4. Model Boundary

The model separates four postures:

1. a requirement states what future independent proof must establish;
2. source and assurance describe a claim about where a result came from;
3. a binding deterministically commits the payload-free candidate context;
4. verification remains absent and cannot be inferred from model validity.

`kernel_observed_local_process` is eligible input vocabulary for the future v0
verifier. It is not trusted merely because a caller selects the enum variant.
`external_verifier` remains future vocabulary and is not eligible for v0
verification.

## 5. Fingerprint And Freshness Boundary

The v1 binding uses domain-separated, length-framed SHA-256 fields and has a
stable known test vector. Decision-relevant identity and posture changes alter
the fingerprint. Raw output, summaries, command arguments, paths, environment
values, source contents, and provider payloads are not stored.

Result duration was deliberately excluded from the first identity model. The
planning review identified it as unresolved disclosure versus identity
vocabulary; it should not introduce nondeterministic binding until a concrete
gate requires it.

Freshness is modeled but not evaluated. Time-of-use freshness belongs in the
future verifier.

## 6. Validation And Privacy

- Caller/mock/external assurance cannot define an independent requirement.
- Source and assurance pairs must align.
- Passed results require zero-exit posture; failed results require non-zero;
  unavailable/error postures require unavailable exit posture.
- Observation completion cannot precede observation start.
- Maximum age is bounded to 30 days and cannot be zero.
- Serialized fingerprint tampering fails closed.
- Deserialization cannot introduce a `verified` posture.
- `Debug` redacts identities, fingerprints, and observation timestamps.
- Errors do not echo caller values.

## 7. Test Coverage

Focused tests cover valid requirement serde, caller/mock insufficiency,
explicit unverified construction, v0 verifier eligibility vocabulary,
fingerprint stability and sensitivity, fingerprint tamper rejection,
non-leaking invalid verification posture, payload exclusion, source/assurance
alignment, observation ordering, and freshness bounds.

## 8. Validation Commands

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `git diff --check`

All listed commands passed. The workspace test command briefly waited for the
shared artifact-directory lock, then completed successfully without test
failure.

## 9. Known Limitations

- The model cannot prove that a process ran.
- The model cannot prove that a caller claiming kernel observation is truthful.
- No canonical command-contract fingerprint helper exists yet.
- No handler binary/version identity is currently verified.
- Freshness and immutable identity alignment are not evaluated at time of use.
- No accepted record, store, event, evidence, or report citation exists.

## 10. Recommended Next Phase

Perform a focused blocker-fix review. The initial review found that caller ID
changed proof identity and the binding omitted complete requirement identity;
the focused fix now removes the ID from canonical proof identity and commits a
complete requirement fingerprint. Re-review those changes before implementing
a pure verifier.

Do not integrate executor gates, persist records, emit events, expose schemas or
CLI behavior, or broaden provider mutations before that verifier is separately
implemented and reviewed.

## 11. Governed Phase Record

- Workflow: `dg/implement`
- Run ID: `run-1784508509057445000-2`
- Approval ID: `approval/run-1784508509057445000-2/implementation-approved`
- Presentation ID: `presentation/9c36701b2964ebcd`
- Approval outcome: granted with persisted presentation proof
- Out-of-kernel work: Rust model and test edits, documentation edits, and local
  validation commands
