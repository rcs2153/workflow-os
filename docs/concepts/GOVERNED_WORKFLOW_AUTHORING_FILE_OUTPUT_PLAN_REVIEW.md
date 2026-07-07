# Governed Workflow Authoring File Output Plan Review

## 1. Executive Verdict

Plan accepted with non-blocking follow-ups.

The plan defines a conservative future boundary for `workflow-os author workflow` file output. It correctly treats file output as the first repository mutation point in the governed workflow authoring lane and keeps the future implementation explicit, inactive, conflict-checked, non-overwriting, and separate from registration, promotion, command execution, provider calls, runtime state, schemas, examples, hosted behavior, write-capable adapters, and release posture changes.

No planning blocker was found.

## 2. Scope Verification

The plan stayed within planning-only scope.

Confirmed in scope:

- recommended future CLI shape;
- output path policy;
- inactive draft lifecycle policy;
- conflict handling requirements;
- bounded input policy;
- draft content policy;
- promotion boundary;
- error-code candidates;
- privacy/redaction requirements;
- documentation requirements;
- future test plan;
- proposed implementation sequence;
- open questions.

No accidental authorization was found for:

- implementation in the planning phase;
- workflow file writes now;
- active workflow generation;
- workflow registration;
- workflow promotion or activation;
- workflow catalog storage;
- command execution;
- local check execution;
- provider calls;
- runtime state creation;
- approval decisions;
- report artifacts;
- workflow schema changes;
- examples;
- hosted or distributed runtime behavior;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- write-capable adapters;
- release posture changes.

## 3. Mutation Boundary Assessment

The plan correctly identifies file output as a materially different boundary from dry-run preview.

The recommended future command requires an explicit source recommendation id and an explicit output path. That is the right posture for the first repository-mutating authoring feature because it avoids hidden generation, silent scaffold changes, and accidental active governance.

The plan also preserves the existing dry-run command:

```sh
workflow-os author workflow --from-recommendation <id> --dry-run
```

This matters because users should be able to preview obligations without writing files.

## 4. CLI Shape Assessment

The proposed command shape is appropriate:

```sh
workflow-os author workflow \
  --from-recommendation <id> \
  --output workflows/drafts/<workflow-id>.workflow.yml
```

Strengths:

- keeps authoring separate from `run`;
- keeps source recommendation explicit;
- keeps output path explicit;
- keeps file-output behavior auditable;
- avoids interactive prompts in the first slice.

The first implementation should avoid adding a broad prompt-driven authoring experience until typed inputs, redaction, and conflict handling are stronger.

## 5. Output Location Assessment

The output policy is appropriately conservative.

Required future checks include:

- relative path only;
- project-local path only;
- no parent-directory traversal;
- approved draft boundary;
- `.workflow.yml` extension;
- no overwrite by default;
- no unmanaged workflow replacement.

This is strong enough for a first file-output implementation if tests prove each path-safety rule.

## 6. Draft Lifecycle Assessment

The plan correctly requires generated files to be inactive drafts.

The lifecycle posture is well scoped:

- lifecycle status `draft`;
- no fabricated owner or escalation values;
- bounded policy/evidence/check/side-effect/report obligations;
- no final WorkReport generation;
- no execution posture without future promotion.

Important non-blocking follow-up: the implementation phase should decide whether draft files are valid loaded workflow specs or separate proposal artifacts. If draft files are placed under a directory that the loader reads today, validation/runtime behavior must prove that draft lifecycle prevents accidental execution or promotion. If the current loader cannot distinguish inactive drafts safely, the first implementation should either write outside the active workflow loading path or add fail-closed validation before writing.

## 7. Conflict Handling Assessment

The plan correctly requires fail-closed conflict handling.

Covered conflicts include:

- existing output file;
- unsafe output path;
- duplicate workflow id;
- invalid workflow id;
- purpose conflict where deterministic checks exist;
- unsafe or unknown recommendation id;
- proposal helper rejection;
- raw payload requirement.

The plan’s posture is appropriate: incomplete conflict detection must be disclosed, not silently ignored, and generated output must remain inactive.

## 8. Input Policy Assessment

The plan keeps the first file-output inputs narrow:

- `--from-recommendation <id>`;
- `--output <relative-path>`;
- optional `--workflow-id <id>` only if validation is ready;
- `--dry-run` for preview mode.

It correctly defers owner values, escalation values, policy bodies, command/check input, provider identifiers, approval assignments, raw YAML snippets, and natural-language workflow bodies. Those inputs are higher-risk because they can leak private identity, imply authority, or smuggle executable behavior.

## 9. Draft Content Assessment

Allowed content is bounded to Workflow OS vocabulary and proposal obligations.

Forbidden content is comprehensive:

