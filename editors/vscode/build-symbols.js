// Writes the equipment symbols (src/symbols.ts) as the model tree's icon
// files, dist/symbols/<name>-{light,dark}.svg. They ship in the extension:
// VS Code loads a tree icon only from where it trusts, such as the
// extension's own folder, not its storage.
const fs = require("fs");
const path = require("path");
const { SYMBOLS, symbolSvg } = require("./dist/symbols.cjs");

const out = path.join(__dirname, "dist", "symbols");
fs.mkdirSync(out, { recursive: true });
for (const name of Object.keys(SYMBOLS)) {
  // The default icon colours of VS Code's light and dark themes.
  fs.writeFileSync(path.join(out, `${name}-light.svg`), symbolSvg(name, "#424242"));
  fs.writeFileSync(path.join(out, `${name}-dark.svg`), symbolSvg(name, "#c5c5c5"));
}
