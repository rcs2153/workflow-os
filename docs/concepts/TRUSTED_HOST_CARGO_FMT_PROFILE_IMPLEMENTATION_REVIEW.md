# Trusted-Host Cargo Format Check Profile Implementation Review

## 1. Executive Verdict

**Needs blocker fixes.**

The implementation establishes the correct closed command, explicit handler,
empty inherited environment, protected repository final-state comparison,
one-step trusted-host lifecycle, report artifact, and terminal replay boundary.
It remains unpublished and does not broaden command or provider authority.

Acceptance is blocked because the resolved identity does not yet commit the
complete explicit toolchain-state and exclusion-policy boundary required by the
accepted plan. The focused Cargo-format tests also omit several mandatory
failure, timeout, toolchain-mutation, and privacy proofs. Those gaps matter at
this trusted-host boundary: the implementation must prove the exact material
and policy it treats as authoritative before the profile is composed into a
larger workflow.

## 2. Scope Verification

The phase stayed within the approved one-step profile scope. It added:

- one closed `CargoFmtCheck` profile and canonical command contract;
- one private explicit handler;
- explicit Cargo, cargo-fmt, rustfmt, toolchain-root, and repository-root
  inputs;
- one unpublished local-host route;
- focused Core and local-host tests; and
- implementation and roadmap documentation.

It did not add clippy, tests, arbitrary Cargo arguments, providers, external
writes, public CLI behavior, schemas, SDK behavior, onboarding, hosted parity,
parallelism, branching, loops, nested harnesses, or release changes.

## 3. Closed Command And Handler Assessment

The command boundary is appropriately narrow. `LocalCheckCommandContract`
fixes the executable name, `fmt --all --check` arguments, repository working
directory, explicit allowlist-only environment, 120-second timeout,
no-source-write side-effect class, bounded output, and bounded redaction
policy. The handler constructor requires complete equality with that canonical
contract and accepts no caller-authored command, extra argument, environment,
output path, network posture, or exclusion.

The local-host parser rejects mixed npm, Workflow OS, and Cargo profile
arguments and keeps the Cargo profile out of the accepted two-step topology.
That preserves the intended one-step unpublished proof.

## 4. Toolchain Identity Assessment

The implementation improves materially on Cargo-only hashing. Resolution
requires regular executable files named `cargo`, `cargo-fmt`, and `rustfmt`
inside a canonical explicit toolchain root. It hashes all three executable
contents, synthesizes `PATH` from the formatter directories, sets `RUSTFMT` to
the exact canonical rustfmt path, and rechecks those hashes before process
entry and after process return.

That is not yet the complete accepted boundary. The formatter-chain commitment
contains only the three executable content hashes. The distinct canonical
toolchain root, a selector or equivalent resolved-toolchain identity, and any
other explicit read-only toolchain inputs are not part of a before/after
toolchain-state commitment. A persistent change elsewhere under the supplied
toolchain root is therefore invisible. The current overlap check also rejects
only a toolchain root nested inside the repository; it does not reject a broad
toolchain root that contains the repository.

The blocker fix must define a bounded Core-owned toolchain input inventory. It
may prove that the three final backing executables plus canonical root/selector
are the complete required inventory, or commit additional required material,
but it may not leave the supplied toolchain-state root as uncommitted ambient
authority. The inventory and its policy must be checked before and after the
process, with stable non-leaking failure.

## 5. Repository Integrity Assessment

The protected repository commitment is deterministic and stronger than git
status. It sorts encoded path names, commits directory and file identities plus
file contents, includes ignored and untracked files, rejects symbolic links,
and detects addition, deletion, content change, and file-to-directory change.
The post-process commitment is evaluated before a process result can become
authoritative.

The exact exclusions are Core-owned and narrow: `.git`, `.workflow-os`,
`dogfood/.workflow-os`, and `target`. Callers cannot add exclusions. However,
the exclusion vocabulary itself is not committed into the protected-state or
resolved-execution fingerprint. The accepted plan explicitly requires that
policy to be part of the execution identity. Hashing only the resulting
included tree cannot distinguish two exclusion policies when the newly
excluded paths are absent or have the same omitted posture.

This mechanism proves persistent final-state equivalence only. It does not
prevent writes or detect a write restored before the post-process commitment.
The implementation documentation preserves that limitation.

## 6. Environment And Network Assessment

The standard process runner clears the inherited environment. The Cargo
profile supplies only synthesized `PATH` and exact `RUSTFMT`; it does not pass
provider credentials, proxy values, registry authority, compiler wrappers,
Rust flags, target overrides, or user-home configuration variables.

`LocalCheckNetworkPolicy::Disabled` is contract posture, not operating-system
egress enforcement. The trusted process receives no network credentials and
the selected Cargo-format operation does not require network access, but the
current host runner has no network sandbox. This is acceptable only as an
explicit trusted-host limitation and must not be represented as hard egress
denial.

## 7. Result And Failure Assessment

The handler validates the protected repository and executable chain after the
process returns and before `LocalCheckResult` construction. A zero exit cannot
become authoritative after a detected repository or executable mutation.
Only a passed result becomes skill success; failed and timed-out results follow
the terminal-failure path. Integrity errors do not create a successful result,
report, or artifact.

Errors at the new boundary use stable static codes and messages. The
trusted-host executor deliberately collapses internal handler errors into
terminal failure rather than persisting path, environment, source, output, or
toolchain details.

## 8. Runtime, Report, Artifact, And Replay Assessment

