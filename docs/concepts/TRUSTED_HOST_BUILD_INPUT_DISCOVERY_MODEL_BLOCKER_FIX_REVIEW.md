# Trusted-Host Build-Input Discovery Core Model Blocker Fix Review

## 1. Executive Verdict

**Blockers fixed; proceed to fixture-only structured metadata parser and graph
resolver implementation.**

The repair closes both findings from the original model review. Version-one
canonical framing is architecture independent, request and result commitments
are pinned, and the discovery wire boundary now rejects unknown top-level and
nested fields without echoing caller-supplied names or values.

## 2. Scope Verification

The fix stayed within the approved model boundary. It changed canonical field
framing, discovery-only deserialization, focused tests, and phase documents.
It did not add repository reads, fixture parsing, TOML parsing, Cargo or process
execution, provider calls, handler registration, persistence, runtime events,
reports, artifacts, schemas, CLI or SDK behavior, writes, capability grants, or
authority expansion.

## 3. Canonical Framing Assessment

`hash_field` now encodes label and value lengths as eight-byte big-endian `u64`
values. The encoded width no longer depends on host `usize`, so supported
32-bit and 64-bit hosts produce the same bytes for the same inputs.

The request fingerprint and result graph commitment both have fixed vectors.
The request vector protects the transitive request binding directly. The result
vector remains unchanged on the current 64-bit host, as expected, while the
source type now makes the cross-architecture contract explicit.

## 4. Strict Wire Boundary Assessment

Private discovery-specific wire representations reject unknown fields at:

- the request definition;
- the result definition;
- each host-obligation entry;
- each reused input-kind count entry; and
- the optional reused observation-reference entry.

Unknown values are consumed without being copied into the public model, and
the resulting errors are fixed generic text. Reused public Core types were not
globally tightened, avoiding an unrelated compatibility change.

## 5. Validation And Compatibility Assessment

Valid request and result serialization shapes are unchanged. Deserialization
still recomputes derived commitments and re-enters the established request and
result constructors. Reused counts and references are reconstructed through
their validated constructors before the enclosing result is accepted.

The intentional compatibility tightening is limited to rejecting additional
unknown fields that were previously ignored at this exact evidence-binding
boundary. That is the planned fail-closed behavior, not an accidental public
model redesign.

## 6. Privacy And Error Assessment

The model remains payload free. It stores no paths, manifests, lockfiles,
package identities, source contents, commands, outputs, environment values,
credentials, provider payloads, or unknown wire data.

Focused regressions inject secret-like field names and values at every repaired
boundary and prove they are absent from returned errors. Existing redaction-safe
Debug behavior and value-independent closed-enum errors remain intact.

## 7. Test Quality Assessment

The 22-test focused suite covers both fixed vectors, unknown top-level request
and result fields, unknown nested obligation/count/reference fields,
non-leakage, validated serde reconstruction, exact request binding,
completeness coherence, obligation coverage, count reconciliation, and the
absence of runtime dependencies.

The complete local matrix and all seven GitHub CI jobs passed. The first local
workspace run reported two unrelated concurrent scheduler-test failures; each
exact test passed independently, the complete workspace rerun passed, and the
GitHub Rust job passed without retrying this change.

## 8. Remaining Blockers

None.

## 9. Non-Blocking Follow-Ups

- Before this wire contract is exposed through an untrusted public ingress,
  either reject the first unknown field with a custom visitor or enforce a
  reviewed outer document-size bound. The current private map is safe for the
  model-only phase but temporarily retains unknown field names until rejection.
- Add invalidation tests for each request commitment field when the fixture
  parser begins producing requests.
- Add a target-specific canonicalization job only if 32-bit Rust becomes a
  supported build target; the fixed-width source contract does not require one
  for acceptance today.
- Continue treating public construction of a valid result as a claim, not proof
  that repository discovery occurred.

## 10. Validation

- focused discovery model suite: passed, 22 tests;
- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed on the complete rerun;
- `npm run check`: passed under the repository Node 20 toolchain;
- `npm run check:integrations`: passed;
- `npm run check:docs`: passed;
- `git diff --check`: passed; and
- GitHub CI: all seven required jobs passed for PR 645.

Documentation checks and diff hygiene must be rerun after this review and
roadmap update.

## 11. Recommended Next Phase

Implement the fixture-only structured metadata parser and graph resolver from
the accepted proof plan. Keep it deterministic, root-bounded, process-free,
network-free, provider-free, and non-authorizing. It may parse only the closed
metadata set, must fail closed for unsupported or changing inputs, and must
produce the already reviewed request/result model.

Do not combine fixture parsing with live workspace proof, host observation,
provider integration, persistence, schemas, runtime execution, or writes.

## 12. Governed Review Evidence

- Dogfood workflow: `dg/review`.
- Run ID: `run-1792825802583092000-2`.
- Approval ID:
  `approval/run-1792825802583092000-2/review-scope-approved`.
- Presentation ID: `presentation/cfb08370188f3559`.
- Approval outcome: granted under standing delegated-maintainer authority
  through the proof-enforced approval path.
- Final status: `Completed`.
- Event summary: 39 events, one approval, zero retries, and zero escalations;
  one matching presentation record and its marker are present in the approval
  event trail.
- Out-of-kernel work: the delegated maintainer inspected implementation,
  tests, plans, reports, local validation, and merged CI evidence and authored
  this review. The kernel governed scope and approval only.
- Report posture: this document is the focused review record. No runtime
  WorkReport or report artifact was generated or persisted.
