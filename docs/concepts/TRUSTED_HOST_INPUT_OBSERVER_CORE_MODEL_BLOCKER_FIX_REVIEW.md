# Trusted-Host Input Observer Core Model Blocker Fix Review

## 1. Executive Verdict

Needs additional blocker fixes.

Six of the seven original findings are fixed. The remaining freshness check
compares revisions that the model documents as belonging to different
ordering authorities, so it cannot prove that observation occurred after
static discovery.

## 2. Scope Verification

The fix stayed within the approved model-only boundary. It added no callable
observer, host inspection, filesystem traversal, environment access, process
execution, provider integration, fixture proof, runtime admission,
persistence, workflow events, schemas, CLI behavior, examples, writes,
capability grants, or release changes.

The shared enum deserializer changes are broader than the observer module but
are limited to making existing wire vocabulary non-echoing. The full workspace
suite passed, and no runtime semantics changed.

## 3. Producer Identity And Provenance Assessment

Fixed. Requests bind an exact expected producer ID and version. Results carry
one validated producer and every fact carries the producer ID/version that
created it. Result construction enforces the documented homogeneous-producer
rule, rejecting any substituted fact producer or version.

Producer identifiers are bounded, reject secret-like text, redact Debug
output, and fail deserialization without echoing invalid values.

## 4. Non-Proof Reason And Fact Coherence Assessment

Fixed. Observed facts require no reason, an observation method, and both
binding axes proven. Declared-only, unavailable, unsupported, and uncertain
facts each require their exact closed reason. Every non-observed posture
rejects either proven binding axis, including partial false-proof states.

The complete posture/method matrix is closed. `Uncertain` requires an actual
observation method rather than unavailable or unsupported vocabulary.

## 5. Producer And Method Coherence Assessment

Fixed. Fixture producers may assert observed facts only through the fixture
method. Host observers cannot claim fixture or provider-native methods.
Execution providers may use the bounded provider-capable method set.
Caller-supplied assurance cannot assert observed proof.

These are claim-coherence rules only. They do not trust a producer or grant
admission authority.

## 6. Nested Wire Privacy Assessment

Fixed. Root role, root access, root observation posture, root observation
method, and hosted execution reference kind now use closed manual
deserializers with fixed errors. Focused tests exercise every nested path with
secret-like unknown input and prove the value is not echoed. Workspace tests
show the shared enum changes preserve existing valid wire behavior.

## 7. Freshness Authority Assessment

Blocker remains.

The request adds `static_discovery_completed_revision`, while the result fields
are explicitly documented as `producer-local` start and completion revisions.
`validate_for_request` compares those numbers directly. A static-discovery
revision of 10 and an observer-local revision of 11 do not prove temporal
ordering unless both counters are bound to the same sequence identity and
authority.

The current test proves only numeric comparison. It does not prove a shared
ordering domain, reject a substituted domain, or establish that the observer
started after the exact static discovery operation.

Required fix: represent and bind one closed ordered-revision authority/domain
shared by discovery completion and observation, or replace the comparison with
an equivalent request-bound non-clock freshness challenge whose semantics do
not depend on unrelated counters. The canonical request/result commitments
must include that binding and substitution tests must fail closed.

## 8. Canonical Commitment Assessment

The version-two domain, expected producer ID/version, per-fact provenance,
closed reasons, and static revision value are all committed deterministically.
Fact ordering remains canonical, and pinned request/result vectors detect
wire changes.

The hashing implementation is correct for represented fields. It cannot make
the current cross-domain revision comparison semantically authoritative; the
remaining fix requires an explicit domain or equivalent freshness binding.

## 9. Test Quality Assessment

Focused coverage is strong for the six closed findings: producer substitution,
per-fact provenance, missing/wrong reasons, partial proof, method coherence,
caller-supplied false proof, nested error non-leakage, secret-like producer
identifiers, serde revalidation, deterministic ordering, and pinned vectors.

The freshness test is shallow because it changes only revision numbers. The
remaining fix needs tests for shared-domain equality, domain substitution,
static-result substitution, and stale/replayed observation rejection.

## 10. Privacy And Non-Authorizing Boundary

The fix stores only closed vocabulary, bounded identifiers, commitments,
counts, ordered revisions, and optional stable telemetry references. It stores
no paths, environment values, source content, command output, provider
payloads, executable bytes, credentials, or tokens.

No model value executes a process, observes a host, trusts a provider, admits
runtime work, persists state, or grants authority.

## 11. Blockers

1. Bind static-discovery completion and observation revisions to one explicit
   shared ordering authority/domain, or use an equivalent request-bound
   non-clock freshness mechanism. Reject domain substitution and stale replay,
   commit every semantic field, and add focused regressions.

## 12. Non-Blocking Follow-Ups

- A later provider review must decide which producer identities and methods
  are trusted for live admission.
- Mixed-producer result composition remains deferred; the homogeneous rule is
  acceptable for version two.
- Fixture-backed proof and live observation remain separate future phases.

## 13. Recommended Next Phase

Trusted-host input observer freshness-authority blocker fix.

Fix only the shared ordering/freshness relation and its canonical/test
coverage. Do not add a callable observer, inspect a host, execute a process,
integrate a provider, admit runtime work, persist observations, add events,
schemas, CLI behavior, examples, writes, or authority.

## 14. Validation

- `cargo fmt --all --check`: passed on reviewed commit `aa6581f1`.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed on reviewed
  commit `aa6581f1`.
- `cargo test --workspace`: passed on reviewed commit `aa6581f1`; opt-in live
  tests remained ignored behind their existing environment gates.
- `npm run check:docs`: passed before this review update.
- `git diff --check`: passed before this review update.

## 15. Governed Review Evidence

- Workflow ID: `dg/review`.
- Run ID: `run-1792837935390670000-2`.
- Approval ID:
  `approval/run-1792837935390670000-2/review-scope-approved`.
- Approval presentation ID: `presentation/ad483aa855ef6a02`.
- Approval presentation content hash:
  `ad483aa855ef6a02f4a337790d3eb77d83539b560c82b3cf584471f4c4bcad53`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact review-only scope.
- Phase status: `Completed`.
- Event summary: 39 ordered events, including one approval request, one
  approval grant, six scheduled steps, six successful skill invocations, zero
  retries, and zero escalations. Approval presentation enforcement was
  proof-enforced and its event marker was present.
- Out-of-kernel work: Codex inspected the implementation, tests, plans, and
  reports and authored this review. The kernel governed scope and approval but
  did not perform the review.