- raw source contents;
- raw package script command bodies;
- raw dependency values;
- raw CI logs;
- provider payloads;
- issue or pull request bodies;
- parser payloads;
- absolute private paths;
- environment values;
- credentials, authorization headers, private keys, token-like strings;
- existing agent instruction bodies.

The plan preserves the first-run safe metadata boundary and does not authorize copying repository payloads into generated YAML.

## 10. Promotion Boundary Assessment

The promotion boundary is clear and correct.

The plan states that file output must not mean promotion. Future promotion requires validation, explicit steward or delegated maintainer approval, owner/escalation completion, policy/evidence/check posture completion, side-effect posture completion, report/handoff posture completion, conflict checks, and an auditable handoff.

The first implementation must not add promotion flags.

## 11. Error Handling Assessment

The error-code candidates are appropriate for a future implementation.

The review agrees with the requirement that errors avoid leaking unsafe ids, raw paths, private directory material, source snippets, command bodies, provider payloads, parser payloads, credentials, and token-like values.

The implementation should be especially careful with output paths: even rejected paths may contain private usernames or secret-like strings.

## 12. Privacy And Redaction Assessment

The privacy posture is suitable for public repositories.

The plan requires:

- bounded safe metadata only;
- no raw payloads;
- no private absolute paths;
- secret-like id and path segment rejection;
- no user names, emails, machine names, home directories, or temp paths;
- inactive review-only drafts.

This is the right foundation for a feature that writes files into user repositories.

## 13. Test Plan Assessment

The planned tests cover the major risks:

- dry-run compatibility;
- required output path;
- absolute path rejection;
- traversal rejection;
- extension rejection;
- no overwrite;
- unsafe recommendation id rejection;
- workflow id validation;
- duplicate workflow id rejection;
- inactive lifecycle;
- authoring obligations;
- no runtime state;
- no command execution;
- no provider calls;
- no raw source/script/dependency copying;
- existing regression coverage.

Non-blocking planned-test additions:

- prove rejected output path errors do not echo private path material;
- prove draft placement cannot accidentally make the draft executable in the current loader/runtime path;
- prove generated drafts do not include comments or placeholders that look like completed approvals, evidence, checks, or owners;
- prove repeated dry-run preview remains non-mutating when `--output` is supplied with `--dry-run`.

## 14. Documentation Review

Docs correctly state:

- file output is planned, not implemented;
- file output must be explicit and opt-in;
- drafts are inactive;
- no workflow registration is performed;
- no promotion is performed;
- no commands, providers, checks, or writes beyond the explicit future draft file are authorized;
- no runtime state is created;
- examples, schemas, hosted behavior, write-capable adapters, and release posture changes remain unimplemented.

## 15. Planning Blockers

No planning blockers.

## 16. Non-Blocking Follow-Ups

- Decide whether draft files are active workflow specs with `lifecycle_status: draft` or separate proposal artifacts before implementation.
- If draft files are written under the current workflow loading path, prove the loader and executor cannot treat them as active runnable workflows.
- Add explicit non-leaking path-error tests in the implementation phase.
- Keep file output opt-in and non-overwriting until replacement semantics are separately planned.
- Keep promotion and registration out of the first file-output implementation.

## 17. Recommended Next Phase

Recommended next phase: governed workflow authoring file-output implementation.

The implementation should be narrow:

- explicit recommendation id;
- explicit relative output path;
- inactive draft only;
- path safety checks;
- no overwrite;
- conflict detection for duplicate workflow ids where possible;
- bounded generated content only;
- no registration;
- no promotion;
- no command execution;
- no provider calls;
- no runtime state;
- no schemas;
- no examples;
- no hosted behavior;
- no write-capable adapters;
- no release posture changes.

## 18. Governed Dogfood Summary

- Workflow: `dg/review`.
- Run ID: `run-1783401249484270000-2`.
- Approval ID: `approval/run-1783401249484270000-2/review-scope-approved`.
- Approval outcome: granted by delegated maintainer after the full approval handoff was emitted.
- Event summary: 39 events total; 1 approval; 0 retries; 0 escalations; terminal status `Completed`.
- Scope approved: create the maintainer review document for the governed workflow authoring file-output plan.
- Strict non-goals: no implementation, workflow file writes, registration, promotion, command execution, provider calls, runtime state, schemas, examples, hosted behavior, writes, or release posture changes.
- Out-of-kernel work disclosed: file editing, docs validation, diff check, git commit, PR creation, and PR merge remain agent/GitHub actions outside the kernel.

## 19. Validation Commands Run

- `npm run check:docs`: passed.
- `git diff --check`: passed.
- `npm run dogfood:benchmark -- phase-close run-1783401249484270000-2 --phase review`: passed.
