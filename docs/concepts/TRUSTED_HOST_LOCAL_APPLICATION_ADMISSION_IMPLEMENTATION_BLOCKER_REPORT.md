# Trusted-Host Local Application Admission Implementation Blocker Report

## 1. Executive Summary

The trusted-host local application admission implementation cannot safely
begin at the accepted runtime boundary because Core cannot yet represent the
required explicit zero-obligation required-context contract.

The accepted plan permits complete empty current-authority inventories only
after Core proves that the immutable required-context contract and selected
closed profile explicitly require zero capability grants, zero capability
availability facts, and zero governed-context references. The canonical
`RequiredContextContractBinding` currently rejects an empty requirement set,
and `consume_required_context` rejects an empty projection set. The planned
proof therefore cannot be constructed or consumed through the canonical
model.

No runtime wiring, binary target, authority source, prepared session, handler
execution, state mutation, provider access, or release behavior was added.

## 2. Approved Scope Attempted

The governed implementation phase authorized inspection and implementation of
one unpublished path:

```text
workflow-local-host binary
  -> Core admission over one SQLite backend
  -> explicit zero-authority docs-check contract
  -> opaque prepared pair
  -> LocalHostPreparedOperation
  -> synchronous bounded result
```

Implementation inspection covered the accepted admission plan, the focused
blocker-fix review, the required-context model, registered current-authority
source, explicit docs-check profile, SQLite operational opening, private
trusted-host preparation SPI, and unpublished local-host package.

## 3. Blocking Invariant

The accepted review requires an explicit Core-owned statement that the exact
admitted docs-check operation needs no current-authority facts. It explicitly
forbids deriving that statement from:

- an absent contract;
- an absent registry;
- an empty caller-supplied vector;
- missing integrations;
- an optional fabricated context requirement; or
- a bypass around registered current-authority resolution.

The current model cannot state that invariant:

- `RequiredContextContractBinding::new` rejects zero requirements with
  `required_context.contract.requirements_empty`;
- deserialized bindings run the same validation;
- `consume_required_context` rejects zero projections with
  `required_context.consumption.projections_empty`; and
- existing registered-authority fixtures use non-empty context requirements
  and therefore do not prove the planned zero-authority posture.

## 4. Unsafe Workarounds Rejected

The implementation did not:

- fabricate a governed-context target only to satisfy non-empty validation;
- mark a requirement optional and reinterpret its gap as zero authority;
- skip required-context consumption;
- treat missing fields as an explicit declaration;
- expose caller-authored authority inventories;
- reuse test-only authority issuers as a production source;
- construct the opaque prepared pair outside Core; or
- wire a binary that could only operate through synthetic test state.

Each workaround would make the application appear governed while weakening
the exact authority proof required by the accepted review.

## 5. Required Prerequisite

The next phase must define the smallest canonical representation for an
explicit zero-required-context contract. It must answer, before implementation:

- how zero requirements are distinguished from missing or incomplete
  requirements;
- how the zero posture participates in content hashing, Debug, serde, and
  fail-closed deserialization;
- how consumption proves satisfaction with zero projections without allowing
  ordinary non-empty contracts to bypass projection validation;
- how the private docs-check profile binds the explicit zero posture into its
  command-contract and execution commitments;
- how registered current authority verifies complete empty inventories for
  that exact binding; and
- how non-empty, unknown, stale, incomplete, or conflicting facts remain
  blocked.

The prerequisite should remain model-only until focused review accepts the
contract. The admission vertical slice should then resume without broadening
profiles or runtime surfaces.

## 6. Scope Explicitly Not Completed

- No `workflow-local-host` binary was added.
- No Core admission request or outcome was added.
- No SQLite run admission was added.
- No current-authority source was registered in production.
- No opaque prepared session was issued to local host.
- No handler or local check was executed.
- No released CLI, SDK, schema, example, hosted path, discovery, polling,
  scheduling, provider access, provider mutation, OpenShell integration,
  nested harness execution, or release posture changed.

## 7. Validation

This blocker report requires documentation validation and diff validation.
Rust validation is not a substitute for the missing model contract and no
Rust files changed in this blocked implementation phase.

## 8. Governed Phase Record

- workflow: `dg/implement`
- run: `run-1791391386463224000-2`
- approval: `approval/run-1791391386463224000-2/implementation-approved`
- presentation: `presentation/e62200b34f8b439d`
- presentation hash:
  `e62200b34f8b439d1452c1d057351a803b6932942df7bbb35b0965d084d15496`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- phase outcome: blocked before runtime edits by a canonical-model prerequisite
- out-of-kernel work: source inspection and blocker documentation

## 9. Recommended Next Phase

Plan and review an explicit zero-required-context contract model and
consumption semantics. Do not resume trusted-host admission implementation
until that model can prove an exact zero-obligation posture without absence
inference or fabricated context.
