# Trusted-Host Live-Execution Security Contract Model Blocker Fix Report

## 1. Executive Summary

The focused blocker fix closes the four model-boundary defects identified by
the original maintainer/security review. The requirements contract now binds a
closed, versioned strict security profile into its canonical fingerprint; root
observations reject commitment aliasing across typed roles; assessment
orchestration is crate-private; and requirements serde reconstructs its
derived fingerprint from authoritative fields only.

The phase remains model-only, provider-neutral, payload-free, deterministic,
and non-authorizing. It adds no discovery, provider composition, OpenShell
integration, runner, live Cargo execution, persistence, schemas, writes, or
release changes.

## 2. Blockers Fixed

1. Complete requirements semantics are represented by
   `StrictBuildExecutionV1`, a closed versioned profile whose documented
   contract includes every decision-relevant control used by assessment.
2. The security profile participates in the domain-separated requirements
   fingerprint, with a fixed-vector regression.
3. Root observations reject reuse of one commitment across different typed
   root roles before assessment.
4. `assess_trusted_host_live_execution_security` is no longer exported from
   the crate root and remains crate-private pending an approved composition
   consumer.
5. Requirements serialization omits the derived fingerprint and
   deserialization recomputes it from authoritative fields.

## 3. Implementation Approach

The fix uses the smallest contract expansion allowed by the review: one closed
versioned strict-profile identity rather than many independently configurable
requirement switches. This avoids generic policy extensibility while making
the complete accepted contract explicit, serializable, fingerprinted, and
reviewable.

`StrictBuildExecutionV1` requires enforce mode; enforced filesystem, process,
network, protected-root, and resource-budget controls; filesystem and network
default denial; exclusion of host home, credential stores, and ambient
temporary paths; complete observation; zero allowed network events; terminal
process-tree proof; quiescent outputs; retained-artifact commitment; and
completed cleanup.

## 4. Requirements And Fingerprint Boundary

The requirements fingerprint now commits the strict-profile identity in
addition to the exact run bundle, command contract, attempt, request, provider
selection, requested policy, egress-probe requirement, and canonical typed
root requirements. Root declaration order remains canonical.

The fixed known vector for the focused fixture is:

```text
0c5ab92eaec1a17229f303b0196c601ffb451401a9924b6e222aab1e3ebd4a84
```

The profile is closed rather than caller-extensible. A future semantic change
requires a new profile variant and therefore a new fingerprint.

## 5. Root Commitment Uniqueness

Observation construction now requires every typed root role to carry a
distinct commitment. Reuse across protected inputs, writable outputs, or any
other role pair fails with the stable
`trusted_host.live_security.observation.root_binding.invalid` code without
including the rejected commitment.

This first slice models no safe alias class. Any future aliasing must be an
explicit separately reviewed contract.

## 6. Public Surface And Serde Boundary

Public construction vocabulary remains available for future provider-neutral
composition, but the assessment function is crate-private and absent from the
crate-root exports. A compile-fail documentation regression guards that public
surface.

Requirements serialization contains only authoritative fields. It excludes
the derived fingerprint, and validated deserialization canonicalizes the
roots and recomputes the fingerprint. Unknown profile values fail closed with
a value-independent error message.

## 7. Redaction And Privacy

The fix adds no paths, source contents, dependency contents, command text or
output, environment values, credentials, provider payloads, denied endpoints,
or sandbox logs. Errors remain static and non-leaking. Debug output continues
to redact commitments, provider identity, references, and fingerprints.

## 8. Test Coverage

The focused module suite now has 15 tests. New regressions cover:

- the strict profile accessor and serialized vocabulary;
- omission and deterministic recomputation of the derived fingerprint;
- a fixed requirements-fingerprint vector;
- unknown-profile failure without rejected-value leakage;
- duplicate root commitment rejection without leakage; and
- continued crate-private assessment coverage through an internal unit module.

The existing satisfaction, denial, reconciliation, substitution, bounds,
serde, process-tree, policy-drift, observation-channel, egress-probe,
canonicalization, and transient-write tests remain intact.

## 9. Commands And Results

- Focused trusted-host live-security unit tests: passed, 15 tests.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check`: passed under the repository Node 20 toolchain.
- `npm run check:integrations`: passed under the repository Node 20 toolchain.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 10. Remaining Known Limitations

No component discovers complete Cargo inputs, binds operating-system objects,
owns a process tree, enforces filesystem, network, process, or resource policy,
proves transient-write exclusion, selects a provider, or composes this model
with a live attempt. OpenShell remains a candidate only. Every real-process
blocker remains open.

The strict profile is intentionally not configurable. Broader policy profiles
must be justified by concrete reviewed consumers rather than speculative
extensibility.

## 11. Recommended Next Phase

Perform a focused maintainer/security review of this blocker fix. Verify the
profile's completeness, fixed-vector sensitivity, role-unique commitments,
serde reconstruction, public API narrowing, and non-authorizing boundary
before planning any discovery or provider-composition work.

## 12. Governed Fix Record

- Workflow ID: `dg/blocker`.
- Run ID: `run-1792815843112251000-2`.
- Approval ID: `approval/run-1792815843112251000-2/fix-approved`.
- Approval-presentation ID: `presentation/5ddc0882aa2418ad`.
- Approval-presentation content hash:
  `5ddc0882aa2418ade11c44c26b5074cd70601a4f74cc3cdc98238ef83a53d36a`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact narrow blocker-fix scope.
- Phase status: `Completed`.
- Event summary: 39 events, including one approval request, one approval grant,
  six scheduled steps, six successful skill invocations, zero retries, and zero
  escalations. Approval presentation enforcement was proof-enforced and the
  presentation marker is present in the event trail.
- Validation summary: the full Rust, Node, integration, docs, formatting, and
  diff-check matrix passed.
- Out-of-kernel work: the delegated maintainer edited code, tests, roadmap, and
  documentation and ran validation. The kernel governed scope and approval but
  did not edit files, run checks, or perform git or PR operations.
