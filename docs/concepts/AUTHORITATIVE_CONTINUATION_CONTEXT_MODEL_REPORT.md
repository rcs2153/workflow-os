# Authoritative Continuation Context Model Report

## Status

The first model-only increment of the Authoritative Agent Continuation Context
And Rehydration P0 is implemented in Core and accepted after focused
maintainer/security review.

The increment is deliberately Core-private. It does not expose the complete
inventory, semantic values, or private source topology through the public crate
surface. A later consumer-visible projection must pass separate capability,
redaction, and security review before CLI, schema, report, hook, or child-harness
exposure.

## Implemented Model

The new pure decision layer defines:

- closed continuation purposes, source classes, and authority domains;
- source freshness, integrity, access, and sensitivity posture;
- purpose-bound collector expectations and observed outcomes;
- a complete private `ContinuationContextInventory`;
- a non-authorizing purpose-bound `ContinuationContextBundle`;
- bounded `AcceptedIntentRecord` values with explicit supersession;
- closed understanding-claim vocabulary and source candidates;
- authoritative, supported, advisory, conflicting, unknown, and withheld claim
  posture;
- explicit required-source, required-collector, and required-claim gaps;
- a non-authorizing `ContinuationUnderstandingProjection`; and
- a final ready-for-authority-resolution or blocked decision posture.

The decision does not contain an authority token and cannot authorize
execution. A ready result means only that the bounded context model has no
declared blocking gap. Material execution still requires the existing private
same-call current-authority boundary.

## Deterministic Decision Behavior

The pure decision function accepts only explicit in-memory inputs. It:

1. validates bounded input counts, identities, semantic text, and intent scope;
2. requires every closed source class to have exactly one collector-coverage
   entry and binds that coverage to the declared purpose;
3. rejects collector outcomes that contradict whether sources were actually
   supplied;
4. rejects duplicate collector classes, source identities, intent identities,
   and duplicate source claims;
5. rejects source-class/authority-domain substitution and claim combinations
   outside the closed precedence matrix;
6. converts current accepted intent into bounded context candidates while
   ignoring superseded intent;
7. admits required sources before optional sources;
8. enforces access, sensitivity, integrity, freshness, and materialization
   budget posture;
9. records every source in the private inventory, including omitted, stale,
   superseded, unauthorized, and invalid entries;
10. derives claim posture from closed source-specific precedence rules;
11. gives immutable run input precedence over later accepted objective or
    amendment intent for an active run;
12. blocks equal-authority conflicts and missing, withheld, or unknown required
    claims; and
13. derives domain-separated deterministic commitments for source coverage,
    private inventory, selected source content, purpose-bound understanding,
    and final decision.

Input order does not change canonical outputs. Required context cannot be
silently displaced by optional context under budget pressure. Optional budget
omission is inspectable but does not block when no required obligation depends
on it.

## Security Boundary

The complete inventory is private to Core. The module is not re-exported from
the public crate surface in this increment. Debug output redacts semantic values,
source identities, directive commitments, and accepted-intent content.

Denied or above-ceiling sources contribute only a withheld posture for an
already-required claim. Their identities are absent from the bounded bundle and
understanding claim. This avoids using the first model slice as a protected
source-existence oracle.

Accepted intent preserves bounded semantic effect only. It does not satisfy
policy, approval, capability, evidence, checks, SideEffects, or execution
requirements. Superseded intent cannot restore an old objective.

No collector, persistence, event, host, CLI, schema, provider, process, or
runtime execution behavior was added.

## Focused Tests

Twenty-one focused tests prove:

- order-independent canonical commitments;
- immutable-run scope precedence over operating-document guidance;
- equal-authority conflict blocking;
- required collector incompleteness blocking;
- complete closed source-class accounting and purpose-bound coverage;
- observed-without-source and source-with-non-observed-outcome rejection;
- source-class/authority-domain substitution rejection;
- denied source withholding without source-identity projection;
- accepted-intent orientation without execution authority;
- immutable objective precedence for an active run and accepted-intent
  precedence before a run exists;
- required-source budget exhaustion blocking;
- optional-source budget omission without blocking;
- required-before-optional budget admission;
- sensitivity-ceiling enforcement;
- superseded intent non-revival;
- source-content-sensitive bundle and understanding commitments;
- run/workflow scope-shape validation; and
- debug redaction of semantic values and source identities.

## Validation

Passed:

- repository-pinned `cargo fmt --all`;
- `cargo test -p workflow-core --lib continuation_context`;
- `cargo clippy -p workflow-core --all-targets -- -D warnings -A dead-code`;
- `cargo test --workspace`;
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`;
- `npm run check:docs`; and
- `git diff --check`.

The local pinned future Rust toolchain reports one pre-existing feature-gated
dead-code warning in `required_context.rs` when clippy runs without the
feature that consumes it. The focused clippy run allowed only that lint class;
all new model warnings remained denied. Exact repository CI remains required on
the final PR head.

## Governed Phase

- workflow: `dg/implement`
- phase: `implementation`
- run ID: `run-1793694626656172000-2`
- approval ID:
  `approval/run-1793694626656172000-2/implementation-approved`
- presentation ID: `presentation/43de100fe50ff2c2`
- approval outcome: granted under delegated-maintainer authority after review
  of the complete persisted handoff
- phase status: completed before implementation began; final phase-close
  inspection remains required after all validation and reporting

## Out-Of-Kernel Work

The delegated maintainer performed source inspection, branch creation, Rust
implementation, tests, formatting, clippy, documentation edits, and report
writing outside the kernel. The kernel governed scope and approval but did not
edit files, execute validation commands, create commits, or mutate GitHub.

## Deferred Work

Not implemented in this increment:

- local collectors or collector interfaces;
- durable accepted-intent capture;
- inventory or bundle persistence;
- consumer-visible inventory projection;
- CLI preview or JSON schema;
- hook, executor, or same-call consumer integration;
- trusted-host compaction signaling;
- projection delivery acknowledgment;
- replacement-executor dogfood proof;
- Salsa dependency or prototype; or
- any new operation or provider-mutation family.

The required focused maintainer/security review is complete in [Authoritative
Continuation Context Model Review](AUTHORITATIVE_CONTINUATION_CONTEXT_MODEL_REVIEW.md).
Trusted purpose-specific collector-expectation profile planning is next;
collectors, persistence, and consumer-visible projection remain blocked until
their separately governed phases.
