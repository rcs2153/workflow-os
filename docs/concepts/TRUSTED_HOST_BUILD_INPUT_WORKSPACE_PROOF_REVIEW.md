# Trusted-Host Build-Input Workspace Proof Review

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups; proceed to provider-neutral
host-input observer planning.**

The static Workflow OS workspace proof is deterministic, bounded,
non-authorizing, and honest about what the approved metadata snapshot cannot
prove. It closes the planned test-only proof without implying live workspace,
host, provider, or execution authority.

## 2. Scope Verification

The phase stayed within the approved static-proof scope:

- one isolated copy of the root manifest, lockfile, and four member manifests;
- the minimum closed Cargo subset required by current workspace metadata;
- deterministic parser invocation and payload-free result assertions;
- focused fail-closed tests; and
- roadmap, plan, implementation report, and review documentation.

It did not add ambient workspace discovery, Git object verification, Cargo or
package execution, host observation, runtime admission, handlers, persistence,
events, artifacts, schemas, SDKs, CLI behavior, examples, providers, writes,
authority broadening, or release changes.

## 3. Snapshot And Binding Assessment

Test setup copies exactly six approved metadata files into a fresh temporary
root. Discovery receives that root and explicit repository and selection
commitments. The reader rejects symlinks, nested repositories, root escapes,
duplicate reads, file or operation bound violations, and detectable metadata
changes during a read.

Every read metadata file contributes its relative path and full content hash
to the metadata source-set commitment. The proof therefore binds the bytes it
actually parsed. The separately supplied repository-tree commitment remains an
opaque caller assertion; the implementation and report correctly do not call
it independent Git-tree verification.

## 4. Cargo Surface Assessment

The new surface is appropriately narrow for the current repository:

- explicit `[[bin]]` name and root-contained path;
- selected string-valued workspace package inheritance;
- workspace resolver validation; and
- `[lints] workspace = true` only when root workspace lints are present.

Workspace dependency inheritance and workspace package-version inheritance
remain rejected. Packages still require an explicit library or binary target.
Explicit binary paths use the previously reviewed root-containment helper.
Missing inherited values and malformed or false markers fail closed with
stable non-leaking errors.

The parser treats root workspace lint contents as committed metadata rather
than interpreting their semantics. That is acceptable for this non-authorizing
static input proof because the complete manifest bytes are committed and no
compiler invocation is admitted. A later live or admission-capable boundary
must validate the exact supported lint shape or delegate that validation to an
independently governed source.

## 5. Determinism And Completeness Assessment

Two discoveries over the isolated snapshot must be exactly equal. The proof
asserts reviewable values rather than an unexplained whole-workspace golden
hash:

- 1 repository input;
- 6 manifest/configuration inputs;
- 208 reachable external dependency packages;
- 0 locally declared build scripts;
- 0 locally declared procedural macros; and
- 215 total inputs.

All nine host obligations are `Unavailable`, the observation reference is
absent, and external package metadata keeps static completeness `Incomplete`
with `SelectionAmbiguous`. The proof does not infer external build-script or
procedural-macro roles from package names or lockfile entries.

## 6. Privacy And Error Assessment

The public request/result boundary contains only closed vocabulary, bounded
counts, commitments, and obligation posture. It does not expose paths, package
identities, versions, source URLs, TOML, source code, commands, logs,
environment values, credentials, or provider payloads. Debug redacts fixture
roots and commitments, while stable parser errors avoid caller values.

Existing malformed, secret-like, symlink, path, bound, and serde regression
tests remain green. The new errors reuse the existing stable
`workspace_inheritance_unsupported` and `root_escape` families.

## 7. Authority Assessment

The result validates only against its discovery request. It cannot observe a
host, attest an immutable Git object, start a process, select a provider,
admit trusted-host execution, mutate runtime state, or authorize a write.
Static completeness and live input completeness remain separate, and the
proof does not satisfy the unavailable host obligations.

This is the correct authority posture for a test-only proof.

## 8. Test Quality Assessment

Focused coverage proves:

