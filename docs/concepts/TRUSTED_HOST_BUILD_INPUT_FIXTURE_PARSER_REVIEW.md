# Trusted-Host Build-Input Fixture Parser Review

## 1. Executive Verdict

**Needs blocker fixes. Do not begin live Workflow OS workspace proof.**

The fixture parser has the right non-authorizing architecture. It uses a pinned
structured parser, reads a closed metadata set beneath an explicit fixture
root, applies hard file and graph bounds, rejects broad Cargo surfaces, keeps
external-package classification incomplete, and returns payload-free request
and result models. Its dependency and privacy posture are appropriate.

Three narrow contract blockers prevent acceptance. Explicit library and build
script paths are not normalized against the package directory, so `..` paths
can be accepted even though the plan requires root-escaping paths to fail
closed. Feature bounds are applied per manifest rather than across the whole
operation, and dependency feature/default/optional semantics are not included
in the resolved semantic graph. Finally, optional metadata discovery uses
`Path::exists`, allowing a broken symlink to be treated as absence instead of
rejected ambiguity. These are fixture-parser boundary defects; they do not
authorize execution or expose payloads, but they must be fixed before this
parser can support a live workspace proof.

## 2. Scope Verification

The implementation stayed within fixture-only scope. It added a parser and
static graph resolver, an exact parser dependency, bounded public input and
output types, deterministic commitments, tests, documentation, and an
implementation report.

It did not run Cargo, build scripts, procedural macros, providers, network
clients, or shell commands. It did not inspect Rust source contents, admit
trusted-host execution, persist discovery, append runtime events, expose
schemas or CLI behavior, create examples, enable provider writes, broaden
authority, or change release posture.

## 3. Dependency And Parser Boundary Assessment

The exact `toml` `0.8.23` dependency uses `default-features = false` and only
the `parse` feature. Its resolved subtree is limited to Serde, `toml_edit`,
`toml_datetime`, `serde_spanned`, `winnow`, and their existing foundational
dependencies. The newly resolved crates are MIT or MIT/Apache-2.0 licensed.

Typed TOML structures use `deny_unknown_fields`. Malformed or unsupported
metadata maps to stable value-independent errors rather than parser text.
Cargo lockfile versions are limited to 3 and 4. Non-empty Cargo configuration,
target-specific dependencies, workspace inheritance, Git dependencies,
registry overrides, and implicit targets fail closed.

The parser does not evaluate interpolation, commands, aliases, plugins, or
Cargo configuration. This boundary is minimal and appropriate.

## 4. Filesystem And Root-Containment Assessment

Metadata reads validate normal relative path components, reject symlink path
components, canonicalize the candidate, require it to remain beneath the
canonical fixture root, bound the read, and compare file metadata before and
after reading. Nested repositories under selected local packages are rejected.

Local dependency paths are normalized against the package directory and reject
escape above the root. Explicit library and build-script paths are not. They
call `validate_relative_text_path`, which rejects absolute and overlong values
but permits parent components such as `../../outside.rs`. The path is then
classified without being normalized or root-checked. Workspace exclusion paths
have the same lexical gap when they are not also selected members.

This violates the accepted plan's rule that root-escaping paths must be
rejected. The fact that this phase does not execute the path reduces immediate
impact, but accepting it as a complete static graph would create an unsound
input for the next live proof.

The reader's before/after metadata comparison covers size, modification time,
and file type but does not establish immutable object identity. The
implementation report discloses this limitation. A live proof will require a
stronger object-binding design; it is not a blocker for a fixture-only parser
once every accepted path is root-contained.

## 5. Optional Metadata And Symlink Assessment

Required metadata is routed through the symlink-rejecting reader. Optional
toolchain and Cargo configuration paths are first discovered with
`Path::exists`. A broken symlink returns false and is therefore treated as
absence without entering the reader. That contradicts the fail-closed symlink
posture and permits ambiguous optional metadata to disappear from the source
set commitment.

Optional presence detection must distinguish not-found from every other file
type and error. Any present symlink, including a broken symlink, must fail
closed through a stable non-leaking code. Focused tests must cover both
toolchain and Cargo config cases.

