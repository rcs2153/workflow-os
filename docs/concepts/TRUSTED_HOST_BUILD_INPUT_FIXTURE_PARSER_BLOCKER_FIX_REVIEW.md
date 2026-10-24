# Trusted-Host Build-Input Fixture Parser Blocker Fix Review

## 1. Executive Verdict

**Blockers fixed; proceed to the static Workflow OS workspace discovery proof.**

The focused change resolves all four blockers from the fixture-parser review
without broadening the fixture-only, process-free, non-authorizing boundary.

## 2. Scope Verification

The fix stayed within the approved scope:

- path normalization and root containment;
- operation-wide feature-expression bounds;
- dependency feature/default/optional graph commitment;
- canonical set-like feature ordering;
- optional metadata symlink handling;
- focused adversarial tests; and
- honest roadmap, plan, and phase evidence.

It did not add live workspace proof, Cargo or build-code execution, host
observation, runtime admission, providers, persistence, schemas, CLI behavior,
examples, writes, authority broadening, or release changes.

## 3. Path Containment Assessment

The shared `normalize_relative_join` boundary now covers explicit library and
build-script paths as well as workspace member, workspace exclusion, and local
dependency paths. It validates bounded relative text, removes current and
parent components, and rejects any parent traversal that would cross the
fixture root.

Focused regressions prove root-escaping explicit library paths, explicit build
scripts, workspace exclusions, and local dependencies fail with the stable
non-leaking `root_escape` code. Normalization inside the approved root remains
valid, which is the correct lexical policy for this fixture-only parser.

## 4. Aggregate Bounds Assessment

One checked `feature_expression_count` now spans all selected manifests.
Feature names, canonical top-level feature entries, and canonical dependency
feature selections consume the same caller-supplied budget. Checked conversion
and addition prevent integer overflow, and the bound is no longer multiplied
implicitly by workspace size.

The adversarial two-member fixture crosses the operation limit only in
aggregate and fails closed. This resolves the accepted bound blocker.

## 5. Semantic Commitment Assessment

Manifest dependency records retain `optional`, effective `default-features`,
and sorted/deduplicated feature selections. Manifest feature entries are also
sorted and deduplicated. The resolved graph edge commitment binds each of
these dependency facts, while manifest feature commitments separately bind the
feature name and canonical expression set.

Tests prove incidental dependency feature order does not change the commitment
and a change to optional/default semantics does. The fixed semantic and outer
result vectors were deliberately advanced. This resolves the graph-semantic
substitution blocker without exposing feature payloads publicly.

## 6. Optional Metadata Assessment

Optional toolchain and Cargo configuration presence now uses
`symlink_metadata`. Only `NotFound` is absence. A broken or live symlink is
therefore treated as present and reaches the existing reader, which rejects
symlink path components with a stable non-leaking code.

Unix regressions cover broken `rust-toolchain.toml` and
`.cargo/config.toml` symlinks. Optional metadata can no longer silently
disappear from the observed source-set posture through `Path::exists` behavior.

## 7. Privacy And Error Assessment

The fix adds no public raw metadata. New state is private parser state and is
reduced into the existing payload-free graph commitment. Error paths retain
stable generic fixture-discovery codes and do not copy paths, package names,
feature strings, TOML, parser details, source contents, commands, environment
values, credentials, or provider payloads.

Existing Debug and serialization non-leakage tests remain green.

## 8. Test Quality Assessment

Focused coverage now includes every blocking regression requested by the
original review:

- escaping explicit library and build-script paths;
- escaping workspace exclusions and local dependencies;
- broken optional toolchain and Cargo-config symlinks;
- aggregate feature overflow across manifests and dependencies;
- canonical order-independent feature semantics; and
- commitment sensitivity to dependency optional/default/feature facts.

The focused trusted-host suite passes 39 tests, and the complete workspace
suite passes. Direct graph-cycle coverage and a deterministic metadata-change
test seam remain useful before the static workspace proof is accepted, but the
existing visited-set termination and before/after metadata checks make these
non-blocking for this narrow repair.

## 9. Validation Assessment

The reviewed commit passed:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `cargo metadata --locked --format-version 1`;
- the CI-pinned RustSec audit at
  `309ad29d8fe448bf986019e05d47b9e0e29a2218`, with one allowed yanked
  dependency warning and no vulnerability finding;
- `npm audit --audit-level=moderate`, with zero vulnerabilities;
- `npm run check` under Node 20;
- `npm run check:integrations`;
- `npm run check:docs`; and
- `git diff --check`.

## 10. Remaining Non-Blocking Follow-Ups

- Add direct local dependency cycle and graph-depth regressions.
- Add a deterministic metadata-change seam or bounded concurrent-change stress
  test before accepting live or mutable filesystem proof.
- Keep platform object identity and immutable snapshot binding explicit in the
  next proof; this fix does not upgrade metadata comparison into object proof.
- Continue tracking the pre-existing current-feed PostgreSQL/TLS RustSec
  advisories separately from this parser change.
- Investigate the earlier missing implementation-run state without
  reconstructing kernel state by hand.

## 11. Blockers

None for the next static Workflow OS workspace discovery proof.

## 12. Recommended Next Phase

Implement one static Workflow OS workspace discovery proof using the accepted
parser against an explicitly supplied immutable repository snapshot and
selection commitment. Assert deterministic repeatability, bounded reviewable
workspace invariants, and honest incompleteness for unresolved external and
host facts.

Do not run Cargo or build code during discovery, and do not add host
observation, runtime admission, providers, persistence, schemas, CLI behavior,
examples, writes, authority broadening, or release changes.

## 13. Governed Review Evidence

- Dogfood workflow: `dg/review`.
- Run ID: `run-1792830129172894000-2`.
- Approval ID:
  `approval/run-1792830129172894000-2/review-scope-approved`.
- Approval-presentation ID: `presentation/5156f3201d773259`.
- Approval-presentation content hash:
  `5156f3201d773259d0b41b8e029ba34c7fd85aa0c68102410203c6db3eac94bc`.
- Approval outcome: granted with persisted presentation proof under standing
  delegated-maintainer authority for the exact focused review scope.
- Phase-close status: `Completed`.
- Event summary: 39 events, including one approval request, one approval grant,
  six scheduled steps, six successful skill invocations, zero retries, and zero
  escalations.
- Approval-presentation enforcement: proof enforced; one matching record and
  the persisted presentation marker were present in the event trail.
- Out-of-kernel work: the delegated maintainer inspected source, tests,
  commitment semantics, validation evidence, and phase documentation and
  authored this review. The kernel coordinated governance only.
- Report posture: no runtime WorkReport artifact was generated or persisted.
