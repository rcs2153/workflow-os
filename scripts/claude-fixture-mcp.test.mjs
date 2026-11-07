import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { mkdtempSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";
import { fileURLToPath } from "node:url";

const bridge = fileURLToPath(new URL("./claude-fixture-mcp.mjs", import.meta.url));
const tool = "verify_workflow_os_fixture";
const call = (method, params, id = 1) => ({ jsonrpc: "2.0", id, method, params });
const fixture = (args) => call("tools/call", { name: tool, arguments: args });
const handshake = [
  call("initialize", { protocolVersion: "2025-11-25" }),
  { jsonrpc: "2.0", method: "notifications/initialized" },
];

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
    const input = messages.map((m) => JSON.stringify(m) + "\n").join("") + raw;
    const child = spawnSync(process.execPath, [bridge, path], { input, encoding: "utf8", timeout: 10000 });
    assert.ifError(child.error);
    return { ...child, responses: child.stdout.trim().split("\n").filter(Boolean).map(JSON.parse) };
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
