import { dirname, join } from "node:path";
import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import {
  field,
  literal,
  policyDefinition,
  projectFiles,
  projectManifest,
  skillDefinition,
  workflowDefinition
} from "../dist/index.js";

export function writeProject(files, prefix = "workflow-os-sdk-") {
  const root = mkdtempSync(join(tmpdir(), prefix));
  mkdirSync(join(root, "tests"), { recursive: true });
  for (const [path, content] of Object.entries(files)) {
    const fullPath = join(root, path);
    mkdirSync(dirname(fullPath), { recursive: true });
    writeFileSync(fullPath, content);
  }
  return root;
}

export function baseManifest() {
  return projectManifest({
    project: {
      id: "acme/sdk",
      name: "SDK Project"
    }
  });
}

export function basePolicy() {
  return policyDefinition({
    id: "local/allow",
    name: "Local Allow",
    rules: [
      { id: "local", effect: "allow_local" },
      { id: "approval", effect: "require_approval" }
    ]
  });
}

export function baseSkill() {
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

export function baseWorkflow(overrides = {}) {
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

export function approvalWorkflow() {
  return baseWorkflow({
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
}

export function validFiles(workflow = baseWorkflow()) {
  return projectFiles({
    manifest: baseManifest(),
    workflows: [workflow],
    skills: [baseSkill()],
    policies: [basePolicy()]
  });
}
