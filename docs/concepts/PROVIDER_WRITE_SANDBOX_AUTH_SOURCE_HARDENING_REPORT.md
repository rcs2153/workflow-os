# Provider Write Sandbox Auth/Source Hardening Report

## 1. Executive Summary

Implemented the focused provider write sandbox auth/source hardening pass.

The hardening keeps provider-write behavior explicit, local, injected, and
non-default. It strengthens the pre-transport auth boundary for GitHub pull
request comment provider paths and adds focused sandbox readiness regression
coverage for hidden or unknown auth posture.

This phase does not implement provider writes as default behavior, live sandbox
mutation from the executor, hidden auth loading, CLI mutation commands, workflow
schemas, examples, hosted behavior, broader adapters, reasoning lineage, or
release posture changes.

## 2. Scope Completed

- Updated the concrete GitHub pull request comment HTTP provider to compare the
  full validated `GitHubPullRequestCommentProviderAuth` wrapper before
  transport.
- Updated the concrete GitHub pull request comment lookup HTTP client to compare
  the full validated `GitHubPullRequestCommentProviderAuth` wrapper before
  transport.
- Added regression tests proving same-secret/different-scope auth is rejected
  before provider transport for provider-call and lookup paths.
- Added readiness tests proving hidden/ambient and unknown auth posture are
  denied as not explicit.
- Updated roadmap/planning documentation to point at this hardening report.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- provider writes by default;
- live sandbox mutation from executor paths;
- hidden auth loading from environment, keychain, GitHub CLI, git credentials,
  browser sessions, OAuth state, secret managers, or config files;
- CLI mutation commands;
- workflow schema fields;
- examples;
- hosted or distributed runtime behavior;
- broad GitHub writes;
- Jira, CI, filesystem, HTTP, or arbitrary provider writes;
- automatic retries, repair, or recovery mutation;
- reasoning lineage;
- release posture changes.

## 4. Implementation Approach

The previous concrete provider clients compared only the bearer-secret value
between a validated request and the injected concrete provider/client. That
protected transport from mismatched tokens, but it left the bounded
`scope_summary` metadata outside the equality check.

The hardening now compares the full auth wrapper:

- provider-call request auth must equal concrete provider auth;
- lookup request auth must equal concrete lookup client auth;
- mismatch uses the existing stable non-leaking auth mismatch error codes;
- rejection happens before transport is called.

The auth wrapper remains intentionally non-serializable and redaction-safe in
Debug output.

## 5. Validation Boundary Summary

The strengthened boundary means a caller cannot pass one validated request auth
scope and another injected provider auth scope while reusing the same secret.

Sandbox readiness still remains a pure decision helper. It does not carry auth
material or provider handles, and it never authorizes provider calls, workflow
event appends, side-effect record writes, or report artifact writes.

Hidden/ambient and unknown auth posture now have explicit regression coverage
and are denied with `ProviderWriteSandboxReadinessIssue::AuthNotExplicit`.

## 6. Redaction And Privacy Summary

The hardening preserves the existing privacy posture:

- auth secrets are not serialized;
- auth scope summaries are not copied into errors;
- Debug output redacts auth material;
- mismatch errors use stable codes and do not include raw tokens, scope
  summaries, provider payloads, target strings, paths, command output, or secret
  markers;
- readiness output remains bounded and redaction-safe.

## 7. Test Coverage Summary

Added focused tests for:

- hidden/ambient sandbox auth posture denied;
- unknown sandbox auth posture denied;
- lookup HTTP client rejects same-secret/different-scope auth before transport;
- provider HTTP client rejects same-secret/different-scope auth before
  transport;
- mismatch errors do not leak auth secrets or scope summaries.

Existing focused auth tests continue to cover missing auth, missing explicit
auth posture, secret mismatch before transport, and provider-call request debug
redaction.

## 8. Commands Run And Results

Validation commands:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p workflow-core --test provider_write auth
cargo test --workspace
npm run check:docs
git diff --check
```

Result: passed.

Focused provider-write auth run: 11 tests passed.

Full Rust workspace run: passed.

## 9. Remaining Known Limitations

- No live sandbox provider write is implemented.
- No hidden auth-source model is implemented.
- No CLI mutation surface is implemented.
- No workflow-declared provider-write schema exists.
- No production credential management exists.
- No broad write-capable adapter surface exists.
- This phase hardens auth/source matching and readiness posture only; it does
  not prove a disposable live sandbox write.

## 10. Recommended Next Phase

Recommended next phase: provider write sandbox auth/source hardening review.

That review should verify the full-wrapper auth match, hidden/unknown auth
readiness denial, non-leaking error posture, documentation honesty, and scope
cleanliness before any live sandbox validation planning begins.

## 11. Dogfood Governance

- workflow: `dg/implement`
- run ID: `run-1783754384966568000-2`
- approval ID: `approval/run-1783754384966568000-2/implementation-approved`
- presentation ID: `presentation/4b9fe401a334883d`
- approval outcome: granted by delegated maintainer
- event summary: completed run with 39 events, 1 approval, 0 retries, and 0
  escalations
- approval-presentation proof: enforced, with proof marker present on the
  approval event
- validation summary: focused provider-write auth tests, full Rust workspace
  tests, docs check, format check, clippy, and whitespace check passed

Out-of-kernel work disclosed:

- Rust code edits;
- focused provider-write tests;
- documentation and roadmap updates;
- validation commands;
- no provider calls;
- no hidden auth loading;
- no report artifacts;
- no runtime writes performed by the kernel.
