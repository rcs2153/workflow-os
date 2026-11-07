import { spawnSync } from "node:child_process";
import { closeSync, constants, fstatSync, openSync, readSync } from "node:fs";
import { isAbsolute, join, dirname } from "node:path";

const TOOL = "verify_workflow_os_fixture";
const CASE = /^[a-z][a-z0-9-]{0,63}$/;
const GRANT = /^grant\/[a-zA-Z0-9._/-]{1,120}$/;
const VERSIONS = new Set(["2024-11-05", "2025-03-26", "2025-06-18", "2025-11-25"]);
const MAX_REQUEST = 64 * 1024;
const MAX_FRAME = 8192;
const tool = {
  name: TOOL,
  description: "Validate one operator-prepared Workflow OS fixture. Never dispatches an action.",
  inputSchema: {
    type: "object",
    properties: { case_id: { type: "string", pattern: CASE.source } },
    required: ["case_id"],
    additionalProperties: false,
  },
  annotations: { readOnlyHint: true, destructiveHint: false, openWorldHint: false },
};

function hasKeys(value, keys) {
  return value !== null && typeof value === "object" && !Array.isArray(value) &&
    Object.keys(value).length === keys.length && keys.every((key) => Object.hasOwn(value, key));
}

function privateFile(path, max) {
  const fd = openSync(path, constants.O_RDONLY | constants.O_NOFOLLOW | constants.O_NONBLOCK);
  try {
    const stat = fstatSync(fd);
    if (!stat.isFile() || stat.uid !== process.getuid() || (stat.mode & 0o077) !== 0) {
      throw new Error("not an operator-owned private file");
    }
    const buffer = Buffer.alloc(max + 1);
    let size = 0;
    while (size < buffer.length) {
      const count = readSync(fd, buffer, size, buffer.length - size);
      if (count === 0) break;
      size += count;
    }
    if (size > max) throw new Error("private file too large");
    return buffer.subarray(0, size);
  } finally {
    closeSync(fd);
  }
}

function loadManifest(path) {
  if (!isAbsolute(path)) throw new Error("manifest path must be absolute");
  const value = JSON.parse(privateFile(path, 1024 * 1024).toString("utf8"));
  if (!hasKeys(value, ["kernel", "project", "state", "cases"]) ||
      ![value.kernel, value.project, value.state].every((p) => typeof p === "string" && isAbsolute(p)) ||
      !Array.isArray(value.cases) || value.cases.length === 0 || value.cases.length > 32) {
    throw new Error("invalid manifest");
  }
  const cases = new Map();
  for (const entry of value.cases) {
    if (!hasKeys(entry, ["id", "grant", "principal_file", "input"]) ||
        typeof entry.id !== "string" || !CASE.test(entry.id) || cases.has(entry.id) ||
        typeof entry.grant !== "string" || !GRANT.test(entry.grant) ||
        typeof entry.principal_file !== "string" || !CASE.test(entry.principal_file) ||
        !hasKeys(entry.input, ["request", "fixture"])) {
      throw new Error("invalid case");
    }
    const input = Buffer.from(JSON.stringify(entry.input));
    if (input.length > MAX_REQUEST) throw new Error("case input too large");
    cases.set(entry.id, { grant: entry.grant, principal: join(dirname(path), entry.principal_file), input });
  }
  return { kernel: value.kernel, project: value.project, state: value.state, cases };
}

const MUTATIONS = ["provider_call_performed", "side_effect_mutated", "workflow_event_appended", "report_artifact_written"];
const rejected = (reason) => ({ status: "rejected", reason });

