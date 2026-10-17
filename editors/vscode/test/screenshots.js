// Takes the README's screenshots: `npm run screenshots`. Runs
// test/screenshots.ts in VS Code with this extension, on a copy of MicroGrid
// (the tree, a finding, a hover, a deletion previewed, inside a substation, a
// quick fix) and of RealGrid (the grid), and writes 2x PNGs into screenshots/,
// reduced to 256 colours. On Linux VS Code renders headless; elsewhere a window
// opens. CIMOXIDE_SHOTS_TAKE names the shots to retake
// (`quick-fix,model-tree`). VS Code is downloaded into .vscode-test unless
// CIMOXIDE_TEST_VSCODE names an executable. Needs server/cimlsp, as npm test.
const fs = require("fs");
const os = require("os");
const path = require("path");
const { runTests } = require("@vscode/test-electron");
const UPNG = require("upng-js");

const root = path.resolve(__dirname, "..");
const configurations = path.resolve(root, "../../CGMES-Test-Configurations/v3.0");
const out = path.join(root, "screenshots");
const port = process.env.CIMOXIDE_SHOTS_PORT || "9333";

// The workbench as shown, in the default theme: no welcome, chat, minimap or tips; every check on, so the tree shows findings.
const settings = {
  "workbench.startupEditor": "none",
  "workbench.secondarySideBar.defaultVisibility": "hidden",
  "workbench.tips.enabled": false,
  "chat.disableAIFeatures": true,
  "editor.minimap.enabled": false,
  "editor.occurrencesHighlight": "off",
  "telemetry.telemetryLevel": "off",
  "update.mode": "none",
  "window.commandCenter": false,
  "git.enabled": false,
  "cimoxide.validation.common": true,
  "cimoxide.validation.quality": true,
};

/** Each session: a model set, named as the grid's title shows it, its file to open, the shots taken on it. */
const sessions = [
  { name: "MicroGrid", data: "MicroGrid/MicroGid-BaseCase/MicroGrid-BaseCase-Merged", main: "_BE_EQ_", shots: "model-tree,validation,hover,delete-preview,grid-substation,quick-fix" },
  // Its EQ file is over 50 MB, which VS Code keeps from extensions: the SSH file is opened.
  { name: "RealGrid", data: "RealGrid/RealGrid-Merged", main: "RealGrid_SSH", shots: "grid-realgrid" },
];

// Kept in full colour: 256 cannot hold RealGrid's thousands of blended lines,
// and the reduction shifts the legend's voltage colours.
const fullColour = ["grid-realgrid"];

/** `file` reduced to 256 colours, about a third of its size. */
function quantize(file) {
  const img = UPNG.decode(fs.readFileSync(file));
  fs.writeFileSync(file, Buffer.from(UPNG.encode(UPNG.toRGBA8(img), img.width, img.height, 256)));
}

async function main() {
  const server = path.join(root, "server", process.platform === "win32" ? "cimlsp.exe" : "cimlsp");
  if (!fs.existsSync(server)) throw new Error(`no ${server}: build cimlsp and copy it there`);
  fs.mkdirSync(out, { recursive: true });
  // CIMOXIDE_SHOTS_TAKE: only these shots (comma-separated names).
  const only = process.env.CIMOXIDE_SHOTS_TAKE?.split(",");
  for (const s of sessions) {
    const take = s.shots.split(",").filter((n) => !only || only.includes(n));
    if (!take.length) continue;
    const data = path.join(configurations, s.data);
    if (!fs.existsSync(data)) throw new Error(`no test data at ${data}: git submodule update --init`);
    const tmp = fs.mkdtempSync(path.join(os.tmpdir(), "cimoxide-shots-"));
    // A copy: the quick fix's shot edits it. Its directory's name is the grid's title.
    const work = path.join(tmp, s.name);
    fs.cpSync(data, work, { recursive: true });
    const user = path.join(tmp, "user");
    fs.mkdirSync(path.join(user, "User"), { recursive: true });
    fs.writeFileSync(path.join(user, "User", "settings.json"), JSON.stringify(settings, null, 2));
    try {
      await runTests({
        vscodeExecutablePath: process.env.CIMOXIDE_TEST_VSCODE || undefined,
        extensionDevelopmentPath: root,
        extensionTestsPath: path.join(root, "dist", "test", "screenshots.js"),
        launchArgs: [
          work,
          "--user-data-dir", user,
          // An empty extensions directory rather than --disable-extensions, which shows a notification.
          "--extensions-dir", path.join(tmp, "extensions"),
          "--disable-workspace-trust", "--skip-welcome", "--skip-release-notes",
          `--remote-debugging-port=${port}`,
          ...(process.platform === "linux" ? ["--ozone-platform=headless"] : []),
        ],
        extensionTestsEnv: {
          CIMOXIDE_SHOTS: out,
          CIMOXIDE_SHOTS_DATA: work,
          CIMOXIDE_SHOTS_MAIN: s.main,
          CIMOXIDE_SHOTS_TAKE: take.join(","),
          CIMOXIDE_SHOTS_PORT: port,
        },
      });
    } finally {
      fs.rmSync(tmp, { recursive: true, force: true });
    }
    for (const n of take) if (!fullColour.includes(n)) quantize(path.join(out, `${n}.png`));
  }
  console.log(`screenshots in ${out}`);
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