## 6. Cargo Graph Resolution Assessment

The resolver maps every selected local package to one source-less lock record,
walks the reachable lock graph, requires unambiguous dependency identities,
and adds explicit normal, build, and development role edges from manifests.
Local path edges must match their selected local manifest package. Nodes and
edges use ordered collections, and the commitment uses fixed-width
length-prefixed hashing inherited from the accepted model conventions.

External reachable packages are deliberately counted but force
`Incomplete` with `SelectionAmbiguous`, because the fixture lockfile cannot
prove their build-script or procedural-macro behavior. This is conservative
and correct.

The all-potential-edge selection includes optional and development edges rather
than silently dropping them. However, dependency `features`, `optional`, and
`default-features` are parsed but discarded from the semantic graph. Only
top-level `[features]` names and entries are committed. The implementation
report therefore overstates feature-expression coverage. The resolved graph
must bind all accepted feature-selection semantics, even when it conservatively
includes every package edge.

## 7. Bounds Assessment

File count, per-file bytes, aggregate bytes, workspace members, local path
dependencies, locked packages, package edges, and graph depth are bounded with
stable errors. Arithmetic uses checked conversion and addition where counts
can overflow.

`max_feature_expressions` is checked independently for each manifest. A
workspace can therefore accept approximately the per-manifest limit multiplied
by the number of selected manifests, exceeding both the caller limit and the
documented hard maximum for one operation. Dependency feature lists are not
included in this count at all.

The blocker fix must apply one checked aggregate feature-expression budget
across every selected manifest and dependency declaration. Tests must cross
the limit with individually valid manifests and dependency feature lists.

## 8. Determinism And Commitment Assessment

The request binds caller-supplied repository and selection commitments, the
canonical metadata-source set, and the parser policy. The result now separately
binds a resolved semantic graph commitment, and its outer graph commitment
binds the request, graph, completeness, counts, obligations, and reasons. This
closes the count-equivalent graph substitution gap.

Package nodes and dependency edges are ordered. Top-level feature maps are
ordered by key, but feature-entry vectors preserve author order even though
Cargo feature-list ordering is not semantic. The blocker fix should define and
test a canonical ordering for accepted set-like feature expressions and bind
dependency feature/default/optional facts. Equivalent semantics should not
depend on incidental array order unless the contract explicitly declares raw
manifest ordering authoritative.

Fixed vectors protect both the resolved graph and outer result commitment.
Repository and selection commitments remain caller supplied; this phase
correctly does not claim independent Git-object proof.

## 9. Completeness And Authority Assessment

`Complete` applies only to the accepted static fixture graph. Every live host
obligation remains `Unavailable`, and the result cannot authorize a process,
provider, handler, state transition, or write. This distinction is clear in
the API and documentation.

The parser never recreates host evidence or claims that lock metadata proves
runtime-discovered build inputs. External packages remain incomplete. No live
workspace or execution claim is authorized by this review.

## 10. Privacy And Error Assessment

Inputs redact fixture paths and caller commitments in Debug. Outputs contain
closed vocabulary, bounded counts, and commitments only. Output Debug redacts
commitments, and serialization contains no paths, package identities, source
URLs, feature strings, manifest text, build-script paths, commands, logs,
environment values, or provider payloads.

Parser, filesystem, consistency, and bound errors use stable
`trusted_host.build_input_fixture_discovery.*` codes and generic messages.
Secret-like malformed and unknown fields are not echoed. The privacy boundary
is accepted.

## 11. Test Quality Assessment

Existing tests cover a supported virtual workspace, fixed commitments, local
role edges, explicit build-script and procedural-macro classification,
external-package incompleteness, malformed and unknown metadata, local
dependency root escape, selected-member symlinks, nested repositories,
non-empty Cargo config, target-specific dependencies, workspace inheritance,
implicit targets and build scripts, file bounds, lock inconsistency, privacy,
and invalid limits.

Blocking regression gaps:

