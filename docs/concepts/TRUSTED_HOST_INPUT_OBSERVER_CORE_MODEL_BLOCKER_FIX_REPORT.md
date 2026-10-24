# Trusted-Host Input Observer Core Model Blocker Fix Report

## 1. Executive Summary

The seven blockers from the trusted-host input observer Core model review are
fixed. The version-two contract now binds the expected producer, per-fact
provenance, non-proof reasons, producer/method coherence, nested wire privacy,
and observation freshness relative to static discovery.

The implementation remains provider neutral, process free, payload free, and
non-authorizing. It does not observe a host or admit execution.

## 2. Blockers Fixed

1. Requests bind an exact expected producer identity and version.
2. Every fact carries producer identity/version and a result requires all facts
   to match its producer claim.
3. Every non-proven fact requires one closed reason matching its posture.
4. Non-observed facts reject either proven binding axis and the complete
   posture/method matrix fails closed.
5. Observed fact methods must be coherent with producer kind and assurance;
   caller-supplied producers cannot assert observed proof.
6. Every reused enum reachable through the observer wire boundary uses a
   closed non-echoing deserializer.
7. Requests bind the static-discovery completion revision and result validation
   requires observation to begin strictly afterward.

## 3. Implementation Approach

The canonical contract advanced from `cargo_host_inputs_v1` to
`cargo_host_inputs_v2`. Producer identity and version are bounded validated
newtypes whose Debug output is redacted and whose deserialization errors do not
echo invalid input. Requests bind those expected values and the ordered static
discovery completion revision.

Facts now bind the producer identity/version and an optional closed reason.
Construction requires no reason for observed proof and an exact reason for
declared-only, unavailable, unsupported, or uncertain posture. Result
construction enforces one homogeneous producer across every fact and checks
observed methods against the producer kind and assurance.

## 4. Validation Boundary Summary

The fix validates semantic coherence at construction and deserialization.
`validate_for_request` additionally rejects request substitution, repository
substitution, producer substitution, producer-version substitution, and an
observation whose start revision is not later than static discovery.

Canonical request and result commitments include every new semantic field.
Pinned version-two vectors make intentional contract changes explicit.

## 5. Redaction And Privacy Summary

Producer identity/version values reject secret-like text and are redacted in
Debug output. Validation errors use fixed stable messages. Closed manual
deserializers now cover nested root role, access, binding posture, observation
method, and hosted reference kind values, preventing unknown caller values from
appearing in deserialization errors.

No paths, environment names or values, command output, source content,
provider payloads, executable bytes, credentials, or tokens are stored.

## 6. Test Coverage Summary

Focused tests cover expected producer and freshness binding, homogeneous
per-fact provenance, exact non-proof reasons, partial false-proof rejection,
posture/method coherence, producer/method coherence, caller-supplied false
proof rejection, nested enum error non-leakage, secret-like producer identity
and version rejection, deterministic ordering, pinned vectors, and existing
request/result behavior.

## 7. Commands And Results

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed; opt-in live tests remained ignored behind
  their existing environment gates.
- `npm run check:docs`: passed.
- `git diff --check`: passed.
- Sixteen focused observer tests passed, including the new blocker
  regressions.

## 8. Remaining Known Limitations

- No callable observer exists.
- No host, filesystem, environment, process, or provider is inspected.
- Producer assurance remains a validated claim rather than trusted admission
  evidence.
- Version two intentionally requires one homogeneous producer for all facts;
  mixed-producer composition is deferred.
- No static-plus-host assessment or runtime admission path exists.
- No persistence, events, schemas, CLI behavior, examples, writes, or authority
  are added.

## 9. Recommended Next Phase

Perform a focused maintainer/security review of this blocker fix. Verify each
of the seven original findings, pinned commitment behavior, privacy-safe wire
errors, and preservation of the model-only non-authorizing boundary before any
fixture-backed proof or live observation work.

## 10. Governed Phase Evidence

- Workflow ID: `dg/blocker`.
- Run ID: `run-1792836794377168000-2`.
- Approval ID: `approval/run-1792836794377168000-2/fix-approved`.
- Approval presentation ID: `presentation/6e5af93fdf76b92d`.
- Approval presentation content hash:
  `6e5af93fdf76b92d853edae23c806f81a6308de2b648e9b0258c23a9fa2db5ad`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact blocker-fix scope.
- Phase status: `Completed`.
- Event summary: 39 ordered events, including one approval request, one
  approval grant, six scheduled steps, six successful skill invocations, zero
  retries, and zero escalations. Approval presentation enforcement was
  proof-enforced and its event marker was present.
- Out-of-kernel work: Codex implemented the model and test changes, authored
  this report, and ran validation. The kernel governed scope and approval but
  did not edit files or run commands.
