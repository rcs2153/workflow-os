# Local Project Authority Source Runtime Composition Report

## 1. Executive Summary

Workflow OS now derives workload authority for project-declared authoritative
execution from the validated declaration captured in the immutable run bundle.
The CLI no longer preclassifies authority for that path.

Fresh execution and approval reassessment both verify the immutable activation
before Core supplies `Sufficient` authority to the existing
proportional-governance route. Caller-preclassified authority fails closed.

The standalone CLI flag remains a compatibility path when the project does not
declare authoritative execution.

## 2. Scope Completed

- Added a private Core binding from immutable project activation to workload
  authority.
- Removed caller-preclassified authority from the project-declared CLI path.
- Verified the exact supported activation, profile, and local-check profile.
- Reused the binding for fresh execution and approval reassessment.
- Rejected caller-preclassified authority before local-check execution or
  workflow event creation.
- Preserved existing report, artifact, disclosure, denial, approval, and
  execution behavior.

## 3. Scope Explicitly Not Completed

The phase did not add:

- actor-specific authority, RBAC, or enterprise stewardship;
- ambient or inferred authority;
- automatic approvals;
- a public generic authority source;
- removal of the standalone compatibility flag;
- OpenShell, sandbox execution, credentials, providers, SideEffects, or writes;
- schemas, additional local-check profiles, hosted behavior, or release
  changes.

## 4. Runtime Boundary

The source is the validated closed project declaration:

- `observe_and_report`; and
- `workflow_os_project_validation`.

Core confirms that the immutable run bundle carries the same activation and
that the request uses the matching profile. Only then does Core clone the
runtime facts and insert sufficient authority for immediate consumption.

Approval resume rereads the immutable bundle and performs the same binding
before comparing the reassessed governance result with the durable assessment.

## 5. Failure And Privacy Posture

Stable failures cover missing activation, activation mismatch, and
caller-preclassified authority. Existing immutable-run and governance
reassessment failures remain unchanged.

The new failures contain no governed identifiers, paths, commands, raw
configuration, provider payloads, credentials, or secret-like values.

## 6. Test Coverage

Focused tests prove:

- project-declared authority is derived from the immutable activation;
- the exact activation remains inspectable in the stored bundle;
- caller-preclassified authority fails before checks, skills, or events; and
- approval reassessment rebinds authority and completes with an unchanged
  durable event trail.

Existing CLI tests cover project-declared execution and the standalone
compatibility flag.

## 7. Validation

- `cargo fmt --all --check`: passed;
- `cargo check -p workflow-core --lib`: passed;
- `cargo check -p workflow-cli --bin workflow-os`: passed;
- `cargo clippy -p workflow-core --lib -- -D warnings`: passed;
- `cargo clippy -p workflow-cli --bin workflow-os -- -D warnings`: passed;
- `npm run check:docs`: passed;
- `git diff --check`: passed;
- the Rust test target compiled successfully;
- the focused local-executor test binary encountered the known macOS
  post-launch stall and was stopped without claiming a pass;
- `cargo check --workspace --all-targets` and
  `cargo clippy --workspace --all-targets -- -D warnings` checked Core, Hosted,
  and CLI without diagnostics but encountered the same local test-target stall
  and were stopped without claiming a pass; and
- GitHub CI remains the authoritative complete workspace-test gate.

## 8. Remaining Limitations

- The standalone flag remains caller-classified compatibility behavior.
- The local project declaration is a workload-level source, not actor-specific
  delegated authority.
- Only the closed project-validation profile is supported.
- No external identity, revocation service, or enterprise administrator source
  exists.
- OpenShell may later provide execution containment, but it is not an authority
  source.

## 9. Recommended Next Phase

Perform focused maintainer review. If accepted, decide the standalone flag
retirement posture, then continue scoped capability projection and authority
receipt work.

## 10. Governed Phase Record

- workflow: `dg/runtime-composition`;
- run ID: `run-1785412215467666000-2`;
- approval ID:
  `approval/run-1785412215467666000-2/composition-approved`;
- approval presentation ID: `presentation/6a50b9e8b9229921`;
- approval presentation content hash:
  `6a50b9e8b9229921f00d6ac8f6daa5db38125c469ba4178a46c8529bb6331124`;
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted handoff was presented;
- phase status: completed;
- event summary: 39 events, 1 approval, 0 retries, 0 escalations;
- approval-presentation enforcement: the proof record matched the granted
  approval; inspect output does not yet expose a presentation proof-use marker;
- out-of-kernel work: the delegated maintainer edited code, tests, and
  documentation and ran validation; the kernel governed scope and approval but
  did not edit files, execute checks, mutate git, or invoke providers.
