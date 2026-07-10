# GitHub PR Connector Boundary Blocker Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The blocker report correctly separates Codex GitHub connector failures, GitHub
UI/browser state, local git access, direct REST access, and Workflow OS
provider-call behavior. This distinction is important maintainer hygiene: a
connector-session failure must not be described as a Workflow OS provider
failure unless a Workflow OS provider path actually ran.

## 2. Scope Verification

The blocker report stayed within approved maintainer-ops diagnosis scope.

Confirmed in scope:

- documented the failing boundary as external connector/session behavior;
- documented that local git branch push can succeed independently;
- documented that direct REST access has its own permission boundary;
- documented that Workflow OS GitHub provider code is scoped to explicit GitHub
  pull request comment provider calls;
- added an operating rule for future PR and connector diagnosis.

No accidental implementation was found for:

- connector session repair;
- GitHub app regrant;
- browser automation changes;
- GitHub PR creation inside Workflow OS;
- hidden access loading;
- automatic provider writes;
- broad GitHub write support;
- CLI PR commands;
- schemas;
- examples;
- hosted behavior;
- release posture changes.

## 3. Boundary Assessment

The report's central correction is sound:

```text
Connector-session failures, browser-state failures, git transport failures,
direct REST failures, and Workflow OS provider failures are separate evidence
classes.
```

The current Codex GitHub connector is external to this repository and reports
its own session state. That does not prove local git access is broken, and it
does not prove the Workflow OS GitHub provider path failed.

The local git credential path is also distinct. It can authorize branch push
without proving PR creation API access or connector availability.

The direct REST path is a third boundary. It can be used for maintainer
automation when appropriately authorized, but it is not the Codex GitHub
connector and it is not a Workflow OS runtime provider.

The Workflow OS provider path is narrower still: it is a caller-supplied,
injected-transport GitHub pull request comment provider boundary. It is not PR
creation, it does not load hidden access material, and it does not make default
executor writes automatic.

## 4. Provider-Code Assessment

The reviewed provider documentation and code support the blocker report.

The GitHub pull request comment provider implementation keeps:

- explicit caller-supplied access material;
- injected transport;
- bounded provider response classification;
- no hidden environment, keychain, GitHub CLI, git remote, or config discovery;
- no default executor write behavior;
- no PR creation behavior;
- no workflow event append by the provider client itself;
- no report artifact write by the provider client itself.

This confirms that the observed PR creation tool failure did not exercise a
Workflow OS provider-write path.

## 5. Operating Rule Assessment

The report's operating rule is accepted for future maintainer work:

- identify which system produced the error;
- do not collapse connector, browser, git, REST, and Workflow OS provider
  failures into one diagnosis;
- avoid describing a session failure as a token or provider failure without
  proof of the failing credential source;
- do not fall back to a different browser when the user explicitly asked to use
  Chrome;
- preserve pushed branches and report the exact failing boundary when PR
  creation is blocked.

This rule is especially useful because Workflow OS increasingly uses GitHub in
three different ways: maintainer PR hygiene, read/write provider modeling, and
repo-local dogfood governance.

## 6. Privacy And Redaction Assessment

The report does not publish secrets, raw provider payloads, command transcripts
with sensitive values, browser session contents, access material, or GitHub
connector internals.

It references only bounded status classes and high-level outcomes. That is the
right posture for a maintainer-ops blocker report.

## 7. Documentation Review

Roadmap language correctly treats this as maintainer tooling reliability, not a
Workflow OS runtime capability. The report does not claim that Workflow OS can
repair external connector sessions, open pull requests through its runtime, or
perform broad GitHub writes.

The report also preserves the current release posture: GitHub PR comment
provider work remains an explicit, caller-supplied provider boundary, and PR
creation remains outside Workflow OS runtime capability.

## 8. Test And Validation Assessment

The phase is documentation and diagnosis only. The implementation report
recorded:

- governed `dg/blocker` run;
- docs validation;
- phase close;
- inspected repo-owned provider files and reviews.

For this review, the relevant evidence is document consistency and provider
boundary inspection rather than Rust behavior changes.

Review validation:

- `npm run check:docs` - passed.
- `git diff --check` - passed.
- Provider boundary inspection confirmed the GitHub pull request comment
  provider remains explicit, injected, and separate from PR creation tooling.

## 9. Blockers

None.

## 10. Non-Blocking Follow-Ups

- Add a maintainer runbook note, if this failure pattern recurs, that explains
  how to distinguish connector session state from local REST and provider
  boundaries.
- Consider installing a supported GitHub CLI path or documenting the local REST
  fallback for PR hygiene, while keeping it outside Workflow OS runtime
  capability.
- Keep future Workflow OS provider-auth work separate from Codex connector
  repair work.

## 11. Recommended Next Phase

Recommended next phase: return to the runtime-composition roadmap lane.

Reason: this blocker was a diagnosis and maintainer-tooling boundary issue, not
a Workflow OS runtime provider defect. The repo should continue composing
already-built governance primitives into explicit runtime paths while preserving
the accepted boundary between maintainer tooling and Workflow OS provider
capability.

## 12. Dogfood Governance

Workflow OS governed this review phase:

- workflow: `dg/review`;
- run: `run-1783707147153586000-2`;
- approval: `approval/run-1783707147153586000-2/review-scope-approved`;
- approval presentation: `presentation/920e95d46d479916`;
- approval outcome: granted;
- approval reason: `approved-github-pr-connector-boundary-review`;
- close status: `Completed`;
- events: 39 total;
- event summary: `ApprovalGranted:1`, `ApprovalRequested:1`,
  `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`,
  `RunResumed:1`, `RunStarted:1`, `RunValidated:1`,
  `SkillInvocationRequested:6`, `SkillInvocationStarted:6`,
  `SkillInvocationSucceeded:6`, `StepScheduled:6`;
- approval presentation enforcement: proof enforced;
- approval presentation event marker: present;
- scope: review the GitHub PR connector boundary blocker report;
- expected validation: `npm run check:docs`, `git diff --check`, and provider
  boundary inspection.

Codex performed repository inspection, documentation edits, validation, git,
and PR operations outside the kernel. The kernel governed the review approval
boundary.
