import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";
import { fileURLToPath } from "node:url";

const bridge = fileURLToPath(new URL("./claude-fixture-mcp.mjs", import.meta.url));
const tool = "verify_workflow_os_fixture";
const call = (method, params, id = 1) => ({ jsonrpc: "2.0", id, method, params });
const fixture = (args) => call("tools/call", { name: tool, arguments: args });
const recoveryTool = "inspect_workflow_os_synthetic_recovery";
const recovery = (args) => call("tools/call", { name: recoveryTool, arguments: args });
const handshake = [
  call("initialize", { protocolVersion: "2025-11-25" }),
  { jsonrpc: "2.0", method: "notifications/initialized" },
];

function runBridge(path, messages, raw = "") {
  const input = messages.map((m) => JSON.stringify(m) + "\n").join("") + raw;
  const child = spawnSync(process.execPath, [bridge, path], {
    input, encoding: "utf8", timeout: 15000,
    env: { ...process.env, WORKFLOW_OS_MCP_TEST_SENTINEL: "parent-only" },
  });
  assert.ifError(child.error);
  return { ...child, responses: child.stdout.trim().split("\n").filter(Boolean).map(JSON.parse) };
}

function rig(t) {
  const dir = mkdtempSync(join(tmpdir(), "workflow-os-fixture-mcp-"));
  t.after(() => rmSync(dir, { recursive: true, force: true }));
  const path = join(dir, "manifest.json");
  const kernel = join(dir, "fixture-kernel");
  const principal = join(dir, "principal");
  const manifest = {
    kernel, project: join(dir, "project"), state: join(dir, "state"),
    cases: [{ id: "exact", grant: "grant/exact", principal_file: "principal", input: { request: {}, fixture: {} } }],
  };
  writeFileSync(principal, Buffer.alloc(32, 42), { mode: 0o600 });
  const put = (code) => writeFileSync(kernel, `#!${process.execPath}\n${code}\n`, { mode: 0o700 });
  const run = (messages, raw = "") => {
    writeFileSync(path, JSON.stringify(manifest), { mode: 0o600 });
    return runBridge(path, messages, raw);
  };
  return { manifest, principal, put, run };
}

test("only the allowlisted case reaches the pinned public CLI; output fields are selected", (t) => {
  const { manifest, put, run } = rig(t);
  put(`
    const fs = require("node:fs");
    const assert = require("node:assert/strict");
    assert.deepEqual(process.argv.slice(2), ${JSON.stringify([
      "--project-dir", manifest.project, "--state-dir", manifest.state, "--json",
      "provider", "github-pr-comment", "continue-sandbox-delegated", "--grant", "grant/exact",
      "--credential-and-request-stdin",
    ])});
    assert.equal(process.env.GITHUB_TOKEN, undefined);
    const input = fs.readFileSync(0);
    assert.deepEqual(input.subarray(0,32), Buffer.alloc(32, 42));
    assert.deepEqual(JSON.parse(input.subarray(32)), {request: {}, fixture: {}});
    console.log(JSON.stringify({
      status:"validated", mode:"fixture", outcome:"fixture_validated", actor_posture:"verified_delegation",
      approval_reference_count:1, linked_approval_reference_count:1, provider_call_performed:false,
      side_effect_mutated:false, workflow_event_appended:false, report_artifact_written:false,
      extra:"must-not-leak",
    }));
  `);
  const result = run([...handshake, call("tools/list"), fixture({ case_id: "exact" })]);
  assert.equal(result.status, 0);
  assert.deepEqual(result.responses[1].result.tools.map((entry) => entry.name), [tool]);
  assert.equal(result.responses[2].result.isError, false, result.stdout);
  assert.equal(JSON.parse(result.responses[2].result.content[0].text).actor_posture, "verified_delegation");
  assert.doesNotMatch(result.stdout, /must-not-leak/);
});

