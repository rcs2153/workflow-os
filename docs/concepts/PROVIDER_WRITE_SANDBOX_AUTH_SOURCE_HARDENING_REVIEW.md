# Provider Write Sandbox Auth/Source Hardening Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The hardening fixes the main auth/source ambiguity identified by the plan
review: concrete provider and lookup clients now compare the full validated
`GitHubPullRequestCommentProviderAuth` wrapper before transport rather than
comparing only the bearer-secret value. Hidden/ambient and unknown sandbox auth
postures also have explicit deny-path coverage.

The phase remains narrow and does not authorize live provider mutation. The next
phase may plan a single disposable live sandbox validation path, but it should
remain explicit, local, injected, caller-supplied, and non-default.

## 2. Scope Verification

The phase stayed within approved hardening scope.

It did not introduce:

- provider writes by default;
- live sandbox mutation from executor paths;
- hidden auth loading;
- CLI mutation commands;
- workflow schema fields;
- examples;
- hosted or distributed runtime behavior;
- broad GitHub writes;
- Jira, CI, filesystem, HTTP, or arbitrary provider writes;
- automatic retries, repair, or recovery mutation;
- reasoning lineage;
- release posture changes.

## 3. Auth Matching Assessment

The implementation now compares the full validated auth wrapper in both
transport-adjacent paths:

- `GitHubPullRequestCommentHttpProvider::create_pull_request_comment`;
- `GitHubPullRequestCommentLookupHttpClient::lookup_pull_request_comment`.

This is the right conservative choice. It means a caller cannot reuse the same
secret while changing bounded scope metadata between the validated request and
the injected provider/client boundary.

The existing stable mismatch error codes are preserved:

- `github_pr_comment_provider_http.auth.mismatch`;
- `github_pr_comment_provider_lookup_http.auth.mismatch`.

The errors remain non-leaking and transport is not called on mismatch.

## 4. Sandbox Readiness Assessment

The sandbox readiness helper already denied any auth posture other than
`ExplicitCallerSupplied`. The new tests make that behavior explicit for:

- `HiddenOrAmbient`;
- `Unknown`.

Both postures are denied with
`ProviderWriteSandboxReadinessIssue::AuthNotExplicit`.

This aligns with the product contract: core helpers must not discover or infer
credentials from ambient state.

## 5. Privacy And Redaction Assessment

The privacy posture remains sound:

- auth secrets are not serialized;
- auth scope summaries are not copied into mismatch errors;
- provider auth Debug output redacts secret and scope metadata;
- readiness Debug output redacts redaction metadata;
- mismatch tests assert that secret-like auth and scope summary values do not
  appear in errors;
- no raw provider payloads, target strings, command output, paths, tokens, or
  secret markers are copied into new output surfaces.

## 6. Test Quality Assessment

The phase adds focused tests for the reviewed gap:

- same-secret/different-scope lookup auth mismatch rejects before transport;
- same-secret/different-scope provider-call auth mismatch rejects before
  transport;
- hidden/ambient auth posture is denied;
- unknown auth posture is denied;
- errors do not leak auth secrets or scope summaries.

The full provider-write suite passed with 134 tests, and the full Rust workspace
test suite passed.

No test blockers found.

## 7. Documentation Review

The new hardening report clearly states:

- full auth-wrapper matching is implemented;
- hidden/ambient and unknown auth posture are denied;
- provider writes by default are not implemented;
- live sandbox mutation is not implemented;
- hidden auth loading is not implemented;
- CLI mutation commands are not implemented;
- schemas, examples, hosted behavior, broad adapters, reasoning lineage, and
  release posture changes are not implemented.

The roadmap and planning document link to the hardening report. Documentation
does not overclaim current provider-write capability.

## 8. Blockers

No blockers.

## 9. Non-Blocking Follow-Ups

- In the next live sandbox validation planning phase, define the exact
  disposable sandbox target proof for GitHub pull request comments.
- Keep hidden auth loading explicitly out of scope until a separate typed
  auth-source model is planned and reviewed.
- Consider adding a small current-product docs note when live sandbox validation
  planning begins, so preview users do not confuse sandbox validation with
  production write support.

## 10. Recommended Next Phase

Recommended next phase: single disposable live sandbox validation planning.

Why: the auth/source ambiguity is fixed, hidden/unknown auth posture is covered,
and the provider-write stack now has enough explicit local boundaries to plan a
single live sandbox validation without making writes default, automatic,
CLI-facing, schema-declared, or production-ready.

## 11. Validation

Validation for the implementation phase:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p workflow-core --test provider_write auth
cargo test --workspace
npm run check:docs
git diff --check
```

Result: passed.

Validation for this review:

```sh
npm run check:docs
git diff --check
```

Result: passed.

## 12. Dogfood Governance

- workflow: `dg/review`
- run ID: `run-1783756551800031000-2`
- approval ID: `approval/run-1783756551800031000-2/review-scope-approved`
- presentation ID: `presentation/a394efcc1a465839`
- approval outcome: granted by delegated maintainer
- event summary: completed run with 39 events, 1 approval, 0 retries, and 0
  escalations
- approval-presentation proof: enforced, with proof marker present on the
  approval event
- validation summary: docs check and whitespace check passed

Out-of-kernel work disclosed:

- documentation review file creation;
- docs and whitespace validation;
- no code changes in the review phase;
- no provider calls;
- no hidden auth loading;
- no report artifacts;
- no runtime writes performed by the kernel.
