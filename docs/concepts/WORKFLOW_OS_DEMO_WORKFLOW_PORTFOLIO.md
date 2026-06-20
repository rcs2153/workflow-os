# Workflow OS Demo Workflow Portfolio

Status: Concept portfolio. These demo workflows are candidates for future examples, benchmark projects, or narrative demos. They are not implemented examples today and do not authorize schemas, CLI behavior, runtime side-effect execution, write-capable adapters, hosted behavior, reasoning lineage, recursive agents, agent swarms, or release posture changes.

## 1. Purpose

Workflow OS should demonstrate more than SDLC governance. Software engineering demos are essential, but the larger thesis is broader:

```text
Probabilistic work can be useful at enterprise scale only when deterministic governance controls context, evidence, authority, side effects, approvals, auditability, and final reporting.
```

The demos below show Workflow OS as a governed work runtime across product, engineering, revenue, HR, supply chain, finance, healthcare, security, and release operations.

They should avoid positioning Workflow OS as agent swarms, recursive agents, or magic orchestration. The right framing is:

```text
Agent executes. Workflow OS governs.
```

## 2. Demo Selection Principles

Strong demos should show:

- typed workflow phases;
- deterministic validation and policy gates;
- evidence references rather than copied payloads;
- approval checkpoints;
- explicit side-effect posture;
- typed handoffs;
- local check or validation references where relevant;
- audit/event history;
- a final WorkReport that is useful to a real stakeholder.

They should not claim:

- automatic write execution;
- production nested harness execution;
- hosted/distributed runtime;
- broad live provider support;
- Level 3/4 autonomy by default;
- replacement of deterministic governance with model self-review.

## 3. Candidate Demo Workflows

### 3.1 PMM Launch Narrative Control Tower

**User/problem:** A product marketing lead needs a launch narrative, sales talk track, competitive angle, FAQ, and executive-ready risk summary from EPD inputs without manually asking product, design, or engineering for status.

**Workflow phases:**

- EPD intake from PRDs, roadmap notes, design reviews, release criteria, support readiness notes, customer discovery, and win/loss data;
- narrative hypothesis generation;
- claim-risk mapping;
- audience-specific packaging;
- launch readiness review;
- final GTM WorkReport.

**Deterministic governance points:**

- input manifest validation;
- required EPD source classes present;
- every public-facing claim cites an approved source;
- unsupported claims fail closed;
- competitor claims require dated citations;
- customer quotes require permission metadata;
- launch readiness score uses deterministic checklist thresholds.

**Evidence/citations:**

- PRD/version reference;
- design decision log;
- roadmap item;
- customer interview summary;
- win/loss note;
- support macro draft;
- pricing/packaging doc;
- legal-approved terms;
- competitive source snapshot.

**Approvals/policy gates:**

- product approval for capability claims;
- design approval for screenshots or UX claims;
- legal approval for claims and comparisons;
- sales enablement approval for objection handling;
- executive approval if readiness thresholds are not met.

**Side-effect/write posture:** Demo-safe mode creates local draft artifacts and proposed SideEffect records only: publish launch page, update sales deck, notify field, create CMS draft. No external writes occur.

**Final WorkReport value:** A launch packet with cited claims, unresolved risks, approval trail, readiness score, blocked claims, draft side effects, and exact evidence map.

**Why it shows Workflow OS:** The creative work is probabilistic. The claim boundary is deterministic.

### 3.2 Enterprise Deal Desk Exception Governor

**User/problem:** Revenue operations and finance need to evaluate a seven-figure discount, nonstandard terms, procurement concessions, security exceptions, and implementation promises under time pressure.

**Workflow phases:**

- opportunity intake;
- contract and quote parsing;
- discount and term classification;
- margin impact calculation;
- policy risk comparison;
- fallback-package generation;
- approval routing;
- final negotiation packet.

**Deterministic governance points:**

- deal-size thresholds;
- discount bands;
- margin floor;
- term deviation taxonomy;
- mutually exclusive exception categories;
- expired pricing policy rejection;
- deterministic escalation by ARR, region, data residency, and liability exposure.

**Evidence/citations:**

- CRM opportunity;
- CPQ quote;
- current price book;
- finance margin model output;
- legal fallback language;
- security questionnaire summary;
- implementation capacity note;
- approved exception precedents.

**Approvals/policy gates:**

