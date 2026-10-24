# GitHub Draft Pull Request Live Sandbox Proof Review

Review date: 2026-10-23

## 1. Executive Verdict

**Proof accepted with non-blocking follow-ups.**

The dedicated live proof demonstrates the narrow claim it was designed to
test: the accepted GitHub transport can create one draft pull request in the
compile-time allowlisted sandbox and a fresh independent invocation can
reconcile that exact managed draft without issuing a second create.

The proof does not authorize default writes, another repository, another
provider, another mutation family, merge, close, cleanup, executor or CLI write
wiring, or production use.

## 2. Review Scope

This review assessed:

- the unchanged ignored live smoke;
- the exact sandbox repository and pinned head/base refs;
- the least-privilege access boundary used by the smoke process;
- the persistent provider object created by the proof;
- lookup-before-create reconciliation and idempotency posture;
- post-call ambiguity and retry behavior;
- privacy, redaction, and stored-state boundaries; and
- the implementation report and repository validation evidence.

No runtime fix or provider mutation was implemented during review.

## 3. Live Provider State

Independent provider inspection confirmed one open draft:

- reference: [`rcs2153/workflow-os-sandbox#1`](https://github.com/rcs2153/workflow-os-sandbox/pull/1);
- state: open;
- draft: true;
- merged: false;
- base: `main` at `aa520c7d51a05ab72c12121161acb03d2c6dc210`;
- head: `workflow-os-sandbox-proof-v1` at
  `dd8718cb44ba7eb076cceb6bcc3b074971bdedaf`;
- commits: one; and
- managed marker: `workflow-os-github-draft-pr-sandbox-v1`.

The observed state matches the proof report. No second pull request, merge,
close, edit, or cleanup occurred.

## 4. Target And Access Boundary Assessment

The concrete provider rejects every target except the compile-time
`rcs2153/workflow-os-sandbox` owner/repository pair before URL or request-header
construction. It also requires a same-repository head branch. The smoke binds
the exact head and base branch names plus their expected SHAs.

The operator-created access material was limited to the sandbox repository,
with repository contents read and pull requests write. It was supplied only to
the explicit smoke process from a dedicated operating-system credential entry.
Core does not discover environment, Keychain, GitHub CLI, or git credentials.

This is acceptable for a dedicated proof. It is not an access-management,
rotation, hosted identity, or production secret-provider design.

## 5. Governance And Mutation Boundary Assessment

The live smoke entered the existing full helper rather than calling the HTTP
transport directly. Before provider observation, the helper validates the
terminal run, immutable run bundle, WorkReport artifact identity, proportional
governance assessment and fresh runtime-fact binding, exact capability scope,
durable policy decision, durable proof-enforced approval presentation and
decision, SideEffect preflight, and approval linkage.

The first successful path persisted proposed and attempted SideEffect posture,
performed lookup before create, created a draft-only provider object, observed
refs again, and completed the SideEffect only after exact reconciliation. The
second path used a new local state store and provider instance and reached the
same completed posture through lookup without calling create.

The proof therefore exercises governance-to-provider composition, not merely
the wire client.

## 6. Idempotency And Reconciliation Assessment

The test's two independent invocations are meaningful evidence that provider
state, rather than process memory or local state reuse, prevents a duplicate
create for the exact managed identity. Lookup requires the exact repository,
head owner and branch, base branch, head/base SHAs, draft posture, complete
managed body, and marker.

Ambiguous or multiple candidates do not create. Conflicting unmanaged or
non-draft candidates do not create. Ref drift before create fails closed, and
post-create ref drift prevents terminal completion.

This is sufficient for the exact sandbox proof. It is not a generic global
idempotency guarantee and does not cover user edits to a formerly managed
draft, branch deletion, or future API changes.

## 7. Failure And External-State Ambiguity Assessment

The transport disables redirects and connection pooling and configures bounded
timeouts. It performs no automatic create retry. A transport failure after
entering the create boundary and every unrecognized response remain
may-have-started ambiguity. Only the reviewed rejection statuses are terminal
known rejection, and those require remediation before a fresh governed
attempt.

The successful proof does not test a live ambiguous network outcome. The code
and report do not claim otherwise. Operator reconciliation remains required
for that posture.

## 8. Privacy And Redaction Assessment

No access value, authorization header, raw request, raw provider response, PR
body payload, or raw error body enters durable events, evidence, reports,
diagnostics, or Debug output. The smoke explicitly checks that Debug output
does not contain its access value or sandbox repository name. Provider evidence
contains only the stable provider reference and bounded outcome summary.

The proof report records public-to-the-maintainer branch names, commit SHAs,
permission posture, and the stable draft reference. That is appropriate for
the private sandbox proof and does not expose authentication material.

## 9. Validation And Test Quality

The explicitly enabled ignored smoke passed once against the live sandbox.
Repository-wide validation then passed:

```text
cargo fmt --all --check                                      passed
cargo clippy --workspace --all-targets -- -D warnings        passed
cargo test --workspace                                      passed
npm run check:docs                                          passed
git diff --check                                            passed
```

The ordinary workspace suite keeps live tests ignored and network-free. The
focused ordinary suite covers target allowlisting, same-repository enforcement,
request/response mapping, response bounds, ref drift, conflict and ambiguity,
known rejection, no transparent retry, governance ordering, access safety,
Debug safety, evidence, citations, and create-or-reconcile behavior.

## 10. Scope Verification

The phase stayed within its approved proof boundary. It added no:

- default or automatic provider write;
- executor or CLI mutation path;
- merge, close, edit, branch, commit, or cleanup operation;
- second repository, provider, or mutation family;
- hidden access discovery;
- automatic retry;
- schema, SDK, example, or hosted surface; or
- production-readiness claim.

## 11. Blockers

None for accepting the dedicated live sandbox proof.

## 12. Non-Blocking Follow-Ups

- Keep the compile-time repository binding until a separately reviewed target
  registration/classification boundary exists.
- Keep access rotation and expiry as an explicit operator prerequisite; do not
  imply that Core owns access lifecycle.
- Preserve the persistent draft until a separately governed cleanup operation
  is intentionally scoped.
- Add a live ambiguity/reconciliation exercise only if it can be made
  deterministic without unsafe retries or nondeterministic network fault
  simulation.
- Revisit the concrete API/version dependency before any production claim.

## 13. Governed Review Evidence

- workflow: `dg/review`
- run: `run-1792800205139574000-2`
- approval: `approval/run-1792800205139574000-2/review-scope-approved`
- presentation: `presentation/aa94a3269b529e5a`
- presentation content hash:
  `aa94a3269b529e5a4dd06f6fc11ab231fb5c5a47e5ba57dceb8bf8e0c394cf45`
- approval outcome: granted under delegated-maintainer authority through the
  proof-enforced path
- close status: `Completed`
- event summary: 39 total events, one approval, zero retries, and zero
  escalations
- presentation enforcement: `proof_enforced`, with one durable presentation
  record and the approval event marker present

## 14. Recommended Next Phase

Return to runtime execution depth rather than broadening provider mutations.
Plan the next closed, no-provider trusted-host engineering-check profile only
after reconciling the existing local-check policy and artifact boundaries. A
Cargo clippy profile is the leading candidate because it would add real
engineering evidence, but its build/cache writes, toolchain identity, output
bounds, timeout, source-tree integrity, and report/artifact semantics must be
explicitly governed before implementation.

Do not add another provider mutation, arbitrary command execution, nested
harness runtime, hosted execution, or default writes first.

## 15. Out-Of-Kernel Work

The kernel governed review scope and approval. Codex inspected source, tests,
prior reviews, the proof report, and the current external draft; authored this
review; and ran validation outside the kernel. The kernel did not read provider
state, edit files, run checks, or perform Git/PR actions.
