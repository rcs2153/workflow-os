import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { copyFileSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { mkdtemp, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import test from "node:test";
import { join, resolve } from "node:path";

import {
  buildApprovalPresentationApproveCommand,
  buildApprovalPresentationPersistCommand,
  buildWorkflowCommand,
  containsSecretLike,
  discoverApprovalPresentationProof,
  displayCommand,
  parseArgs,
} from "./self-governed-benchmark.mjs";

const repoRoot = resolve(new URL("..", import.meta.url).pathname);
const helperScript = join(repoRoot, "scripts", "self-governed-benchmark.mjs");
const nodeBin = process.execPath;

test("start dry-run builds expected dogfood command shape", () => {
  const parsed = parseArgs([
    "start",
    "--dry-run",
    "--no-build",
    "--state-dir",
    "/tmp/workflow-os-self-governance-state",
    "--run-id",
    "run/dogfood-test",
  ]);
  const command = buildWorkflowCommand(parsed, "target/debug/workflow-os");

  assert.deepEqual(command, [
    "target/debug/workflow-os",
    "--project-dir",
    join(repoRoot, "dogfood", "workflow-os-self-governance"),
    "--state-dir",
    "/tmp/workflow-os-self-governance-state",
    "--mock-all-local-skills",
    "run",
    "dg/d",
    "--run-id",
    "run/dogfood-test",
  ]);
});

test("phase-start dry-run maps implementation phase to dg implement workflow", () => {
  const parsed = parseArgs([
    "phase-start",
    "--phase",
    "implementation",
    "--dry-run",
    "--no-build",
    "--state-dir",
    "/tmp/workflow-os-governed-phase-state",
    "--run-id",
    "run/governed-phase-test",
  ]);
  const command = buildWorkflowCommand(parsed, "target/debug/workflow-os");

  assert.deepEqual(command, [
    "target/debug/workflow-os",
    "--project-dir",
    join(repoRoot, "dogfood", "workflow-os-self-governance"),
    "--state-dir",
    "/tmp/workflow-os-governed-phase-state",
    "--mock-all-local-skills",
    "run",
    "dg/implement",
    "--run-id",
    "run/governed-phase-test",
  ]);
});

test("phase-start dry-run prints explicit approval boundary without approving", () => {
  const result = runHelper([
    "phase-start",
    "--phase",
    "review",
    "--dry-run",
    "--no-build",
    "--state-dir",
    "/tmp/workflow-os-governed-phase-state",
  ]);

  assert.equal(result.status, 0);
  assert.match(result.stdout, /workflow_id: dg\/review/);
  assert.match(result.stdout, /approval_policy: explicit_human_approval_required/);
  assert.match(result.stdout, /approval_outcome: not_requested/);
  assert.match(result.stdout, /approval_reason: approved-review-phase/);
  assert.match(result.stdout, /runner_boundary: governance coordination only/);
  assert.match(result.stdout, /approval_handoff_required: true/);
  assert.match(result.stdout, /approval_handoff:/);
  assert.match(result.stdout, /  workflow_id: dg\/review/);
  assert.match(result.stdout, /  run_id: <run-id-after-start>/);
  assert.match(result.stdout, /  approval_id: <approval-id-after-start>/);
  assert.match(result.stdout, /  approval_presentation_proof: not_persisted/);
  assert.match(result.stdout, /  status: NotRequestedDryRun/);
  assert.match(result.stdout, /  approval_reason: approved-review-phase/);
  assert.match(result.stdout, /  work_summary: <work-summary-required>/);
  assert.match(result.stdout, /  approved_scope: <approved-scope-required>/);
  assert.match(result.stdout, /  strict_non_goals: .*hidden approvals/);
  assert.match(result.stdout, /  expected_touched_surfaces: <expected-touched-surfaces-required>/);
  assert.match(result.stdout, /  validation_required: <validation-required>/);
  assert.match(result.stdout, /  why_now: <why-now-required>/);
  assert.match(result.stdout, /  approval_allows: proceed with the maintainer review phase only/);
  assert.match(result.stdout, /  approval_does_not_allow: .*hidden approvals/);
  assert.match(result.stdout, /  next_action_after_approval: run phase-start without --dry-run/);
  assert.match(result.stdout, /  redaction_note: approval command display redacts the approval reason/);
  assert.match(result.stdout, /  agent_instruction: relay this complete approval_handoff block/);
  assert.match(result.stdout, /copy_safe_approval_request_required: true/);
  assert.match(result.stdout, /copy_safe_approval_request:/);
  assert.match(result.stdout, /Governed approval required before proceeding\./);
  assert.match(result.stdout, /    approval_handoff:/);
  assert.match(result.stdout, /      workflow_id: dg\/review/);
  assert.match(result.stdout, /      run_id: <run-id-after-start>/);
  assert.match(result.stdout, /      approval_id: <approval-id-after-start>/);
  assert.match(result.stdout, /      approval_presentation_proof: not_persisted/);
  assert.match(
    result.stdout,
    /agent_instruction: preserve this complete block in the final approval request/,
  );
  assert.match(result.stdout, /Please approve this governed phase if you want me to proceed\./);
  assert.match(result.stdout, /end: copy_safe_approval_request/);
  assert.doesNotMatch(result.stdout, / approve .*--reason /);
  assert.doesNotMatch(result.stdout, /approval_presentation_persisted: true/);
});

test("phase-start builds bounded approval presentation persistence command", () => {
  const parsed = parseArgs([
    "phase-start",
    "--phase",
    "implementation",
    "--state-dir",
    "/tmp/workflow-os-governed-phase-state",
    "--work-summary",
    "Implement dogfood runner presentation proof persistence.",
    "--approved-scope",
    "Update helper, focused tests, and docs only.",
    "--strict-non-goals",
    "No automatic approvals, writes, schemas, or release posture changes.",
    "--expected-touched-surfaces",
    "scripts/self-governed-benchmark.mjs, tests, docs.",
    "--validation-required",
    "npm run test:dogfood-helper; npm run check:docs.",
    "--why-now",
    "Accepted persistence plan requires durable presentation proof.",
  ]);
  const command = buildApprovalPresentationPersistCommand(
    "target/debug/workflow-os",
    parsed.options,
    "implementation",
    {
      runId: "run/dogfood-test",
      approvalId: "approval/run-dogfood-test/implementation-approved",
    },
    {
      workSummary: parsed.options.workSummary,
      approvedScope: parsed.options.approvedScope,
      strictNonGoals: parsed.options.strictNonGoals,
      expectedTouchedSurfaces: parsed.options.expectedTouchedSurfaces,
      validationRequired: parsed.options.validationRequired,
      whyNow: parsed.options.whyNow,
    },
  );

  assert.deepEqual(command, [
    "target/debug/workflow-os",
    "--project-dir",
    join(repoRoot, "dogfood", "workflow-os-self-governance"),
    "--state-dir",
    "/tmp/workflow-os-governed-phase-state",
    "dogfood",
    "approval-presentation",
    "persist",
    "--run-id",
    "run/dogfood-test",
    "--approval-id",
    "approval/run-dogfood-test/implementation-approved",
    "--phase",
    "implementation",
    "--work-summary",
    "Implement dogfood runner presentation proof persistence.",
    "--approved-scope",
    "Update helper, focused tests, and docs only.",
    "--strict-non-goals",
    "No automatic approvals, writes, schemas, or release posture changes.",
    "--expected-touched-surfaces",
    "scripts/self-governed-benchmark.mjs, tests, docs.",
    "--validation-required",
    "npm run test:dogfood-helper; npm run check:docs.",
    "--why-now",
    "Accepted persistence plan requires durable presentation proof.",
    "--presented-by",
    "user/dogfood-reviewer",
  ]);
});

test("phase-start builds proof-enforced approval command with presentation id", () => {
  const parsed = parseArgs([
    "phase-start",
    "--phase",
    "implementation",
    "--state-dir",
    "/tmp/workflow-os-governed-phase-state",
  ]);
  const command = buildApprovalPresentationApproveCommand(
    "target/debug/workflow-os",
    parsed.options,
    {
      runId: "run/dogfood-test",
      approvalId: "approval/run-dogfood-test/implementation-approved",
    },
    {
      presentationId: "presentation/1234abcd5678ef90",
      contentHash: "1234abcd",
    },
    "approved-implementation-phase",
  );

  assert.deepEqual(command, [
    "target/debug/workflow-os",
    "--project-dir",
    join(repoRoot, "dogfood", "workflow-os-self-governance"),
    "--state-dir",
    "/tmp/workflow-os-governed-phase-state",
    "--mock-all-local-skills",
    "dogfood",
    "approval-presentation",
    "approve",
    "--run-id",
    "run/dogfood-test",
    "--approval-id",
    "approval/run-dogfood-test/implementation-approved",
    "--presentation-id",
    "presentation/1234abcd5678ef90",
    "--max-presentation-age-ms",
    "86400000",
    "--actor",
    "user/dogfood-reviewer",
    "--reason",
    "approved-implementation-phase",
  ]);
});

test("repo agent instructions require preserving approval handoff blocks", () => {
  const instructions = readFileSync(join(repoRoot, "AGENTS.md"), "utf8");

  assert.match(instructions, /approval_handoff_required: true/);
  assert.match(instructions, /complete `approval_handoff` block/);
  assert.match(instructions, /Do not replace it with vague prose/);
  assert.match(instructions, /final response must include the complete handoff block/);
  assert.match(instructions, /Do not ask for approval from an underspecified governed phase handoff/);
  assert.match(instructions, /concrete work being approved/);
  assert.match(instructions, /copy_safe_approval_request_required: true/);
  assert.match(instructions, /copy-safe approval request/);
  assert.match(instructions, /final approval request/);
});

test("phase-start live mode fails closed when work context is missing", () => {
  const result = runHelper([
    "phase-start",
    "--phase",
    "implementation",
    "--no-build",
    "--state-dir",
    "/tmp/workflow-os-governed-phase-state",
  ]);

  assert.notEqual(result.status, 0);
  assert.match(result.stderr, /dogfood\.helper\.work_context_missing/);
  assert.doesNotMatch(result.stdout, /approval_handoff_required: true/);
});

test("phase-start dry-run prints supplied bounded work context before approval handoff", () => {
  const result = runHelper([
    "phase-start",
    "--phase",
    "implementation",
    "--dry-run",
    "--no-build",
    "--work-summary",
    "Implement governed approval work-summary handoff fields.",
    "--approved-scope",
    "Update repo-local dogfood phase runner and focused tests only.",
    "--strict-non-goals",
    "No runtime approval semantic changes, schemas, writes, or artifacts.",
    "--expected-touched-surfaces",
    "scripts/self-governed-benchmark.mjs, tests, docs.",
    "--validation-required",
    "npm run test:dogfood-helper; npm run check:docs; git diff --check.",
    "--why-now",
    "P0 approval work-summary bug blocks meaningful dogfood approvals.",
  ]);

  assert.equal(result.status, 0);
  assert.match(
    result.stdout,
    /work_summary: Implement governed approval work-summary handoff fields\./,
  );
  assert.match(
    result.stdout,
    /approved_scope: Update repo-local dogfood phase runner and focused tests only\./,
  );
  assert.match(
    result.stdout,
    /strict_non_goals: No runtime approval semantic changes, schemas, writes, or artifacts\./,
  );
  assert.match(result.stdout, /expected_touched_surfaces: scripts\/self-governed-benchmark\.mjs/);
  assert.match(result.stdout, /validation_required: npm run test:dogfood-helper/);
  assert.match(result.stdout, /why_now: P0 approval work-summary bug/);
});

test("phase-start rejects secret-like work context without leaking value", () => {
  const secret = "token-sk-work-summary";
  const result = runHelper([
    "phase-start",
    "--phase",
    "implementation",
    "--dry-run",
    "--no-build",
    "--work-summary",
    secret,
  ]);

  assert.notEqual(result.status, 0);
  assert.match(result.stderr, /dogfood\.helper\.usage/);
  assert.doesNotMatch(result.stderr, new RegExp(secret));
  assert.doesNotMatch(result.stdout, new RegExp(secret));
});

test("phase-start requires a known phase without echoing unsupported value", () => {
  const secret = "token-sk-bad-phase";
  const result = runHelper([
    "phase-start",
    "--phase",
    secret,
    "--dry-run",
    "--no-build",
  ]);

  assert.notEqual(result.status, 0);
  assert.match(result.stderr, /dogfood\.helper\.usage/);
  assert.doesNotMatch(result.stderr, new RegExp(secret));
  assert.doesNotMatch(result.stdout, new RegExp(secret));
});

test("phase-start dry-run displays bounded approval reason while preserving command redaction", () => {
  const result = runHelper([
    "phase-start",
    "--phase",
    "implementation",
    "--dry-run",
    "--no-build",
    "--state-dir",
    "/tmp/workflow-os-governed-phase-state",
  ]);

  assert.equal(result.status, 0);
  assert.match(result.stdout, /approval_reason: approved-implementation-phase/);
  assert.match(result.stdout, /approval_handoff_required: true/);
  assert.doesNotMatch(result.stdout, /--reason approved-implementation-phase/);
});

test("phase-start rejects secret-like run id without leaking value", () => {
  const secret = "token-sk-bad-run-id";
  const result = runHelper([
    "phase-start",
    "--phase",
    "implementation",
    "--run-id",
    secret,
    "--dry-run",
    "--no-build",
  ]);

  assert.notEqual(result.status, 0);
  assert.match(result.stderr, /dogfood\.helper\.usage/);
  assert.doesNotMatch(result.stderr, new RegExp(secret));
  assert.doesNotMatch(result.stdout, new RegExp(secret));
});

test("phase-close dry-run prints status and inspect commands", () => {
  const result = runHelper([
    "phase-close",
    "run/governed-phase-test",
    "--phase",
    "implementation",
    "--dry-run",
    "--no-build",
    "--state-dir",
    "/tmp/workflow-os-governed-phase-state",
  ]);

  assert.equal(result.status, 0);
  assert.match(result.stdout, /workflow_id: dg\/implement/);
  assert.match(result.stdout, /status_command: .* --json status run\/governed-phase-test/);
  assert.match(result.stdout, /inspect_command: .* --json inspect run\/governed-phase-test/);
  assert.match(result.stdout, /next_action: run without --dry-run/);
});

test("phase-close proof discovery reports matching proof record with bounded fields", async () => {
  const tempRoot = await mkdtemp(join(tmpdir(), "workflow-os-phase-close-proof-"));
  try {
    const recordsDir = join(tempRoot, "approval_presentations", "records", "bucket");
    mkdirSync(recordsDir, { recursive: true });
    writeFileSync(
      join(recordsDir, "record.json"),
      JSON.stringify({
        presentation_id: "presentation/1234abcd5678ef90",
        run_id: "run/proof-test",
        approval_id: "approval/run-proof-test/implementation-approved",
        content_hash: "1234abcd5678ef901234abcd5678ef90",
      }),
    );

    const proof = discoverApprovalPresentationProof(
      { stateDir: tempRoot },
      "run/proof-test",
      {
        approvals: 1,
        events: [{ kind: "ApprovalGranted" }],
      },
    );

    assert.equal(proof.status, "proof_record_present_granted_approval_seen");
    assert.equal(proof.records, 1);
    assert.equal(proof.presentationId, "presentation/1234abcd5678ef90");
    assert.equal(proof.contentHash, "1234abcd5678ef901234abcd5678ef90");
    assert.equal(proof.approvalId, "approval/run-proof-test/implementation-approved");
    assert.equal(proof.eventMarker, "not_available");
    assert.match(proof.note, /does not yet expose/);
  } finally {
    await rm(tempRoot, { recursive: true, force: true });
  }
});

test("phase-close proof discovery reports absent proof store without leaking paths", async () => {
  const tempRoot = await mkdtemp(join(tmpdir(), "workflow-os-phase-close-no-proof-"));
  try {
    const proof = discoverApprovalPresentationProof({ stateDir: tempRoot }, "run/no-proof", {
      events: [],
    });

    assert.equal(proof.status, "no_proof_record_store");
    assert.equal(proof.records, 0);
    assert.equal(proof.eventMarker, "not_available");
    assert.doesNotMatch(proof.note, new RegExp(tempRoot));
  } finally {
    await rm(tempRoot, { recursive: true, force: true });
  }
});

test("phase-close proof discovery reports proof-enforced event marker when inspect exposes marker", async () => {
  const tempRoot = await mkdtemp(join(tmpdir(), "workflow-os-phase-close-proof-marker-"));
  try {
    const recordsDir = join(tempRoot, "approval_presentations", "records", "bucket");
    mkdirSync(recordsDir, { recursive: true });
    writeFileSync(
      join(recordsDir, "record.json"),
      JSON.stringify({
        presentation_id: "presentation/1234abcd5678ef90",
        run_id: "run/proof-marker-test",
        approval_id: "approval/run-proof-marker-test/implementation-approved",
        content_hash: "1234abcd5678ef901234abcd5678ef90",
      }),
    );

    const proof = discoverApprovalPresentationProof(
      { stateDir: tempRoot },
      "run/proof-marker-test",
      {
        approvals: 1,
        events: [
          {
            kind: "ApprovalGranted",
            approval_proof_marker: { status: "present" },
          },
        ],
      },
    );

    assert.equal(proof.status, "proof_enforced");
    assert.equal(proof.records, 1);
    assert.equal(proof.eventMarker, "present");
    assert.match(proof.note, /exposes a presentation proof marker/);
  } finally {
    await rm(tempRoot, { recursive: true, force: true });
  }
});

test("phase-close proof discovery treats multiple matching records as ambiguous", async () => {
  const tempRoot = await mkdtemp(join(tmpdir(), "workflow-os-phase-close-ambiguous-proof-"));
  try {
    for (const bucket of ["a", "b"]) {
      const recordsDir = join(tempRoot, "approval_presentations", "records", bucket);
      mkdirSync(recordsDir, { recursive: true });
      writeFileSync(
        join(recordsDir, "record.json"),
        JSON.stringify({
          presentation_id: `presentation/${bucket}234abcd5678ef90`,
          run_id: "run/ambiguous-proof",
          approval_id: "approval/run-ambiguous-proof/implementation-approved",
          content_hash: `${bucket}234abcd5678ef901234abcd5678ef90`,
        }),
      );
    }

    const proof = discoverApprovalPresentationProof(
      { stateDir: tempRoot },
      "run/ambiguous-proof",
      {
        approvals: 1,
        events: [{ kind: "ApprovalGranted" }],
      },
    );

    assert.equal(proof.status, "proof_record_ambiguous");
    assert.equal(proof.records, 2);
    assert.equal(proof.presentationId, undefined);
    assert.match(proof.note, /multiple approval presentation records/);
  } finally {
    await rm(tempRoot, { recursive: true, force: true });
  }
});

test("approve requires explicit reason", () => {
  const result = runHelper(["approve", "run/dogfood-test", "approval/test", "--dry-run"]);

  assert.notEqual(result.status, 0);
  assert.match(result.stderr, /dogfood\.helper\.usage/);
  assert.match(result.stderr, /approve requires --reason/);
});

test("status dry-run builds expected dogfood command shape", () => {
  const parsed = parseArgs([
    "status",
    "run/dogfood-test",
    "--dry-run",
    "--state-dir",
    "/tmp/workflow-os-self-governance-state",
  ]);
  const command = buildWorkflowCommand(parsed, "target/debug/workflow-os");

  assert.deepEqual(command, [
    "target/debug/workflow-os",
    "--project-dir",
    join(repoRoot, "dogfood", "workflow-os-self-governance"),
    "--state-dir",
    "/tmp/workflow-os-self-governance-state",
    "status",
    "run/dogfood-test",
  ]);
});

test("inspect dry-run builds expected dogfood command shape", () => {
  const parsed = parseArgs([
    "inspect",
    "run/dogfood-test",
    "--dry-run",
    "--state-dir",
    "/tmp/workflow-os-self-governance-state",
  ]);
  const command = buildWorkflowCommand(parsed, "target/debug/workflow-os");

  assert.deepEqual(command, [
    "target/debug/workflow-os",
    "--project-dir",
    join(repoRoot, "dogfood", "workflow-os-self-governance"),
    "--state-dir",
    "/tmp/workflow-os-self-governance-state",
    "inspect",
    "run/dogfood-test",
  ]);
});

test("approve dry-run redacts reason in displayed command", () => {
  const result = runHelper([
    "approve",
    "run/dogfood-test",
    "approval/test",
    "--reason",
    "reviewed-governance-task",
    "--dry-run",
    "--no-build",
  ]);

  assert.equal(result.status, 0);
  assert.match(result.stdout, /<redacted-reason>/);
  assert.doesNotMatch(result.stdout, /reviewed-governance-task/);
});

test("secret-like approval metadata is rejected without leaking value", () => {
  const secret = "token-sk-test-secret-value";
  const result = runHelper([
    "approve",
    "run/dogfood-test",
    "approval/test",
    "--reason",
    secret,
    "--dry-run",
  ]);

  assert.notEqual(result.status, 0);
  assert.match(result.stderr, /dogfood\.helper\.usage/);
  assert.doesNotMatch(result.stderr, new RegExp(secret));
  assert.doesNotMatch(result.stdout, new RegExp(secret));
});

test("commands output describes helper boundary", () => {
  const result = runHelper(["commands"]);

  assert.equal(result.status, 0);
  assert.match(result.stdout, /repo-local development helper only/);
  assert.match(result.stdout, /no automatic approval/);
  assert.match(result.stdout, /phase-start/);
  assert.match(result.stdout, /implementation\s+-> dg\/implement/);
  assert.match(result.stdout, /no hidden approval/);
});

test("prompt output preserves benchmark boundary", () => {
  const result = runHelper(["prompt"]);

  assert.equal(result.status, 0);
  assert.match(result.stdout, /Agent executes\. Workflow OS governs\./);
  assert.match(result.stdout, /Treat approval checkpoints as mandatory/);
  assert.match(result.stdout, /Do not invent run IDs/);
});

test("display command redacts reason values", () => {
  const display = displayCommand([
    join(repoRoot, "target", "debug", "workflow-os"),
    "approve",
    "run/dogfood-test",
    "approval/test",
    "--reason",
    "reviewed-governance-task",
  ]);

  assert.match(display, /\.\/target\/debug\/workflow-os/);
  assert.doesNotMatch(display, new RegExp(repoRoot));
  assert.match(display, /<redacted-reason>/);
  assert.doesNotMatch(display, /reviewed-governance-task/);
});

test("secret-like helper values are detected", () => {
  assert.equal(containsSecretLike("normal governance reason"), false);
  assert.equal(containsSecretLike("private_key=abc123"), true);
  assert.equal(containsSecretLike("sk-test-value"), true);
});

test("unsupported command error does not echo arbitrary command text", () => {
  const secret = "token-sk-unknown-command";
  const result = runHelper([secret]);

  assert.notEqual(result.status, 0);
  assert.match(result.stderr, /dogfood\.helper\.unsupported/);
  assert.doesNotMatch(result.stderr, new RegExp(secret));
  assert.doesNotMatch(result.stdout, new RegExp(secret));
});

test("missing binary with no-build fails closed", async () => {
  const tempRoot = await mkdtemp(join(tmpdir(), "workflow-os-helper-missing-bin-"));
  try {
    mkdirSync(join(tempRoot, "scripts"), { recursive: true });
    copyFileSync(helperScript, join(tempRoot, "scripts", "self-governed-benchmark.mjs"));
    const packageJson = JSON.parse(readFileSync(join(repoRoot, "package.json"), "utf8"));
    mkdirSync(join(tempRoot, "dogfood", "workflow-os-self-governance"), { recursive: true });
    assert.equal(packageJson.scripts["dogfood:benchmark"], "node scripts/self-governed-benchmark.mjs");

    const result = spawnSync(nodeBin, [
      join(tempRoot, "scripts", "self-governed-benchmark.mjs"),
      "validate",
      "--no-build",
    ], {
      cwd: tempRoot,
      encoding: "utf8",
    });

    assert.notEqual(result.status, 0);
    assert.match(result.stderr, /dogfood\.helper\.binary_missing/);
    assert.match(result.stderr, /target\/debug\/workflow-os is missing/);
  } finally {
    await rm(tempRoot, { recursive: true, force: true });
  }
});

function runHelper(args) {
  return spawnSync(nodeBin, [helperScript, ...args], {
    cwd: repoRoot,
    encoding: "utf8",
  });
}
