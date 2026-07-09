# GitHub PR Comment Provider Lookup HTTP Client Review

## 1. Executive Verdict

Phase accepted; proceed to lookup integration planning.

The implementation satisfies the approved injected-transport-only scope for the first concrete GitHub PR comment provider lookup HTTP client. It adds bounded provider-side observation plumbing without automatic lookup, hidden auth loading, provider writes, retries, event append, state repair, report artifact writes, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, approval-presentation enforcement, or release posture changes.

## 2. Scope Verification

The phase stayed within the approved scope.

Implemented:

- explicit `GitHubPullRequestCommentLookupHttpClient`;
- bounded lookup HTTP request, response, and injected transport types;
- implementation of `GitHubPullRequestCommentProviderLookupClient`;
- caller-supplied auth validation;
- conservative HTTP status classification;
- reuse of existing lookup reconciliation;
- focused fake-transport tests;
- roadmap, plan, and phase report updates.

Not introduced:

- automatic provider lookup;
- hidden auth loading;
- GitHub provider writes;
- retries;
- workflow event append;
- state repair;
- report artifact writes;
- CLI lookup or recovery command;
- schemas or examples;
- hosted/distributed runtime behavior;
- reasoning lineage;
- approval-presentation enforcement;
- release posture changes.

## 3. API And Boundary Assessment

The API is appropriately narrow and explicit.

`GitHubPullRequestCommentLookupHttpClient<T>` accepts an injected transport, explicit API base URL, explicit `GitHubPullRequestCommentProviderAuth`, sensitivity, and redaction metadata. It does not inspect environment variables, git remotes, GitHub CLI state, keychain state, config files, OAuth state, or hosted credentials.

The request type exposes the HTTP method, constructed URL, authorization header for transport use, idempotency key, expected provider reference, and expected managed marker. Its `Debug` implementation redacts URL, auth, idempotency key, and caller-supplied lookup identifiers.

The response type is bounded to HTTP status and validated lookup observations. Raw GitHub response bodies, comment bodies, pull request bodies, diffs, review-thread payloads, source files, CI logs, command output, and provider payloads are not part of the model.

## 4. HTTP Classification Assessment

The status mapping is conservative and reviewable:

- `2xx` maps to `Completed`;
- `401` maps to `NotAuthorized` with `github.auth_failed`;
- `403` maps to `NotAuthorized` with `github.forbidden`;
- `404` maps to `Unavailable` with `github.not_found`;
- `408` maps to `Unavailable` with `github.timeout`;
- `429` maps to `RateLimited` with `github.rate_limited`;
- `5xx` maps to `Unavailable` with `github.server_error`;
- other statuses map to `ResponseUntrusted` with `github.response_untrusted`.

Treating `404` as unavailable is the right v1 posture because the comments endpoint alone does not prove whether the target is missing or merely inaccessible/ambiguous.

## 5. Auth And Privacy Assessment

The auth boundary is acceptable.

The client is constructed with explicit caller-supplied auth and rejects lookup requests whose validated auth does not match the client auth before the transport is invoked. Transport failures are remapped to a stable non-leaking error code instead of carrying raw provider failure text through the boundary.

Debug output redacts:

- API base URL;
- authorization header;
- idempotency key;
- expected provider reference;
- expected managed marker;
- transport;
- redaction metadata.

The implementation does not serialize provider auth or introduce hidden credential discovery.

## 6. Reconciliation Assessment

The client correctly remains an observation source only. It returns `GitHubPullRequestCommentProviderLookupResponse` values that the existing lookup reconciliation helper can classify.

It does not create `EvidenceReference` values, does not create report citations, does not append workflow events, and does not mutate side-effect records. Durable workflow event proof remains distinct from provider-side observation, preserving the strict report artifact event-proof boundary.

## 7. Non-Mutation Assessment

The implementation does not mutate workflow state, side-effect stores, report artifact stores, local files, or provider resources.

The explicit helper methods report:

- `workflow_event_append_allowed() == false`;
- `report_artifact_write_allowed() == false`;
- `side_effect_record_write_allowed() == false`.

Focused tests verify the lookup path leaves an attempted side-effect record in its original lifecycle state.

## 8. Test Quality Assessment

The added tests are focused and meaningful.

Covered:

- successful request construction and bounded observations;
- reuse of the existing reconciliation helper;
- status classification for auth failure, forbidden, not found, timeout, rate limit, server error, and untrusted responses;
- transport failure remapping without leaking raw failure text;
- auth mismatch rejection before transport invocation;
- Debug non-leakage for client and request;
- no state/event/artifact/CLI side effects.

Remaining test gaps are non-blocking:

- no live GitHub transport smoke test, which is correctly deferred;
- no pagination behavior beyond the single bounded page URL, which matches current scope;
- no executor integration test, because executor lookup integration is not implemented.

## 9. Documentation Review

The plan, report, and roadmap correctly state that the first explicit injected-transport lookup HTTP client is implemented.

The docs also preserve the key exclusions:

- automatic provider lookup is not implemented;
- hidden auth loading is not implemented;
- provider writes are not implemented by this phase;
- retries are not implemented;
- workflow event append from lookup/recovery is not implemented;
- state repair is not implemented;
- report artifact writes are not implemented;
- CLI behavior is not implemented;
- schemas and examples are not implemented;
- hosted behavior is not implemented;
- reasoning lineage is not implemented;
- approval-presentation enforcement is not implemented;
- release posture changes are not implemented.

## 10. Validation Assessment

The implementation report records:

- `cargo test -p workflow-core --test provider_write` - passed;
- `cargo fmt --all --check` - passed after formatting;
- `cargo clippy --workspace --all-targets -- -D warnings` - passed;
- `cargo test --workspace` - passed;
- `npm run check:docs` - passed.

This review reran the required validation commands:

- `cargo fmt --all --check` - passed;
- `cargo clippy --workspace --all-targets -- -D warnings` - passed;
- `cargo test --workspace` - passed;
- `npm run check:docs` - passed.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Plan the next lookup integration boundary before wiring the concrete client into executor, repair, or artifact paths.
- Keep live transport/smoke testing opt-in and separate from default CI.
- Preserve event-proof gates: provider lookup observation alone must not authorize report artifact writes.

## 13. Recommended Next Phase

Recommended next phase: lookup integration planning.

The concrete lookup HTTP client is accepted as a bounded observation source. The next work should decide where, if anywhere, this client is composed into recovery, executor, or operator workflows while preserving explicit auth, opt-in behavior, non-mutation by default, and strict event-proof gates.

## 14. Governed Dogfood Run

- workflow_id: `dg/review`
- run_id: `run-1783565992512102000-2`
- approval_id: `approval/run-1783565992512102000-2/review-scope-approved`
- approval outcome: granted by delegated maintainer
- approval reason: `approved-provider-lookup-http-client-review-scope`

The governed run completed before this review was written.
