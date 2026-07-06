# First-Run Summary And Demo Separation Review

## 1. Executive Verdict

Phase accepted; proceed to the next real-repo onboarding UX slice or merge readiness.

The implementation makes `workflow-os first-run` easier to understand without changing runtime semantics. It adds a concise `what_matters_now` block, changes the recommended next action to review/setup work, and labels the generated mock workflow command as an optional approval/audit demo rather than repository analysis.

## 2. Scope Verification

The phase stayed within the approved UX/output scope.

Implemented:

- concise human summary;
- clearer recommended next action;
- optional mock demo labeling;
- focused CLI assertions;
- CLI docs and roadmap updates;
- implementation report.

Not implemented:

- command execution;
- source-content inspection;
- automatic workflow generation;
- provider calls;
- writes;
- schema changes;
- hosted behavior;
- recursive agents;
- agent swarms;
- release posture changes.

## 3. UX Assessment

The previous human output was correct but directed users immediately toward the mock workflow as the recommended next action. That created a product ambiguity: the useful repository posture analysis happens in `first-run`, while the generated mock workflow demonstrates approval and event-history mechanics.

The new output separates these concerns:

- `recommended_next_action` now points to reviewing findings and assigning ownership/check obligations.
- `optional_approval_audit_demo` contains the mock workflow command.
- `optional_demo_note` explains that the mock run is not additional repository analysis.
- `what_matters_now` gives users a short operator-facing summary before the detailed posture matrix.

This is a meaningful first-use UX improvement and does not weaken governance posture.

## 4. Runtime Semantics Assessment

The implementation preserves runtime boundaries:

- `first-run` still does not start a run;
- `first-run` still does not create runtime state;
- no local checks are executed;
- no package scripts are executed;
- no providers are called;
- no WorkReport artifact is created;
- recommendations remain review-only.

The optional mock command remains available, but it is no longer framed as the primary next action.

## 5. Privacy And Redaction Assessment

The added summary lines use static bounded text and existing metadata posture only. They do not copy package script bodies, dependency values, source contents, command output, provider payloads, owner values, escalation contacts, credentials, or token-like strings.

The package-aware summary branch remains bounded to detected posture and does not print manifest payloads.

## 6. Test Quality Assessment

Focused tests now assert:

- `what_matters_now` is present;
- the generic no-metadata path prompts ownership/escalation/evidence/check setup;
- the TypeScript/package path prompts package-aware workflow/validation thinking;
- the recommended action is review/setup;
- the mock workflow command is explicitly optional;
- the demo note says it is not additional repository analysis.

The broader first-run test suite continues to cover report-ready context, safe metadata output, non-leakage, no state creation, and review-only recommendations.

## 7. Validation Evidence

Local validation:

- `cargo test -p workflow-cli --test cli first_run -- --nocapture`: passed.
- `cargo fmt --all --check`: passed after applying `cargo fmt --all`.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

GitHub validation:

- PR head `91bc6bcdf524056fe89d1b7321783b3748d4ad84` had all five required checks passing before this review document was added.

## 8. Blockers

None.

## 9. Non-Blocking Follow-Ups

- Consider a future `--verbose` or grouped output mode so the detailed posture matrix can be collapsed for first-time users.
- Consider making suggested validation obligations more structured in preview JSON.
- Expand safe metadata detection to other ecosystems only with the same bounded allowlist posture.

## 10. Recommended Next Phase

Merge readiness for the real-repo onboarding UX PR.

The onboarding lane now has implementation, review, focused validation, full local validation, and CI evidence. The remaining issue is not code readiness; it is authenticated GitHub mutation tooling for marking the PR ready/updating metadata/merging.
