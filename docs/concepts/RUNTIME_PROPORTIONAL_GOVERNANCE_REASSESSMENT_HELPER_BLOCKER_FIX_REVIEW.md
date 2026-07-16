# Runtime Proportional-Governance Reassessment Helper Blocker Fix Review

## 1. Executive Verdict

**Blocker fixed; proceed to durable assessment-binding model and event
vocabulary.**

The exact step-fact boundary now carries explicit runtime escalation, composes
it monotonically with static declarations on both governance axes, and binds
decision-relevant escalation into the existing per-step and aggregate
fingerprints. The missing immutable-definition boundary tests are also present.

## 2. Scope Verification

The fix stayed within the approved pure-helper boundary. It added no executor
integration, durable binding, event emission, persistence, schema, CLI, UI,
provider behavior, writes, approval redesign, enterprise behavior, or default
runtime enforcement.

## 3. Runtime Escalation Assessment

`StepGovernanceRuntimeFacts` now accepts one optional validated
`GovernancePostureRequirement` through `with_runtime_escalation`. The shared
resolved-definition derivation combines the static escalation declaration and
the explicit runtime fact independently on execution and disclosure axes.

The composition selects the stricter ordered requirement on each axis. A quiet
runtime fact cannot weaken a visible static requirement. Visible, approval,
denied, and unsupported requirements can only hold or raise posture.

## 4. Fingerprint And Invalidation Assessment

Runtime escalation flows into the accepted workload-assessment input and its
versioned input fingerprint. The aggregate assessment-set fingerprint binds the
ordered per-step fingerprints, so a decision-relevant runtime escalation
invalidates the aggregate result. Explicit quiet escalation remains equivalent
to no additional escalation.

The new definition-boundary test proves that changing a referenced immutable
workflow definition changes the aggregate fingerprint, while adding an
unreferenced policy outside the bundle does not cause churn. The stable known
vector and fixed-width framing test remain intact.

## 5. Exact Fact And Source-Of-Truth Boundary

The helper still resolves static workflow, skill, and policy definitions only
from the validated stored immutable run bundle. It requires exactly one fact
record per ordered step and fails closed on missing, duplicate, extra, or
mismatched records. Mutable project files remain outside the assessment source
of truth.

The helper fingerprints caller-supplied facts but does not independently prove
their freshness. That limitation remains explicit and belongs to the next
durable binding and later executor phases.

## 6. Privacy And Error Assessment

The fix adds only typed bounded posture. It does not add raw evidence, check
output, definitions, source content, paths, commands, provider payloads,
environment values, credentials, or free-form explanations. Existing redacted
`Debug` output and stable bounded errors remain unchanged.

## 7. Test Quality Assessment

Focused coverage now proves:

- visible, approval, and denied runtime escalation raise posture and invalidate
  the aggregate fingerprint;
- explicit quiet escalation does not create false churn;
- a relevant immutable definition change invalidates the result;
- an unreferenced external definition does not invalidate the result;
- exact runtime-fact validation, immutable-source behavior, deterministic
  ordering, known-vector stability, framing safety, and non-leakage continue to
  pass.

A future test may combine a visible static escalation declaration with a quiet
explicit runtime fact to make the non-weakening property direct rather than
structurally inferred. This is non-blocking because the composition function is
axis-wise monotonic and the accepted selector regression suite passes.

## 8. Validation Assessment

Passing validation:

- focused immutable-bundle reassessment tests;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`;
- `git diff --check`.

No required check was skipped. Opt-in live provider tests remain intentionally
outside this pure local helper phase.

## 9. Blockers

None.

## 10. Non-Blocking Follow-Ups

- Add a direct static-visible plus runtime-quiet monotonicity regression when
  the next assessment test surface is touched.
- Define trusted runtime-fact references and validity windows with durable
  assessment binding.
- Keep serialized assessment bindings classified as sensitive operational
  metadata.

## 11. Recommended Next Phase

Implement the durable assessment-binding model and additive event vocabulary
only. Bind algorithm, aggregate fingerprint, immutable bundle root, run and
workflow identity, and bounded posture without adding executor enforcement,
retry/resume behavior, schema, CLI, UI, provider calls, writes, or default
runtime behavior.

## 12. Governed Review Record

- Dogfood workflow: `dg/review`
- Run ID: `run-1784187625502249000-2`
- Approval ID:
  `approval/run-1784187625502249000-2/review-scope-approved`
- Approval outcome: granted with persisted presentation proof
- Presentation ID: `presentation/2bbb9fddf144e0f8`
- Validation summary: all required local checks passed
- Out-of-kernel work: code and test inspection, diff analysis, review drafting,
  documentation validation, roadmap status update, and phase reporting