test("pre-init, arbitrary case, extra argument, and operator surface are rejected before invocation", (t) => {
  const { run } = rig(t);
  const result = run([fixture({ case_id: "exact" }), ...handshake,
    fixture({ case_id: "../../sqlite" }), fixture({ case_id: "exact", state: "/tmp" }),
    call("tools/call", { name: "revoke_grant", arguments: { case_id: "exact" } }),
    call("resources/list"),
  ]);
  assert.deepEqual(result.responses.filter((r) => r.error).map((r) => r.error.code), [-32002, -32602, -32602, -32602, -32601]);
});

test("kernel denial does not expose raw diagnostics; unexpected success fails closed", (t) => {
  const { put, run } = rig(t);
  const request = [...handshake, fixture({ case_id: "exact" })];
  put('console.error("error[github_pr_comment_fixture_authority.grant_revoked]: must-not-leak"); process.exit(2);');
  const denial = run(request);
  assert.equal(denial.responses.at(-1).result.isError, true);
  assert.match(denial.stdout, /grant_revoked/);
  assert.doesNotMatch(denial.stdout, /must-not-leak/);
  put('console.log(JSON.stringify({status:"validated", mode:"fixture", provider_call_performed:true}));');
  assert.match(run(request).stdout, /unexpected_kernel_result/);
});

test("symlinked principal, manifest traversal, and oversized frames fail closed", (t) => {
  const { manifest, principal, run } = rig(t);
  symlinkSync(principal, join(manifest.project, "..", "link"));
  manifest.cases[0].principal_file = "link";
  assert.match(run([...handshake, fixture({ case_id: "exact" })]).stdout, /fixture_unavailable/);
  manifest.cases[0].principal_file = "../principal";
  assert.equal(run(handshake).status, 1);
  manifest.cases[0].principal_file = "principal";
  assert.equal(run(handshake, " ".repeat(8193)).status, 1);
});

function configureRecovery(manifest) {
  const state = manifest.state;
  delete manifest.project;
  delete manifest.state;
  manifest.mode = "synthetic-recovery";
  const entry = {
    id: "unresolved", state, run_id: "run/hidden", operation_id: "dispatch/hidden", side_effect_id: "effect/hidden",
  };
  manifest.cases = [entry];
  return entry;
}

test("recovery mode isolates the tool and projects only verified read-only posture", (t) => {
  const { manifest, put, run } = rig(t);
  const entry = configureRecovery(manifest);
  put(`
    const assert = require("node:assert/strict");
    assert.deepEqual(process.argv.slice(2), ${JSON.stringify([
      "--state-dir", entry.state, "--json", "provider", "github-pr-comment", "inspect-synthetic-recovery",
      "--run-id", entry.run_id, "--operation-id", entry.operation_id, "--side-effect-id", entry.side_effect_id,
    ])});
    assert.equal(require("node:fs").readFileSync(0).length, 0);
    assert.equal(process.env.WORKFLOW_OS_MCP_TEST_SENTINEL, undefined);
    assert.equal(process.env.GITHUB_TOKEN, undefined);
    assert.equal(process.env.WORKFLOW_OS_SYNTHETIC_RECOVERY_MCP_MANIFEST, undefined);
    console.log(JSON.stringify({
      evidence_source:"synthetic_sandbox", historical_admission_verified:true,
      dispatch_authorized:false, provider_called:false,
      execution_posture:"reconciliation_required", completion_proof_source:null,
      provider_reference:"must-not-leak",
    }));
  `);
  const result = run([...handshake, call("tools/list"), fixture({ case_id: "unresolved" }),
    recovery({ case_id: "unresolved", operation_id: "substitute" }), recovery({ case_id: "other" }),
    recovery({ case_id: "unresolved" })]);
  assert.deepEqual(result.responses[1].result.tools.map((item) => item.name), [recoveryTool]);
  assert.deepEqual(result.responses.slice(2, 5).map((reply) => reply.error?.code), [-32602, -32602, -32602]);
  assert.equal(result.responses.at(-1).result.isError, false, result.stdout);
  assert.equal(JSON.parse(result.responses.at(-1).result.content[0].text).execution_posture, "reconciliation_required");
  assert.equal(JSON.parse(result.responses.at(-1).result.content[0].text).completion_proof_source, null);
  assert.doesNotMatch(result.stdout, /must-not-leak|dispatch\/hidden/);
});

