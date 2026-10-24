# Trusted-Host Local Cargo Input Observer Source Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups. Proceed to planning the same-call
local Cargo observation and execution-composition boundary.

The fixed source is acceptable as private, conservative preflight evidence.
It is not execution authority, machine attestation, or proof that the observed
inputs remain identical through a later Cargo clippy invocation.

## 2. Scope Verification

The implementation stayed within the approved fixed-source scope. It added
closed typed local roots and executable roles, four fixed read-only probes,
bounded filesystem and process behavior, injected process-free tests, one
ignored Apple Silicon smoke, dependency updates, and honest phase docs.

It did not add runtime composition, admission, Cargo clippy execution,
OpenShell wiring, persistence, workflow or audit events, schemas, CLI
behavior, provider writes, additional live platforms, or release changes.

## 3. Selection And Binding Assessment

`LocalCargoFixedProbeDefinition` requires exactly five executable roles and
canonicalizes the repository root, Cargo home, and executable files. Direct
symlink entries, missing roles, extra roles, non-directories, and non-regular
files fail closed. The resulting selection commitment is bound to the exact
Core observation request together with its operation and security
commitments.

The paths remain private local-host inputs and Debug redacts them. Because the
definition is private and no runtime caller exists, this phase does not expose
caller-selected process authority. The later composition plan must identify
one authoritative source for these inputs rather than widening this
constructor into public configuration.

## 4. Filesystem Assessment

Repository traversal is deterministic, sorted, symlink-rejecting, and bounded
by file count and aggregate bytes. Each included object is revalidated as a
regular file before reading, and the resulting tree commitment must equal the
request's immutable repository-tree commitment.

This establishes a bounded preflight snapshot, not immutable custody. A
repository may change after observation, and a file can change around a
pathname-based read. Those limitations are correctly reflected by uncertain
root integrity and continuity. Same-call composition must either retain exact
custody or detect drift again at the execution boundary.

## 5. Process And Probe Assessment

The standard runner can select only rustc verbose version, rustc sysroot,
rustc target libdir, and locked offline Cargo metadata without dependencies.
Executables are absolute, no shell is used, stdin is null, ambient environment
is cleared, and only explicit `CARGO_HOME` is supplied.

Stdout and stderr are independently bounded. A 15-second timeout terminates
the Unix process group before reader threads are joined. Nonzero status, any
stderr, missing streams, invalid UTF-8, empty output, malformed rustc identity,
non-absolute path output, and malformed metadata fail closed with stable
non-leaking errors.

Executable files are committed before and after each invocation, and the
complete executable set is committed around collection. This detects ordinary
drift but does not prevent a pathname substitution race between commitment
and process creation. The conservative posture makes that residual risk
non-blocking for preflight evidence and blocking for any future claim of
`Observed` continuity.

## 6. Fact And Posture Assessment

All nine obligation kinds remain complete and within the accepted matrix:

- toolchain, target, sysroot, native tools, and package sources are
  `Uncertain`;
- linker and environment are `DeclaredOnly`; and
- dynamic libraries and runtime base are `Unsupported`.

No root or fact claims proven integrity or continuity. `IndependentlyReviewed`
continues to identify the reviewed producer implementation, not the current
machine or its inputs. The result cannot satisfy live-execution admission.

## 7. Parser And Privacy Assessment

The parsers accept only the minimum reviewed shapes and store only hashes and
counts. Core receives no path, Cargo metadata document, stdout, stderr,
environment value, source content, registry location, or credential.

Debug output redacts roots, paths, and selection identity. Errors are stable
and generic and do not echo failed values. Serialization does not expose a new
public source shape because the implementation remains private.

## 8. Test Quality Assessment

Ordinary tests start no process. They prove complete source construction with
an injected runner, closed executable-role validation, symlink rejection,
parser closure, repository and executable drift detection, and Debug
redaction. Existing observer tests continue to prove request, producer,
ordering, fact, posture, and in-memory evidence bindings.

The ignored opt-in Apple Silicon smoke exercises the real absolute executable,
cleared-environment, fixed-process path in a disposable Cargo project and
passes. The workspace snapshot was honestly updated for the two new locked
dependency packages after the full suite caught an unsupported target-specific
manifest declaration.

## 9. Blockers

None for this private preflight source.

## 10. Non-Blocking Follow-Ups

- Keep the fixed definition private until an authoritative runtime owner for
  roots and executable selection is reviewed.
- Do not infer same-call custody or `Observed` posture from pre/post pathname
  commitments.
- Define how repository and executable identity are rechecked or retained at
  the actual Cargo clippy invocation boundary.
- Decide whether the current fixed timeout and byte bounds remain exact
  constants in the first composed operation; do not make them caller-tunable
  by default.
- Keep additional operating systems and provider-based containment in
  separate reviewed phases.

## 11. Recommended Next Phase

Plan one private same-call composition with the existing Cargo clippy
local-check boundary. The plan must bind the observer request and result to the
exact immutable run bundle, operation contract, security requirements,
selected executables, repository root, sanitized environment, admission
decision, invocation result, and terminal report evidence.

The planning phase must not implement admission, Cargo execution composition,
OpenShell wiring, persistence, events, schemas, CLI behavior, provider writes,
or broader authority.

## 12. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `cargo test -p workflow-local-host --lib`: passed; 18 passed and one ignored.
- Opt-in Apple Silicon smoke: passed; one live smoke passed.
- `npm run check:docs`: passed before review edits.
- `git diff --check`: passed before review edits.

## 13. Governed Review Evidence

- Workflow ID: `dg/review`.
- Run ID: `run-1792845405393046000-2`.
- Approval ID:
  `approval/run-1792845405393046000-2/review-scope-approved`.
- Approval presentation ID: `presentation/100a6ebc2a1fe5f8`.
- Approval presentation content hash:
  `100a6ebc2a1fe5f8eed092fce0bee77e6ffb879a9d69c57db0e4d7267ed7cafd`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact focused review scope.
- Phase status: `Completed`.
- Event summary: 39 ordered events, including one approval request, one
  approval grant, six scheduled steps, six successful skill invocations, zero
  retries, and zero escalations. Approval presentation enforcement was
  proof-enforced and its event marker was present.
- Validation summary: workspace formatting, clippy, tests, focused
  process-free tests, the opt-in live smoke, documentation validation, and
  diff checks passed.
- Out-of-kernel work: Codex inspected the implementation, tests, dependency
  changes, and phase docs; performed the security review; authored this
  review; and ran validation. The kernel governed scope and approval but did
  not inspect files, execute tests, edit documentation, or make the review
  decision.
