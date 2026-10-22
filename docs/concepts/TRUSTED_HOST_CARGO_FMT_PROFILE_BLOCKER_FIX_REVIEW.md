# Trusted-Host Cargo Format Check Profile Blocker Fix Review

## 1. Executive Verdict

**Blocker fix accepted; proceed to trusted-host Cargo format workflow
composition planning.**

The merged fix closes all three blockers from the implementation review. The
resolved identity commits the complete supported toolchain input inventory,
the protected repository commitment commits the exact Core-owned exclusion
policy, and the Cargo-specific regression matrix now covers the required Core
and foreground failure and privacy boundaries.

This verdict accepts only the unpublished one-step profile. It does not add the
profile to the accepted two-step workflow or authorize clippy, tests, arbitrary
commands, providers, writes, public surfaces, schemas, hosted behavior, nested
harnesses, or release changes.

## 2. Scope Verification

The fix stayed within the approved blocker scope. It changed the private Cargo
format handler identity, protected-tree commitment, focused Core tests,
foreground-admission tests, and phase documentation. It did not add a command
profile, broaden topology, expose a public CLI or schema, invoke a provider, or
enable an external write.

## 3. Toolchain Inventory Assessment

The toolchain input inventory is now explicit and closed. It commits:

- the inventory policy version;
- the canonical toolchain root;
- logical selectors for Cargo, cargo-fmt, and rustfmt;
- each executable's root-relative path; and
- each executable's content hash.

Core computes the inventory at resolution, verifies it before process entry,
and verifies it again after process return. The exact final executables plus
their canonical root and selectors are the complete supported inventory for
this direct invocation. Ambient lookup, symbolic-link proxies, additional
toolchain inputs, installation, and updates remain unsupported rather than
uncommitted authority.

Repository/toolchain overlap is rejected in both directions. The accepted
disjoint-root Core and foreground paths remain covered.

## 4. Protected Inventory Policy Assessment

The protected repository commitment now includes a separate hash over the
exact Core-owned exclusion policy. The policy contains only `.git`,
`.workflow-os`, `dogfood/.workflow-os`, and `target`; callers have no API for
adding or replacing exclusions.

Changing the exclusion vocabulary changes the protected commitment and,
through the composite execution identity, changes the resolved execution
identity even when the excluded paths are absent. This closes the reviewed
identity gap rather than merely reasserting the resulting tree hash.

## 5. Failure And Integrity Assessment

Non-zero exit and timeout produce honest closed result statuses. A missing
component fails before process entry without installation. Mutation of a bound
toolchain executable during execution fails before result acceptance. Source
and toolchain integrity errors retain stable non-leaking codes.

The foreground failure path terminalizes the workflow as failed, records one
bounded local-check result, persists one failed WorkReport artifact, and does
not emit successful invocation or run-completed truth. Existing exact terminal
replay behavior remains covered by the accepted foreground suite.

## 6. Privacy Assessment

Handler Debug output redacts executable paths, repository and toolchain roots,
toolchain and execution commitments, protected-state commitments, exclusion-
policy identity, and runner details. Errors do not echo changed paths or
toolchain material.

The new foreground regression checks stdout, stderr, serialized durable events,
and the stored artifact payload together. Neither the raw formatter-output
marker nor the fixture root appears on those surfaces. Reports continue to
cite bounded local-check references rather than process transcripts.

## 7. Test Quality Assessment

The reviewed additions cover:

- deterministic non-zero and timeout result mapping;
- missing component with zero process invocation and no installation;
- persistent toolchain-input mutation;
- repository/toolchain overlap in both directions;
- exact Core-owned exclusion behavior with no caller override;
- successful disjoint-root execution;
- foreground failed-run, report-artifact, and citation behavior; and
- Debug, error, serialization, event, report, artifact, path, and raw-output
  non-leakage.

The opt-in real smoke also passes against the bundled stable Cargo,
cargo-fmt, and rustfmt binaries under one canonical toolchain root.

## 8. Documentation Assessment

The plan, implementation report, original implementation review, blocker-fix
report, and roadmap preserve the unpublished trusted-host boundary. They state
that final-state commitments do not prevent writes, transient restored writes
are not detectable, disabled-network posture is not operating-system egress
isolation, and executable identity is self-committed rather than third-party
attested.

## 9. Remaining Limitations

- There is no operating-system process, filesystem, or network sandbox.
- Persistent final-state comparison cannot detect a transient restored write.
- The executable inventory is not independently attested.
- The pre-provisioned toolchain must run without declared writes.
- The profile remains unpublished and is not part of the accepted two-step
  workflow.

These are explicit trusted-host limitations, not blockers for accepting this
bounded profile.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Preserve the distinction between contract-level disabled network posture and
  operating-system egress enforcement.
- Keep process containment in the separately governed sandbox/provider lane.
- Require a separate plan and review before composing the profile into a
  multi-step workflow.
- Keep clippy, tests, builds, arbitrary Cargo commands, and source-writing
  format mode deferred.

## 12. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test -p workflow-core --test local_check`: passed, 82 active tests;
  two opt-in live smokes skipped by default.
- opt-in real Cargo format smoke against the bundled stable toolchain: passed.
- `cargo test -p workflow-local-host --test foreground_admission`: passed,
  29 tests.
- `cargo test --workspace`: passed.
- `npm run check`: passed.
- `npm run check:integrations`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

The full validation matrix ran against the exact merged blocker-fix revision
before review documentation was written. Documentation-only review edits are
validated separately at phase close.

## 13. Governed Review Record

- Workflow ID: `dg/review`
- Run ID: `run-1792678677326839000-2`
- Approval ID:
  `approval/run-1792678677326839000-2/review-scope-approved`
- Presentation ID: `presentation/5c9ba23598e6a0b3`
- Approval outcome: granted under delegated-maintainer authority for the exact
  persisted review-only scope
- Phase status: `Completed`
- Event summary: 39 events, one approval, zero retries, and zero escalations
- Approval presentation enforcement: proof enforced with one persisted
  presentation record and the approval-event proof marker present
- Out-of-kernel work: code, test, report, and runtime inspection; review
  writing and validation were performed by the delegated maintainer. The
  kernel governed scope and approval but did not perform those actions.

## 14. Recommended Next Phase

Plan the smallest trusted-host workflow composition that adds the accepted
Cargo format profile to the existing ordered no-write proof without changing
the profiles themselves. The plan must preserve exact step order, immutable
bindings, per-step result truth, terminal report/artifact integrity, and exact
replay. It must not add clippy, tests, generic command execution, providers,
external writes, public surfaces, hosted behavior, or release changes.