The one-step local-host proof reuses immutable run bundles, explicit-zero
required context, current authority, operational execution windows, atomic
dispatch admission, authoritative local-check result events, workflow
terminalization, terminal WorkReport composition, and private artifact
persistence.

The integration test proves one completed run, one result binding, one report
artifact with one local-check citation, and exact terminal replay with no
additional events. The report path cites the stable result reference and does
not persist formatter stdout or stderr.

## 9. Privacy And Redaction Assessment

The new handler's `Debug` implementation redacts executable paths, toolchain
and repository roots, commitments, and process-runner details. Focused errors
do not echo source paths or executable paths. Events and report artifacts use
bounded IDs and result posture rather than raw command output.

The model remains payload-free at the authority and result-event boundaries.
No credential, authorization header, provider payload, source content,
environment value, or raw formatter transcript is intentionally persisted.

## 10. Test Quality Assessment

Strong focused coverage proves:

- exact profile, skill, command, arguments, executable, and environment;
- no process execution during profile resolution;
- pre-entry executable-content substitution rejection;
- repository content mutation, addition, deletion, type change, and ignored
  or untracked addition rejection;
- protected symlink rejection;
- a real opt-in invocation against the bundled stable formatter chain;
- one-step result/report/artifact behavior; and
- terminal replay without event duplication.

The accepted test plan is not complete. There is no Cargo-specific injected
runner test for non-zero failure or timeout, no missing-component/no-install
proof, no mutation-during-execution test for the toolchain inventory, no
disjoint-root regression, no caller-selected exclusion rejection proof, and no
direct Cargo-profile serialization/report assertion proving paths and command
output remain absent. The local-host test uses shell fixtures that always exit
zero, so it does not cover the failure/report boundary.

## 11. Documentation Assessment

The roadmap, plan, and implementation report accurately keep the profile
unpublished and one-step only. They correctly disclaim process sandboxing and
transient-write detection. The implementation report has been corrected to
state that operating-system network egress is not enforced and to include the
completed docs validation command.

The report's claim of complete formatter-chain binding is supported for the
three named final executables, but it must not be read as commitment of all
supplied toolchain-state material until the blocker is fixed.

## 12. Blockers

1. Define and commit the complete bounded toolchain input inventory, including
   the canonical root/selector posture and every backing input required by the
   direct Cargo-to-rustfmt chain. Compare that inventory before and after
   execution, reject repository/toolchain root overlap in either direction,
   and fail closed without leaking paths or toolchain details.
2. Include the exact Core-owned protected-tree exclusion policy in the resolved
   execution identity, not only the tree produced after applying it.
3. Add the missing Cargo-specific regressions for deterministic failure,
   timeout, missing component/no-install behavior, persistent toolchain-input
   mutation, disjoint roots, exclusion-policy closure, and direct privacy of
   Debug, errors, events, reports, artifacts, and serialization.

## 13. Non-Blocking Follow-Ups

- Preserve the distinction between disabled-network contract posture and
  operating-system egress enforcement.
- Consider process-level filesystem and network containment only through the
  separately planned sandbox/provider lane.
- Keep the Cargo format profile out of the accepted two-step workflow until the
  blocker fix and its focused review pass.
- Keep clippy, tests, arbitrary Cargo commands, and source-writing format mode
  deferred.

## 14. Recommended Next Phase

Perform one narrow **trusted-host Cargo format profile blocker fix** covering
the three blockers above, then run a focused blocker-fix review.

Do not add another command profile, broader workflow topology, provider
access, external write, public surface, schema, hosted behavior, nested
harness, or release change.

## 15. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test -p workflow-core --test local_check`: passed, with 77
  active tests passing and two opt-in live smoke tests skipped by design.
- `cargo test -p workflow-local-host --test foreground_admission`: passed,
  with 28 tests passing.
- `cargo test --workspace`: passed.
- `npm run check`: passed.
- `npm run check:integrations`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

The implementation-phase opt-in live Cargo formatter smoke was reviewed as
existing evidence and was not rerun during this review. The focused review
instead reran the default deterministic suite and records the missing mandatory
Cargo-specific regressions as blockers rather than simulating them.

## 16. Governed Review Record

- Workflow ID: `dg/review`
- Run ID: `run-1792676165892982000-2`
- Approval ID:
  `approval/run-1792676165892982000-2/review-scope-approved`
- Presentation ID: `presentation/21601caa586a7293`
- Approval outcome: granted under delegated-maintainer authority for the exact
  persisted review-only scope
- Phase status: `Completed`
- Event summary: 39 events, one approval, zero retries, zero escalations, with
  one persisted presentation record and a present approval-event proof marker
- Out-of-kernel work: code, test, plan, report, and runtime inspection; review
  writing and validation were performed by the delegated maintainer. Git and
  PR operations remain outside the kernel and will be disclosed when complete.
  The kernel governed scope and approval but did not perform those actions.

## 17. Fix-Forward Note

The blockers identified by this review were addressed in the subsequent
narrow fix recorded in [Trusted-Host Cargo Format Check Profile Blocker Fix
Report](TRUSTED_HOST_CARGO_FMT_PROFILE_BLOCKER_FIX_REPORT.md). This note does
not erase or revise the original verdict. A separate focused blocker-fix
review remains required before acceptance or workflow composition.
That review is now recorded in [Trusted-Host Cargo Format Check Profile
Blocker Fix Review](TRUSTED_HOST_CARGO_FMT_PROFILE_BLOCKER_FIX_REVIEW.md).
