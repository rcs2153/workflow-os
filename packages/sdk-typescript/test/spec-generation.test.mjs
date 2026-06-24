import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { existsSync } from "node:fs";
import { join, resolve } from "node:path";
import test from "node:test";
import { literal, projectManifest, schemaVersion, skillDefinition } from "../dist/index.js";
import { approvalWorkflow, baseSkill, baseWorkflow, validFiles, writeProject } from "./contract-fixtures.mjs";

const repoRoot = resolve(new URL("../../..", import.meta.url).pathname);

function cargoBuildEnv() {
  return {
    ...process.env,
    CARGO_HTTP_MULTIPLEXING: process.env.CARGO_HTTP_MULTIPLEXING ?? "false",
    CARGO_NET_RETRY: process.env.CARGO_NET_RETRY ?? "10"
  };
}

function buildWorkflowOsBin(cargo) {
  const args = ["build", "--locked", "-p", "workflow-cli", "--bin", "workflow-os"];
  const configuredAttempts = Number.parseInt(process.env.WORKFLOW_OS_CLI_BUILD_ATTEMPTS ?? "3", 10);
  const maxAttempts = Number.isFinite(configuredAttempts) && configuredAttempts > 0 ? configuredAttempts : 3;

  for (let attempt = 1; attempt <= maxAttempts; attempt += 1) {
    const result = spawnSync(cargo, args, {
      cwd: repoRoot,
      env: cargoBuildEnv(),
      stdio: "inherit"
    });

    if (result.status === 0) {
      return;
    }

    if (attempt === maxAttempts) {
      const detail = result.signal ? `signal ${result.signal}` : `exit status ${result.status ?? "unknown"}`;
      throw result.error ?? new Error(`Command failed after ${maxAttempts} attempts: cargo ${args.join(" ")} (${detail})`);
    }

    console.warn(`workflow-os CLI build failed; retrying (${attempt + 1}/${maxAttempts})`);
  }
}

function workflowOsBin() {
  if (process.env.WORKFLOW_OS_CLI_BIN) {
    return process.env.WORKFLOW_OS_CLI_BIN;
  }
  const bin = join(repoRoot, "target", "debug", "workflow-os");
  if (existsSync(bin)) {
    return bin;
  }
  const cargo = existsSync(join(repoRoot, ".tools", "cargo", "bin", "cargo"))
    ? join(repoRoot, ".tools", "cargo", "bin", "cargo")
    : "cargo";
  buildWorkflowOsBin(cargo);
  return bin;
}

function validateProject(root) {
  return spawnSync(workflowOsBin(), ["--project-dir", root, "validate"], {
    cwd: repoRoot,
    encoding: "utf8"
  });
}

test("generate minimal valid project", () => {
  const files = validFiles();

  assert.equal(JSON.parse(files["workflow-os.yml"]).schema_version, schemaVersion);
  assert.ok(files["workflows/local-main.workflow.yml"]);
  assert.ok(files["skills/local-summarize.skill.yml"]);
});

test("generated project passes Rust CLI validation", () => {
  const root = writeProject(validFiles());
  const result = validateProject(root);

  assert.equal(result.status, 0, result.stderr);
  assert.match(result.stdout, /Project is valid/);
});

test("generate approval-gated workflow", () => {
  const workflow = approvalWorkflow();

  assert.equal(workflow.autonomy_level, "level_2");
  assert.equal(workflow.steps[0].policy_requirements[0].id, "local/allow");
  assert.equal(workflow.steps[0].approval_policy.policy.id, "approval/required");
});

test("approval-gated generated project passes Rust CLI validation", () => {
  const result = validateProject(writeProject(validFiles(approvalWorkflow())));

  assert.equal(result.status, 0, result.stderr);
});

test("generated invalid project fails Rust validation in expected way", () => {
  const invalid = baseWorkflow({ triggers: [] });
  const result = validateProject(writeProject(validFiles(invalid)));

  assert.notEqual(result.status, 0);
  assert.match(result.stdout, /validation.workflow.triggers_missing/);
});

test("sensitive fields are represented with redaction", () => {
  const skill = skillDefinition({
    id: "local/classify",
    version: "v0",
    display_name: "Classify",
    owner: { lifecycle_status: "stable" },
    input_contract: {
      fields: [{ name: "customer_payload", field_type: "string", sensitive: true, redaction: "reference_only" }],
      required: ["customer_payload"]
    },
    output_contract: {
      fields: [{ name: "summary_ref", field_type: "string", sensitive: true, redaction: "reference_only" }],
      required: ["summary_ref"]
    },
    failure_modes: [{ code: "failed", description: "Failure." }],
    evaluation_criteria: [{ name: "reviewable", description: "Reviewable output." }],
    audit_requirements: { required: true, events: ["SkillInvocationRequested"] },
    observability_requirements: { metrics: ["skill_latency"] }
  });

  assert.equal(skill.input_contract.fields[0].redaction, "reference_only");
  assert.equal(skill.output_contract.fields[0].sensitive, true);
});

test("schema version and lifecycle status are emitted", () => {
  const skill = baseSkill();

  assert.equal(skill.schema_version, schemaVersion);
  assert.equal(skill.owner.lifecycle_status, "stable");
});

test("helper APIs reject secret-like literals", () => {
  assert.throws(() => literal("password-value"));
  assert.throws(() =>
    projectManifest({
      project: { id: "acme/sdk", name: "SDK Project" },
      config: [{ environment: "dev", vars: [{ name: "token", value: "x" }] }]
    })
  );
});
