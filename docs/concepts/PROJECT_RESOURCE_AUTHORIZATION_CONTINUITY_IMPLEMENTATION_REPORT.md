# Project Resource Authorization Continuity Implementation Report

## Decision

Workflow OS now has a bounded implementation candidate for continuity-of-
authority Gate 3, **Approved Means Still Authorized**. The existing GitHub
repository-metadata path commits the exact current project restriction into the
approval's resolved execution context and reconstructs it before approval
acceptance and immediately before fixture or live provider admission.

This report does not close Gate 3. Focused maintainer/security review and full
local validation passed. Exact-head CI and an independent parent-versus-fix
drift rerun are still required.

## Problem Closed By The Candidate

Before this change, an approval for `acme/widgets` committed the immutable run
and fixture payload but not the current project-level provider declaration. A
maintainer could replace the allowed repository with `acme/replacement` after
the approval presentation, and the original run could still receive approval
and execute against `acme/widgets` from its immutable bundle.

The immutable declaration correctly preserved original intent, but it did not
prove that the current project still authorized that intent.

## Canonical Commitment

The existing approval `resolved_execution_context_hash` now carries a
versioned `workflow-os/github-repository-metadata-project-restriction/v1`
commitment over:

- stable project identity;
- exact run identity and deterministic approval identity;
- provider family, operation, and resource kind;
- provider enabled posture, endpoint, and non-secret access-reference identity;
- the sorted exact repository resource set;
- the current workflow, skill, and referenced-policy context already derived
  by the executor; and
- the fixture payload commitment when fixture mode is selected.

Resource ordering is canonical and does not create false drift. Unrelated
project display metadata is not committed. Any semantic change to the exact
provider declaration, relevant definitions, run, approval, or fixture context
changes the commitment.

## Enforcement Boundaries

At run creation, the commitment is stored in both the immutable run bundle and
the authored approval request through the existing resolved-context field.

Before a granted approval event is appended, Core reconstructs the commitment
from the current validated project and exact fixture. Missing, disabled,
duplicate, malformed, removed, replaced, broadened, or definition-drifted
declarations fail before `ApprovalGranted`.

Immediately before atomic provider admission, Core reconstructs the commitment
again and requires equality with both:

- the immutable run bundle; and
- the exact granted approval request.

Failure returns the payload-free code
`executor.github_repository_metadata.atomic.project_restriction_mismatch`
before transient access resolution, provider invocation, telemetry, or
SideEffect creation.

## Provider-Family Inventory

The current project manifest exposes one project-declared provider-read family:
GitHub repository metadata. Jira and GitHub Actions have adapter/read models,
but they do not expose an equivalent project-level authoritative provider-read
declaration through this manifest contract. This phase therefore covers only
the GitHub repository-metadata fixture and opt-in live-read composition. It
does not claim generic cross-provider enforcement.

## Focused Regression Evidence

The public CLI contract tests prove:

- unchanged restrictions complete normally;
- equivalent resource sets in a different declaration order complete normally;
- replacement, broadening, and stable project-identity drift fail as commitment
  mismatches;
- unsupported endpoint and access-reference substitutions fail project
  validation before approval;
- removal, duplicate declaration, and disablement fail deterministic project
  validation;
- referenced-policy drift fails before approval;
- fixture drift remains blocked;
- post-approval repository replacement fails at atomic provider admission;
- no failed pre-approval case records `ApprovalGranted`, provider invocation,
  or adapter telemetry; and
- no post-approval drift case records provider invocation or adapter telemetry.

The private opt-in live-read lifecycle test replaces the allowed repository
after approval and proves that atomic preparation returns the stable mismatch
code with zero provider calls and no target, replacement, or access value in
the rendered error.

Focused results at implementation-report creation:

- GitHub read-only CLI example target: 15 passed;
- atomic live-read drift regression: 1 passed;
- full Core all-targets/all-features suite: passed, including 785 unit tests and
  all integration targets;
- full CLI all-targets suite: passed, including 179 primary CLI tests and all
  provider/example targets;
- warnings-denied workspace Clippy: passed;
- warnings-denied workspace rustdoc: passed;
- workspace formatting: passed;
- documentation checks: passed;
- release-state consistency for `0.4.0-preview.1`: passed; and
- diff-integrity checks: passed.

Opt-in live credential-backed tests were not run. Exact-head CI and the
independent parent-versus-fix drift rerun remain later Gate 3 admission
evidence, not implementation-phase evidence.

## Honest Boundary

This candidate does not provide:

- provider writes or additional provider families;
- live credential-backed acceptance evidence;
- schemas or SDK exposure for the internal commitment;
- hosted or distributed enforcement;
- storage-owner tamper resistance;
- nested harness execution;
- automatic approval or broader delegated authority; or
- release or v1 admission.

## Governed Phase Record

The implementation phase began under:

- workflow: `dg/implement`;
- run: `run-1793886808979578000-2`;
- approval: `approval/run-1793886808979578000-2/implementation-approved`;
- approval presentation: `presentation/27a735f01955d4e4`;
- approval outcome: granted after the complete handoff was presented under
  delegated-maintainer authority; and
- isolated state root: `/private/tmp/workflow-os-gate3-state`.

Phase close reported `Completed` with 39 events, one approval, zero retries,
and zero escalations. The durable approval event carries the matching
presentation proof marker. Repository edits, shell commands, validation, and
this report were performed outside the kernel by the delegated maintainer; the
kernel coordinated governance and did not simulate execution or evidence.

Two bounded phase-start attempts failed before run creation: the helper's
secret-like detector rejected the phrase `authorization continuity`, and its
line-length guard rejected the first approved-scope wording. The shared
historical dogfood state then failed closed with a stale `event-log` lock and
an orphaned approval projection reported by `doctor state`. No state was edited
or recreated. The successful governed phase used the helper's supported
explicit isolated state directory. The shared-state defect remains separate,
disclosed follow-up work.
