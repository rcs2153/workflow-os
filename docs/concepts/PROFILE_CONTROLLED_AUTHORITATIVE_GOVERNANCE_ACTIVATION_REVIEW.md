# Profile-Controlled Authoritative Governance Activation Review

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups.**

The implementation turns the accepted authoritative quiet-success preview into
one explicit, validated project contract without creating a second governance
engine. The declaration activates the existing Core-owned route selection,
closed local-check authority, approval-presentation proof, reassessment, and
in-memory WorkReport path.

Two compatibility blockers found during implementation and review were fixed
before acceptance:

- absent activation initially changed legacy immutable bundle hashes; and
- undeclared invalid projects were briefly preflighted through the
  authoritative CLI error surface.

Projects without the declaration now retain their existing serialized identity
and ordinary run validation behavior.

## 2. Scope Verification

The phase stayed within its approved activation scope.

It added:

- one optional typed project declaration;
- synchronized Rust, JSON Schema, and TypeScript SDK contracts;
- automatic selection of the existing authoritative `run` path when declared;
- durable selection of the matching authoritative `approve` path;
- immutable declaration and project-manifest identity binding; and
- bounded first-run disclosure.

It did not add:

- scaffold defaults or inferred activation;
- arbitrary commands or ambient executable discovery;
- additional governance or local-check profile families;
- automatic or model self-approval;
- report persistence or artifacts;
- providers, OpenShell, credentials, network access, or writes;
- enterprise policy administration, hosted controls, or RBAC;
- workflow generation, reasoning lineage, nested harness execution, recursive
  agents, or agent swarms; or
- release posture changes.

## 3. Public Contract Assessment

`ProjectManifest` now accepts optional `governance` configuration. The only
supported v0 activation is:

```yaml
governance:
  authoritative_execution:
    profile: observe_and_report
    local_check_profile: workflow_os_project_validation
```

The contract is domain-neutral within the current local kernel boundary. It
selects an accepted minimum profile and closed execution authority; it does not
let the project choose a route, provide a check result, lower policy or
approval requirements, or supply a command.

The declaration remains optional. This is important for compatibility and for
the product's current experimental posture.

## 4. Parser, Schema, And SDK Assessment

Rust parsing, JSON Schema, and the TypeScript SDK agree on field names and the
single supported value pair.

Parser behavior fails closed for:

- missing required fields;
- unsupported profile values;
- unsupported local-check profile values;
- unsupported combinations;
- unknown fields; and
- existing secret-like spec values.

Errors use stable bounded codes and do not echo raw manifest content or caller
values. The semantic validation in Rust appropriately goes beyond the JSON
shape so future enum vocabulary cannot silently become executable authority.

## 5. Runtime Routing Assessment

The CLI inspects the validated project declaration and reuses the existing
authoritative execution path. Core remains responsible for choosing quiet
proceed, visible proceed, approval required, or denial.

For approval resume, the CLI derives the route from the durable immutable run
bundle rather than trusting a repeated flag or the current project's desired
route. It then reloads and fully validates the current project before
reassessment.

The explicit `--authoritative-governance` flag remains as a compatibility
preview. It does not override an immutable declaration mismatch.

## 6. Immutable Run And Approval-Resume Assessment

Declaration-bound runs store:

- the validated authoritative-execution configuration; and
- the canonical project-manifest content hash.

That activation contributes to the immutable bundle root hash when present.
Approval resume requires the current validated project to reproduce the exact
immutable execution posture. Removing the declaration, changing its values, or
changing project-manifest content fails closed before gated work resumes.

Older and undeclared bundles omit the optional activation field. Their root
hash input remains byte-for-byte compatible with the accepted historical
shape.

This is the correct conservative boundary. It prevents an approval obtained
for one project contract from authorizing work after that contract changes.

## 7. Compatibility Blockers And Fixes

### Legacy Immutable Hash Compatibility

The first implementation included explicit absence markers in immutable hash
input. Full workspace tests caught changes to known legacy root-hash vectors.

The fix hashes authoritative activation fields only when an activation exists.
Undeclared runs therefore retain their historical identity, while declared
runs bind the new posture.

### Undeclared Run Validation Compatibility

Review found that dispatch initially called full authoritative project
validation before determining whether a declaration existed. That could
replace the ordinary `executor.project.invalid` path with an authoritative CLI
error for an undeclared invalid project.

The dispatch probe now only reads a successfully loaded declaration. An
undeclared invalid project continues through ordinary execution and its
existing validation semantics. A focused regression test verifies that the
authoritative error namespace is not introduced.

