# Trusted-Host Input Observer Core Model Review

## 1. Executive Verdict

Needs blocker fixes.

The implementation is narrowly scoped, provider neutral, process free,
payload free, and explicitly non-authorizing. It also closes the nine accepted
host-input obligations and canonically commits the represented fields. The
current model cannot yet prove the expected producer identity, per-fact
provenance, or why a non-proven fact was not established. Several coherence
and deserialization boundaries can therefore admit stronger or less safe
claims than the accepted plan permits.

## 2. Scope Verification

The phase stayed within the approved model-only scope. It added no host
inspection, filesystem traversal, process execution, provider selection,
OpenShell integration, runtime assessment or admission, persistence, workflow
events, reports, artifacts, schemas, SDKs, CLI behavior, examples, approvals,
capability grants, provider mutations, external writes, or release changes.

## 3. Request And Result Binding Assessment

The request binds the immutable run, workflow, step, operation, static
discovery, repository tree, live-security requirements, selection, and attempt
commitments. The result binds that request commitment, the same repository
tree, ordered observation revisions, a repository-root observation, one
producer claim, and the complete fact set. Result-to-request validation rejects
request and repository substitution.

One required binding is absent: the request does not identify the expected
producer identity and version. The result can therefore carry a producer claim
without proving that it came from the observer channel the request intended to
use.

## 4. Producer Provenance And Assurance Assessment

Producer kind, assurance, implementation commitment, and configuration
commitment are represented. Fixture assurance is coherently restricted to the
fixture producer kind. These values remain claims and correctly grant no
authority.

The model does not represent a stable producer identity or version, and one
global producer is applied to every fact. It cannot explain which component
observed each fact, reject producer identity/version substitution, or represent
a result assembled from independently reviewed components. It also does not
validate the producer kind and assurance against each fact's observation
method. A fixture producer can therefore claim provider-native or protected
handle methods, and a host observer can claim fixture observations.

## 5. Observation Fact Assessment

The result requires exactly one fact for each of the nine accepted host-input
obligation kinds. Counts are bounded, references are telemetry-only, and
observed facts require proven object binding and continuity.

The following fact-level blockers remain:

- non-proven facts do not carry the required closed, bounded reason code;
- non-observed postures can retain one `Proven` axis when the other axis is not
  proven, creating a partial false-proof claim;
- `Uncertain` accepts method combinations whose meaning is not coherently
  defined; and
- fact methods are not constrained by the declared producer channel.

## 6. Repository-Root Composition Assessment

The model correctly reuses `TrustedHostRootObservation` rather than inventing
a parallel root type. Construction requires `RepositoryInput`, read-only,
non-invocation-scoped posture and exact equality with the request repository
tree commitment. Root substitution fails closed.

## 7. Freshness Assessment

Observation start and completion revisions are nonzero and monotonically
ordered. That establishes internal result ordering, but the request does not
bind a static-discovery revision or observation floor. The result therefore
cannot prove that host observation occurred after the static discovery context
it claims to supplement. The freshness relation required by the plan is not
yet representable.

## 8. Canonical Commitment Assessment

Request and result hashing use versioned domains, fixed-width framing, closed
tags, deterministic fact ordering, and pinned vectors. Every field currently
stored by the request, result, producer, root observation, and fact contributes
to its canonical commitment.

The commitment is deterministic, but it cannot bind semantic fields that the
model omits: expected producer identity/version, per-fact provenance,
non-proof reason codes, and a static-discovery freshness floor.

## 9. Serde, Debug, And Error Assessment

The newly introduced observer enums use closed non-echoing deserializers,
definitions deny unknown fields, and model constructors revalidate wire data.
Debug output redacts commitments and references. New validation errors are
stable and payload free.

