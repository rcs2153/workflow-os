# Trusted-Host Live-Execution Security Contract Model Blocker Fix Review

## 1. Executive Verdict

**Blocker fixed; proceed to deterministic build-input discovery planning.**

The narrow fix closes every blocker from the original model review without
adding a live process or widening the runtime. The complete evaluated security
contract is now identified by one closed, versioned strict profile committed
into the canonical requirements fingerprint; typed roots cannot reuse one
commitment; assessment orchestration is crate-private; and requirements serde
reconstructs the derived fingerprint from authoritative fields.

## 2. Scope Verification

The fix remained inside the approved model boundary. It changed Core model
vocabulary, canonicalization, serde behavior, crate exports, focused tests,
roadmap state, and phase documentation only.

It did not add input discovery, provider selection or composition, OpenShell
integration, a process runner, shell or Cargo execution, handler registration,
persistence, runtime events, reports, artifacts, CLI behavior, configuration,
schemas, SDK changes, examples, dependencies, provider mutations, credentials,
writes, hosted expansion, or release changes.

## 3. Original Blocker Resolution

The original review identified four required corrections:

1. commit every decision-relevant requirement into the accepted contract;
2. reject root commitment aliasing across typed roles;
3. remove premature public assessment orchestration; and
4. add focused fingerprint, serde, API-surface, and alias regressions while
   correcting documentation.

All four are resolved. The fix did not erase or rewrite the original review;
it adds a separate fix-forward report and this acceptance review.

## 4. Requirements Contract Assessment

`TrustedHostLiveExecutionSecurityProfile::StrictBuildExecutionV1` is a closed,
versioned identity for the full set of controls currently evaluated by the
private assessor. Its documented semantics include enforce mode, filesystem,
process, network, protected-root, and resource-budget enforcement,
default-deny filesystem and network posture, host-home, credential-store, and
ambient-temp exclusion, complete observation, zero allowed network events,
terminal process-tree proof, quiescent outputs, retained-artifact commitment,
and completed cleanup.

The separate denied-egress-probe requirement remains explicit because the
accepted architecture requires that proof only for selected live-proof
contexts, not every ordinary invocation. Root roles, accesses, bindings, and
invocation scoping also remain explicit rather than being hidden inside the
profile.

The single-profile approach is appropriately smaller than adding speculative
independent control switches. A semantic change requires a new profile variant
and therefore a changed fingerprint.

## 5. Fingerprint And Serde Assessment

The domain-separated requirements fingerprint now includes the strict-profile
tag along with exact run-bundle, command-contract, attempt, request,
provider-selection, policy, probe, and canonical root facts. A fixed-vector
test protects the reviewed encoding, and root declaration order remains
canonical.

Requirements serialization omits the derived fingerprint. Deserialization
re-enters the validated definition, canonicalizes roots, and recomputes the
fingerprint. This makes the authoritative wire shape unambiguous rather than
silently accepting a caller-supplied derived value.

Unknown profile values fail closed with a value-independent error. The focused
test verifies that a secret-like unknown value is not echoed.

## 6. Root Commitment Assessment

Observation canonicalization now compares all commitment strings and rejects
any duplicate before assessment. Because the invariant is global, it prevents
protected-input/write-root, write-root/write-root, and every other cross-role
aliasing combination. The stable error contains no rejected commitment.

The focused regression currently uses one representative duplicate pair. That
is sufficient to prove the implementation's role-independent set invariant,
but named protected/write and write/write cases would improve future test
readability. This is non-blocking.

## 7. Public Surface And Authority Assessment

`assess_trusted_host_live_execution_security` is no longer re-exported from the
crate root and is crate-private. Its internal tests moved beside the module so
the pure comparison remains testable without creating public compatibility
surface. A compile-fail documentation test proves an external consumer cannot
name the function.

Public construction vocabulary remains non-authorizing. No public function can
start, dispatch, register, persist, approve, retry, or mutate an execution from
these values. A satisfied assessment, once composed later, remains evidence
for one exact attempt rather than reusable authority.

