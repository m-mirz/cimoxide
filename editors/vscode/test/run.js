// Runs test/suite.ts in VS Code with this extension, on a copy of MicroGrid:
// `npm test`. VS Code is downloaded into .vscode-test unless
// CIMOXIDE_TEST_VSCODE names an executable. Needs server/cimlsp (built and
// copied there, as for packaging) and a display (xvfb-run in CI).
const fs = require("fs");
const os = require("os");
const path = require("path");
const { runTests } = require("@vscode/test-electron");

async function main() {
  const root = path.resolve(__dirname, "..");
  const data = path.resolve(root, "../../CGMES-Test-Configurations/v3.0/MicroGrid/MicroGid-BaseCase/MicroGrid-BaseCase-Merged");
  if (!fs.existsSync(data)) throw new Error(`no test data at ${data}: git submodule update --init`);
  const server = path.join(root, "server", process.platform === "win32" ? "cimlsp.exe" : "cimlsp");
  if (!fs.existsSync(server)) throw new Error(`no ${server}: build cimlsp and copy it there`);
  // A copy: the tests edit and save it.
  const work = fs.mkdtempSync(path.join(os.tmpdir(), "cimoxide-test-"));
  fs.cpSync(data, work, { recursive: true });
  try {
    await runTests({
      vscodeExecutablePath: process.env.CIMOXIDE_TEST_VSCODE || undefined,
      extensionDevelopmentPath: root,
      extensionTestsPath: path.join(root, "dist", "test", "suite.js"),
      launchArgs: [work, "--disable-extensions", "--disable-workspace-trust", "--skip-welcome", "--skip-release-notes"],
      extensionTestsEnv: { CIMOXIDE_TEST_DATA: work },
    });
  } finally {
    fs.rmSync(work, { recursive: true, force: true });
  }
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
