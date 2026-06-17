import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
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

test("approve requires explicit reason", () => {
  const result = runHelper(["approve", "run/dogfood-test", "approval/test", "--dry-run"]);

  assert.notEqual(result.status, 0);
  assert.match(result.stderr, /dogfood\.helper\.usage/);
  assert.match(result.stderr, /approve requires --reason/);
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

function runHelper(args) {
  return spawnSync(nodeBin, [helperScript, ...args], {
    cwd: repoRoot,
    encoding: "utf8",
  });
}
