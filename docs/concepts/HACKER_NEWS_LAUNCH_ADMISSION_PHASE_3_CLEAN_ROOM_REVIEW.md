# Hacker News Launch Admission Phase 3 Clean-Room Review

Status: accepted pending terminal-green exact-head CI

## Scope Reviewed

This focused maintainer/product/security review covered:

- exact source, binary, archive, checksum, and install identity;
- native macOS clean-room execution and exact-commit Linux packaging proof;
- preservation of existing downstream agent guidance;
- bounded TypeScript and Rust first-run recommendations;
- non-mock authoritative project-validation execution;
- WorkReport citation identity in human and JSON inspection;
- exact terminal replay and artifact idempotency;
- terminal-run/missing-artifact recovery under a filesystem fault;
- uninstall and local-state preservation;
- admission claims, limitations, plan status, and roadmap sequencing; and
- out-of-kernel work disclosure.

## Finding Reconciled

### Native candidate coverage required exact merged-commit evidence

The local clean-room evaluation necessarily executed the Apple Silicon package,
while the launch-admission plan also requires a Linux x86-64 native candidate
from the post-composition commit. Branch-head CI is not equivalent to the exact
merged source used by the clean-room evaluator.

GitHub Actions run `41747008662` is bound to exact merged commit
`c91228dcf6e7f6a8b6f92654b940efb72a11cedf`. Its native Release Packaging
Rehearsal successfully completed:

- packaging-contract tests;
- native musl toolchain installation;
- Linux x86-64 release build;
- provisional archive construction;
- checksum verification;
- isolated installation; and
- uninstall verification.

No artifact was uploaded. The overall exact-commit run was still in progress
at review time, so terminal-green completion remains mandatory rather than
being inferred from the packaging job alone.

## Accepted Evidence

No P0 or unresolved P1 finding remains in the reviewed Phase 3 evidence.

The installed macOS binary was built from the exact clean merged checkout and
not from the development worktree. Both unrelated repositories reached a useful
governance result far below the ten-minute threshold without a source checkout,
mock handler, fixture provider, network call, access material, repository build
command, or hidden state repair.

The artifact operation exercised ordinary immutable-run, local-check,
proportional-governance, event, and WorkReport machinery. Both primary runs
completed with ten events and exactly one cited report. Human and JSON inspect
surfaces exposed identical stable local-check identities without exposing
evidence bodies or repository content. Exact replay preserved event and artifact
cardinality.

The recovery proof is materially stronger than simple replay. A fault after the
run reached terminal state produced an honest artifact-write error and no
artifact. Removing only the OS permission fault allowed the exact same run to
reassess validation and create one artifact while preserving its original ten
events. No runtime record was edited or removed.

Uninstall removed only the isolated executable and preserved all downstream
governance state. The report accurately distinguishes validation of the
Workflow OS governance project from unexecuted downstream package, compiler,
lint, and test commands.

## Remaining Gates

Phase 3 acceptance does not authorize publication. Before the lane advances:

- exact merged-commit CI run `41747008662` must finish terminal green; and
- the Phase 3 documentation commit must pass its own exact-head required checks.

Phase 4 is a separate release-candidate review covering the complete validation,
dependency, security, documentation, release-checklist, and product-claim
matrix. Phase 5 remains a separate publication decision.

## Validation Reviewed

- exact source, archive, binary, and manifest identities reconciled;
- human and JSON inspect references reconciled against persisted artifacts;
- event and artifact cardinality reconciled before and after replay;
- recovery state reconciled before and after artifact-store restoration;
- exact merged-commit native Linux packaging job inspected;
- `npm run check:docs`: passed; and
- `git diff --check`: passed.

## Governed Review

- workflow: `dg/review`
- run: `run-1793752573206193000-2`
- approval: `approval/run-1793752573206193000-2/review-scope-approved`
- presentation proof: `presentation/8e784777d778779e`
- outcome: granted under delegated-maintainer authority
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval-presentation enforcement: proof enforced with one persisted record

The kernel governed review scope and approval. Evidence inspection, GitHub
Actions inspection, reconciliation, validation commands, and review writing
occurred outside the kernel and are disclosed here.