- repeatability over the current Workflow OS metadata snapshot;
- exact bounded counts and honest incomplete posture;
- request/result binding;
- unavailable host obligations and absent observation reference;
- selected workspace package inheritance;
- required root workspace lint presence;
- unsupported workspace package-version inheritance; and
- explicit binary target root containment.

All earlier parser tests for canonicalization, graph semantics, operation-wide
bounds, symlinks, optional metadata, privacy, and errors remain green.

Useful non-blocking additions before any live parser authority are:

- reject duplicate explicit binary target names and paths directly;
- validate the exact accepted root workspace-lints shape rather than presence
  alone; and
- add the previously identified direct graph-cycle and deterministic
  concurrent-metadata-change seams.

These do not block the current proof because its metadata is fixed, copied,
fully committed, non-authorizing, and covered by exact repeatability checks.

## 9. Documentation Assessment

The implementation report accurately distinguishes isolated test setup from
production object binding and static graph inputs from live host closure. One
stale planning sentence implied that external procedural-macro packages could
be classified from lockfile data alone. This review corrects it to match the
implemented and documented unresolved posture.

The roadmap and proof plan now identify provider-neutral host-input observer
planning as the next phase. They do not authorize implementation.

## 10. Validation Assessment

The reviewed commit passed:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- focused trusted-host build-input tests: 42 passed;
- exact workspace proof test;
- `cargo test --workspace`;
- `cargo metadata --locked --format-version 1`;
- `npm run check` under Node 20;
- `npm run check:integrations`;
- `npm run check:docs`;
- `git diff --check`;
- `npm audit --audit-level=moderate`, with zero vulnerabilities; and
- the CI-pinned RustSec audit at
  `309ad29d8fe448bf986019e05d47b9e0e29a2218`, with no advisory finding and one
  allowed yanked transitive dependency warning.

The first full-workspace test invocation overlapped an earlier Cargo process
and produced one ambiguous-persistence failure in a pre-existing timer test.
That exact test passed in isolation and the uncontended full workspace rerun
passed. The review does not attribute that transient to this parser change.

## 11. Blockers

None for provider-neutral host-input observer planning.

## 12. Non-Blocking Follow-Ups

- Add duplicate explicit binary target validation before live parser use.
- Replace opaque workspace-lints presence with an exact accepted shape before
  any runtime admission claim.
- Add direct graph-cycle and concurrent metadata-change test seams.
- Bind repository identity to an independently verified immutable object
  rather than a caller-supplied commitment.
- Keep external package build-script and procedural-macro roles unresolved
  until their manifests are obtained through a governed source.

## 13. Recommended Next Phase

Plan the provider-neutral host-input observer for toolchain, sysroot, linker,
dynamic-library, and immutable object-binding facts. The plan must define an
independently reviewable observation and freshness boundary without selecting
a provider or authorizing execution.

Do not implement a live observer, process execution, runtime admission,
provider integration, persistence, schemas, CLI behavior, examples, writes,
authority broadening, or release changes in that planning phase.

## 14. Governed Review Evidence

- Dogfood workflow: `dg/review`.
- Run ID: `run-1792832300827614000-2`.
- Approval ID:
  `approval/run-1792832300827614000-2/review-scope-approved`.
- Approval-presentation ID: `presentation/b0911631ce2c81a2`.
- Approval-presentation content hash:
  `b0911631ce2c81a21a5dbb40265269977d3023e2c6716996650089eaab2e805f`.
- Approval outcome: granted with persisted presentation proof under standing
  delegated-maintainer authority for the exact focused review scope.
- Phase-close status: `Completed`.
- Event summary: 39 events, 1 approval, 0 retries, and 0 escalations.
- Approval-presentation enforcement: proof enforced with one persisted record
  and the presentation marker present in the approval event trail.
- Out-of-kernel work: the delegated maintainer inspected the committed parser,
  tests, source-set and graph commitments, trust assumptions, validation
  evidence, roadmap, plan, and report and authored this review. The kernel
  coordinated governance only.
- Report posture: no runtime WorkReport artifact was generated or persisted.
