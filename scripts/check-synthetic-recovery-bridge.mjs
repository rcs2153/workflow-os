import { spawnSync } from "node:child_process";
import { accessSync, constants, mkdtempSync, realpathSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";

const exporterPrefix = "sqlite_state::operational_opening_store::tests::";

function run(command, args, env, stage, timeout) {
  const result = spawnSync(command, args, {
    env, timeout, encoding: "utf8", maxBuffer: 1024 * 1024,
  });
  if (result.error || result.status !== 0) {
    if (result.stderr) process.stderr.write(result.stderr);
    throw new Error(`${stage} failed`);
  }
  return result.stdout;
}

function exportSnapshot(root, kernel, env, exporter) {
  const exportOutput = run("cargo", [
    "test", "--locked", "-p", "workflow-core", "--lib", `${exporterPrefix}${exporter}`,
    "--", "--exact", "--nocapture",
  ], { ...env, WORKFLOW_OS_SYNTHETIC_RECOVERY_KERNEL: kernel }, "synthetic snapshot export", 600_000);
  const paths = [...exportOutput.matchAll(/^synthetic_recovery_mcp_manifest: (.+)$/gm)];
  if (paths.length !== 1 || !/^test result: ok\. 1 passed; 0 failed;/m.test(exportOutput)) {
    throw new Error("expected one passing exporter and exactly one manifest");
  }
  const manifest = realpathSync(paths[0][1]);
  if (dirname(dirname(manifest)) !== root || manifest !== join(dirname(manifest), "manifest.json")) {
    throw new Error("exported manifest is outside the disposable check directory");
  }
  return manifest;
}

function check(root, kernel) {
  const env = { ...process.env, TMPDIR: root };
  const lookupManifest = exportSnapshot(root, kernel, env, "live_comment_recovers_lost_acknowledgement_without_resending");
  const responseManifest = exportSnapshot(root, kernel, env, "live_comment_verified_response_survives_reopen_without_resend");
  const output = run(process.execPath, ["--test", "scripts/claude-fixture-mcp.test.mjs"], {
    ...env, WORKFLOW_OS_SYNTHETIC_RECOVERY_MCP_MANIFEST: lookupManifest,
    WORKFLOW_OS_SYNTHETIC_RESPONSE_MCP_MANIFEST: responseManifest,
  }, "real CLI bridge verification", 60_000);
  process.stdout.write(output);
}

try {
  const kernel = resolve("target/debug/workflow-os");
  accessSync(kernel, constants.X_OK);
  const root = mkdtempSync(join(realpathSync(tmpdir()), "workflow-os-recovery-ci-"));
  try {
    check(root, kernel);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
} catch (error) {
  process.stderr.write(`Synthetic recovery bridge check failed: ${error.message}\n`);
  process.exitCode = 1;
}
