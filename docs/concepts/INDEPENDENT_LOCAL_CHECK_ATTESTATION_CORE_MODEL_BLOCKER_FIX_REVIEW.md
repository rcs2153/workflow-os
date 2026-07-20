# Independent Local Check Attestation Core Model Blocker Fix Review

## 1. Executive Verdict

Blockers fixed; proceed to pure verifier planning.

The corrected model now has deterministic proof identity independent of record
naming and commits the complete independent-check requirement identity. It
remains explicitly unverified and adds no runtime authority.

## 2. Scope Verification

The fix stayed within the accepted core-model boundary. It did not add a
verifier, process observation, check execution, registration, persistence,
events, executor enforcement, schema, CLI, UI, provider behavior, writes,
hosted behavior, or release changes.

## 3. Original Blocker Resolution

### Caller-Chosen Record Identity

Resolved. `attestation_id` remains a validated record identifier but is no
longer included in `compute_binding_fingerprint`. Focused tests construct two
otherwise identical candidates with different record IDs and assert identical
proof fingerprints.

### Missing Complete Requirement Identity

Resolved. `LocalCheckAttestationRequirement` computes a domain-separated,
length-framed fingerprint over command identity, minimum assurance, canonical
accepted statuses, freshness, exact immutable-run binding posture, and
truncation policy. Candidate bindings store and fingerprint that requirement
fingerprint. Requirement serde recomputes it and fails closed on mismatch.

## 4. Determinism Assessment

Accepted statuses are sorted before fingerprinting, so equivalent status sets
have one identity. Stable known vectors protect the requirement and candidate
algorithms. Field-sensitivity tests cover every valid requirement variation;
invalid assurance weakening and immutable-binding relaxation fail validation.

Record identity and proof identity now have separate responsibilities. A later
store must enforce create-only record-ID conflicts independently; that is not a
verifier blocker.

## 5. Authority And Privacy Assessment

All candidates remain `Unverified`. A caller can describe a kernel-shaped
candidate but cannot construct an accepted attestation. Caller, mock, and future
external assurance cannot define the v0 independent requirement.

The fix stores only another fingerprint. No raw output, command data, paths,
environment values, source, credentials, or provider payloads were introduced.
Requirement fingerprints are redacted in `Debug`, and serde errors remain
fixed and non-leaking.

## 6. Test And Validation Assessment

Focused and workspace tests pass. Formatting, workspace clippy with warnings
denied, docs checks, and diff checks pass. The added tests directly prove both
blocker resolutions and preserve the existing tamper, privacy, freshness,
source/assurance, and unverified-posture coverage.

## 7. Remaining Blockers

None for pure verifier planning.

## 8. Non-Blocking Follow-Ups

- The future store must define record-ID conflict and exact-idempotency behavior
  separately from proof identity.
- The verifier must recompute the requirement fingerprint from the supplied
  requirement rather than trusting the candidate field.
- The verifier must obtain command, handler, invocation, result, and timestamp
  facts from kernel-owned inputs, not from the candidate.

## 9. Recommended Next Phase

Plan the pure independent local check attestation verifier. The verifier should
accept explicit kernel-owned observation and immutable-definition inputs,
recompute all fingerprints, evaluate time-of-use freshness and requirement
alignment, and return a distinct accepted value only on success.

Do not execute checks, persist records, emit events, integrate executor gates,
expose schemas or CLI behavior, invoke providers, or add writes in that plan.

## 10. Governed Review Record

- Workflow: `dg/review`
- Run ID: `run-1784510167951036000-2`
- Approval ID: `approval/run-1784510167951036000-2/review-scope-approved`
- Presentation ID: `presentation/e22dad02af457ec5`
- Approval outcome: granted with persisted presentation proof
- Out-of-kernel work: source and test inspection, maintainer reasoning,
  documentation edits, and validation commands
