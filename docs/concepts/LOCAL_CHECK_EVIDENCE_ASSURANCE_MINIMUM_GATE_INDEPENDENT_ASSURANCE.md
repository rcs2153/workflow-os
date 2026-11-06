# Local Check Evidence Assurance Minimum Gate Independent Assurance

## Verdict

Independent agent assurance passed the local-check evidence-assurance minimum
gate with one low-severity hardening finding and one informational finding. No
P0, P1, or P2 blocker was found in the tested boundary.

The reviewer used a fresh detached checkout at exact commit
`30eb590f0d9e2b6824b3e3daa258bf1b33199c5a`. The primary workspace was not
modified. This is independent agent evidence for the narrow minimum gate, not
third-party organizational assurance, independent evidence authority, or
broader release/security assurance.

## Findings

### Low: unknown assurance fields are accepted

`LocalCheckEvidenceAssuranceProjectionWire` and
`LocalCheckEvidenceAssuranceMinimum` do not reject unknown serialized fields.
An external harness added an unknown field to a valid projection and observed:

```text
ACCEPTED_UNKNOWN_FIELD
```

Recognized fields and the projection fingerprint still control minimum
evaluation, so this did not elevate assurance or bypass the gate. It does
weaken the claimed closed wire posture and permits ignored data to travel with
an otherwise valid record. Before any stronger producer posture becomes
reachable, these wire types should reject unknown fields and retain a direct
regression.

### Informational: producer posture remains ordinal

`LocalCheckEvidenceProducerPosture` still derives `Ord` and `PartialOrd`. The
active gate correctly uses the explicit closed comparison table, so the
derived ordering does not currently authorize anything. The earlier focused
review incorrectly stated that the traits had been removed. They should be
removed before future code can accidentally reuse declaration order as policy.

## Attack Matrix

| Attack | Result | Evidence |
| --- | --- | --- |
| Declaration/minimum substitution | Passed | Absent, weak, and registered-producer minimums have distinct declaration fingerprints; stale-identity substitution was rejected. |
| Projection posture escalation | Passed within the local trust boundary | Stale-fingerprint escalation and unknown enum values were rejected. Unknown extra fields produced the low finding above. |
| Same-run downgrade after denial | Passed | Retry was rejected as `existing_run_unsupported`; one validator call, zero skill calls, and zero workflow events were observed. |
| Denial ordering | Passed | The projection was persisted and the minimum denied before workflow events or skill invocation. |
| Missing, duplicate, or ambiguous declarations | Passed | Resolver and immutable-bundle selection require exact unique records and declarations. |
| No declared minimum | Passed | Existing authoritative route behavior remained compatible and completed normally. |
| Projection corruption or subject mismatch | Passed within the local trust boundary | Fingerprint, address, run/result identity, bundle binding, and reassessed subject/posture checks rejected mismatch. |
| Replay and reassessment | Passed | Terminal reassessment preserved the first projection; semantic drift failed closed; denied same-run replay did not re-enter execution. |

The reassessment path relies on prior minimum admission plus immutable,
create-only posture matching rather than evaluating the minimum a second time.
That is safe for the currently reachable weak and no-minimum routes. It must be
reviewed again before any producer can establish a stronger posture.

## Independent Procedure

The reviewer created a clean detached checkout without hard links:

```sh
tmpdir=$(mktemp -d /private/tmp/workflow-os-security-review.XXXXXX)
git clone --no-hardlinks --no-checkout /Users/rsegar/Documents/WorkflowOS "$tmpdir/repo"
git -C "$tmpdir/repo" checkout --detach 30eb590f0d9e2b6824b3e3daa258bf1b33199c5a
git -C "$tmpdir/repo" rev-parse HEAD
git -C "$tmpdir/repo" status --short --branch
```

The observed checkout was detached, clean, and exactly
`30eb590f0d9e2b6824b3e3daa258bf1b33199c5a`.

Focused assurance tests passed:

- 7 assurance unit tests;
- 16 immutable-run-bundle-store tests;
- 7 declaration tests;
- 10 declaration-set tests; and
- 5 exact executor minimum, replay, and reassessment tests.

The first executor invocation truthfully failed because the isolated target did
not yet contain `target/debug/workflow-os`. After the reviewer built the CLI
prerequisite with `cargo build --locked --offline -p workflow-cli`, all five
focused executor tests passed.

The complete Core suite then passed with:

- 792 library tests;
- 351 local-executor tests, with 1 opt-in live test ignored;
- all remaining non-live integration targets; and
- 6 documentation tests.

The reviewer also compiled two external stdin-fed harnesses. The unknown-field
harness produced `ACCEPTED_UNKNOWN_FIELD`. A second harness recomputed the
projection commitment after changing posture and produced:

```text
RECOMPUTED_POSTURE_ACCEPTED
```

That second result is expected inside the disclosed local trust boundary. The
fingerprint is an unkeyed integrity commitment, not an authenticity proof. A
privileged storage owner that can replace all Core-owned records and recompute
their commitments remains inside the current threat model.

## Trust Boundary

The review proves bounded minimum enforcement, ordinary tamper detection,
identity-bound downgrade resistance, and replay behavior inside the current
local trust domain. It does not prove:

- trusted-host independence;
- validator semantic trustworthiness;
- producer registration or authentication;
- cryptographic signatures or external witnessing;
- resistance to a privileged storage owner that can rewrite every record and
  commitment;
- hosted execution safety; or
- provider safety.

The authoritative route still accepts no caller-supplied projection, and the
Core-owned writer remains crate-private. Serialized vocabulary alone remains
non-authorizing.

## Skipped Checks

Seven opt-in live provider or access-material tests were skipped, including
GitHub and Jira reads, provider writes, live local-check smoke tests, and the
PR-comment proof. PostgreSQL live and recovery bodies self-skipped because no
test database endpoints were supplied. No network calls, provider writes,
release actions, Clippy, TypeScript, or broader repository checks were part of
the independent run.

## Governed Review

- Workflow: `dg/review`
- Run: `run-1793936599787800000-2`
- Approval: `approval/run-1793936599787800000-2/review-scope-approved`
- Presentation: `presentation/f98687741fb4176b`
- Presentation hash:
  `f98687741fb4176b2552d7a0143ac96a6a2c01921f9d3757f08c9b21ed4ed157`
- Approval outcome: granted by the delegated maintainer after complete handoff
  presentation
- Independent reviewer: separate context-free agent in an isolated checkout
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- Approval-presentation enforcement: proof enforced

The kernel governed the review scope and approval proof. The independent agent
executed the attacks and tests outside the kernel and returned the verdict. The
maintainer preserved that verdict and did not rewrite either finding.

## Next Action

The narrow independent-assurance requirement for the current minimum gate is
satisfied. Before stronger producer posture is implemented or consumed, apply
the bounded hardening follow-up:

1. reject unknown fields on the assurance projection and minimum wire types;
2. remove producer ordering traits; and
3. rerun focused and complete Core validation.

This closure does not satisfy the repository's separate independent
release/security-assurance requirements.