function verify(config, entry) {
  try {
    const principal = privateFile(entry.principal, 32);
    if (principal.length !== 32) return rejected("principal_unavailable");
    const input = Buffer.concat([principal, entry.input]);
    let child;
    try {
      child = spawnSync(config.kernel, [
        "--project-dir", config.project, "--state-dir", config.state, "--json",
        "provider", "github-pr-comment", "continue-sandbox-delegated",
        "--grant", entry.grant, "--credential-and-request-stdin",
      ], { input, encoding: "utf8", timeout: 5000, maxBuffer: 16 * 1024, env: {} });
    } finally {
      principal.fill(0);
      input.fill(0);
    }
    if (child.error || child.signal) return rejected("kernel_unavailable");
    if (child.status !== 0) {
      const code = /^error\[((?:github_pr_comment_fixture_(?:authority|continuation)|capability_grant)[a-z0-9_.]{0,100})\]:/m.exec(child.stderr)?.[1];
      return rejected(code ?? "kernel_rejected");
    }
    const result = JSON.parse(child.stdout);
    if (result.status !== "validated" || result.mode !== "fixture" ||
        result.outcome !== "fixture_validated" || result.actor_posture !== "verified_delegation" ||
        !MUTATIONS.every((key) => result[key] === false) ||
        !Number.isSafeInteger(result.approval_reference_count) || result.approval_reference_count < 1 ||
        result.approval_reference_count !== result.linked_approval_reference_count) {
      return rejected("unexpected_kernel_result");
    }
    return {
      status: "validated", mode: "fixture", outcome: "fixture_validated", actor_posture: "verified_delegation",
      approval_reference_count: result.approval_reference_count,
      linked_approval_reference_count: result.linked_approval_reference_count,
      ...Object.fromEntries(MUTATIONS.map((key) => [key, false])),
    };
  } catch {
    return rejected("fixture_unavailable");
  }
}

async function serve(config) {
  let initialized = false;
  let ready = false;
  function respond(message) {
    const id = message?.id;
    const reply = (body) => process.stdout.write(JSON.stringify({ jsonrpc: "2.0", id, ...body }) + "\n");
    if (message?.jsonrpc !== "2.0" || typeof message.method !== "string") return;
    if (id === undefined) {
      if (initialized && message.method === "notifications/initialized") ready = true;
      return;
    }
    if (!(typeof id === "string" || Number.isSafeInteger(id))) return;
    const ok = (result) => reply({ result });
    const error = (code, text) => reply({ error: { code, message: text } });
    if (message.method === "initialize" && !initialized &&
        typeof message.params?.protocolVersion === "string") {
      initialized = true;
      const version = message.params.protocolVersion;
      return ok({
        protocolVersion: VERSIONS.has(version) ? version : "2025-11-25",
        capabilities: { tools: {} },
        serverInfo: { name: "workflow-os-claude-fixture", version: "0.1.0" },
      });
    }
    if (message.method === "ping") return ok({});
    if (!ready) return error(-32002, "Session is not initialized");
    if (message.method === "tools/list") return ok({ tools: [tool] });
    if (message.method !== "tools/call") return error(-32601, "Method not found");
    const args = message.params?.arguments;
    if (!hasKeys(message.params, ["name", "arguments"]) || message.params.name !== TOOL ||
        !hasKeys(args, ["case_id"]) || typeof args.case_id !== "string" ||
        !config.cases.has(args.case_id)) return error(-32602, "Expected an allowlisted fixture case_id");
    const result = verify(config, config.cases.get(args.case_id));
    return ok({ isError: result.status !== "validated", content: [{ type: "text", text: JSON.stringify(result) }] });
  }

  let pending = Buffer.alloc(0);
  for await (const chunk of process.stdin) {
    let start = 0;
    while (start < chunk.length) {
      const end = chunk.indexOf(0x0a, start);
      const part = chunk.subarray(start, end === -1 ? chunk.length : end);
      if (pending.length + part.length > MAX_FRAME) throw new Error("frame too large");
      pending = Buffer.concat([pending, part]);
      if (end === -1) break;
      let message;
      try { message = JSON.parse(pending.toString("utf8")); } catch { /* Drop malformed frames. */ }
      respond(message);
      pending = Buffer.alloc(0);
      start = end + 1;
    }
  }
}

try {
  if (process.argv.length !== 3) throw new Error("expected manifest");
  await serve(loadManifest(process.argv[2]));
} catch {
  process.stderr.write("Workflow OS Claude fixture bridge unavailable.\n");
  process.exitCode = 1;
}
