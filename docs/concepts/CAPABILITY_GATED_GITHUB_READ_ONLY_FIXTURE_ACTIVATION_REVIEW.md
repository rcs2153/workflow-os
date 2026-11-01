# Capability-Gated GitHub Read-Only Fixture Activation Review

## Decision

Accepted after one blocking correction.

The Phase 3 public path remains one exact fixture-backed
`github.repository.metadata` read. It is selectable through ordinary CLI run,
approval, status, and inspect commands without enabling live transport, access
material resolution, provider mutation, or generic connector dispatch.

## Reviewed Boundary

The review verified that:

- only the exact declared repository-metadata operation selects the path;
- fixture bytes participate in the resolved execution-context commitment;
- the immutable workflow, skill, policy, project declaration, target, and
  fixture commitment are reassessed before grant and provider admission;
- durable approval-presentation proof is required before a grant is accepted;
- approval records the decision and resume state without entering the provider;
- fixture mode never resolves access material or opens live transport;
- provider entry uses the accepted atomic execution-window, attempt, and
  continuity machinery;
- bounded adapter audit and observability records are persisted;
- terminal replay does not invoke the adapter again; and
- status and inspect disclose fixture mode, disabled live transport, report
  posture, and explicit recovery posture.

## Blocking Finding And Correction

The initial approval path reloaded and validated the fixture for both grant and
denial. Removing or changing the fixture after presentation could therefore
prevent an operator from denying a waiting run.

That was incorrect. Denial must remain available with valid presentation proof
and must not depend on reconstructing executable provider context. The
corrected path:

- requires the exact fixture payload only for a grant;
- preserves full immutable-context and fixture-drift validation for grants;
- allows a proof-backed denial when the fixture is missing or changed;
- appends the denial and terminal failure without requesting a skill
  invocation; and
- produces no adapter telemetry on denial.

Focused CLI coverage removes the fixture after presentation, denies the run,
and proves `ApprovalDenied` is durable while no skill invocation or adapter
telemetry exists.

## Validation

- focused fixture lifecycle, drift, and denial tests pass;
- Rust formatting passes;
- warnings-denied Clippy passes for Core and CLI across all targets and
  features under the repository toolchain;
- the local-host foreground suite passes 43/43 with the repository's pinned
  Node 20 runtime;
- full workspace Rust tests pass with all features under the pinned runtime;
- repository, integration, and documentation checks pass; and
- `git diff --check` passes.

The first full-workspace probe used the ambient Node runtime and caused the
local-host docs-check profiles to fail with their bounded security error. The
same suite passes under the repository's declared pinned Node 20 runtime. This
is recorded as environment/tooling evidence, not hidden as a successful first
probe.

## Remaining Boundaries

This acceptance does not authorize live GitHub access, access-material lookup,
provider writes, additional GitHub operations, PR lifecycle mutations,
OpenShell execution, hosted behavior, or broader autonomy claims.

Before broader provider mutation families or continuous delegated-maintainer
claims, complete the P0 Delegated Authority Projection Into External
Connectors lane. That work must extend existing authority, continuation,
approval-presentation, and proportional-governance models rather than create a
parallel permission system.

## Governance Record

- workflow: `dg/review`;
- run: `run-1793528082632812000-2`;
- approval: `approval/run-1793528082632812000-2/review-scope-approved`;
- presentation proof: `presentation/8babdf2a66813ee4`;
- approval outcome: granted under delegated maintainer authority;
- phase status: completed;
- event summary: 39 events, one approval, zero retries, zero escalations; and
- out-of-kernel work: source and documentation edits, validation commands,
  git operations, and future PR actions remain executor work rather than
  kernel-executed side effects.