- root-escaping explicit library path;
- root-escaping explicit build-script path;
- root-escaping workspace exclusion path;
- broken optional toolchain symlink;
- broken optional Cargo-config symlink;
- aggregate feature overflow across manifests;
- aggregate dependency-feature overflow;
- dependency optional/default/feature commitment sensitivity; and
- order-independent canonical feature semantics.

Metadata-changing-during-read and graph-cycle tests from the accepted plan are
also absent. The former is difficult to make deterministic but needs a reviewed
test seam or bounded stress strategy before live proof. Cycle handling should
have a direct regression even though the visited set currently terminates it.

## 12. Documentation Assessment

The roadmap and plan correctly keep live workspace proof, host observation,
runtime admission, providers, persistence, schemas, and writes deferred. The
implementation report is strong but its broad statement that root-escaping
inputs are rejected and its feature-expression coverage summary are not fully
true for the defects above. The blocker-fix report must narrow or substantiate
those claims rather than erasing this review.

The implementation report honestly records the unavailable final dogfood event
inspection. No event count or reconstructed state is fabricated.

## 13. Blockers

1. Normalize and root-check every accepted path-bearing Cargo field used by
   the graph, including explicit library paths, explicit build-script paths,
   and workspace exclusions. Add non-leaking regressions.
2. Enforce one aggregate feature-expression budget across the complete
   discovery operation, including dependency feature lists.
3. Bind dependency optional/default/feature semantics and canonicalize
   set-like feature expressions in the resolved semantic graph.
4. Replace `Path::exists` optional metadata discovery with fail-closed presence
   handling that rejects broken and live symlinks.

## 14. Non-Blocking Follow-Ups

- Add a deterministic metadata-change test seam or a bounded concurrent-change
  stress test before live workspace proof.
- Add direct local dependency cycle and graph-depth regressions.
- Consider binding platform file identity where available in the future live
  immutable-object reader; do not imply the fixture metadata comparison is an
  immutable snapshot proof.
- Track the current-feed PostgreSQL and TLS RustSec advisories in a separate
  supply-chain phase. The CI-pinned advisory snapshot passes and the parser
  dependency subtree does not introduce those paths.
- Investigate why the preceding implementation run could not be rehydrated at
  phase close; do not reconstruct missing governance state by hand.

## 15. Recommended Next Phase

Implement a focused fixture-parser blocker fix, then perform a focused
blocker-fix review. Do not begin live Workflow OS workspace proof, process or
network observation, runtime admission, providers, persistence, schemas, CLI
behavior, examples, writes, authority broadening, or release changes before
acceptance.

## 16. Validation

The committed implementation passed:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `cargo metadata --locked --format-version 1`;
- the CI-pinned RustSec audit;
- `npm audit --audit-level=moderate`;
- `npm run check`;
- `npm run check:integrations`;
- `npm run check:docs`; and
- `git diff --check`.

Documentation checks and diff hygiene must be rerun after this review.

## 17. Governed Review Evidence

- Dogfood workflow: `dg/review`.
- Run ID: `run-1792828854452528000-2`.
- Approval ID:
  `approval/run-1792828854452528000-2/review-scope-approved`.
- Approval presentation: `presentation/7675c6a643a565f2`.
- Approval presentation content hash:
  `7675c6a643a565f28c4986d5e4eb71ff8500d88e0a606da080cf34f13fe06457`.
- Approval outcome: granted with persisted presentation proof under standing
  delegated-maintainer authority for the exact review-only scope.
- Phase-close status: `Completed` with 39 events, one approval, zero retries,
  and zero escalations.
- Event kinds: `ApprovalGranted:1`, `ApprovalRequested:1`,
  `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`,
  `RunResumed:1`, `RunStarted:1`, `RunValidated:1`,
  `SkillInvocationRequested:6`, `SkillInvocationStarted:6`,
  `SkillInvocationSucceeded:6`, and `StepScheduled:6`.
- Approval-presentation enforcement: proof enforced; one matching record and
  the persisted presentation marker were present in the event trail.
- Out-of-kernel work: the delegated maintainer inspected source, tests,
  dependency and license surfaces, roadmap evidence, and validation results and
  authored this review. The kernel coordinated governance only.
- Report posture: no runtime WorkReport artifact was generated or persisted.
