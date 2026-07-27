# Registered Current-Authority Source Resolver Composition Report

## 1. Executive Summary

Workflow OS now composes the private registered current-authority source with
the existing capability-resolution, governed-context projection, and
required-context consumption helpers in one Core-owned call.

The composition keeps selected source records and their coherent source
snapshot together. It derives one exact fact set from the immutable execution
binding and required-context contract, resolves current capability authority,
rebuilds step-scoped projections, and consumes that exact contract before
returning a private `Ready` or `Blocked` assessment.

This phase does not expose production readiness, a reusable authorization
handle, target dereference, executor integration, persistence, providers,
OpenShell, sandbox execution, SideEffect execution, or writes.

## 2. Scope Completed

- Added one crate-private source-backed resolution input and outcome.
- Reused the registered source's exact request, completeness, consistency, and
  freshness checks.
- Kept selected grants, availability records, governed context references, and
  the coherent source snapshot inside the private source module.
- Built the exact `CurrentAuthorityFactSet` from the selected source records.
- Reused existing capability-authority resolution for every required context
  requirement.
- Reused existing step-scoped governed-context projection.
- Reused existing required-context consumption against the exact immutable
  contract.
- Returned a bounded private `Ready` or `Blocked` posture with typed reasons.
- Bound the assessment to the execution binding, contract, source snapshot,
  fact set, evaluation time, reasons, and consumption result.
- Added stable non-leaking error mapping and redaction-safe Debug behavior.
- Added focused composition tests.

## 3. Scope Explicitly Not Completed

This phase does not add a public source trait, public assessment API, reusable
readiness token, authorization lease, target dereference, context payload
access, executor consumer, runtime configuration, persistence, events, audit
projection, artifacts, providers, OpenShell, sandbox execution, SideEffects,
writes, schemas, SDKs, CLI behavior, UI, examples, hosted behavior, reasoning
lineage, or release changes.

## 4. Composition Boundary

The registered source now has one additional crate-private operation. It
accepts:

- the exact immutable execution binding;
- the exact required-context contract;
- an injected evaluation timestamp; and
- validated redaction metadata.

The operation performs one private source selection and immediately consumes
the selected records. The public payload-free source snapshot remains
descriptive; it cannot be passed back later to obtain readiness. Callers also
cannot substitute a caller-built fact set or prefiltered authority inventory.

## 5. Source And Fact-Set Integrity

The source derives the exact request from the binding and contract, selects all
matching grants plus exact availability and governed-context-reference
coverage, and fails before resolution when source coverage, freshness, or
coherence is invalid.

On success, the composition constructs `CurrentAuthorityFactSet` from those
same selected records and the same source snapshot commitment. The fact set is
therefore a bounded intermediate inside the trusted call, not caller-asserted
current authority.

## 6. Resolution And Required Context

For each contract requirement, the composition:

1. finds the exact payload-free governed context reference;
2. derives the required capability and resource;
3. resolves authority for the exact actor, workflow, run, step, harness,
   sensitivity, and evaluation time;
4. records unresolved policy, approval, evidence, or check prerequisites;
5. builds a projection candidate from current authority;
6. projects only through the existing step-scoped projection helper; and
7. reruns required-context consumption against the exact contract.

Required gaps block. Optional gaps remain explicit without independently
turning a satisfied required contract into denial.

## 7. Result And Replay Posture

The private assessment contains:

- `Ready` or `Blocked`;
- bounded typed reasons;
- the required-context consumption result;
- source snapshot and fact-set commitments;
- the injected evaluation time; and
- a deterministic assessment commitment.

It has no public serializer, no public export, no target data, and no method
that dereferences context. This phase does not claim one-time use or replay
protection. Those semantics require focused review before any runtime
consumer.

## 8. Privacy And Redaction

The composition handles only validated authority metadata and payload-free
references. It does not read or retain target contents, provider payloads,
source files, command output, credentials, environment values, endpoints, or
raw configuration.

Assessment Debug output redacts source, fact-set, time, and assessment
commitments. Stable errors describe the failed boundary without echoing
source values, targets, IDs, paths, tokens, or payloads.

## 9. Test Coverage

Focused tests cover:

- a complete source-backed `Ready` assessment;
- source failure short-circuiting before resolution;
- unresolved approval prerequisites blocking required context;
- revoked grants failing to produce readiness;
- deterministic assessment commitments across canonical inventory order; and
- redaction-safe assessment Debug output.

The pre-existing registered-source tests continue to cover exact source
registration, completeness, ordering, stale and future-dated failure,
duplicate rejection, and source Debug safety.

## 10. Validation Commands And Results

- focused registered-source composition unit tests: passed, 12 tests.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed before the governed phase close.

## 11. Remaining Known Limitations

- The source and assessment remain private and in-memory only.
- No persistent or external current-authority source exists.
- No prerequisite decision fact families are sourced independently.
- No one-time-use, nonce, or replay-prevention posture exists.
- No runtime consumer can act on the assessment.
- No target can be dereferenced through this boundary.
- Proportional governance cannot yet select authoritative quiet success from
  this result.
- OpenShell remains a separate optional execution-provider concern.

## 12. Recommended Next Phase

Perform a focused maintainer review of the private source-backed assessment
semantics.

The review should verify that source failure always short-circuits, selected
records remain bound to the exact source snapshot, current authority is
recomputed at the injected time, unresolved prerequisites cannot project
required authority, and the private result cannot become a reusable readiness
or dereference handle.

Do not add executor integration, persistence, providers, OpenShell, sandbox
execution, SideEffect execution, schemas, CLI behavior, or writes.

## 13. Governed Phase Record

- workflow: `dg/runtime-composition`
- run ID: `run-1785171238333563000-2`
- approval ID:
  `approval/run-1785171238333563000-2/composition-approved`
- approval presentation ID: `presentation/afb705bcb95b8050`
- approval presentation content hash:
  `afb705bcb95b80507a7efe75e002f273bdf3724bdf384d43f4732e0476973434`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted handoff was presented
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval-presentation enforcement: proof enforced with event marker
- out-of-kernel work: the delegated maintainer inspected and edited the
  implementation, tests, roadmap, and report; the kernel governed scope and
  approval but did not inspect code, edit files, execute checks, or mutate git
