# Explicit Local Trusted-Host Production Caller Plan Blocker Fix Report

## 1. Executive Summary

All five blockers from the focused caller-plan review are corrected. The plan
now defines a reachable cancellation API, exact production entropy and identity
encoding, all-or-nothing identity-set construction, a fixed two-wake budget,
an existing bounded outcome without resume advice, and source-of-truth language
that leaves current authority exclusively with Core.

No runtime code or dependency change was made.

## 2. Blockers Fixed

1. The explicit owner creates the existing private cancellation pair, retains
   the handle, and passes the receiver into the synchronous caller.
2. Production identity generation uses a direct `getrandom` 0.4 dependency,
   128 random bits per identifier, fixed lowercase hexadecimal encoding, and
   operation-family prefixes.
3. A redispatch or wake identity set is filled and validated atomically before
   any part is returned.
4. The caller uses exactly two wakes and accepts no budget input.
5. The caller returns the accepted repeated-scheduling outcome unchanged, with
   no reconstruction or resume-advice boolean.
6. “Already-authorized” wording is corrected to “already-selected”; only Core
   may establish current authority in the same call.

## 3. Scope Preserved

The fix remains documentation-only. It adds no caller, entropy dependency,
scheduler, thread, run discovery, public API, CLI, SDK, schema, approval
automation, provider mutation, OpenShell, nested harness, hosted behavior, or
release change.

## 4. Security Posture

Cancellation is explicit but non-authorizing. Identity generation provides
collision resistance and idempotency material but no execution authority.
Replay conflicts remain fail-closed and are never hidden by automatic retries.
Budget exhaustion remains non-terminal. Every later owner decision must reenter
Core rather than trusting caller advice.

## 5. Test Plan Correction

The future implementation must now prove handle reachability during a blocked
synchronous call, all-or-nothing identity construction, independent random
material, deterministic test injection, fixed two-wake posture, absence of
resume advice, restart safety, aggregate at-most-once execution, privacy, and
no background or public surface.

## 6. Validation

- `npm run check:docs`
- `git diff --check`
- governed phase-close inspection

Rust checks are not required because this phase changes documentation only.

## 7. Remaining Limitations

- No caller or identity source exists yet.
- The direct dependency is authorized only for the future bounded
  implementation and remains subject to dependency and security validation.
- A later internal adoption site, operator surface, and detached owner-loss
  semantics remain separately planned work.

## 8. Recommended Next Phase

Perform focused maintainer/security re-review of the corrected plan. If
accepted, proceed to one private implementation phase only.

## 9. Governed Blocker-Fix Record

- workflow: `dg/blocker`
- run: `run-1791362742739076000-2`
- approval: `approval/run-1791362742739076000-2/fix-approved`
- presentation: `presentation/3bf2a68ef7143b3e`
- presentation hash:
  `3bf2a68ef7143b3e55898977c429cd4a3060462675cf0a82658590aecbce7fd5`
- approval outcome: granted by delegated maintainer through proof enforcement
- approved boundary: documentation-only planning blocker fix
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations;
  approval-presentation proof enforced with one matching record and event
  marker
- validation summary: `npm run check:docs` and `git diff --check` passed; Rust
  checks were not required because the phase changed documentation only
- out-of-kernel work: plan editing, documentation validation, and later git
  and pull-request actions
