# GitHub PR Comment Provider Lookup Reconciliation Model Report

## 1. Executive Summary

The first GitHub PR comment provider lookup/query reconciliation model/helper is implemented.

The implementation adds an explicit injected-client boundary for bounded provider-side observations. It can classify whether a matching remote comment was observed, absent, ambiguous, unauthorized, unavailable, rate-limited, or untrusted. Provider lookup remains observation only: it does not create durable workflow event proof, repair state, retry provider writes, mutate side-effect records, write report artifacts, expose CLI behavior, add schemas, add examples, or change release posture.

## 2. Scope Completed

- Added provider lookup response and observation model types.
- Added provider lookup reconciliation posture and next-action vocabulary.
- Added `GitHubPullRequestCommentProviderLookupClient` as an injected lookup boundary.
- Added `GitHubPullRequestCommentProviderLookupRequest` with explicit caller-supplied auth.
- Added `reconcile_github_pr_comment_provider_lookup`.
- Added deterministic matching for exact provider reference and bounded managed marker.
- Added bounded, redaction-safe lookup reconciliation result.
- Added focused tests for observed, absent, ambiguous, unauthorized, unavailable, rate-limited, untrusted, invalid input, client error, debug, serialization, and no-artifact-permission behavior.
- Updated roadmap and implementation-plan documentation.

## 3. Scope Explicitly Not Completed

- No automatic provider lookup.
- No hidden auth loading.
- No GitHub comment creation, update, or deletion.
- No automatic retries.
- No workflow event append.
- No workflow event repair.
- No side-effect record mutation.
- No manual state repair helper.
- No report artifact writes.
- No CLI behavior.
- No schema changes.
- No examples.
- No hosted or distributed runtime behavior.
- No broader write-capable adapter expansion.
- No reasoning lineage.
- No approval-presentation enforcement.
- No release posture changes.

## 4. Model And Helper Summary

Added model/helper surface:

- `GitHubPullRequestCommentProviderLookupOutcome`
- `GitHubPullRequestCommentProviderLookupObservationDefinition`
- `GitHubPullRequestCommentProviderLookupObservation`
- `GitHubPullRequestCommentProviderLookupResponseDefinition`
- `GitHubPullRequestCommentProviderLookupResponse`
- `GitHubPullRequestCommentProviderLookupReconciliationPosture`
- `GitHubPullRequestCommentProviderLookupReconciliationNextAction`
- `GitHubPullRequestCommentProviderLookupReconciliationInput`
- `GitHubPullRequestCommentProviderLookupRequest`
- `GitHubPullRequestCommentProviderLookupClient`
- `GitHubPullRequestCommentProviderLookupReconciliationResult`
- `reconcile_github_pr_comment_provider_lookup`

The helper accepts an attempted GitHub PR comment side-effect record, expected target, optional expected provider reference, optional bounded managed marker, caller-supplied auth, sensitivity, redaction metadata, and an injected lookup client.

## 5. Matching Boundary Summary

Allowed deterministic match signals:

- exact provider comment reference;
- exact pull request target identity;
- exact bounded Workflow OS managed marker.

Rejected or not implemented:

- raw comment body matching;
- fuzzy text matching;
- model interpretation of comments;
- repository path inference;
- unbounded natural-language summaries;
- matching on secrets, tokens, private payloads, raw provider payloads, raw parser payloads, raw command output, or raw spec contents.

If the helper cannot match deterministically, it returns ambiguous or untrusted posture.

## 6. Event-Proof And Artifact Boundary Summary

Provider lookup is not durable Workflow OS event proof.

The lookup result keeps:

- `workflow_event_appended=false`;
- `side_effect_record_mutated=false`;
- `report_artifact_written=false`;
- `cli_output_emitted=false`;
- `artifact_write_blocked=true`;
- `artifact_write_may_proceed=false`.

Even a successful remote observation does not unblock strict report artifact writes when durable workflow event proof is missing.

## 7. Redaction And Privacy Summary

The helper and model:

- do not store raw provider responses;
- do not store raw comment bodies;
- do not store PR bodies, diffs, review text, or file contents;
- do not store authorization headers or credentials;
- reject secret-like expected references, managed markers, provider references, and redaction metadata;
- use redaction-safe `Debug`;
- validate serialized/deserialized result shapes through constructors.

Errors use stable codes and avoid raw metadata or secret-like values.

## 8. Test Coverage Summary

Focused tests cover:

- remote comment observed by exact provider reference;
- remote comment observed by bounded managed marker;
- remote comment absent;
- ambiguous multiple matches;
- lookup not authorized;
- lookup unavailable;
- lookup rate limited;
- lookup response untrusted;
- invalid target rejected before lookup client invocation;
- secret-like lookup input rejection without leakage;
- unclassified injected-client error mapped without leakage;
- debug and serialization non-leakage;
- serde round trip and invalid wire failure;
- artifact writes remain blocked even after successful remote observation;
- no workflow event append, side-effect mutation, report artifact write, or CLI output.

Existing provider-write tests continue to pass.

## 9. Commands Run And Results

- `cargo fmt --all`: passed.
- `cargo test -p workflow-core --test provider_write`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `cargo fmt --all --check`: passed.
- `npm run check:docs`: passed.

## 10. Remaining Known Limitations

- No concrete lookup HTTP client exists.
- No automatic recovery flow exists.
- No manual state repair helper exists.
- No CLI recovery command exists.
- No workflow-declared recovery policy exists.
- Provider lookup cannot satisfy durable event-proof gates.
- Explicit missing/repair evidence attachment remains deferred.

## 11. Recommended Next Phase

Recommended next phase: GitHub PR comment provider lookup reconciliation model/helper review.

Why: this phase adds a safety-sensitive recovery-adjacent boundary. It should be reviewed before any concrete HTTP lookup client, CLI recovery surface, state repair helper, or artifact-write composition uses it.
