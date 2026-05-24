import assert from "node:assert/strict";
import { execFileSync, spawnSync } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { tmpdir } from "node:os";
import test from "node:test";
import {
  field,
  literal,
  policyDefinition,
  projectFiles,
  projectManifest,
  schemaVersion,
  skillDefinition,
  workflowDefinition
} from "../dist/index.js";

const repoRoot = resolve(new URL("../../..", import.meta.url).pathname);

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
  execFileSync(cargo, ["build", "-p", "workflow-cli", "--bin", "workflow-os"], {
    cwd: repoRoot,
    stdio: "inherit"
  });
  return bin;
}

function writeProject(files) {
  const root = mkdtempSync(join(tmpdir(), "workflow-os-sdk-"));
  mkdirSync(join(root, "tests"), { recursive: true });
  for (const [path, content] of Object.entries(files)) {
    const fullPath = join(root, path);
    mkdirSync(dirname(fullPath), { recursive: true });
    writeFileSync(fullPath, content);
  }
  return root;
}

function validateProject(root) {
  return spawnSync(workflowOsBin(), ["--project-dir", root, "validate"], {
    cwd: repoRoot,
    encoding: "utf8"
  });
}

function baseManifest() {
  return projectManifest({
    project: {
      id: "acme/sdk",
      name: "SDK Project"
    }
  });
}

function basePolicy() {
  return policyDefinition({
    id: "local/allow",
    name: "Local Allow",
    rules: [
      { id: "local", effect: "allow_local" },
      { id: "approval", effect: "require_approval" }
    ]
  });
}

function baseSkill() {
  return skillDefinition({
    id: "local/summarize",
    version: "v0",
    display_name: "Summarize",
    owner: {
      lifecycle_status: "stable",
      owning_team: "platform"
    },
    input_contract: {
      fields: [{ name: "request", field_type: "string" }],
      required: ["request"]
    },
    output_contract: {
      fields: [{ name: "summary", field_type: "string" }],
      required: ["summary"]
    },
    failure_modes: [{ code: "failed", description: "Local failure." }],
    evaluation_criteria: [{ name: "deterministic", description: "Output is deterministic." }],
    audit_requirements: { required: true, events: ["SkillInvocationRequested"] },
    observability_requirements: { metrics: ["skill_latency"] }
  });
}

function baseWorkflow(overrides = {}) {
  return workflowDefinition({
    id: "local/main",
    version: "v0",
    display_name: "Local Main",
    owner: {
      lifecycle_status: "stable",
      owning_team: "platform"
    },
    autonomy_level: "level_1",
    triggers: [{ id: "manual", kind: "manual" }],
    steps: [
      {
        id: "summarize",
        skill_ref: { id: "local/summarize", version: "v0" },
        input_mapping: [{ from: literal("hello"), to: "request" }],
        policy_requirements: [{ id: "local/allow" }],
        terminal_behavior: "fail_workflow"
      }
    ],
    cancellation_behavior: "stop",
    audit_requirements: { required: true, events: ["RunCreated"] },
    observability_requirements: { metrics: ["workflow_latency"] },
    ...overrides
  });
}

function validFiles(workflow = baseWorkflow()) {
  return projectFiles({
    manifest: baseManifest(),
    workflows: [workflow],
    skills: [baseSkill()],
    policies: [basePolicy()]
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
  const workflow = baseWorkflow({
    autonomy_level: "level_2",
    approval_requirements: [
      {
        id: "human-review",
        reason: "Review before execution.",
        expires_after: { duration: "30m" }
      }
    ],
    steps: [
      {
        id: "summarize",
        skill_ref: { id: "local/summarize", version: "v0" },
        input_mapping: [{ from: field("request.description"), to: "request" }],
        policy_requirements: [{ id: "local/allow" }],
        approval_policy: { policy: { id: "local/allow" } },
        terminal_behavior: "fail_workflow"
      }
    ]
  });

  assert.equal(workflow.autonomy_level, "level_2");
  assert.equal(workflow.steps[0].approval_policy.policy.id, "local/allow");
});

test("approval-gated generated project passes Rust CLI validation", () => {
  const workflow = baseWorkflow({
    autonomy_level: "level_2",
    approval_requirements: [
      {
        id: "human-review",
        reason: "Review before execution.",
        expires_after: { duration: "30m" }
      }
    ],
    steps: [
      {
        id: "summarize",
        skill_ref: { id: "local/summarize", version: "v0" },
        input_mapping: [{ from: literal("hello"), to: "request" }],
        policy_requirements: [{ id: "local/allow" }],
        approval_policy: { policy: { id: "local/allow" } },
        terminal_behavior: "fail_workflow"
      }
    ]
  });
  const result = validateProject(writeProject(validFiles(workflow)));

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
