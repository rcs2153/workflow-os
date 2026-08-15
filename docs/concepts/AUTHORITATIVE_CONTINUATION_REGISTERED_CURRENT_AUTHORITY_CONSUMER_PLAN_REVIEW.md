# Authoritative Continuation Registered Current-Authority Consumer Plan Review

## 1. Executive Verdict

**Plan accepted with implementation clarifications; proceed to the
crate-private source-backed continuation consumer.**

The plan defines the smallest credible bridge between the accepted private
registered current-authority source and the accepted durable continuation
claim. It does not mistake a preview, source snapshot, or caller assertion for
authority.

## 2. Scope Verification

The plan remains within the P0 continuation boundary. It authorizes one
crate-private local skill consumer composition and focused tests only.

It does not authorize a public source trait, runtime source configuration,
provider mutation, OpenShell integration, nested harness runtime, typed child
runtime, generic context dereference, automatic command execution, schemas,
hosted behavior, or release changes.

## 3. Trust-Boundary Assessment

The selected source remains the existing Core-owned private registered
in-memory aggregate. Its generic `FnOnce` capability remains private and
borrowed. The plan adds no caller-built `Ready` value, public authorization
method, or reusable authority object.

Keeping the first executor composition crate-private is necessary. The repo
does not yet have trusted runtime source configuration, so a public consumer
would either be unusable or would make caller-supplied facts appear
authoritative.

## 4. Binding Assessment

The plan requires exact agreement among the durable run, immutable bundle,
executor plan, actor, workflow, run, step, harness contract, contract hash, and
invocation identity. This reuses `RequiredContextExecutionBinding` and the
existing continuation binding rather than adding another identity model.

Static substitutions are rejected before new invoke-policy events. Dynamic
cursor drift is rechecked after durable rehydration and again after the
continuation claim. That distinction is now explicit in the plan.

## 5. Freshness And Required-Context Assessment

Every attempted use calls the registered source again. Source selection,
fact-set construction, capability resolution, governed-context projection,
and exact required-context consumption occur before the continuation claim and
handler.

Expired, revoked, unavailable, incomplete, stale, or prerequisite-blocked
posture cannot fall back to the immutable-only path. Optional context gaps
retain the already-reviewed registered-source semantics; required gaps block.

## 6. Commitment And Atomicity Assessment

The accepted source commitments are folded into the existing continuation
governance commitment, which is already part of the durable cursor-bound
idempotency claim. The review tightened the API direction: the borrowed source
capability should return one domain-separated internal continuation
commitment, not expose its assessment fields to the executor or a public API.

The composition gives one local first writer for one current cursor and source
assessment. It does not claim transactional atomicity with a future external
source. Crash-after-claim ambiguity also remains explicitly deferred.

## 7. Executor-Semantics Assessment

The plan preserves default and immutable-only executor paths. The new private
builder attaches one exact source/binding/contract context and enables the
existing continuation guard. The current authorized local skill path remains
the only consumer, preserving hooks, SideEffect disclosure, invocation events,
attempt idempotency, retries, and handler behavior.

A source block may follow an already-recorded invoke-policy event. The plan no
longer overclaims zero event change; it guarantees no continuation claim,
hook, attempt, or handler use for that blocked path.

## 8. Privacy And Error Assessment

The plan excludes source inventories, grants, availability records, context
contents, prompts, commands, provider data, paths, environment values, and
credentials. Stable Core-owned errors replace source-local details. Debug
output remains posture/count-oriented and commitments remain redacted.

## 9. Test-Plan Assessment

The proposed tests cover the important positive and negative behavior:

- exact ready use and event ordering;
- source resolution before claim and handler;
- grant expiry/revocation and source failure;
- required context and independent prerequisites;
- identity substitution;
- duplicate first-writer and stale cursor behavior;
- fresh resolution on later attempts;
- unchanged default, immutable-only, and preview paths; and
- privacy/non-leakage.

Implementation review should require direct counters or durable-state checks
for source reads, claim creation, handler calls, and event ordering rather than
construction-only assertions.

## 10. Blockers

None for the crate-private implementation proof.

This acceptance does not unblock provider mutation or nested harness runtime.
Those lanes still require a separately accepted trusted runtime source-
configuration boundary after the internal composition is reviewed.

## 11. Non-Blocking Follow-Ups

- Decide how a future operational source is configured without turning caller
  facts into authority.
- Add bounded continuation outcome event/report projection before wider use.
- Define crash recovery for a claimed continuation whose consumer outcome was
  not durably observed.
- Review typed child launch and result acceptance separately.

## 12. Recommended Next Phase

Implement the crate-private registered-current-authority continuation
composition exactly as planned, add focused behavioral tests, run the complete
repository validation suite, and perform focused maintainer/security review.

Do not resume provider mutation broadening or nested harness execution after
the internal proof alone.

## 13. Validation

- `npm run check:docs`
- `git diff --check`
- Claims checked against current registered-source, required-context,
  immutable-run, continuation, executor, and local-skill code.

## 14. Governed Review Record

- workflow ID: `dg/review`
- run ID: `run-1786776082304961000-2`
- approval ID:
  `approval/run-1786776082304961000-2/review-scope-approved`
- approval presentation ID: `presentation/6c5613685e18351b`
- approval presentation content hash:
  `6c5613685e18351b3ff4c00cca8188d0f8da576802f18f77e59989e2c25d9c1b`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted handoff was presented
- out-of-kernel work: code and documentation inspection, review authorship,
  accuracy corrections, and validation were performed by the delegated
  maintainer; the kernel governed scope and approval but did not inspect code,
  edit files, execute checks, or mutate git
