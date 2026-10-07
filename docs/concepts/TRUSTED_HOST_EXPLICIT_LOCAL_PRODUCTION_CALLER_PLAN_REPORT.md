# Explicit Local Trusted-Host Production Caller Planning Report

## 1. Executive Summary

The next P0 trusted-host boundary is planned as one crate-private synchronous
local caller over the accepted production timer. The caller owns only process
lifetime, cancellation, fixed finite budget selection, and fresh
non-authorizing identities. Core continues to own wait satisfaction,
continuation classification, authority, dispatch admission, workflow state,
and completion.

No runtime behavior was implemented.

## 2. Scope Completed

- Defined one explicit synchronous caller boundary.
- Defined process/thread ownership and cancellation posture.
- Defined fresh bounded domain-separated identity requirements.
- Defined fixed finite wake-budget selection.
- Defined restart reconstruction from durable Core state.
- Defined bounded operator outcomes and failure semantics.
- Defined focused future tests and implementation sequencing.
- Updated the authoritative roadmap.

## 3. Scope Explicitly Not Completed

- No Rust caller or identity source.
- No daemon, detached thread, queue, worker pool, or run discovery.
- No durable timer jobs or automatic restart scanning.
- No public API, CLI, SDK, schema, or runtime configuration.
- No automatic approval or model turn.
- No provider mutation, OpenShell, nested harness, or hosted scheduling.
- No release posture change.

## 4. Boundary Summary

The future caller receives an already-selected durable operational locator and
exact injected execution inputs. It creates process-local cancellation and
fresh operation identities, calls the accepted private timer once, and returns
bounded posture. It cannot construct authority, claim deadline satisfaction,
reinterpret workflow state, or fabricate completion.

## 5. Identity And Budget Decisions

Identity generation is non-authorizing and must use bounded domain-separated
random identities with deterministic test injection. Replay conflict is not
automatically retried.

The first caller uses one reviewed private budget below or equal to the
existing maximum of eight. Budget exhaustion remains non-terminal and requires
a later explicit owner decision.

## 6. Restart And Ownership

The calling process and thread remain the execution owner. Restart discards
timer, cancellation, identity-provider, and remaining-budget state. A new
explicit call reopens SQLite and lets Core rehydrate the exact durable posture.
No event or receipt reconstructs authority.

## 7. Privacy And Failure Posture

The planned outcome contains only disposition, counts, and a closed stop
reason. It carries no source content, command, prompt, credential, approval
reason, evidence body, skill output, path, provider payload, or reusable
authority. Errors remain stable and non-leaking, and no error is retried
automatically.

## 8. Validation

Planning validation:

- `npm run check:docs`
- `git diff --check`
- governed event-trail inspection

Rust validation is not required because this phase changes documentation only.

## 9. Remaining Limitations

- The caller and production identity source do not exist yet.
- The first caller remains private, local, synchronous, SQLite-only, and
  `TimeWindow`-only.
- No public or hosted surface selects the caller.
- Owner loss, metrics, notifications, and automatic restart remain deferred.
- A later explicit adoption site requires separate planning and review.

## 10. Recommended Next Phase

Perform focused maintainer/security review of the explicit local production
caller plan. If accepted, implement only the private caller and identity source.

## 11. Governed Planning Record

- workflow: `dg/d`
- run: `run-1791361830972153000-2`
- approval: `approval/run-1791361830972153000-2/planning-approved`
- presentation: `presentation/15ce2bed9121582b`
- presentation hash:
  `15ce2bed9121582bd616d4c505cfd943365f4738aaf8940c284f7b04c8b1924d`
- approval outcome: granted by delegated maintainer through proof enforcement
- approved boundary: planning and documentation only
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations;
  approval-presentation proof enforced with one matching record and event
  marker
- validation summary: `npm run check:docs` and `git diff --check` passed; Rust
  checks were not required because the phase changed documentation only
- out-of-kernel work: architecture inspection, plan authoring, documentation
  validation, and later git and pull-request actions
