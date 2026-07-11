# Provider Write Sandbox Auth/Source Plan Review

## 1. Executive Verdict

Plan accepted with non-blocking follow-ups.

The provider write sandbox auth/source plan is the right review gate between the
current explicit write-adjacent helper stack and any future live sandbox
provider mutation. It consolidates caller-supplied auth, no-hidden-auth posture,
sandbox target proof, authority separation, current-product contract honesty,
and review-before-live-write sequencing without authorizing provider writes.

The next phase should be a focused hardening pass, not live mutation: add or
confirm tests/docs around full auth-wrapper matching, hidden-auth non-discovery,
and sandbox target proof denial/defer behavior.

## 2. Scope Verification

The plan stayed within planning scope.

It did not authorize:

- provider writes;
- live sandbox mutation;
- hidden auth loading;
- automatic executor writes;
- automatic report generation or artifact writes;
- automatic retries, repair, or recovery mutation;
- CLI mutation commands;
- workflow schema fields;
- examples;
- hosted or distributed runtime behavior;
- broader GitHub writes;
- Jira, CI, filesystem, HTTP, or arbitrary provider writes;
- recursive agents, agent swarms, or Level 3/4 autonomy;
- release posture changes.

## 3. Auth Source Assessment

The plan correctly keeps auth caller supplied and non-ambient.

Accepted rules:

- core helpers must not load auth from environment variables, shell profiles,
  keychains, GitHub CLI state, git credentials, git remotes, repo config, OAuth
  state, browser sessions, or secret managers;
- auth material must not be serialized, debug-formatted, copied into errors,
  workflow events, SideEffect records, WorkReports, report artifacts, or CLI
  output;
- auth scope summaries may be represented only as bounded non-secret metadata;
- missing or mismatched auth fails before provider transport.

This matches the existing concrete provider-client review and the current
product contract. A future auth-source model should be separate and reviewed
before any CLI mutation surface exists.

## 4. Authority Assessment

The plan correctly separates credential possession from write authority.

Future sandbox writes must still require supported capability, policy allowance,
SideEffect attempted posture, approval linkage where required,
approval-presentation proof where required, high-assurance posture where
configured, and event-proof/report-artifact posture where configured.

That distinction is essential: a token only enables transport. It must not
replace policy, approval, SideEffect, event proof, or report obligations.

## 5. Sandbox Target Proof Assessment

The proposed sandbox target proof is bounded and appropriate.

The plan requires repository owner/name, pull request number, sandbox/test
classification, non-production evidence, capability, intended actor,
correlation/idempotency binding, sensitivity, and redaction metadata.

It also forbids raw PR body text, raw issue comments, repository file contents,
tokens, command output, CI logs, and browser/session state. Production-looking,
unknown, or ambiguous targets remain denied or deferred.

This is enough for a first review/hardening pass. It is not enough yet to run a
live sandbox write.

## 6. Current Product Contract Assessment

The plan properly absorbs recent external feedback.

Workflow OS should continue to present itself as a credible local governance
kernel, not a production automation runtime. Provider-write work must not
overclaim the current product state.

The plan keeps these statements intact:

- default behavior is local governance, validation, approval, event state,
  first-run posture, report/artifact helper paths, and explicit write-adjacent
  helpers;
- automatic provider writes are not implemented;
- CLI mutation commands are not implemented;
- hidden auth loading is not implemented;
- broader write-capable adapters are not implemented;
- hosted/distributed runtime and production credential management are not
  implemented.

## 7. Relationship To Existing Write-Adjacent Work

The plan is consistent with the existing accepted work:

- provider client/auth loading implementation and review;
- provider write reconciliation model/helper;
- executor-integrated live provider write helper and blocker fix;
- provider-write runtime composition helper;
- provider-write sandbox readiness helper;
- artifact-gated provider-write composition helper.

It does not duplicate those primitives. It creates a necessary decision point:
verify auth/source and sandbox posture before any live sandbox mutation plan.

## 8. Test And Hardening Assessment

The plan identifies the right future tests:

- explicit caller-supplied auth remains the only accepted posture;
- hidden/ambient auth posture is denied;
- auth mismatch fails before transport;
- production-like and unknown targets are denied or deferred;
- sandbox target proof does not serialize raw provider or PR content;
- readiness errors remain non-leaking;
- default executor paths still do not call providers;
- CLI mutation commands remain absent.

One test/documentation gap should be handled before live sandbox planning:
decide whether provider auth matching should compare the full auth wrapper,
including bounded scope summary, or keep the current narrower secret-bearing
credential match and document why.

## 9. Blockers

No planning blockers.

Do not proceed to live sandbox mutation until the auth/source hardening pass is
complete.

## 10. Non-Blocking Follow-Ups

- Decide and test full auth-wrapper equality versus narrower credential
  matching.
- Add explicit hidden-auth non-discovery coverage if a stable test path exists.
- Add focused sandbox target proof denied/deferred coverage if not already
  covered by the readiness helper.
- Keep current-product docs aligned whenever live sandbox validation planning
  begins.

## 11. Recommended Next Phase

Recommended next phase: provider write sandbox auth/source hardening.

That phase should be code/test/docs hardening only. It should not perform live
provider mutation, load hidden auth, expose CLI mutation behavior, add schemas,
add examples, broaden provider writes, or change release posture.

After that hardening is reviewed, the project can plan a single disposable live
sandbox validation path for GitHub PR comments.

## 12. Validation

Validation commands for this review:

```sh
npm run check:docs
git diff --check
```

Result: passed.

Dogfood review run:

- workflow: `dg/review`
- run ID: `run-1783753517538489000-2`
- approval ID: `approval/run-1783753517538489000-2/review-scope-approved`
- presentation ID: `presentation/8a866b52554c282d`
- approval outcome: granted by delegated maintainer
