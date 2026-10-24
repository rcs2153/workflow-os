# Trusted-Host Build-Input Discovery Core Model Review

## 1. Executive Verdict

**Needs blocker fixes. Do not begin fixture parsing or workspace discovery.**

The implementation has the correct product and security boundary. It is a
small non-authorizing Core model that binds one exact static-discovery request,
keeps static graph completeness separate from live build-input completeness,
requires every unresolved host obligation, and stores only bounded typed facts,
commitments, and one stable reference.

Two contract blockers prevent acceptance. Canonical field framing uses
machine-width `usize`, so fingerprints differ across host architectures. The
new serde definition structs also accept unknown fields, contradicting the
planned fail-closed wire posture and allowing uncommitted data to accompany an
otherwise valid request or result. Both fixes are narrow and local.

## 2. Scope Verification

The phase stayed within the approved model-only scope. It added request/result
types, closed vocabulary, validation, canonical commitments, serde, Debug, and
focused tests. The CI blocker fix changed only two concurrency-test loser
classifications and documentation after proving the one-entry invariant.

The phase did not read repositories, traverse filesystems, parse TOML, invoke
Cargo or build code, select a provider, register a handler, persist discovery
state, append runtime events, expose configuration or schemas, create CLI or
SDK behavior, enable writes, or grant execution authority.

## 3. Model Assessment

The public vocabulary is appropriately narrow and provider-neutral:

- one closed `StaticCargoGraphV1` contract version;
- explicit or conservative-all-potential-edge selection posture;
- static completeness distinct from live-input completeness;
- nine required host-obligation kinds with only unavailable or unsupported
  posture;
- bounded input-kind counts and total count;
- closed payload-free non-completeness reasons; and
- an optional stable telemetry reference.

The request and result own private fields and validated accessors. Public
definition structs are justified as constructor and serde boundaries. A valid
result remains a caller-constructed claim, not observed truth or authority.

## 4. Request And Result Binding Assessment

The request commits the contract version, repository tree, selection posture,
selection, metadata source set, and discovery policy. The result stores the
exact request fingerprint and `validate_for_request` rejects substitution with
a stable non-leaking error.

The result commitment transitively binds the request fields through that
fingerprint, then commits static completeness, total and per-kind counts, all
host obligations, unsupported reasons, and the optional telemetry reference.
Collections are sorted before hashing, so caller order does not change the
result commitment.

No ambient current directory, branch name, raw path, manifest text, package
identity, provider payload, or command output is accepted.

## 5. Canonicalization Assessment

Domain separation, closed tags, stable sorting, and length-prefixed fields are
the right design. The implementation also pins one result vector.

The length prefix is not architecture-stable. `hash_field` calls
`usize::to_be_bytes()` for label and value lengths. A 64-bit host writes eight
bytes per length while a 32-bit host writes four. Identical request and result
facts can therefore produce different fingerprints and graph commitments.
That violates the versioned canonical-encoding claim and repeats a previously
fixed proportional-governance blocker.

The blocker fix must use checked fixed-width framing, preferably `u64`
big-endian lengths, and update the version-one fixed vectors. It must pin both
the request fingerprint and result commitment so the transitive binding is
reviewable.

## 6. Completeness And Host-Obligation Assessment

`TrustedHostStaticGraphCompleteness::Complete` is explicitly limited to the
approved static metadata graph. It is not convertible to
`BuildTimeInputCompletenessPosture::Complete`.

Every result must contain exactly one entry for each of the nine closed host
obligations. Their only postures are `Unavailable` and `Unsupported`; the model
cannot claim that static parsing proved toolchain, target, sysroot, linker,
native tools, dynamic libraries, package sources, environment, or runtime-base
identity. This is the central honesty boundary and is correctly enforced.

Complete static posture rejects an empty graph and any unsupported reason.
Every non-complete posture requires at least one unique stable reason.

## 7. Validation And Serde Assessment

Construction correctly rejects oversized or incoherent counts, zero entries,
duplicate kinds, missing or duplicate host obligations, invalid reason
coherence, non-telemetry references, and request substitution. Derived hashes
are omitted from serialization and recomputed on deserialization.

The new wire shape is not fully fail closed. The request definition, result
definition, and host-obligation struct do not deny unknown fields. Serde
therefore ignores additional caller-supplied fields before constructing the
validated value. Those fields are neither represented nor committed, creating
an ambiguous extension channel at an exact evidence-binding boundary.

The blocker fix must reject unknown top-level and nested fields for the new
contract without echoing their names or values. If reused nested types cannot
be hardened compatibly, the discovery boundary needs private strict wire
types rather than a broad unrelated contract change.

## 8. Privacy And Redaction Assessment

The model is payload-free. It stores closed enums, bounded counts, commitments,
and a validated reference. It cannot store raw repository paths, manifests,
lockfiles, package names, source URLs, dependency contents, environment values,
tool locations, commands, logs, credentials, or provider payloads.

