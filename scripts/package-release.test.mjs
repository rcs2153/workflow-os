import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { chmodSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import test from "node:test";

import { packageRelease } from "./package-release.mjs";

const projectDir = resolve(import.meta.dirname, "..");
const sourceCommit = "0123456789abcdef0123456789abcdef01234567";
const version = JSON.parse(readFileSync(join(projectDir, "package.json"), "utf8")).version;

function fixture() {
  const directory = mkdtempSync(join(tmpdir(), "workflow-os-package-"));
  const binaryPath = join(directory, "workflow-os");
  writeFileSync(binaryPath, `#!/bin/sh\necho 'workflow-os ${version}'\n`, "utf8");
  chmodSync(binaryPath, 0o755);
  return { directory, binaryPath };
}

test("produces deterministic archives and checksums", () => {
  const { directory, binaryPath } = fixture();
  const first = packageRelease({
    projectDir,
    binaryPath,
    outputDir: join(directory, "first"),
    target: "aarch64-apple-darwin",
    sourceCommit,
    verifyBinary: false,
    verifyRepository: false,
  });
  const second = packageRelease({
    projectDir,
    binaryPath,
    outputDir: join(directory, "second"),
    target: "aarch64-apple-darwin",
    sourceCommit,
    verifyBinary: false,
    verifyRepository: false,
  });
  assert.equal(first.checksum, second.checksum);
  assert.deepEqual(readFileSync(first.archivePath), readFileSync(second.archivePath));
  assert.match(readFileSync(`${first.archivePath}.sha256`, "utf8"), new RegExp(`^${first.checksum}  `));
});

test("archive installs and uninstalls only the binary", () => {
  const { directory, binaryPath } = fixture();
  const result = packageRelease({
    projectDir,
    binaryPath,
    outputDir: join(directory, "output"),
    target: "aarch64-apple-darwin",
    sourceCommit,
    verifyBinary: false,
    verifyRepository: false,
  });
  const extractDir = join(directory, "extract");
  execFileSync("mkdir", ["-p", extractDir]);
  execFileSync("tar", ["-xzf", result.archivePath, "-C", extractDir]);
  const root = join(extractDir, `workflow-os-v${version}-aarch64-apple-darwin`);
  const installDir = join(directory, "install");
  execFileSync(join(root, "install.sh"), { env: { ...process.env, WORKFLOW_OS_INSTALL_DIR: installDir } });
  assert.equal(
    execFileSync(join(installDir, "workflow-os"), ["--version"], { encoding: "utf8" }).trim(),
    `workflow-os ${version}`,
  );
  assert.throws(() => execFileSync(join(root, "install.sh"), { env: { ...process.env, WORKFLOW_OS_INSTALL_DIR: installDir } }));
  assert.throws(() =>
    execFileSync(join(root, "install.sh"), ["--force", "extra"], {
      env: { ...process.env, WORKFLOW_OS_INSTALL_DIR: installDir },
    }),
  );
  execFileSync(join(root, "uninstall.sh"), { env: { ...process.env, WORKFLOW_OS_INSTALL_DIR: installDir } });
  assert.throws(() => readFileSync(join(installDir, "workflow-os")));
});

test("rejects unsupported targets and inexact source commits", () => {
  const { directory, binaryPath } = fixture();
  const base = {
    projectDir,
    binaryPath,
    outputDir: directory,
    sourceCommit,
    verifyBinary: false,
    verifyRepository: false,
  };
  assert.throws(() => packageRelease({ ...base, target: "windows-x86_64" }), /unsupported release target/);
  assert.throws(
    () => packageRelease({ ...base, target: "aarch64-apple-darwin", sourceCommit: "deadbeef" }),
    /exact 40-character Git commit SHA/,
  );
});

test("rejects a source commit that does not match checkout HEAD", () => {
  const { directory, binaryPath } = fixture();
  assert.throws(
    () =>
      packageRelease({
        projectDir,
        binaryPath,
        outputDir: directory,
        target: "aarch64-apple-darwin",
        sourceCommit,
        verifyBinary: false,
      }),
    /source commit mismatch/,
  );
});
