# Trusted-Host Build-Input Workspace Proof Report

## 1. Executive Summary

Workflow OS now has one deterministic, test-only static build-input discovery
proof over its current Rust workspace metadata. The proof copies the root
manifest, lockfile, and four member manifests into an isolated temporary
snapshot, supplies explicit repository and selection commitments, invokes the
accepted fixture parser twice, and requires identical payload-free results.

The current snapshot resolves 1 repository input, 6 manifest/configuration
inputs, and 208 reachable external dependency packages for 215 total inputs.
Every host obligation remains unavailable and external package metadata keeps
the result `Incomplete`. The proof does not run Cargo, package code, build
scripts, procedural macros, providers, network clients, or shell commands as
part of discovery.

## 2. Scope Completed

- Added a Workflow OS workspace snapshot proof using the accepted parser.
- Copied only the approved six metadata files into an isolated test fixture.
- Added the exact closed Cargo subset needed by the current workspace:
  explicit binary targets, selected workspace package-string inheritance, and
  workspace lint inheritance.
- Preserved fail-closed workspace dependency and package-version inheritance.
- Root-checked every accepted explicit binary target path.
- Asserted repeatability, request/result binding, reviewable counts, no
  fabricated local build-script or procedural-macro facts, unavailable host
  obligations, no observation reference, and incomplete static posture.

## 3. Scope Explicitly Not Completed

This phase did not add ambient live-workspace discovery, Git object binding,
Cargo execution, package execution, build-script or procedural-macro
execution, host observation, toolchain inspection, process or network access,
runtime admission, handlers, persistence, events, WorkReports, artifacts,
schemas, SDKs, CLI behavior, examples, providers, writes, authority
broadening, or release changes.

## 4. Workspace Snapshot Boundary

The proof locates the repository only in test setup. It copies `Cargo.toml`,
`Cargo.lock`, and the four current member manifests into a fresh temporary
directory. Discovery receives that isolated directory plus explicit opaque
repository and selection commitments. The parser has no ambient-current-
directory authority and reads only metadata reached beneath the supplied root.

The repository commitment remains caller supplied; this proof does not claim
independent Git-tree or immutable-object verification. The isolated copy
prevents ordinary workspace mutation from changing files during one discovery
operation, while the parser's existing before/after metadata checks remain in
force.

## 5. Minimum Cargo Surface

The prior fixture parser intentionally rejected workspace inheritance and
binary-only packages. Workflow OS itself uses a narrow subset of both. This
phase adds only:

- explicit `[[bin]]` names and root-contained paths;
- string-valued workspace package inheritance for edition, Rust version,
  license, repository, homepage, and readme; and
- `[lints] workspace = true` when root workspace lints are present.

Missing inherited values, unsupported workspace package-version inheritance,
workspace dependency inheritance, false or malformed markers, implicit
targets, and root-escaping binary paths fail closed with stable non-leaking
errors.

## 6. Proof Result

For the current committed workspace metadata, repeated discovery produces the
same request and result:

- repository content: 1;
- manifests/configuration: 6;
- reachable external dependency packages: 208;
- local declared build scripts: 0;
- local declared procedural macros: 0;
- total bounded inputs: 215;
- static completeness: `Incomplete`;
- unsupported reason: `SelectionAmbiguous`;
- host obligations: all 9 `Unavailable`; and
- observation reference: absent.

The dependency count is deliberately reviewable rather than hidden behind a
whole-repository golden hash. A lockfile change must update the expected count
and receive review.

## 7. Honest Incompleteness

The lockfile identifies reachable external packages but does not contain the
external manifests needed to prove which packages declare build scripts or
procedural-macro targets. The proof therefore does not infer those roles from
package names and does not claim complete static or live build-input closure.

Toolchain executable, target specification, SDK/sysroot, linker/archiver,
native compiler tools, dynamic libraries, registry/package-source objects,
environment, and runtime-base facts remain unavailable. A later independently
reviewed observer is required before any live-input completeness or execution
admission claim.

## 8. Privacy And Authority

The public result contains closed vocabulary, bounded counts, commitments, and
obligation posture only. It contains no paths, package names, versions, source
URLs, TOML, source code, commands, logs, environment values, credentials, or
provider payloads. Debug redacts commitments and existing stable errors do not
echo caller values.

The proof result is non-authorizing. It cannot start a process, select a
provider, admit trusted-host execution, mutate state, or write externally.

## 9. Test Coverage

Focused tests cover:

- current Workflow OS workspace repeatability and exact reviewable counts;
- closed workspace package and lint inheritance;
- missing inherited values and lint definitions;
- unsupported workspace package-version inheritance;
- explicit binary target root escape; and
- all existing fixture parser, canonicalization, bounds, symlink, privacy, and
  error regressions.

## 10. Validation

The complete required matrix passed:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- focused trusted-host build-input tests: 42 passed;
- exact Workflow OS workspace proof test: passed;
- `cargo test --workspace`;
- `cargo metadata --locked --format-version 1`;
- `npm run check` under the repository-pinned Node 20 toolchain;
- `npm run check:integrations`;
- `npm run check:docs`;
- `git diff --check`;
- `npm audit --audit-level=moderate`: 0 vulnerabilities; and
- `cargo audit --db /private/tmp/workflow-os-rustsec-db --no-fetch` at the
  CI-pinned advisory database commit
  `309ad29d8fe448bf986019e05d47b9e0e29a2218`: no security advisories and one
  allowed warning for the yanked transitive `chacha20 0.10.1` crate.

An initial full-workspace test invocation overlapped an earlier Cargo process
and reported one ambiguous-persistence failure in the pre-existing
`schedule_once_rehydrates_after_restart` timer test. The exact test passed in
isolation, and an uncontended complete `cargo test --workspace` rerun passed.
No source change was made to suppress or bypass that result.

## 11. Remaining Limitations

- Repository and selection commitments are explicit caller inputs, not
  independently verified Git objects.
- The proof snapshot is assembled by test setup rather than a production
  object-bound reader.
- External package manifests are unavailable, so external build-script and
  procedural-macro classification remains unresolved.
- No deterministic concurrent metadata-change seam or direct graph-cycle test
  is added in this phase.
- No live host fact is observed and no execution is authorized.

## 12. Recommended Next Phase

Perform a focused maintainer/security review of this static workspace proof.
Do not begin host observation, runtime admission, provider integration,
persistence, schemas, CLI behavior, examples, writes, authority broadening,
or release changes before acceptance.

## 13. Governed Implementation Evidence

- Dogfood workflow: `dg/implement`.
- Run ID: `run-1792831016285269000-2`.
- Approval ID:
  `approval/run-1792831016285269000-2/implementation-approved`.
- Approval-presentation ID: `presentation/740ca5ae9a847692`.
- Approval-presentation content hash:
  `740ca5ae9a847692e245927aa648846b176ba6daaaf99b558a54c63dd7630636`.
- Approval outcome: granted with persisted presentation proof under standing
  delegated-maintainer authority for the exact implementation scope.
- Phase-close status: `Completed`.
- Event summary: 39 events, 1 approval, 0 retries, and 0 escalations.
- Approval-presentation enforcement: proof enforced with one persisted record
  and the presentation marker present in the approval event trail.
- Out-of-kernel work: the delegated maintainer edited source, tests, roadmap,
  plan, and report files and ran validation. The kernel coordinated governance
  only.
- Report posture: no runtime WorkReport artifact was generated or persisted.
