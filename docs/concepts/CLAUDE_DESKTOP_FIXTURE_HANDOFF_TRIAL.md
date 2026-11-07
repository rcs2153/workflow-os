# Claude Desktop Fixture Handoff Trial

## Scope And Outcome

On November 7, 2026, Codex prepared the repository's synthetic continuation
fixture at commit `0093ae69018ecd93526487e5358a5fc2749f2db8`. The kernel
identified itself as `workflow-os 0.4.0-preview.1`. Native Claude Desktop
`4.23118.0` on the same Mac called the restricted local stdio tool. This was
an actual second-harness invocation, not a scripted MCP client or browser
replacement. The fixture uses a replayed event prefix and synthetic check
evidence; it does not establish interruption of a live provider run.

The operator started a fresh Claude chat without project, folder, file, or
transcript inputs. Memory, Research, and Claude in Chrome were off; the
connector menu showed only the fixture connector enabled. Tool activity
showed the local fixture tool and no other tool use. Only synthetic case
identifiers were sent in the prompt. The app received no credential, private
store location, raw fixture request, or provider payload in the chat or tool
arguments. Its configured child process used the public delegated CLI with
operator-held inputs. This is tool restriction, not OS-level isolation.

## Observed Cases

| Case | Native result |
| --- | --- |
| Exact, first call | Validated in fixture mode with verified delegation and one linked approval. |
| Exact, repeat | Same verdict; no mutation or duplicate dispatch. |
| Changed action | Rejected: approved action mismatch. |
| Changed target | Rejected: request identity mismatch. |
| Wrong principal | Rejected by the kernel. |
| Missing grant | Rejected by the kernel. |
| Revoked grant | Rejected by the kernel. |
| Expired grant | Rejected by the kernel. |

The app showed two successful and six failed calls in its native activity
record. Its exact-case responses showed no provider call, SideEffect mutation,
workflow event append, or report artifact write. A separate operator-side
comparison verified that the SQLite file was byte-for-byte unchanged between
trial setup and the end of app validation. Native per-call prompts were
allowed once for the exact synthetic requests, not persistently.

After verification, the operator revoked the trial grant and principal through
the public CLI, removed the disposable principal files, restored the original
app configuration bytes, and verified Claude reopened without the trial
configuration. The synthetic chat was left intact; no private transcript or
configuration is published here.

## Native Follow-Up

On November 7, 2026, the operator ran the remaining five state-dependent cases
and one positive control in a fresh native Claude Desktop chat on the same Mac,
using repository commit `3da6cbcde9f20f49edadf26be0a7930f6eb1c6c4`. The operator
verified that only the four temporary fixture connectors were enabled and that
Memory, Research, and Claude in Chrome were off. The app received only
synthetic case identifiers and allowed each call once. The native activity
record showed six fixture calls: one successful and five rejected.

| Case | Native result and verified context |
| --- | --- |
| Exact control | Validated with verified delegation; all side-effect flags were false. |
| Reserved attempt | Rejected. The same request passed before Core reserved the exact attempt; the operator verified the window-specific same-actor refusal after reservation. The native delegated result was generic. |
| Denied approval | Rejected at the missing-attempted-effect boundary after a durable proof-backed denial. |
| Terminal replay | Rejected because the completed run was not active. |
| Changed evidence | Rejected after a later failed check superseded the passing evidence. |
| Stale window | Rejected after the real window expired. The same-actor preflight distinguished the stale window; the delegated result was generic and the grant also expired. |

The four SQLite databases matched their pre-trial hashes before cleanup.
The operator then revoked the grants and principals, removed the disposable
principal files, and verified that Claude reopened with its exact original
configuration. No provider call was made.

## Remaining Acceptance

The named synthetic cases now have native observations. Exact repeat and the
reservation conflict demonstrate local refusal and non-mutation, not
provider-side duplicate prevention. No live call, accepted provider effect,
post-effect recovery, independent reviewer reconstruction, or controlled
token/latency comparison was performed. The separately governed live slice
still requires review and its own explicit authorization.

## Governance

- Workflow: `dg/review`; run: `run-1794044256734674000-2`.
- Approval: `approval/run-1794044256734674000-2/review-scope-approved`;
  granted by the delegated maintainer with persisted presentation
  `presentation/d2b26fa78538a279`.
- App configuration, observation, evidence comparison, cleanup, and this
  report were operator actions outside the kernel. No WorkReport was emitted.
- Follow-up: `dg/review`, `run-1794047809252867000-2`; approval
  `approval/run-1794047809252867000-2/review-scope-approved`, granted with
  persisted presentation `presentation/e997d606ef7d91ac`. The operator obtained
  separate desktop launch consent before configuring the app.