- finance approval for margin exceptions;
- legal approval for indemnity or data terms;
- security approval for control exceptions;
- services approval for delivery commitments;
- CRO/CFO approval when cumulative risk exceeds threshold.

**Side-effect/write posture:** Read-only during analysis. Proposed writes include CRM stage update, CPQ approval annotation, contract redline package, and customer-facing concession memo. All remain proposed until approved.

**Final WorkReport value:** A decision artifact showing approved concessions, denied asks, risk-adjusted value, approvers, cited evidence, and negotiation guardrails.

### 3.3 Workforce Policy Exception Adjudicator

**User/problem:** HR, legal, and managers need to evaluate employee accommodation, relocation, leave, or role-change exceptions consistently without exposing sensitive details broadly.

**Workflow phases:**

- case intake and redaction;
- jurisdiction and policy lookup;
- precedent and fairness analysis;
- operational impact assessment;
- recommendation drafting;
- approval and escalation;
- employee-safe response package.

**Deterministic governance points:**

- jurisdiction required;
- protected-class and medical-data sensitivity classification;
- access scope validation;
- policy freshness check;
- required reviewer roles;
- deterministic insufficient-evidence failure rather than inferred facts.

**Evidence/citations:**

- HRIS case reference;
- policy section;
- jurisdictional rule summary;
- manager impact note;
- benefits/vendor eligibility reference;
- anonymized precedent;
- employee-submitted document reference with redacted summary.

**Approvals/policy gates:**

- HRBP approval;
- legal approval for high-risk jurisdictions;
- privacy approval when sensitive data is referenced;
- business owner approval for operational exceptions.

**Side-effect/write posture:** No direct HRIS mutation. Produces proposed response, proposed case-note summary, proposed accommodation plan, and proposed follow-up task.

**Final WorkReport value:** A defensible case packet with recommendation, rationale, cited policy, redacted evidence, reviewer trail, sensitivity posture, and non-disclosure boundary.

### 3.4 Supply Chain Disruption Response Governor

**User/problem:** Operations, procurement, finance, and customer success must respond to a supplier disruption affecting critical orders, contracts, inventory, penalties, and customer commitments.

**Workflow phases:**

- disruption intake;
- supplier/part/customer impact graph;
- inventory and substitution analysis;
- contract obligation review;
- customer prioritization;
- response scenario modeling;
- approval-gated execution plan;
- final incident report.

**Deterministic governance points:**

- critical-part classification;
- affected-order reconciliation;
- inventory snapshot timestamp required;
- no substitution unless qualification evidence exists;
- customer notification thresholds;
- cost/penalty calculation rules;
- duplicate incident deduplication by supplier/event key.

**Evidence/citations:**

- supplier notice;
- purchase orders;
- ERP inventory snapshot;
- customer contracts/SLA clauses;
- alternate supplier qualification record;
- logistics ETA;
- revenue-at-risk calculation;
- customer priority file.

**Approvals/policy gates:**

- procurement approval for substitute supplier;
- quality approval for substitute part;
- finance approval for expedited freight or penalty exposure;
- legal approval for customer notices;
- executive approval for allocation decisions.

**Side-effect/write posture:** Analysis remains read-only. Proposed side effects include supplier PO change, customer notice drafts, inventory reservation, expedite shipment request, and incident status update.

**Final WorkReport value:** A cross-functional incident command artifact with evidence, approvals, customer impact, pending/denied writes, and remaining risks.

### 3.5 Kernel Primitive Lifecycle Gauntlet

**User/problem:** A maintainer wants to add a serious Workflow OS kernel primitive without drifting into speculative runtime behavior.

**Workflow phases:**

- intake and scope classification;
- required context read;
- ADR/roadmap/plan crosswalk;
- deterministic project validation;
- planning approval;
- implementation handoff;
- code execution by agent or human;
- targeted test expansion;
- review phase;
- blocker-fix phase if needed;
- final report.

**Deterministic governance points:**

- spec hash and run identity at start;
- phase scope checkpoint;
- policy gate denying unsupported writes or higher autonomy;
- approval pause before implementation;
- local check references where supported;
- event log inspection;
- report-generation checkpoint.

**Evidence/citations:**

- roadmap section;
- accepted ADR or implementation plan;
- previous phase report/review;
- validation diagnostic references;
- local check result references;
- workflow/audit event IDs;
- approval decision IDs;
- final review notes.

