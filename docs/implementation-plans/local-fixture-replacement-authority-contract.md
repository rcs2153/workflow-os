# Local Fixture Replacement Authority Contract

Status: local fixture-only implementation candidate; staged SQLite migration
integrity and V9 activation compatibility corrected. See the
[implementation report](../concepts/LOCAL_FIXTURE_REPLACEMENT_AUTHORITY_IMPLEMENTATION_REPORT.md)
and [focused review](../concepts/LOCAL_FIXTURE_REPLACEMENT_AUTHORITY_REVIEW.md).

## Trust Boundary

The fixture validator must distinguish the **original approved actor** from the
**authenticated replacement principal**. The request hash currently commits to
the full original request, including the actor. Preserve that hash, actor,
approval, attempted SideEffect, and immutable bundle. A continuation grant
authorizes a replacement to validate that exact intent. It cannot rewrite it,
become a provider credential, or authorize dispatch.

The initial trust boundary is a maintainer-owned local kernel and a deliberately
restricted Claude Desktop tool connection. Provision a distinct local connector
principal. Verify possession of a high-entropy, operator-provisioned credential
through the tool transport; never accept a model-supplied actor name as proof.
This authenticates the provisioned principal, not a vendor account or the app
binary. The same OS account is not an isolation boundary against its owner or
another arbitrary process with equivalent filesystem access. If the actual app
session cannot be restricted to the public surface, stop the evaluation and
report that limitation rather than claiming a secure handoff.

## Operator And Tool Surfaces

1. A trusted local operator provisions the connector principal and issues a
   short-lived grant for an already approved fixture with an active window. The
   grant subject is distinct from the original actor. The issuer must have
   explicit issuance authority; being the original actor or possessing a run
   identifier is insufficient. Verify approval and binding from kernel state
   before creation, never from caller assertions.
2. Use the existing `CapabilityGrant` as the subject, issuer, resource, scope,
   expiry, and revocation model. Persist its exact use binding: original actor,
   bundle commitment, approval and presentation proof, fixture action and request
   commitment, SideEffect, window and attempt. Use a create-only identity;
   conflicting reissuance must fail. No downstream delegation or wildcard scope.
3. Register the connector credential independently of the grant. Keep only a
   verifier and principal binding in trusted storage; compare the verifier in
   constant time. Do not place secrets in model-visible arguments, environment
   dumps, shell history, logs, errors, reports, or durable grant records.
4. Expose a narrow public fixture-validation tool. Its user-visible input is an
   allowlisted public run/grant locator and documented bounded fixture input,
   not a path, credential, store root, shell command, issuer or actor override.
   A local stdio MCP adapter is a candidate, not an authority source. The adapter
   may call the public kernel interface but cannot inspect SQLite directly or
   implement grant logic. Do not expose issuance or revocation tools to Claude.
5. Revoke through the trusted operator surface; retain durable revoked posture.
   Credential withdrawal and grant revocation both deny subsequent use. No
   refreshing a grant implicitly during validation.

## Kernel Composition

- Keep the existing same-actor `continue-sandbox` behavior unchanged. Add a
  separate authenticated invocation path. A deserialized request or checkpoint
  cannot construct the internal verified-principal value.
- In `executor.rs`, share original-action validation: immutable bundle, exact
  approval and presentation, SideEffect linkage, request, evidence, and window.
  Do not implement replacement by calling the legacy path with a fabricated
  original principal. Instead, compare the original actor to the request and
  effect, and the independently verified principal to the grant subject.
- Use `sqlite_state` for durable grant/credential-verifier storage. Extend the
  final `fixture_continuation_snapshot_is_current` transaction to recheck the
  same credential registration and grant revision along with the event, effect,
  window, attempt, and dispatch-reservation predicates. Evaluate all time bounds
  using the existing trusted-time observation. A stale revision, rollback of
  trusted time, unavailable source, restored ineligible state, or changed
  record fails closed. Issuance likewise binds the committed facts atomically.
- Return only bounded fixture-verification evidence. A successful validation is
  read-only and non-consuming: it is not a reservation, an authority receipt
  for future use, or permission to execute. A later live design must verify and
  reserve consumption atomically at dispatch, before any external effect.

## Runnable Acceptance

| Test surface | Required evidence |
| --- | --- |
| Core and SQLite | Issuance requires an authorized operator and exact approved intent. Wrong, missing, revoked, or expired grants and unknown/wrong credentials fail closed. Reopen SQLite and verify the grant; revoke it and reject the same request. |
| Final snapshot | Change grant or credential registration after preliminary checks and before the final read. Reject the stale verification. Existing approval, check freshness, request drift, window, and reservation tests remain intact. |
| Public interface | Authenticate a distinct provisioned principal, validate the unchanged request, and reject forged actor identity, different target/action, missing evidence, terminal replay, and any dispatch capability. Repeat fixture validation without creating events, effects, reports, or reservations. |
| Claude Desktop | Start a fresh app session with only the approved tool surface, pass bounded public inputs, run the preregistered fixture corpus, and capture redacted tool and kernel evidence for independent reconstruction. Do not substitute a scripted client for this row. |

First deliver Core, SQLite and the public fixture boundary with real reopen and
revocation tests in one implementation slice. Then configure the app and run the
fixture corpus in a separate governed phase. See the
[Claude Desktop fixture handoff runbook](../user-guide/claude-desktop-fixture-handoff.md)
for the evaluator tool and its operator boundaries. No live provider calls, provider
credentials, generalized identity service, cross-machine or enterprise isolation
claims, or new capability family are authorized by this contract.

Planning governance: `dg/d`, `run-1794035845556832000-2`, proof-backed
delegated approval. This document was produced outside the kernel; no app,
credential, provider, or runtime operation was performed.
