# Current Authority Same-Call Time-Of-Use Resolver Plan Report

## 1. Executive Summary

The pure same-call current-authority resolver is now planned. The plan composes
the accepted immutable execution binding, private completeness-proving source,
capability resolver, context projection, and required-context consumer without
target dereference or runtime integration.

The first implementation remains private and test-only because the public
`CurrentAuthorityFactSet` intentionally permits caller-owned completeness
claims. Public `Ready` from that model would erase the accepted source trust
boundary.

## 2. Scope Completed

- Inspected the accepted current-authority, capability, context-projection,
  required-context, and immutable-binding APIs.
- Defined a private same-call resolver boundary.
- Defined a complete source-owned context-reference inventory.
- Defined exact binding, contract, source, reference, and time invariants.
- Defined `Ready | Blocked` semantics and stable bounded reasons.
- Defined independent-prerequisite blocking.
- Defined determinism, privacy, test, and compatibility posture.
- Updated roadmap sequencing and related plans.

## 3. Scope Explicitly Not Completed

No resolver, public source, target dereference, runtime/executor integration,
persistence, event, report, receipt, schema, SDK, CLI, UI, provider,
OpenShell, sandbox, SideEffect execution, write, hosted behavior, lineage, or
release change is implemented.

## 4. Key Architecture Decision

The first resolver must not accept an arbitrary public
`CurrentAuthorityFactSet` as trusted authority.

It will remain co-located with the private test source and compose source query,
capability resolution, projection, and consumption in one call. This proves the
algorithm without turning caller-claimed completeness into readiness.

## 5. Reference Completeness

The existing private source owns complete grant and availability inventories.
Projection also needs current stable context references. The plan adds a
private complete reference inventory that owns and commits all references,
selects the exact contract targets, and never dereferences payloads.

## 6. Prerequisite Posture

Matching grants that require policy, approval, evidence, or check evaluation
remain blocked. IDs alone do not prove those prerequisites. Trusted
prerequisite sources are deferred.

## 7. Privacy And Redaction

The planned result is payload-free. Debug output and errors redact identities,
hashes, timestamps, resources, and references. No content, provider output,
command output, credentials, approval prose, policy input, check output,
sandbox data, or paths are retained.

## 8. Product Alignment

Fresh-pull review confirms that Workflow OS now explains its preview boundary
well and that reducing low-risk ceremony is the next product challenge.

This phase supports that goal without weakening safety: quiet success can only
be trustworthy when current authority is sourced and recomputed rather than
asserted or reused.

## 9. Validation

- `npm run check:docs`: passed with the repository Node 20 toolchain.
- `git diff --check`: passed.
- Architecture claims were checked against current Core APIs and accepted
  phase reports.

## 10. Remaining Limitations

- The first resolver will be private and test-only.
- No production authenticated authority or reference source exists.
- No trusted policy, approval, evidence, or check prerequisite source exists.
- No target dereference or runtime consumer exists.
- Same-call evaluation is not a reusable lease.

## 11. Recommended Next Phase

Focused maintainer review accepts the plan in
[Current Authority Same-Call Time-Of-Use Resolver Plan Review](CURRENT_AUTHORITY_SAME_CALL_TIME_OF_USE_RESOLVER_PLAN_REVIEW.md).
Implement the private test-only same-call resolver next.

## 12. Dogfood Governance

- workflow: `dg/d`
- run ID: `run-1785152780541860000-2`
- approval ID:
  `approval/run-1785152780541860000-2/planning-approved`
- presentation ID: `presentation/6c210bcac21e3430`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted planning handoff was presented
- phase status: completed
- event posture: approval-presentation proof persisted and enforced
- out-of-kernel work: architecture inspection, planning, documentation edits,
  and validation were performed by the delegated maintainer; the kernel
  governed scope and approval but did not edit files, run checks, or mutate git
