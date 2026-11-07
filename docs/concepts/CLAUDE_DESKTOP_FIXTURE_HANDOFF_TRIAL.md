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

## Remaining Acceptance

This is partial fixture acceptance, not the full milestone. An explicit denied
approval, stale execution window, changed durable evidence, terminal replay,
and a dispatch-reservation conflict were not exercised through Claude Desktop.
Exact repeat demonstrates non-mutation, not provider-side duplicate prevention.
No live call, accepted provider effect, post-effect recovery, independent
reviewer reconstruction, or controlled token/latency comparison was performed.
The live slice remains gated on completion and review of the fixture corpus.

## Governance

- Workflow: `dg/review`; run: `run-1794044256734674000-2`.
- Approval: `approval/run-1794044256734674000-2/review-scope-approved`;
  granted by the delegated maintainer with persisted presentation
  `presentation/d2b26fa78538a279`.
- App configuration, observation, evidence comparison, cleanup, and this
  report were operator actions outside the kernel. No WorkReport was emitted.