test("recovery mode rejects mixed manifests and fails closed on kernel output", (t) => {
  const { manifest, put, run } = rig(t);
  const entry = configureRecovery(manifest);
  const request = [...handshake, recovery({ case_id: "unresolved" })];
  put('console.log(JSON.stringify({evidence_source:"synthetic_sandbox",historical_admission_verified:true,execution_posture:"completion_verified",dispatch_authorized:true,provider_called:false}));');
  assert.match(run(request).stdout, /unexpected_kernel_result/);
  for (const [posture, source] of [["completion_verified", undefined], ["completion_verified", null],
    ["completion_verified", "untrusted"], ["reconciliation_required", "creation_response"]]) {
    put(`console.log(${JSON.stringify(JSON.stringify({
      evidence_source: "synthetic_sandbox", historical_admission_verified: true,
      execution_posture: posture, completion_proof_source: source,
      dispatch_authorized: false, provider_called: false,
    }))});`);
    assert.match(run(request).stdout, /unexpected_kernel_result/);
  }
  put('console.error("must-not-leak"); process.exit(2);');
  const denial = run(request);
  assert.match(denial.stdout, /kernel_rejected/);
  assert.doesNotMatch(denial.stdout, /must-not-leak/);
  entry.grant = "grant/hidden";
  assert.equal(run(handshake).status, 1);
  delete entry.grant;
  entry.run_id = "--json";
  assert.equal(run(handshake).status, 1);
});

function checkExportedProofs(exportedManifest, cases) {
  const manifest = JSON.parse(readFileSync(exportedManifest, "utf8"));
  const hashes = () => manifest.cases.map(({ state }) =>
    createHash("sha256").update(readFileSync(join(state, "github-pr-comment-continuation.sqlite3"))).digest("hex"));
  const before = hashes();
  const result = runBridge(exportedManifest, [...handshake, call("tools/list"),
    ...cases.map(([case_id]) => recovery({ case_id }))]);
  assert.equal(result.status, 0, result.stderr);
  assert.deepEqual(result.responses[1].result.tools.map((item) => item.name), [recoveryTool]);
  const postures = result.responses.slice(2).map(({ result: reply }) => {
    assert.equal(reply.isError, false, result.stdout);
    const output = JSON.parse(reply.content[0].text);
    assert.deepEqual(Object.keys(output).sort(), [
      "completion_proof_source", "dispatch_authorized", "evidence_source", "execution_posture",
      "historical_admission_verified", "provider_called", "status",
    ]);
    return [output.execution_posture, output.completion_proof_source];
  });
  assert.deepEqual(postures, cases.map(([, posture, source]) => [posture, source]));
  assert.deepEqual(hashes(), before);
}

const exportedManifest = process.env.WORKFLOW_OS_SYNTHETIC_RECOVERY_MCP_MANIFEST;
test("exported lookup states reconstruct through the real CLI without database changes", {
  skip: !exportedManifest,
}, () => {
  checkExportedProofs(exportedManifest, [
    ["unresolved", "reconciliation_required", null],
    ["completed", "completion_verified", "lookup"],
    ["completed", "completion_verified", "lookup"],
  ]);
});

const responseManifest = process.env.WORKFLOW_OS_SYNTHETIC_RESPONSE_MCP_MANIFEST;
test("exported creation proof reconstructs through the real CLI without database changes", {
  skip: !responseManifest,
}, () => {
  checkExportedProofs(responseManifest, [
    ["response-completed", "completion_verified", "creation_response"],
    ["response-completed", "completion_verified", "creation_response"],
  ]);
});
