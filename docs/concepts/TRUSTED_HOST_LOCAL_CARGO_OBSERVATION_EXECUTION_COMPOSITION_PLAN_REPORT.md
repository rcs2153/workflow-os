# Trusted-Host Local Cargo Observation And Execution Composition Plan Report

## 1. Executive Summary

The same-call composition boundary is planned for one exact Cargo clippy
operation. The plan binds the accepted local observer to immutable run,
command, security, invocation, result, attestation, and report identities
without confusing preflight proximity with continuous custody.

## 2. Scope Completed

- Defined one private local-host composition owner.
- Defined exact immutable, observer, root, executable, environment, security,
  invocation, result, attestation, and report bindings.
- Defined pre-start denial and possibly-started reconciliation semantics.
- Defined a private injected first implementation and focused test plan.

## 3. Scope Explicitly Not Completed

No admission behavior, Cargo execution, standard runner, live smoke,
OpenShell integration, provider selection, persistence, events, audit
projection, report generation, schemas, CLI behavior, provider writes,
additional platforms, broader authority, or release changes were implemented
or authorized.

## 4. Central Security Decision

The accepted native observer is preflight evidence only. Its fixed probes end
before a future Cargo invocation and cannot retain repository, executable,
process-tree, filesystem, network, or cleanup custody. The plan therefore
forbids promoting its current facts to `Observed` and requires the first
composition implementation to deny before runner entry.

## 5. Planned First Implementation

The next implementation, after focused review, is a private injected contract
that validates all bindings, invokes the observer through an injected source,
evaluates the existing strict live-security contract, and proves no Cargo
runner starts when mandatory facts are incomplete.

## 6. Privacy And Failure Posture

Only typed identities, commitments, closed postures, bounded counts, and
stable references may cross the boundary. Raw paths, source contents,
environment values, command output, provider payloads, credentials, and
free-form errors remain excluded. Failures before process start remain proven
pre-start denials; ambiguity after possible start requires reconciliation and
cannot be retried blindly.

## 7. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.
- Runtime tests are not required for this documentation-only phase.

## 8. Remaining Limitations

- No accepted continuous execution owner exists.
- No current observer fact proves admission-to-terminal continuity.
- The Cargo clippy handler remains injected-runner-only and unregistered.
- No provider currently satisfies the complete live-security matrix.
- No live process, workflow composition, or report artifact path is
  authorized.

## 9. Recommended Next Phase

Perform a focused maintainer and security review of the same-call composition
plan. If accepted, implement only its private injected fail-closed composition
contract.

## 10. Governed Planning Evidence

- Workflow ID: `dg/d`.
- Run ID: `run-1792846202820998000-2`.
- Approval ID:
  `approval/run-1792846202820998000-2/planning-approved`.
- Approval presentation ID: `presentation/3de3c756a5ef2158`.
- Approval outcome: granted under standing delegated-maintainer authority.
- Phase status: `Completed`.
- Event summary: 39 ordered events, including one approval request, one
  approval grant, six scheduled steps, six successful skill invocations, zero
  retries, and zero escalations. Approval-presentation enforcement was
  proof-enforced and its event marker was present.
- Validation summary: documentation validation and diff checks passed.
- Out-of-kernel work: document inspection, planning, editing, validation, git,
  and PR work are performed by the delegated maintainer; the kernel governs
  phase scope and approval.
