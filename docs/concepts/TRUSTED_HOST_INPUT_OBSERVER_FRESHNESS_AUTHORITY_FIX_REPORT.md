# Trusted-Host Input Observer Freshness Authority Fix Report

## 1. Executive Summary

The trusted-host input observer freshness-authority blocker is fixed. Requests
and results now bind one exact payload-free ordering-authority commitment, and
static-discovery completion plus observation start/completion revisions are
explicitly defined in that shared domain.

The fix remains model-only and non-authorizing.

## 2. Blocker Fixed

The prior model compared a static-discovery completion revision with revisions
documented as producer local. Numeric comparison across unbound counters could
not prove temporal order.

The model now requires:

- one ordering-authority commitment in the request;
- the same ordering-authority commitment in the result;
- static-discovery completion, observation start, and observation completion
  revisions interpreted only within that shared authority;
- exact authority equality during result-to-request validation; and
- observation start strictly after static-discovery completion.

## 3. Implementation Approach

`TrustedHostInputObservationRequestDefinition` and
`TrustedHostInputObservationResultDefinition` each carry an
`ordering_authority: SpecContentHash`. The opaque commitment identifies the
shared monotonic revision authority without introducing a clock, path,
provider payload, or runtime service.

The request and result canonical commitments include this field. Debug output
redacts it. Missing wire fields fail closed through existing strict definition
deserialization.

## 4. Validation Boundary Summary

Result construction retains nonzero and monotonic observation revision checks.
`validate_for_request` now verifies exact ordering-authority equality before
comparing the observation start revision to static-discovery completion.

Authority substitution, stale observation, request substitution, repository
substitution, and producer substitution remain distinct fail-closed paths with
stable payload-free errors.

## 5. Privacy And Redaction Summary

The ordering authority is a fixed-size content commitment. It stores no raw
clock data, paths, host values, environment values, commands, provider
payloads, credentials, or tokens. Debug output redacts the commitment and
errors do not include it.

## 6. Test Coverage Summary

Focused tests prove:

- changing the request ordering authority changes its canonical commitment;
- changing the result ordering authority changes its canonical commitment;
- authority substitution fails result-to-request validation;
- a missing serialized authority fails closed;
- stale observation revisions fail closed in the exact shared domain; and
- pinned request/result vectors capture the intentional contract change.

Existing producer, provenance, reason, coherence, privacy, serde, and
determinism tests remain green.

## 7. Commands And Results

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed; opt-in live tests remained ignored behind
  their existing environment gates.
- `npm run check:docs`: passed.
- `git diff --check`: passed.
- Sixteen focused observer tests passed.

## 8. Remaining Known Limitations

- The model does not create or operate the ordering authority.
- No callable observer, host inspection, process execution, provider
  integration, fixture proof, persistence, admission, events, schemas, CLI,
  writes, or capability grants exist in this phase.
- Producer assurance and ordering-authority identity remain validated claims;
  later provider review decides what may satisfy live admission.

## 9. Recommended Next Phase

Perform a focused maintainer/security review of this freshness-authority fix.
Verify shared-domain semantics, substitution rejection, canonical binding,
privacy, and preservation of the model-only non-authorizing boundary before
accepting the Core model or beginning fixture-backed proof.

## 10. Governed Phase Evidence

- Workflow ID: `dg/blocker`.
- Run ID: `run-1792838042219999000-2`.
- Approval ID: `approval/run-1792838042219999000-2/fix-approved`.
- Approval presentation ID: `presentation/bbfd1b6993bb4c87`.
- Approval presentation content hash:
  `bbfd1b6993bb4c878d15ca88e130dcfba591fe95c831b8c95d927a3371b43b19`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact freshness-authority blocker-fix scope.
- Phase status: `Completed`.
- Event summary: 39 ordered events, including one approval request, one
  approval grant, six scheduled steps, six successful skill invocations, zero
  retries, and zero escalations. Approval presentation enforcement was
  proof-enforced and its event marker was present.
- Out-of-kernel work: Codex implemented the model and tests, authored this
  report, and ran validation. The kernel governed scope and approval but did
  not edit files or execute commands.