No compatibility blockers remain.

## 8. Approval Presentation And Disclosure Assessment

Declaration-bound approval handoffs omit the now-unnecessary compatibility
flag from the exact next action. Compatibility-flag runs retain it.

First-run verbose and JSON output disclose:

- whether authoritative execution is declared;
- whether the declaration is supported and enforced;
- the selected profile; and
- the closed local-check profile.

The default concise first-run surface remains unchanged. This is consistent
with the product feedback that useful governance detail should remain
available without making the first five minutes unnecessarily dense.

## 9. Privacy And Error Assessment

The immutable activation's Debug implementation discloses configuration and
presence but redacts the project-manifest hash. Existing immutable posture
Debug output exposes only an activation-present boolean.

The implementation stores no:

- raw manifest content;
- source content;
- command output;
- environment values;
- credentials or tokens;
- provider payloads; or
- approval reasons in activation state.

Serialized immutable state necessarily contains the bounded manifest content
hash because it is an integrity input. Human and Debug output do not expose
that value.

## 10. Test Quality Assessment

Focused coverage includes:

- valid declaration parsing;
- incomplete and unsupported declaration rejection;
- TypeScript SDK emission;
- declaration-driven quiet execution without a flag;
- declaration-driven approval and proof-enforced resume without a flag;
- removed declaration rejection;
- unchanged declaration with changed manifest identity rejection;
- first-run verbose and JSON disclosure;
- explicit compatibility-flag behavior;
- undeclared invalid-project validation compatibility; and
- existing authoritative, immutable-bundle, executor, report, and CLI
  behavior through the workspace suite.

The full suite caught the initial legacy-hash regression, demonstrating that
the compatibility vectors are meaningful rather than ceremonial.

## 11. Documentation Assessment

The roadmap, implementation plan, CLI references, project-loader contract,
current product contract, implementation report, and this review agree that:

- project-controlled authoritative activation is implemented;
- only one closed local profile is supported;
- Core still derives the route;
- reports remain in memory;
- activation is not inferred or scaffolded by default; and
- providers, OpenShell, artifacts, writes, hosted controls, and broader
  autonomy remain unsupported.

The latest external user review aligns with this phase. It identifies
proportional governance and quiet success as the next major product value:
reduce ceremony for low-risk work while preserving the evidence trail. This
activation contract is a prerequisite for that behavior, not a complete
quiet-success product by itself.

## 12. Blockers

None remain.

The immutable-hash and undeclared-validation compatibility blockers were fixed
and covered before acceptance.

## 13. Non-Blocking Follow-Ups

- Retire or reposition the compatibility flag only after declaration-driven
  behavior has accumulated real usage evidence.
- Preserve the distinction between project-controlled local minimums and
  future enterprise-controlled tightening sources.
- Continue reducing low-risk operator ceremony through the accepted
  proportional-governance roadmap rather than adding more manual route
  configuration.
- Track the Node 24 integration-check sharp edge and other first-run polish
  separately; they do not alter this runtime contract.
- Keep OpenShell, if pursued, behind a separately reviewed optional execution
  provider boundary and threat model.

## 14. Recommended Next Phase

Return to the current roadmap after merge and select the next incomplete
runtime-composition phase that advances proportional governance and quiet
success. Do not broaden provider mutations or create a Workflow OS runtime
fork.

The next phase should continue composing accepted primitives into runtime
behavior, with evidence completeness and low governance friction measured
together.

## 15. Governed Review Record

- workflow: `dg/review`
- run: `run-1785122395767230000-2`
- approval: `approval/run-1785122395767230000-2/review-scope-approved`
- presentation: `presentation/982fb1cec35f2777`
- approval outcome: granted by delegated maintainer through
  presentation-proof enforcement
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval presentation enforcement: proof enforced
- validation: `cargo fmt --all --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, `npm run check:docs`,
  `npm run check:integrations` under Node 20, focused compatibility tests, and
  `git diff --check` passed
- skipped checks: opt-in live adapter, provider, and local-check smoke tests
  remained skipped by their existing environment-gated contracts
- report posture: the implementation report and this review are persisted in
  the repository; no runtime WorkReport artifact was generated
- out-of-kernel work: code and documentation inspection, compatibility blocker
  fixes, test execution, and review authoring
- kernel boundary: the kernel governed scope and approval; it did not inspect
  code, edit files, execute validation, or perform git and pull-request
  actions
