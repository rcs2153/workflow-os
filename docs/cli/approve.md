# `workflow-os approve`

Records a local approval grant or denial for an approval-gated run.

```text
workflow-os approve <run-id> <approval-id>
workflow-os approve <run-id> <approval-id> --actor user/alice --reason "reviewed locally"
workflow-os approve <run-id> <approval-id> --deny --actor user/alice --reason "risk is unacceptable"
workflow-os --mock-all-local-skills approve <run-id> <approval-id>
workflow-os --json approve <run-id> <approval-id>
```

By default, `approve` grants the approval. Passing `--deny` records a denial. Denial requires `--reason` so the event log captures operator intent.

On grant, the command:

- rehydrates the run from the local backend
- records `ApprovalGranted`
- records the runtime policy decision for resume
- records `RunResumed`
- resumes the local executor path

If grant resumes into a local skill invocation, the same handler boundary as `workflow-os run` applies. A real local handler must be registered by the caller, or the CLI must be run with `--mock-all-local-skills` for explicit deterministic mock execution. Without a handler, the resumed run fails closed instead of pretending the skill is implemented.

On denial, the command:

- rehydrates the run from the local backend
- records `ApprovalDenied`
- records `RunFailed`
- does not invoke the gated skill

Approval decisions include actor, reason, timestamp, decision, and correlation ID in runtime events.

`--json` includes the approval decision and resulting run status. JSON output is experimental in `0.1.0-preview.1`; it is useful for preview automation, but it is not yet a versioned stable machine-output contract.

v0 supports local approval only. It does not integrate with an identity provider, UI, or external approval system.