**Approvals/policy gates:**

- planning approval before edits;
- separate approval for scope expansion;
- denied-by-default external writes;
- fail-closed missing handler behavior.

**Side-effect/write posture:** Repository edits are performed by Codex or a human after approval, not by the kernel. The kernel records or cites proposed SideEffect IDs, skipped side effects, and validation outcomes.

**Final WorkReport value:** A reusable engineering packet showing what primitive changed, which invariants were protected, what tests prove, what remains model-only, and what next phase is safe.

### 3.6 No-Write Adapter Readiness Fire Drill

**User/problem:** The project wants to know whether future GitHub/Jira/CI write-capable adapters are safe to pursue without implementing or performing writes.

**Workflow phases:**

- candidate write inventory;
- read-only evidence collection from fixtures or approved live-smoke references;
- SideEffect intent modeling;
- idempotency-key design;
- policy simulation;
- redaction review;
- approval-context design;
- dry-run plan review;
- failure/retry/replay analysis;
- readiness verdict.

**Deterministic governance points:**

- current adapters remain read-only;
- `external.write` is denied;
- every proposed mutation has capability, policy, idempotency, audit, evidence, and approval fields;
- missing fields fail closed;
- denied/skipped side-effect records are preserved as report citations.

**Evidence/citations:**

- Phase 2 read-only adapter docs;
- fixture-backed adapter telemetry references;
- live-smoke evidence references where available;
- policy decision references;
- SideEffect model references;
- audit projection references;
- security token-scope docs.

**Approvals/policy gates:**

- explicit maintainer approval for write readiness;
- policy denial for actual provider mutation;
- terminal blocker if dry-run intent becomes a real write.

**Side-effect/write posture:** Strict no-write demo. It models proposed side effects, dry-run plans, skipped writes, denied writes, idempotency keys, and required approvals.

**Final WorkReport value:** A write readiness dossier showing satisfied prerequisites, missing prerequisites, denied actions, evidence considered, and safe next implementation phases.

### 3.7 Preview Release Governance Marathon

**User/problem:** A release owner needs to prepare a Workflow OS preview release without overclaiming maturity, breaking contracts, leaking secrets, or publishing artifacts that imply unsupported production behavior.

**Workflow phases:**

- release scope freeze;
- version and schema contract audit;
- Rust/TypeScript/docs/check matrix;
- dependency and supply-chain review;
- known-limitations audit;
- read-only adapter fixture gate;
- optional approved live-smoke evidence review;
- release notes drafting;
- final approval;
- manual publication handoff.

**Deterministic governance points:**

- immutable release scope checkpoint;
- validation of project specs;
- policy gate blocking unsupported claims;
- required check matrix with cited results;
- final approval before publication side effects.

**Evidence/citations:**

- changelog;
- release checklist;
- readiness review;
- known limitations;
- dependency audit summaries;
- test/local-check result references;
- fixture CI references;
- read-only adapter telemetry evidence;
- approval decisions;
- final release notes draft.

**Approvals/policy gates:**

- release scope approval;
- live smoke approval if credentials are involved;
- final release handoff approval;
- policy denial for unsupported claims.

**Side-effect/write posture:** Kernel governs the release process but does not publish. Tagging, pushing, GitHub release creation, package publication, or announcement posting require explicit human approval and manual execution until write-capable adapters exist.

**Final WorkReport value:** An auditable release packet: checks, readiness evidence, unsupported claims, deferred work, approvals, and remaining manual publication actions.

### 3.8 Regulated Bank Credit Risk Exception Workflow

**User/problem:** A commercial bank must evaluate a high-value corporate loan exception where exposure limits, volatile cash flow, and timing pressure collide.

**Workflow phases:**

- intake and deal classification;
- borrower financial evidence collection;
- covenant/risk model execution;
- sanctions/KYC/AML evidence review;
- exception memo drafting;
- credit committee approval;
- final decision package.

**Deterministic governance points:**

- required borrower artifacts present;
- risk model version pinned;
- exposure thresholds evaluated;
- AML/KYC checks completed;
- exception rationale mapped to policy clauses;
- committee quorum required.

**Evidence/citations:**

- audited financials;
- credit bureau reports;
- internal exposure tables;
- covenant model outputs;
- KYC provider result IDs;
- policy references;
- committee minutes.

**Approvals/policy gates:**

