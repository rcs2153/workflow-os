# Trusted-Host Local Cargo Input Observer Provider Plan Review

## 1. Executive Verdict

Plan accepted; proceed to the private local-host observer, selection-binding,
and in-memory evidence-bundle contract implementation.

The plan chooses a useful first live producer without confusing observation
with containment or authority. Its capability matrix is deliberately honest:
the standalone producer can gather real host facts, but no fact may become
`Observed` while admission-to-terminal continuity remains unproven.

## 2. Scope Verification

The plan remains limited to one future read-only local Cargo observer. It does
not authorize host inspection or process execution in the planning phase and
does not combine implementation, live smoke, admission, Cargo execution,
OpenShell wiring, persistence, events, schemas, CLI behavior, writes,
capability grants, or release changes.

The proposed implementation sequence keeps private contracts, injected-source
behavior, opt-in live proof, review, and later same-call composition separate.

## 3. Provider Selection Assessment

Selecting a native `HostObserver` is appropriate for the first live
observation slice. It avoids waiting on external provider APIs while returning
real bounded facts and explicit gaps. The observer is not described as a
sandbox, execution provider, or trusted machine attestor.

OpenShell remains the preferred later optional containment provider. The plan
correctly keeps its current wiring blocked by the accepted evidence matrix and
does not treat unrelated third-party discussion comments as an upstream
contract.

## 4. Ownership And Dependency Assessment

`workflow-local-host` is the correct initial owner for operating-system reads
and fixed process probes. The crate already owns the unpublished local process
boundary and depends on Core. Core continues to own canonical request/result
models and must not depend on the local-host crate.

The observer receives no workflow mutation, approval, event, persistence,
report, or admission APIs. This preserves the existing authority direction.

## 5. Request And Selection Binding Assessment

The corrected plan binds the canonical local observation definition exactly to
`TrustedHostInputObservationRequest.selection`. The closed Cargo operation and
security requirements remain separately bound by their existing request
fields. Extra roots, executables, dependencies, targets, or environment
definitions are substitution and fail closed.

The shared ordering-authority commitment remains authoritative for discovery
completion and observation revisions. Wall-clock time is not ordering
authority and the result remains evidence rather than reusable permission.

## 6. Capability Matrix Assessment

All nine obligations and the repository root have explicit candidate sources,
evidence requirements, and posture ceilings.

The most important correction is explicit: standalone preflight proves no
admission-to-terminal continuity, so even inspected toolchain files remain
`Uncertain`. Target, sysroot, linker, native tool, registry-source, and
environment limitations are disclosed. Dynamic libraries and runtime base
remain `Unsupported` until an execution-bound observing or enforcing surface
exists.

The matrix cannot produce a satisfied admission assessment. That is correct
and useful: the first implementation proves live observation and exact gap
reporting, not live-execution readiness.

## 7. Probe And Failure Assessment

The fixed probe set is bounded and rejects caller commands, shell fragments,
extra arguments, inherited environment, network fallback, and human-output
parsing. Process probes require exact executable identity, fixed arguments,
cleared environment, bounded output, timeout handling, and stable errors.

The corrected failure policy is sound. Safely classifiable probe uncertainty
may produce a complete result with an `Uncertain` fact. Request, producer,
identity, root, ordering, output-bound, parser, or evidence-integrity failures
fail the whole observation and cannot be softened.

## 8. Evidence And Privacy Assessment

The in-memory evidence bundle now has deterministic, non-circular reference
derivation. References bind request, producer, completion revision, evidence
class, and entry commitment. The bundle must resolve exactly the required root
and each present fact reference and reject missing, duplicate, extra,
substituted, or cross-request entries.

Raw paths, source, executable bytes, environment material, Cargo metadata,
command output, registry locations, endpoints, credentials, and provider
errors remain outside Core. Debug, serialization, and validation errors are
required to be non-leaking.

## 9. Platform Assessment

The first opt-in live smoke is explicitly limited to
`aarch64-apple-darwin`. Other platforms return `Unsupported` before starting a
process until separately reviewed. This is preferable to implying portable
host semantics from one machine.

Ordinary tests remain injected and process-free. The platform smoke is later
than the private contract implementation and is excluded from default CI.

## 10. Test Plan Assessment

The planned tests cover exact producer and request binding, all nine facts,
root composition, canonical ordering, substitution, drift, process failures,
environment clearing, command closure, privacy, absence of runtime mutation,
and process-free default tests. They also preserve the existing observer,
static-discovery, local-check, local-host, executor, hosted-provider,
persistence, and workspace suites.

The test plan is sufficient for the first private contract slice. The later
live smoke must not be counted as containment, admission, or portable provider
proof.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Consider a future vocabulary clarification for `IndependentlyReviewed` so
  operators cannot mistake reviewed producer code for independent machine
  attestation.
- Reassess whether the in-memory evidence bundle belongs in Core only if a
  second producer requires the same contract; avoid premature public API.
- Keep the first implementation private and injected. Do not add the live
  process runner or smoke in the same phase.
- Re-review the OpenShell matrix only after an authoritative upstream response
  or release changes the required surfaces.

## 13. Recommended Next Phase

Implement the private local Cargo observer contracts only:

- canonical selection-bound observation definition;
- private observer interface in `workflow-local-host`;
- in-memory evidence entry and bundle with exact reference resolution;
- injected filesystem, process, and revision source interfaces; and
- deterministic tests proving matrix posture ceilings without host access.

Do not implement a standard process source, opt-in live smoke, runtime
composition, admission, Cargo execution, OpenShell wiring, persistence,
events, schemas, CLI behavior, writes, or release changes.

## 14. Validation

- `npm run check:docs`: passed before phase close.
- `git diff --check`: passed before phase close.
- Reviewed the accepted Core observation and continuity semantics,
  `workflow-local-host` ownership boundary, Cargo clippy profile, OpenShell
  provider report, v0.0.101 evidence matrix, current-upstream re-verification,
  and current public upstream discussion state.

## 15. Governed Review Evidence

- Workflow ID: `dg/review`.
- Run ID: `run-1792841520134140000-2`.
- Approval ID:
  `approval/run-1792841520134140000-2/review-scope-approved`.
- Approval presentation ID: `presentation/600e656fe5473d43`.
- Approval presentation content hash:
  `600e656fe5473d434806b615b5f3c7e719bf4e98cb2cd263243cacfadd1aa3c5`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact focused review scope.
- Phase status: `Completed`.
- Event summary: 39 ordered events, including one approval request, one
  approval grant, six scheduled steps, six successful skill invocations, zero
  retries, and zero escalations. Approval presentation enforcement was
  proof-enforced and its event marker was present.
- Validation summary: `npm run check:docs` and `git diff --check` passed.
- Out-of-kernel work: the delegated maintainer inspected code and accepted
  reports, queried the public upstream discussion read-only, tightened the
  planning document, wrote this review, and ran validation. The kernel
  governed scope and approval but did not inspect files, query GitHub, edit
  documents, or run validation.
