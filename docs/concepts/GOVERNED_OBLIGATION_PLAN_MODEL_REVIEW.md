# Governed Obligation Plan Model Review

## Verdict

Accepted after one determinism correction. The Phase 2 model remains private,
non-authorizing, non-serializing, and disconnected from runtime behavior.

A separately governed Phase 3 may add only private deterministic explanation
and independently specified differential validation. Runtime composition,
persistence, and public surfaces remain unauthorized.

## Finding And Correction

### P1: platform-width framing could change V1 commitments

The initial implementation framed each commitment segment with
`usize::to_be_bytes()`. That encoding is eight bytes on 64-bit targets and four
bytes on 32-bit targets, so identical semantic inputs could produce different
V1 commitments across platforms.

The correction:

- uses an explicit unsigned 64-bit big-endian segment length;
- fails closed if a host cannot represent the input length as `u64`; and
- adds fixed known vectors for both obligation identities, the complete source
  closure, ordering, and aggregate inventory commitment.

This is a model-contract correction, not a new feature. No runtime or public
surface was added.

## Accepted Properties

### Independent completeness commitments

The builder receives the canonical source closure independently from the
derived obligations. Omitting an obligation while retaining its source changes
the inventory commitment without changing the source-closure commitment.
Omitting a canonical source changes the source-closure commitment. The model
therefore does not use the derived entries as proof that its own source set was
complete.

### Source-derived identity

Obligation identity is derived from a versioned domain, closed source kind,
bounded source identity, source content commitment, and closed obligation kind.
Callers cannot provide an obligation ID or arbitrary kind string. Posture and
evidence class remain part of the aggregate inventory commitment while the
obligation identity remains stable for one canonical source and obligation
kind.

### Closed, static vocabulary

The model represents only static authored requirements. It contains no policy
decision, approval outcome, evidence result, check result, SideEffect outcome,
authority state, or completion state. Runtime observations cannot manufacture
an authored obligation.

### Fail-closed construction

The builder rejects empty closures, empty inventories, malformed source
identity, conflicting commitments for one source identity, obligations outside
the independent closure, and repeated derived obligations. Ordering is
deterministic over closed source and derived identity values.

### Private and redaction-safe boundary

The module is crate-private and not re-exported. Constructors remain
module-owned. There is no Serde, persistence, schema, SDK, CLI, loader,
executor, report, or provider integration. Debug output redacts source
identity, source and aggregate commitments, and obligation IDs.

## Residual Risks And Required Follow-Up

- Canonical source and obligation counts are not yet bounded. This is
  acceptable while the model is private and unreachable, but any loader,
  persistence, or runtime composition must add reviewed cardinality bounds
  before accepting repository-controlled input.
- V1 uses `SpecContentHash::from_bytes` over the framed SHA-256 digest, making
  the versioned commitment a deliberate double hash. The known vectors now
  make this behavior explicit and prevent accidental drift. A future algorithm
  must use a new version rather than changing V1.
- Exact duplicate canonical sources with the same identity and commitment
  normalize to one set member. Conflicting duplicates fail closed. Any future
  extractor must independently prove that this set normalization matches the
  canonical source contract it consumes.
- The model does not yet derive sources from `ProjectBundle`, immutable run
  bundles, or runtime contracts. Phase 3 fixtures must not pretend otherwise.
- Phase 3 expected obligations and runtime outcomes must be authored
  independently from the derivation helper. Reusing implementation output as
  the oracle remains prohibited.

## Validation

- `cargo test -p workflow-core --lib governed_obligation_plan`: 11 passed;
- fixed V1 commitment-vector coverage: passed;
- `cargo clippy -p workflow-core --all-targets -- -D warnings`: passed;
- `cargo fmt --all --check`: required before phase close;
- `npm run check:docs`: required before phase close;
- `git diff --check`: required before phase close.

The preceding exact-head PR matrix also passed all nine required GitHub checks
for the merged Phase 2 implementation. That CI evidence does not replace this
focused review or the corrected-vector tests.

## Governance

- workflow: `dg/review`;
- run: `run-1793957225402472000-2`;
- approval: `approval/run-1793957225402472000-2/review-scope-approved`;
- presentation proof: `presentation/508930c74bf54d1d`;
- presentation content hash:
  `508930c74bf54d1dc4e93cc544a7ee41927d43e5e6e3c0b1f8201ea74b26e4c6`;
- approval outcome: granted under delegated maintainer authority;
- phase status: `Completed`;
- event summary: 39 events, one approval, zero retries, zero escalations;
- approval-presentation enforcement: `proof_enforced` with one persisted
  presentation record.

Review, source correction, tests, documentation, Git, and provider actions were
performed outside the kernel executor. The kernel governed phase scope and
approval. No independent third-party review, runtime behavior, or public
surface was simulated.

## Next Authorized Phase

Proceed only through a separately governed Phase 3 private explanation and
differential-validation slice. It must:

- produce deterministic, non-authorizing explanations from the accepted model;
- use independently specified expected obligations and runtime outcomes;
- include negative omission, unsupported, SideEffect-inference, stale-closure,
  and unstable-order fixtures;
- fail review on disagreement instead of selecting the permissive result; and
- remain private, non-persistent, and disconnected from executor authority.

Phase 4 runtime composition, incremental dependencies, Salsa, schemas, SDKs,
CLI output, providers, release behavior, and publication remain unauthorized.