Custom Debug output redacts commitments and references. Unknown enum variants
use value-independent errors, including the reused build-input kind hardened by
this phase. Validation errors use stable
`trusted_host.build_input_discovery.*` codes and do not include rejected
commitments or references.

## 9. Public API And Authority Assessment

The exported API is minimal for an additive model contract. There is no
executor, provider, parser, state backend, event sink, capability, receipt,
handler, or filesystem dependency. Constructors return facts only, and no type
can authorize a process or provider call.

Future parser and observer implementations must not treat public construction
of a valid result as proof that discovery occurred. Observation provenance and
host facts remain separately required.

## 10. CI Concurrency Blocker-Fix Assessment

The post-implementation CI fix is accepted. Two existing composition tests
omitted lawful losing-caller outcomes already proved by the adjacent
competing-owner contract:

- `trusted_host_redispatch.directive_missing`; and
- `trusted_host_redispatch.active_yield_missing`.

Both outcomes occur before the loser receives usable attempt authority. The
tests still require one underlying executor call, one aggregate executor entry,
and no caller with more than one entry. Focused stress, the complete local
matrix, and the final GitHub CI run passed. Production behavior did not change.

## 11. Test Quality Assessment

Coverage is strong for deterministic same-input request identity, selection
sensitivity, order-independent result commitment, one fixed result vector,
request substitution, completeness/reason coherence, host-obligation coverage,
count reconciliation, reference-kind validation, redaction-safe Debug,
non-leaking unknown enum values, validated serde reconstruction, closed
vocabulary, and construction without runtime dependencies.

Missing blocker regressions:

- a fixed request-fingerprint vector under fixed-width framing;
- an updated fixed result vector under the same framing;
- rejection of unknown request fields;
- rejection of unknown result fields; and
- rejection of unknown nested obligation fields without value leakage.

A later parser phase, not this blocker fix, must test manifests, lockfiles,
workspace members, feature/target selection, source replacement, symlinks,
cycles, root escape, metadata drift, and bounds against fixtures.

## 12. Documentation Assessment

The roadmap, plan, and implementation report accurately describe the model as
implemented but unreviewed, non-authorizing, and disconnected from parser,
workspace, provider, and runtime behavior. The implementation report honestly
records the CI contention fix and all out-of-kernel work.

The report's deterministic canonicalization and fail-closed serde claims must
not be accepted until the two blockers are fixed and re-reviewed.

## 13. Blockers

1. Replace `usize` field-length framing with checked fixed-width framing and
   pin both request and result version-one vectors.
2. Reject unknown fields throughout the new request/result wire boundary,
   including nested host obligations, with stable value-independent errors.

## 14. Non-Blocking Follow-Ups

- Add explicit invalidation tests for every request commitment field.
- Consider a compile-time or target-specific canonicalization check in CI if a
  supported 32-bit target becomes part of the project matrix.
- Keep package identity details inside commitments until a reviewed diagnostic
  use case requires bounded public vocabulary.
- Do not let future fixture or workspace producers imply observation authority
  merely by constructing these public values.

## 15. Recommended Next Phase

Implement a focused trusted-host build-input discovery model blocker fix, then
perform focused re-review. Do not begin fixture parsing, workspace discovery,
host observation, provider integration, persistence, or execution work before
acceptance.

## 16. Validation

The implementation and CI blocker fix passed:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check`;
- `npm run check:integrations`;
- `npm run check:docs`; and
- `git diff --check`.

Documentation checks and diff hygiene must be rerun after this review document
and roadmap update.

## 17. Governed Review Evidence

- Dogfood workflow: `dg/review`.
- Run ID: `run-1792823351373549000-2`.
- Approval ID:
  `approval/run-1792823351373549000-2/review-scope-approved`.
- Approval presentation: `presentation/6b6300617b9cdd08`.
- Approval outcome: granted with persisted presentation proof under standing
  delegated-maintainer authority for the exact review-only scope.
- Phase-close status: `Completed` with 39 events, one approval, zero retries,
  and zero escalations.
- Approval-presentation enforcement: proof enforced; the approval event trail
  contains the presentation marker and content hash.
- Out-of-kernel work: the delegated maintainer inspected source, tests,
  security documentation, roadmap evidence, CI results, and canonicalization
  precedent and authored this review. The kernel coordinated governance only.
- Report posture: no runtime WorkReport artifact was generated or persisted.

## 18. Fix-Forward Note

The two blockers identified by this review have been fixed without erasing the
original finding. Version-one field lengths now use fixed-width `u64`
big-endian framing, both request and result vectors are pinned, and private
strict wire representations reject unknown request, result, host-obligation,
input-count, and observation-reference fields with value-independent errors.

This note does not change the original verdict. A focused blocker-fix review
must accept the repair before fixture parsing or workspace discovery begins.
