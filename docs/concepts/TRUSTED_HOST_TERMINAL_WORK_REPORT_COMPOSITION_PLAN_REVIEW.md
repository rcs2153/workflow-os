# Trusted-Host Terminal WorkReport Composition Plan Review

## 1. Executive Verdict

**Needs one planning blocker fix.**

The proposed boundary is appropriately narrow and correctly places terminal
report composition after durable workflow terminalization. It preserves
same-call check truth, workflow semantics, privacy, and the unpublished
trusted-host boundary.

One ambiguity must be removed before implementation: exact terminal replay
cannot produce the same evidence-backed report unless the bounded check-result
reference is durably and atomically committed with terminal workflow state.
The plan currently permits either a narrow durable record or an explicit
limitation. The limitation alternative contradicts the plan's deterministic
replay requirement and is not acceptable.

## 2. Scope Verification

The plan stays within the approved private in-memory composition scope. It does
not authorize:

- report artifacts or report persistence;
- released CLI report rendering or export;
- another local-check profile or arbitrary commands;
- scheduling, automatic approval, or policy broadening;
- provider access, network access, credentials, SideEffects, or writes;
- workflow schema changes or report runtime configuration;
- hosted parity, OpenShell, nested harnesses, reasoning lineage, or release
  posture changes.

The proposed payload-free durable check-result binding is supporting execution
evidence, not a persisted WorkReport. The blocker fix must preserve that
distinction.

## 3. Runtime Boundary Assessment

The plan correctly identifies the ownership boundary:

- Core resolves and invokes the exact closed profile;
- the private trusted-host adapter observes the actual same-call result;
- existing lifecycle code owns invocation and terminal workflow events;
- the exact terminal run and immutable bundle are rehydrated before report
  construction; and
- the host sees fixed report posture rather than report contents.

This is preferable to constructing a report in `workflow-local-host`, accepting
caller-supplied references, or rerunning a command to recover evidence.

## 4. Same-Call Check Truth Assessment

The plan correctly treats the concrete `LocalCheckResult` as authoritative.
Only Core may derive a `LocalCheckResultReference` from that result. Passing,
failed, and timed-out results remain distinct, and an internal error without a
result cannot create a fake citation.

The explicit prohibition on reconstructing evidence from output-reference text
is correct. A string such as `local-check-result/{command}/{status}` does not by
itself prove the complete result binding or provide failed-result replay truth.

## 5. Identity And Contract Assessment

The private `trusted-host-terminal-no-write` `v1` contract identity is suitably
distinct from the hosted no-write contract. The report ID inputs cover the
minimum immutable operation binding: run, bundle, profile, step, command, and
check-result identity.

The implementation should use an existing domain-separated digest helper and
a fixed versioned encoding. That is a non-blocking implementation detail as
long as tests prove deterministic replay and substitution resistance.

The fixed system actor, terminal-event timestamp, `Internal` sensitivity, and
validated empty redaction posture are appropriate for this private path.

## 6. Terminal Semantics Assessment

The plan preserves the critical semantic boundary:

- workflow terminalization happens before report composition;
- report failure is secondary posture and cannot rewrite run success or
  failure;
- waiting, blocked, prepared, and denied outcomes produce no terminal report;
- no report failure retries the process or appends a replacement terminal
  event; and
- internal handler errors cannot be represented as successful checks.

These rules are sufficient for the first private in-memory path once the
replay evidence blocker is resolved.

## 7. Replay And Recovery Assessment

The plan requires exact replay to produce the same report identity and bounded
contents without process execution or event append. That requirement is
correct.

However, the current plan says that an unreconstructible result reference may
lead to either a narrow durable record **or an explicit report limitation**.
Those are not equivalent. A limitation-only replay would omit evidence that
was present during fresh execution and therefore produce a different report.
For failed and timed-out checks, the current terminal failure events do not
carry a bounded result reference at all.

Implementation must not begin until the plan requires one exact mechanism:

1. derive a payload-free bounded check-result receipt/reference from the
   same-call result;
2. commit that binding atomically with the invocation terminal event and run
   terminalization;
3. bind it to run, bundle root, workflow, step, profile, command, attempt, and
   result status;
4. reject conflicting replay and substitution;
5. reconstruct it without stdout, stderr, summaries, paths, or payloads; and
6. use the exact same committed binding for fresh composition and replay.

The plan may reuse an existing event or record only if that shape proves every
required binding for passing, failed, and timed-out results. Otherwise it must
authorize one narrow payload-free durable record or event projection.

## 8. Privacy And Redaction Assessment

The plan's privacy boundary is strong. It excludes command output, transcripts,
paths, source/spec/parser/provider payloads, environment values, credentials,
policy text, approval reasons, and caller-authored report prose.

The blocker fix must state that the durable check-result binding stores only
validated identifiers, status vocabulary, immutable commitments, event linkage,
and timestamps. It must not persist result summaries or raw output in order to
solve replay.

## 9. Section And Citation Assessment

All required v1 report sections are present. Explicit unavailable text is used
instead of fabricated citations. The plan correctly cites Core-produced stable
references and does not create `EvidenceReference` values implicitly.

Policy and approval citations must remain derived from exact durable events or
existing validated reference types. Approval reason text and policy source text
must not be copied into report sections.

## 10. Test Plan Assessment

The proposed tests cover both closed profiles, passing and non-passing results,
internal errors, deterministic identity, replay, substitution, required
sections, semantic preservation, non-leakage, and absence of artifacts.

The blocker fix should add explicit tests for:

- atomic check-result-binding commit with terminalization;
- crash recovery after terminal commit and before report composition;
- conflicting replay of a result ID or status;
- failed and timed-out result reconstruction; and
- absence of stdout, stderr, summaries, and paths from the durable binding.

## 11. Planning Blocker

**Blocker: the authoritative replay source for the bounded local-check result
is not selected.**

Action required: amend the plan to mandate an atomic payload-free durable
check-result binding for every concrete passing, failed, or timed-out result.
Remove the option to treat missing replay evidence as a mere known limitation.
Define minimum fields, ownership, atomicity, conflict behavior, and recovery
tests without authorizing raw result persistence or a WorkReport artifact.

## 12. Non-Blocking Follow-Ups

- Select the exact existing domain-separated digest helper during
  implementation.
- Decide whether fixed host output distinguishes generated fresh reports from
  deterministically reconstructed replay reports.
- Add actual foreground timeout coverage later if a deterministic short-timeout
  test profile can be introduced without broadening production behavior.

## 13. Recommended Next Phase

Perform a **trusted-host terminal WorkReport composition planning blocker fix**,
then conduct a focused re-review. Do not begin runtime implementation until the
durable same-call result binding and exact replay source are unambiguous.

## 14. Governed Review Evidence

The review ran under:

- workflow ID: `dg/review`
- run ID: `run-1792664068653701000-2`
- approval ID:
  `approval/run-1792664068653701000-2/review-scope-approved`
- approval outcome: granted by `user/delegated-maintainer`
- presentation proof: `presentation/0c3bc207c58596c1`

`npm run check:docs` and `git diff --check` passed. The terminal governed run
contains 39 events, one approval request and grant, zero retries, and zero
escalations. Approval presentation was proof-enforced and its marker is present
in the event trail.

Repository reads, review writing, shell validation, and git inspection occurred
outside the kernel under the approved review scope. Rust and TypeScript checks
were skipped because no runtime code changed. The kernel did not edit files,
commit, push, open a pull request, generate a WorkReport, or create an artifact.
The missing durable replay evidence boundary is reported as a blocker rather
than simulated.
