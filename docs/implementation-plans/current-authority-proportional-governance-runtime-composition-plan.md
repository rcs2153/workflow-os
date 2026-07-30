# Current-Authority Proportional-Governance Runtime Composition Plan

## 1. Executive Summary

The authoritative local-check CLI path currently supplies
`GovernanceWorkloadAuthorityPosture::Sufficient` as a caller-classified runtime
fact. Workflow OS already has a private, source-bound, same-call
current-authority resolver that blocks before use when grants, availability,
required context, freshness, or independent prerequisites are not satisfied.

This phase composes those accepted boundaries. One private Core path may inject
`Sufficient` authority into one single-step authoritative local-check request
only while a ready current-authority assessment is being consumed in the same
call. The caller must leave authority unclassified. Blocked and failed sources
must not invoke the executor consumer.

This is a runtime-composition prerequisite. It does not yet replace the CLI
shortcut because no reviewed production current-authority source is configured
for the CLI.

## 2. Goals

- prevent callers from preclassifying sufficient authority on the new path;
- bind actor, workflow, run, step, harness contract, and contract content;
- resolve and consume current authority in the same call;
- invoke the existing authoritative route only for ready authority;
- preserve the existing local-check, immutable-bundle, approval, disclosure,
  denial, report, and execution semantics;
- return stable non-leaking failures for blocked or failed authority; and
- prepare a later production-source integration without exporting a reusable
  authority object.

## 3. Non-Goals

- automatic approval;
- ambient or inferred authority;
- a public capability or authority object;
- public source registration;
- CLI activation or schema fields;
- arbitrary commands or additional local-check profiles;
- providers, OpenShell, sandbox execution, SideEffect execution, or writes;
- hosted execution, enterprise identity, or release changes.

## 4. Authority Boundary

The bridge accepts the existing private registered source plus:

- the immutable required-context execution binding;
- the exact required-context contract;
- evaluation time and redaction metadata;
- the existing closed local-check profile;
- one single-step authoritative request whose authority field is absent.

It validates exact execution identity and contract commitment before resolving
authority. The source invokes the executor callback only when its assessment is
`Ready`. The callback injects `Sufficient` authority into a cloned request and
immediately invokes the existing dispatcher.

The injected fact is not returned, persisted independently, serialized, or
made available for later use.

## 5. Failure Semantics

- caller-preclassified authority fails before source use;
- multiple runtime facts fail because one authority decision cannot authorize
  multiple steps in this first slice;
- binding or contract mismatch fails before source use;
- blocked authority does not invoke the executor;
- source failure does not invoke the executor;
- executor failure is returned without being rewritten as authority success;
- inconsistent callback/outcome combinations fail closed.

Errors use stable codes and do not include actor, workflow, run, step, source,
grant, context, path, command, provider, or secret-like values.

## 6. Test Plan

- ready exact authority invokes the consumer once with sufficient authority;
- blocked authority never invokes the consumer;
- source failure never invokes the consumer;
- caller-preclassified authority is rejected;
- execution-binding mismatch is rejected;
- multi-step facts are rejected;
- consumer failure is propagated safely;
- Debug and errors do not expose governed identities;
- existing current-authority, proportional-governance, local-check, executor,
  and workspace tests remain green.

## 7. Documentation And Review

Create an implementation report and focused maintainer review. Record that the
CLI still uses its compatibility shortcut and that replacing it requires a
separately reviewed production current-authority source and configuration
boundary.

## 8. Recommended Follow-Up

After review, define the first production local current-authority source for
the closed project-validation profile. Only then replace the CLI's hardcoded
authority fact. Do not treat OpenShell as an authority source; it remains a
possible future execution provider.