The result boundary reuses several enums that still derive Serde
deserialization: `TrustedHostRootRole`, `TrustedHostRootAccess`,
`TrustedHostObservationPosture`, `TrustedHostRootObservationMethod`, and
`HostedExecutionReferenceKind`. Unknown values for those nested fields may be
echoed by Serde's generated unknown-variant errors. The new observer boundary
therefore does not yet satisfy its complete non-echoing deserialization claim.

## 10. Non-Authorizing Boundary Assessment

The implementation does not inspect a host, execute a process, trust a
producer, select a provider, admit work, persist state, or grant authority.
Serialization and a valid commitment remain observation claims rather than
admission evidence. This boundary is correct and must remain unchanged during
the blocker fix.

## 11. Test Quality Assessment

Focused tests cover valid construction, complete obligation representation,
missing and duplicate facts, deterministic ordering, pinned vectors,
repository substitution, declared-only false proof, producer assurance,
reference kind, root role, semantic commitment sensitivity, strict new-enum
and unknown-field rejection, serde revalidation, and Debug/error non-leakage.

Required blocker regressions are missing for:

- expected producer identity/version substitution;
- per-fact producer provenance;
- missing or invalid non-proof reason codes;
- either proven axis on a non-observed fact;
- the complete posture/method matrix;
- producer/method incoherence;
- nested reused-enum error non-leakage; and
- observation ordering relative to the bound static discovery context.

## 12. Documentation Assessment

The plan and implementation report accurately retain the model-only,
non-authorizing boundary and explicitly defer host observation and provider
integration. The implementation report overstates complete strict
deserialization and complete coherence until the blockers above are fixed.
Fix-forward notes preserve that distinction without erasing the original phase
record.

## 13. Blockers

1. Add a stable producer identity/version model and bind the expected producer
   to the request and actual producer to the result.
2. Bind each fact to its observing producer/component, or define one closed
   homogeneous-producer rule that proves the global producer observed every
   fact.
3. Add a closed bounded reason code required for every non-proven fact.
4. Reject any `Proven` object-binding or continuity axis for every non-observed
   posture and validate the complete posture/method matrix.
5. Validate producer kind/assurance against allowed fact methods.
6. Make every enum reachable through the observer wire boundary reject unknown
   values without echoing caller input.
7. Bind observation freshness to the static discovery context with a
   deterministic ordered revision or equivalent non-clock authority.

## 14. Non-Blocking Follow-Ups

- Decide during later provider review which producer identities and methods
  may satisfy live admission.
- Keep fixture-backed proof, live host observation, and provider adapters in
  separate phases after this model is accepted.
- Preserve pinned vectors whenever the blocker fix intentionally changes the
  canonical contract version.

## 15. Recommended Next Phase

Trusted-host input observer Core model blocker fix.

Fix only the seven model, coherence, privacy, and regression gaps above. Do not
add a callable observer, inspect a host, run a process, integrate a provider,
admit runtime work, persist observations, add events, schemas, CLI behavior,
examples, writes, or authority.

## 16. Governed Review Evidence

- Workflow ID: `dg/review`.
- Run ID: `run-1792836452150205000-2`.
- Approval ID:
  `approval/run-1792836452150205000-2/review-scope-approved`.
- Approval presentation ID: `presentation/eb19a0edaae4349a`.
- Approval presentation content hash:
  `eb19a0edaae4349ae3eaba03302e9c4a9e805d8511ceb6c2b377bc54d2aec060`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact review-only scope.
- Phase status: `Completed`.
- Event summary: 39 ordered events, including one approval request, one
  approval grant, six scheduled steps, six successful skill invocations, zero
  retries, and zero escalations. Approval presentation enforcement was
  proof-enforced and its event marker was present.
- Validation summary: the exact reviewed implementation commit passed format,
  clippy, workspace tests, docs checks, and diff checks; the documentation-only
  review update passed docs and diff checks before phase close.
- Out-of-kernel work: Codex inspected the implementation, tests, plans, and
  reports and authored this review. The kernel governed scope and approval but
  did not perform the review.
