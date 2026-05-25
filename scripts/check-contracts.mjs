import { execFileSync, spawnSync } from "node:child_process";
import { existsSync, mkdirSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { approvalWorkflow, baseWorkflow, validFiles, writeProject } from "../packages/sdk-typescript/test/contract-fixtures.mjs";

const repoRoot = resolve(new URL("..", import.meta.url).pathname);
const currentSchemaVersion = "workflowos.dev/v0";
const requiredSchemas = [
  "project.schema.json",
  "workflow.schema.json",
  "skill.schema.json",
  "policy.schema.json",
  "test.schema.json"
];

function main() {
  const workflowOs = workflowOsBin();
  checkSchemas();
  checkExamples(workflowOs);
  checkSdkGeneratedProjects(workflowOs);
}

function workflowOsBin() {
  if (process.env.WORKFLOW_OS_CLI_BIN) {
    return process.env.WORKFLOW_OS_CLI_BIN;
  }
  const bin = join(repoRoot, "target", "debug", "workflow-os");
  if (!existsSync(bin)) {
    const cargo = existsSync(join(repoRoot, ".tools", "cargo", "bin", "cargo"))
      ? join(repoRoot, ".tools", "cargo", "bin", "cargo")
      : "cargo";
    execFileSync(cargo, ["build", "-p", "workflow-cli", "--bin", "workflow-os"], {
      cwd: repoRoot,
      stdio: "inherit"
    });
  }
  return bin;
}

function checkSchemas() {
  const schemaRoot = join(repoRoot, "schemas", "v0");
  for (const fileName of requiredSchemas) {
    const path = join(schemaRoot, fileName);
    assert(existsSync(path), `missing checked-in schema ${path}`);
    const schema = JSON.parse(readFileSync(path, "utf8"));
    assert(
      schema.$id === `https://workflow-os.dev/schemas/v0/${fileName}`,
      `${fileName} has unexpected $id`
    );
    assert(
      schema.properties?.schema_version?.const === currentSchemaVersion,
      `${fileName} does not pin schema_version to ${currentSchemaVersion}`
    );
  }
}

function checkExamples(workflowOs) {
  for (const exampleRoot of exampleProjects()) {
    const result = validateProject(workflowOs, exampleRoot);
    assert(
      result.status === 0,
      `example ${exampleRoot} failed Rust validation\nstdout:\n${result.stdout}\nstderr:\n${result.stderr}`
    );
  }
}

function exampleProjects() {
  const examplesRoot = join(repoRoot, "examples");
  return readdirSync(examplesRoot)
    .map((entry) => join(examplesRoot, entry))
    .filter((path) => statSync(path).isDirectory())
    .filter((path) => existsSync(join(path, "workflow-os.yml")));
}

function checkSdkGeneratedProjects(workflowOs) {
  const minimal = writeProject(validFiles(), "workflow-os-contract-minimal-");
  assertValid(workflowOs, minimal, "SDK minimal project");

  const approval = writeProject(validFiles(approvalWorkflow()), "workflow-os-contract-approval-");
  assertValid(workflowOs, approval, "SDK approval-gated project");

  const invalid = writeProject(
    validFiles(baseWorkflow({ triggers: [] })),
    "workflow-os-contract-invalid-"
  );
  const invalidResult = validateProject(workflowOs, invalid);
  assert(invalidResult.status !== 0, "SDK intentionally invalid project unexpectedly validated");
  assert(
    invalidResult.stdout.includes("validation.workflow.triggers_missing"),
    "SDK invalid project did not fail with the expected Rust diagnostic"
  );

  const mismatched = writeProject(validFiles(), "workflow-os-contract-schema-mismatch-");
  replaceSchemaVersion(join(mismatched, "workflow-os.yml"), "workflowos.dev/v999");
  const mismatchResult = validateProject(workflowOs, mismatched);
  assert(mismatchResult.status !== 0, "schema version mismatch unexpectedly validated");
  assert(
    mismatchResult.stdout.includes("schema_version") ||
      mismatchResult.stderr.includes("schema_version"),
    "schema version mismatch did not produce a schema-version diagnostic"
  );
}

function assertValid(workflowOs, projectRoot, label) {
  const result = validateProject(workflowOs, projectRoot);
  assert(
    result.status === 0,
    `${label} failed Rust validation\nstdout:\n${result.stdout}\nstderr:\n${result.stderr}`
  );
}

function validateProject(workflowOs, projectRoot) {
  return spawnSync(workflowOs, ["--project-dir", projectRoot, "validate"], {
    cwd: repoRoot,
    encoding: "utf8"
  });
}

function replaceSchemaVersion(path, version) {
  const document = JSON.parse(readFileSync(path, "utf8"));
  document.schema_version = version;
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, `${JSON.stringify(document, null, 2)}\n`);
}

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

main();
