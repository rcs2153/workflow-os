# Authoritative Continuation Registered Current-Authority Consumer Plan Report

## 1. Executive Summary

Workflow OS now has a phase-ready plan for the first continuation consumer
that combines fresh registered current-authority resolution with the existing
durable cursor-bound continuation claim before one local skill invocation.

The plan preserves the governing invariant that only current kernel state may
declare and authorize the next material action. It does not treat the
read-only preview, a source snapshot, an assessment, or conversation memory as
permission.

## 2. Scope Completed

- Inspected the accepted continuation model, immutable-run consumer, read-only
  preview, private registered current-authority source, same-call use boundary,
  required-context binding, and local executor path.
- Defined one crate-private `invoke_current_step_skill` composition.
- Defined exact binding, source-resolution, commitment, durable claim,
  consumer, failure, replay, privacy, and test requirements.
- Updated the parent continuation plan and authoritative roadmap queue.

## 3. Scope Explicitly Not Completed

- No Rust implementation or runtime behavior.
- No public source trait, public source constructor, runtime source
  configuration, or caller-asserted authority.
- No provider call, provider mutation, OpenShell adapter, sandbox, typed child
  runtime, nested harness runtime, SideEffect execution, schema, CLI change,
  hosted behavior, or release change.

## 4. Key Architecture Decision

The first composition remains crate-private because the accepted registered
current-authority source is intentionally Core-owned and private. Exposing the
consumer before a trusted source-configuration boundary exists would invite
caller-assembled authority and overclaim operational enforcement.

The internal proof will still exercise the real local skill consumer. Fresh
source resolution will wrap the existing continuation projection, durable
first-writer claim, post-claim cursor reread, hook path, invocation events, and
handler call.

## 5. Ordering Decision

Fresh registered-source resolution and exact required-context consumption
occur before the continuation claim. The accepted source, fact-set,
assessment, and consumption commitments become part of the claim's governance
commitment. A blocked source therefore creates no continuation claim and
reaches no handler.

Every attempted use resolves the source again. Duplicate or stale claims do
not permit reuse of an earlier accepted assessment.

## 6. Privacy And Security Posture

Only bounded commitments and posture may cross the composition. Source
inventories, grants, availability records, context contents, paths, prompts,
commands, provider payloads, environment values, credentials, and secret-like
metadata remain excluded from Debug, errors, events, and public
serialization.

The plan keeps default execution unchanged and prohibits fallback from blocked
source-backed authority to the immutable-only continuation path.

## 7. Test Plan Summary

The future implementation must prove ready use, fresh resolution, exact
commitment binding, durable first-writer behavior, stale-cursor rejection,
grant expiry/revocation, source failure, required-context and prerequisite
blocking, identity substitution rejection, event ordering, unchanged default
paths, preview non-authority, and non-leakage.

## 8. Validation

Planning validation requires:

- `npm run check:docs`
- `git diff --check`

The plan was also checked against the current Rust source and accepted reviews
for the registered source, same-call use boundary, continuation consumer, and
read-only preview.

## 9. Governed Planning Record

- workflow ID: `dg/d`
- run ID: `run-1786775805522199000-2`
- approval ID:
  `approval/run-1786775805522199000-2/planning-approved`
- approval presentation ID: `presentation/d2ae68400531310b`
- approval presentation content hash:
  `d2ae68400531310bebe0a837e5a32c39fe05fbba2da808d85f8e567840a2c7aa`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted handoff was presented
- out-of-kernel work: source inspection, architecture analysis, documentation
  edits, and validation were performed by the delegated maintainer; the kernel
  governed scope and approval but did not inspect code, edit files, execute
  checks, or mutate git

## 10. Remaining Limitations

- The planned consumer is an internal proof, not a public operational path.
- Trusted runtime source configuration is not implemented.
- No event or report projection records continuation use yet.
- No external shell, editor, git, browser, provider, or child-harness action is
  intercepted by this path.
- Provider mutation and nested harness broadening remain blocked.

## 11. Recommended Next Phase

Perform focused maintainer/security review of the plan. If accepted, implement
the crate-private source-backed local continuation consumer and its focused
tests, then review the implementation before planning trusted runtime source
configuration.
