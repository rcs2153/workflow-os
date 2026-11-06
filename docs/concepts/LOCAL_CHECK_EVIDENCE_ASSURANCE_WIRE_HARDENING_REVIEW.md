# Local Check Evidence Assurance Wire Hardening Review

## Verdict

Accepted. The bounded hardening closes both findings from the independent
minimum-gate assurance pass without changing minimum semantics, reachable
producer posture, runtime routing, persistence, or authorization behavior.

No P0, P1, P2, or lower-severity blocker was found in the reviewed scope.

## Reviewed Scope

The review inspected exact commit `a191c4ddf0ef2938fe72d8dd7e2065b945e6666d`
and covered:

- closed-field deserialization for the Core-owned assurance projection;
- closed-field deserialization for the public assurance minimum;
- removal of producer ordering traits;
- direct unknown-field regressions;
- the existing explicit producer-assurance comparison table;
- the implementation report and roadmap claims; and
- focused, complete Core, static, and documentation validation evidence.

## Findings

No blocking or advisory finding remains in this narrow increment.

### Closed wire behavior is correctly bounded

`LocalCheckEvidenceAssuranceProjection` deserializes through a private wire
record marked `deny_unknown_fields`. `LocalCheckEvidenceAssuranceMinimum` is
itself marked `deny_unknown_fields`. An unknown top-level assurance claim is
therefore rejected for both records rather than silently ignored.

The direct regression constructs otherwise valid records, adds one
unrecognized claim, and requires deserialization failure. The test does not
depend on error text and therefore preserves the fail-closed behavior without
turning diagnostics into public API.

### Declaration order cannot become implicit producer policy

`LocalCheckEvidenceProducerPosture` no longer implements `Ord` or
`PartialOrd`. The active gate continues to evaluate producer minimums through
the exhaustive `producer_satisfies` match table. Current weak and stronger
requirement vocabulary therefore retain their reviewed meaning without an
ordinal fallback.

The stronger `registered_content_bound` posture remains requirement vocabulary
only. This hardening does not make it constructible through a new trusted
runtime source.

## Trust Boundary

Acceptance means that the current closed wire shape rejects unknown assurance
claims and that current producer-policy evaluation cannot accidentally use enum
declaration order. It does not prove:

- that the selected validator is semantically trustworthy;
- producer registration or authentication;
- trusted-host or storage-owner independence;
- signatures, external witnessing, or cryptographic authenticity;
- resistance to replacement of all locally owned records and unkeyed
  commitments by a privileged storage owner; or
- broader release/security assurance.

Serialized vocabulary remains non-authorizing, and stronger producer posture
still requires separately governed implementation and assurance.

## Validation

Reviewed validation passed:

- focused unknown-vocabulary regression;
- `cargo check -p workflow-core --all-features`;
- complete `cargo test -p workflow-core --all-features`, including 792 library
  tests, 351 local-executor tests with one opt-in live test ignored, all
  remaining non-live integration targets, and 6 documentation tests;
- warnings-denied Clippy for all Workflow Core targets and features;
- repository-wide Rust formatting;
- documentation and release-state checks; and
- diff validation.

The reviewer also inspected the exact committed diff and reran the focused
unknown-vocabulary regression. Live provider, credential-backed, hosted, and
external-witness checks were not applicable to this local type and wire-shape
hardening increment and were not run.

## Governed Review

- Workflow: `dg/review`
- Run: `run-1793939008042882000-2`
- Approval: `approval/run-1793939008042882000-2/review-scope-approved`
- Presentation: `presentation/f2b2faa844e449ad`
- Presentation hash:
  `f2b2faa844e449ad437aa2f88d833d985d2847ecb231d269aa21a12166115457`
- Approval outcome: granted by the delegated maintainer after complete handoff
  presentation
- Kernel freshness: rebuilt from the exact checkout before phase start
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- Approval-presentation enforcement: proof enforced

Repository inspection, validation, and documentation were performed outside
the kernel under the approved review scope. The kernel coordinated the phase
and approval; it did not execute commands or manufacture review evidence.

## Outcome

The current local-check evidence-assurance minimum-gate hardening lane is
closed. A stronger producer posture, producer registry, assurance upgrade, or
broader authorization use remains separate future work and requires its own
governed scope and assurance.
