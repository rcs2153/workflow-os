# Provider Write Sandbox Auth/Source Plan

## 1. Executive Summary

Workflow OS now has a deeply staged GitHub pull request comment write lane:
preflight, SideEffect records, approval linkage, attempted/completed/failed
lifecycle transitions, injected provider calls, reconciliation posture, event
proof, report disclosure, artifact gates, sandbox readiness, and artifact-gated
composition all exist as explicit local helper paths.

That does not mean live sandbox writes should become product behavior yet. The
next question is whether the existing explicit caller-supplied auth and sandbox
target boundaries are clear enough to support a future live sandbox validation
phase without weakening the current local-kernel product contract.

This plan does not implement provider writes, live network mutation, hidden auth
loading, automatic executor writes, CLI mutation commands, workflow schemas,
examples, hosted behavior, reasoning lineage, or release posture changes.

## 2. Goals

- Consolidate the explicit auth/source boundary before any live sandbox write.
- Preserve the rule that credentials are caller supplied and never ambient.
- Preserve the rule that credential possession is not write authority.
- Define what a sandbox target proof must show before a provider write can be
  attempted in a future phase.
- Keep `LocalExecutor::execute(...)` write-denied by default.
- Keep provider-write helpers explicit, local, injected, and opt-in.
- Preserve current-product honesty for preview users.
- Prepare a small review/hardening prompt before any live sandbox mutation.

## 3. Non-Goals

This plan does not authorize:

- provider writes;
- live sandbox mutation;
- hidden auth loading from environment, keychain, GitHub CLI, git remotes,
  config files, OAuth state, secret managers, or browser sessions;
- automatic executor provider writes;
- automatic report generation or artifact writes;
- automatic retries, repair, or recovery mutation;
- CLI mutation commands;
- workflow schema fields;
- examples;
- hosted or distributed runtime behavior;
- broad GitHub writes;
- Jira, CI, filesystem, HTTP, or arbitrary provider writes;
- recursive agents, agent swarms, or Level 3/4 autonomy;
- release posture changes.

## 4. Current Implemented Boundary

Implemented and reviewed foundations include:

- caller-supplied GitHub PR comment provider auth wrapper;
- injected provider-call trait and injected-transport HTTP provider client;
- explicit auth matching before provider invocation;
- no hidden auth discovery in core helpers;
- provider write reconciliation model/helper;
- executor-integrated live provider-write helper as an explicit opt-in API;
- provider-write runtime composition helper;
- provider-write sandbox readiness helper;
- artifact-gated provider-write composition helper.

These are local, explicit, and opt-in. None of them make provider writes
automatic, default, CLI-facing, schema-declared, or production-ready.

## 5. Auth Source Policy

Near-term policy:

- Auth material must be supplied explicitly by the caller at the provider/client
  boundary.
- Core helpers must not load auth from environment variables, shell profiles,
  keychains, GitHub CLI state, git credentials, git remotes, repo config, OAuth
  state, browser sessions, or secret managers.
- Auth material must not be serialized, debug-formatted, copied into errors,
  stored in SideEffect records, workflow events, audit records, WorkReports,
  report artifacts, or CLI output.
- Auth scope summaries may be represented only as bounded, non-secret metadata.
- The provider client should continue to fail before transport when supplied
  provider auth does not match the validated provider-call request auth.

Future auth loading should be a separate reviewed phase. It should start with a
typed local auth-source model, not implicit discovery.

## 6. Authority Policy

Possessing a token is not enough to authorize a write.

Future live sandbox phases must still require:

- supported capability;
- explicit sandbox target classification;
- policy allowance;
- SideEffect proposal and attempted lifecycle posture;
- approval-side-effect linkage where required;
- approval-presentation proof where required;
- high-assurance approval posture where configured;
- event-proof and report/artifact policy where configured.

If any required authority signal is missing, stale, ambiguous, or unsupported,
the write must not be attempted.

## 7. Sandbox Target Proof

A future sandbox target proof should be bounded and explicit. For the GitHub PR
comment lane, it should show:

- repository owner and name as a bounded target reference;
- pull request number;
- classification as sandbox/test/disposable target;
- evidence that the target is not production-like;
- expected provider capability, limited to GitHub PR comments;
- intended actor/system actor;
- correlation/idempotency binding;
- sensitivity and redaction metadata.

It must not copy:

- raw PR body text;
- raw issue comments;
- repository file contents;
- provider tokens;
- command output;
- CI logs;
- browser/session state.

Production-looking, unknown, or ambiguous targets must be denied or deferred by
the sandbox readiness path.

## 8. Product Contract Guardrails

Recent evaluator feedback confirms the kernel is credible, but the product
contract must stay honest. Provider-write work must not make preview users
believe Workflow OS is already a production automation platform.

Docs and future CLI surfaces must keep saying:

- current default behavior is local governance, validation, approval, event
  state, first-run posture, report/artifact helper paths, and explicit
  write-adjacent helpers;
- automatic provider writes are not implemented;
- CLI mutation commands are not implemented;
- hidden auth loading is not implemented;
- broader write-capable adapters are not implemented;
- hosted/distributed runtime is not implemented;
- production credential management is not implemented.

The strongest user-facing promise remains: point Workflow OS at a repo and it
tells you how agent work should be governed there. Provider writes must not
blur that boundary before the write path is fully governed and reviewable.

## 9. Review/Harden Before Live Sandbox

Before any live sandbox provider-write phase, perform a focused maintainer
review of:

- auth wrapper matching behavior;
- whether full auth wrapper equality is required, including scope summary;
- hidden-auth-discovery non-regression coverage;
- sandbox target proof vocabulary;
- sandbox readiness helper deny/defer behavior;
- provider/local ambiguity and retry blocking;
- artifact/event-proof gate posture;
- current-product documentation honesty.

This review may identify small hardening fixes. It must not attempt a live
provider write.

## 10. Test Plan For Future Hardening

Future focused tests should cover:

- explicit caller-supplied auth remains the only accepted posture;
- hidden/ambient auth posture is denied;
- auth mismatch fails before transport;
- full auth wrapper equality or documented narrower equality is enforced;
- production-like target proof is denied;
- unknown target proof is deferred or denied;
- sandbox target proof does not serialize raw PR/provider content;
- readiness errors do not leak target strings, auth material, tokens, or
  payload markers;
- default executor paths still do not call providers;
- CLI mutation commands remain absent.

## 11. Proposed Implementation Sequence

1. Perform focused maintainer review of auth/source and sandbox target posture.
2. Add small hardening tests or docs only if the review finds gaps.
3. Review the hardening.
4. Only after that, plan a single disposable live sandbox validation path.
5. Keep any live sandbox validation explicit, local, injected, caller supplied,
   and non-default.

## 12. Open Questions

- Should the provider auth check compare the full auth wrapper or only the
  secret-bearing credential identity?
- What is the smallest acceptable sandbox target proof for GitHub PR comments?
- Should live sandbox validation require high-assurance approval even for a
  disposable target?
- Should auth-source modeling live in core or adapter-specific modules first?
- How should future CLI docs distinguish safe readiness checks from mutation?

## 13. Final Recommendation

Proceed next with a focused provider write sandbox auth/source review.

Do not implement live sandbox writes, hidden auth loading, CLI mutation
behavior, schemas, examples, hosted behavior, broader write-capable adapters,
reasoning lineage, or release posture changes in that review.

