# Evidence Provenance And Report Integrity P0 Assessment

## Executive Verdict

The reported tests are valid and materially sharpen the Workflow OS evidence
boundary.

The kernel currently proves more than a caller assertion, but less than
independent evidence:

- it can prove that one exact content-bound executable request was dispatched
  and returned a bounded process outcome inside the trusted-host domain;
- it cannot prove that an operator-selected executable is the official or
  reviewed validator merely because the closed command profile invoked it; and
- it can prove that a persisted `WorkReport` is structurally valid and
  identity-consistent at read time, but it cannot detect a structurally valid
  report-body edit because the artifact metadata has no complete body
  commitment anchored outside the report payload.

This is a P0 trust-communication and integrity gap. It is not evidence that the
kernel fabricated a dispatch, bypassed an approval, or lost event ordering.

## Code Evidence

### Operator-selected validator

The unpublished local host accepts `--workflow-os-executable` for the closed
project-validation profile. `WorkflowOsProjectValidationLocalHandler` checks
that the path is a file, hashes its contents, fixes the argument list to
`validate`, uses the exact governed project root, and verifies the executable
content hash again before process entry.

Those controls prevent post-resolution executable substitution and prove the
selected process boundary. They do not establish the executable's producer.
The current plan explicitly describes the executable as supplied and
content-bound. A successful alternate executable therefore demonstrates the
documented host trust boundary, while also showing that the word
`authoritative` can be misunderstood without an explicit assurance projection.

### Mutable report body

`WorkReportArtifactRecord::validate()` validates the report and calls
`WorkReportArtifactMetadata::validate_against_report()`. That comparison covers
report, workflow, schema, specification, run, terminal status, generation time,
sensitivity, and redaction identity fields.

It does not commit to report sections, citations, incomplete work, known
limitations, risks, handoff notes, or high-assurance disclosure. Store reads
deserialize and validate the edited record, so a structurally valid summary
edit is accepted by filesystem, SQLite, or PostgreSQL-backed inspection and
replay paths.

The threat model already states that v0 local state lacks cryptographic tamper
evidence. The missing piece is a report-body integrity anchor and an inspection
surface that makes the remaining storage-owner trust explicit.

## Claim Corrections

The current behavior supports these claims:

- exact selected executable bytes and request are content-bound;
- one dispatch and bounded process outcome are durably recorded;
- report artifacts are create-only through supported APIs;
- report records are structurally validated and identity-consistent;
- malformed JSON and identity mismatches fail closed.

It does not support these stronger claims:

- an operator-selected validator is independently trusted;
- a zero exit proves the intended validation logic ran;
- a local report artifact is cryptographically tamper-evident;
- a storage owner cannot rewrite a report;
- replay authenticates report content independently of the state backend.

## Decision

Publication and public-positioning expansion should remain paused while the
truth correction is applied and the P0 runtime sequence is reviewed. Existing
closed profiles and artifact stores should be hardened in place. Workflow OS
should not fork its evidence model, add a generic command runtime, or imply that
a hash stored beside mutable content defeats a privileged storage owner.

The accepted direction is documented in
[Evidence Provenance And Report Integrity P0 Plan](../implementation-plans/evidence-provenance-and-report-integrity-p0-plan.md).

## Governed Phase Record

- Workflow: `dg/blocker`
- Run: `run-1793846367618178000-2`
- Approval: `approval/run-1793846367618178000-2/fix-approved`
- Presentation: `presentation/28d06af2c82f3a66`
- Approval outcome: granted by the delegated maintainer after complete handoff
  presentation
- Runtime implementation: not performed in this phase
- External work: code and documentation inspection, plan authoring, and
  validation were executed outside the kernel under the approved scope

## Validation

- `cargo test -p workflow-core work_report_artifact`: passed; eight artifact
  tests exercised record identity, serialization, storage, duplicate refusal,
  and writer-guard behavior.
- `cargo test -p workflow-local-host --test foreground_admission
  foreground_binary_executes_explicit_project_validation_profile`: passed.
- `cargo test -p workflow-local-host --test foreground_admission
  foreground_binary_rejects_same_path_executable_replacement`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

No runtime code or test expectation was changed. The focused suites confirm the
current bounded guarantees; they do not supply the missing producer provenance
or report-body integrity anchor.