## 8. Privacy And Error Assessment

The fix stores only closed profile vocabulary and existing commitments. It
adds no paths, source or dependency contents, commands, output, environment
values, credentials, endpoints, provider payloads, or sandbox logs. Debug
continues to redact provider identity, references, commitments, and
fingerprints. New failures use stable, value-independent messages.

## 9. Test Quality Assessment

The focused suite now contains 15 tests and covers:

- strict-profile access and serialized vocabulary;
- fixed-vector requirements fingerprint stability;
- deterministic root-order canonicalization;
- derived-fingerprint omission and reconstruction;
- unknown-profile failure without value leakage;
- duplicate root commitment rejection without leakage;
- external compile failure for the private assessment function; and
- all prior satisfaction, denial, reconciliation, substitution, bounds,
  process-tree, policy, observation, egress, and transient-write behavior.

The full workspace and integration suites pass. The moved internal tests are a
deliberate API-surface decision, not a reduction in behavioral coverage.

## 10. Documentation Assessment

The implementation report preserves its original phase record and adds a
fix-forward note. The blocker-fix report explains the chosen strict-profile
approach, root uniqueness, serde boundary, public-surface narrowing, tests,
validation, and remaining live-process blockers. The plan status and roadmap
accurately require this review before further work.

No document claims that discovery, containment, provider composition, or live
execution now exists.

## 11. Remaining Blockers To Live Execution

No blocker remains in this model-only fix.

The following remain blockers to any live process or provider composition:

- complete deterministic build-time input discovery;
- operating-system object binding and TOCTOU control;
- enforced filesystem, network, process-tree, and resource boundaries;
- independently observed effective controls;
- transient-write and cleanup proof; and
- focused acceptance of the complete provider boundary.

## 12. Non-Blocking Follow-Ups

- Add named protected-input/write-root and write-root/write-root alias cases if
  the root model changes or gains a future alias class.
- Keep assessment private until an approved composition consumer proves the
  minimum required API.
- Add a new versioned profile instead of changing
  `StrictBuildExecutionV1` semantics in place.

## 13. Recommended Next Phase

Plan the **deterministic trusted-host build-input discovery proof** described by
the accepted Cargo clippy security sequence.

The plan should define how fixtures and the Workflow OS workspace can produce a
bounded, payload-free, truthful input manifest without executing Cargo metadata,
build scripts, procedural macros, compilers, linkers, or provider code. It must
fail closed for dynamic or unobservable inputs. Provider suitability evaluation
comes only after that proof is implemented and reviewed.

## 14. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- focused trusted-host live-security tests: passed, 15 tests.
- `cargo test --workspace`: passed.
- `npm run check`: passed under the repository Node 20 toolchain.
- `npm run check:integrations`: passed under the repository Node 20 toolchain.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 15. Governed Review Record

- Workflow ID: `dg/review`.
- Run ID: `run-1792817545437591000-2`.
- Approval ID:
  `approval/run-1792817545437591000-2/review-scope-approved`.
- Approval-presentation ID: `presentation/76e811d20de594b7`.
- Approval-presentation content hash:
  `76e811d20de594b794d48c7853b9c112b3f78b1beb9db30779d31b4b4839d4dc`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact review-only scope.
- Phase status: `Completed`.
- Event summary: 39 events, including one approval request, one approval grant,
  six scheduled steps, six successful skill invocations, zero retries, and zero
  escalations. Approval presentation enforcement was proof-enforced and its
  event marker is present.
- Validation summary: the full Rust, Node, integration, docs, formatting, and
  diff-check matrix passed.
- Out-of-kernel work: the delegated maintainer inspected the merged diff,
  original plan, original review, blocker-fix report, model, exports, and tests;
  authored this review and roadmap update; and ran validation. The kernel
  governed scope and approval but did not edit files, run checks, or perform
  git or PR operations.