- relationship manager submitter;
- independent risk reviewer;
- compliance gate;
- credit committee quorum;
- final delegated authority approval.

**Side-effect/write posture:** Read-only analysis and report generation until approval. Later writes could be limited to a decision packet or queued booking instruction, never silent credit booking.

**Final WorkReport value:** A defensible credit decision trail.

### 3.9 Hospital Prior Authorization Appeal Workflow

**User/problem:** A hospital revenue cycle team needs to appeal a denied prior authorization for urgent oncology treatment while preserving clinical accuracy, payer rules, HIPAA boundaries, and physician accountability.

**Workflow phases:**

- case intake;
- PHI-safe evidence inventory;
- payer denial reason classification;
- clinical guideline matching;
- appeal letter drafting;
- physician review;
- compliance review;
- submission readiness.

**Deterministic governance points:**

- patient identifiers handled under privacy rules;
- denial code captured;
- payer policy version cited;
- clinical guideline sources attached;
- missing documentation flagged;
- physician sign-off required.

**Evidence/citations:**

- denial letter;
- payer medical policy;
- clinical guideline references;
- chart excerpts;
- lab/pathology reports;
- prior treatment history;
- physician attestation.

**Approvals/policy gates:**

- PHI handling gate;
- medical necessity evidence gate;
- treating physician approval;
- compliance/legal review for sensitive cases;
- final submission approval.

**Side-effect/write posture:** Draft-only by default. No payer portal submission, chart modification, or external communication occurs without explicit approval and write-capable adapter boundaries.

**Final WorkReport value:** A complete appeal packet plus audit trail showing medical grounding, policy alignment, and approval posture.

### 3.10 Public Company Material Cyber Incident Response Workflow

**User/problem:** A public company suspects a cyber incident may be material and must coordinate containment, legal privilege, executive escalation, disclosure assessment, and regulator-ready documentation.

**Workflow phases:**

- incident intake and severity classification;
- evidence preservation;
- containment recommendation review;
- legal privilege boundary setup;
- materiality assessment;
- executive approval;
- disclosure/report package preparation;
- post-incident remediation tracking.

**Deterministic governance points:**

- incident timeline initialized;
- evidence hashes or references captured;
- severity rubric applied;
- privileged/legal channel marked;
- containment actions require approval;
- materiality checklist completed;
- disclosure deadline tracked.

**Evidence/citations:**

- SIEM alerts;
- endpoint logs;
- cloud audit trails;
- incident commander notes;
- affected asset inventory;
- forensic snapshots;
- legal analysis references;
- disclosure policy;
- board notification records.

**Approvals/policy gates:**

- incident commander validation;
- security leadership approval for containment;
- legal privilege gate;
- CFO/GC materiality review;
- CEO/board escalation if thresholds are met;
- final disclosure approval.

**Side-effect/write posture:** Strong read/prepare-first posture. Destructive security actions, customer notifications, regulator filings, or public disclosures are blocked until explicit approval and logged authorization.

**Final WorkReport value:** A board- and regulator-ready record of what happened, what evidence supports it, what decisions were made, who approved them, and what remediation remains open.

## 4. Recommended Demo Roadmap

Recommended first demo candidates:

1. **Kernel Primitive Lifecycle Gauntlet**: easiest to dogfood immediately and proves Workflow OS builds Workflow OS.
2. **PMM Launch Narrative Control Tower**: strongest cross-functional story for probabilistic work meeting deterministic governance.
3. **No-Write Adapter Readiness Fire Drill**: directly supports the write-capable adapter roadmap while preserving the no-write boundary.
4. **Preview Release Governance Marathon**: strong open-source/product maturity demo and reusable internally.

Recommended later demos:

- Enterprise Deal Desk Exception Governor;
- Public Company Material Cyber Incident Response Workflow;
- Hospital Prior Authorization Appeal Workflow;
- Supply Chain Disruption Response Governor;
- Regulated Bank Credit Risk Exception Workflow;
- Workforce Policy Exception Adjudicator.

## 5. Implementation Boundary

Each demo should be introduced through the normal Workflow OS phase format:

1. planning document;
2. plan review;
3. bounded example implementation;
4. implementation report;
5. maintainer review.

Do not implement demo workflows by weakening runtime boundaries. The demos should make unsupported behavior explicit through proposed side effects, skipped side effects, denied writes, missing citations, known limitations, and final WorkReports.
