# `workflow-os approve`

Records a local approval grant and resumes an approval-gated run where appropriate.

```text
workflow-os approve <run-id> <approval-id>
workflow-os approve <run-id> <approval-id> --actor user/alice --reason "reviewed locally"
workflow-os --json approve <run-id> <approval-id>
```

The command:

- rehydrates the run from the local backend
- records `ApprovalGranted`
- records the runtime policy decision for resume
- records `RunResumed`
- resumes the local executor path

Approval decisions include actor, reason, timestamp, decision, and correlation ID in runtime events.

v0 supports local approval only. It does not integrate with an identity provider, UI, or external approval system.
