import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { copyFileSync, mkdirSync, readFileSync } from "node:fs";
import { mkdtemp, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import test from "node:test";
import { join, resolve } from "node:path";

import {
  buildWorkflowCommand,
  containsSecretLike,
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
  assert.match(result.stdout, /runner_boundary: governance coordination only/);
  assert.doesNotMatch(result.stdout, / approve .*--reason /);
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
