// The grid view's page: draws `cimoxide/grid` with Cytoscape.js, coloured by
// each substation's and line's worst finding, in VS Code's theme colours; and,
// double-clicking a substation, what it holds (`cimoxide/substation`).

import cytoscape, { Core, ElementDefinition, NodeSingular, EdgeSingular } from "cytoscape";
import { Symbol, symbolOf, symbolSvg } from "../symbols";

declare function acquireVsCodeApi(): { postMessage(msg: unknown): void };
const vscode = acquireVsCodeApi();

interface Findings {
  violation: number;
  warning: number;
  info: number;
}

interface Location {
  uri: string;
  range: unknown;
}

interface GridNode {
  id: string;
  label: string;
  kind: string;
  region: string | null;
  kV: number[];
  equipment: Record<string, number>;
  findings: Findings;
  location: Location | null;
}

interface GridEdge {
  id: string;
  label: string;
  class: string;
  source: string;
  target: string;
  kV: number | null;
  findings: Findings;
  location: Location | null;
}

/** Positions from one DL diagram or one GL coordinate system, as written. */
interface Layout {
  id: string;
  name: string;
  kind: "diagram" | "geographic";
  flipY: boolean;
  nodes: Record<string, [number, number]>;
  /** Edge id → its route, from its source's end to its target's. */
  edges: Record<string, [number, number][]>;
}

interface Grid {
  files: string[];
  nodes: GridNode[];
  edges: GridEdge[];
  elsewhere: Findings;
  layouts: Layout[];
  /** The automatic layout, computed by the server. */
  auto: Layout | null;
}

/** Equipment, or a connectivity node (`junction`), inside a substation. */
interface Item {
  id: string;
  label: string;
  class: string;
  junction: boolean;
  /** Its voltage level's id. */
  group: string | null;
  open: boolean | null;
  inService: boolean | null;
  /** The other grid nodes the equipment reaches. */
  far: { id: string; label: string; kind: string }[];
  findings: Findings;
  location: Location | null;
}

/** What a substation holds: its items by voltage level, and the terminals linking them. */
interface Inside {
  id: string;
  label: string;
  region: string | null;
  kV: number[];
  /** Findings on the substation that no item stands for. */
  findings: Findings;
  location: Location | null;
  groups: { id: string; label: string; kV: number | null }[];
  items: Item[];
  links: { id: string; source: string; target: string; connected: boolean | null }[];
  layouts: Layout[];
}

type Point = { x: number; y: number };

/** The layout picked in this panel for the grid, and for substations: a layout id, or "auto". */
const chosen: { grid?: string; inside?: string } = {};

let cy: Core | undefined;
let grid: Grid | undefined;
/** The substation shown, if any; else the grid. */
let inside: Inside | undefined;
/** The grid's viewport when a substation was opened, to return to. */
let gridView: { zoom: number; pan: Point; selected: string } | undefined;

const style = document.createElement("style");
style.textContent = `
  html, body { height: 100%; margin: 0; padding: 0; overflow: hidden; }
  body { display: flex; flex-direction: column; color: var(--vscode-foreground); background: var(--vscode-editor-background);
         font-family: var(--vscode-font-family); font-size: var(--vscode-font-size); }
  #bar { display: flex; gap: 12px; align-items: center; padding: 6px 10px; border-bottom: 1px solid var(--vscode-panel-border); }
  #layout { color: var(--vscode-dropdown-foreground); background: var(--vscode-dropdown-background);
            border: 1px solid var(--vscode-dropdown-border, transparent); padding: 2px 4px; }
  #search { width: 220px; color: var(--vscode-input-foreground); background: var(--vscode-input-background);
            border: 1px solid var(--vscode-input-border, transparent); padding: 3px 6px; }
  #back { display: none; color: var(--vscode-button-secondaryForeground); background: var(--vscode-button-secondaryBackground);
          border: none; padding: 3px 10px; cursor: pointer; }
  #back:hover { background: var(--vscode-button-secondaryHoverBackground); }
  #summary { color: var(--vscode-descriptionForeground); }
  #main { flex: 1; display: flex; min-height: 0; }
  #graph { flex: 1; min-width: 0; }
  #info { width: 280px; overflow: auto; padding: 8px 12px; border-left: 1px solid var(--vscode-panel-border); }
  #info h3 { margin: 4px 0 2px; font-size: 1.1em; }
  #info .sub { color: var(--vscode-descriptionForeground); margin-bottom: 8px; }
  #info table { border-collapse: collapse; width: 100%; }
  #info td { padding: 1px 4px 1px 0; vertical-align: top; }
  #info td.n { text-align: right; }
  #info button { margin: 10px 6px 0 0; color: var(--vscode-button-foreground); background: var(--vscode-button-background);
                 border: none; padding: 4px 10px; cursor: pointer; }
  #info button:hover { background: var(--vscode-button-hoverBackground); }
  .violation { color: var(--vscode-errorForeground); }
  .warning { color: var(--vscode-editorWarning-foreground); }
  .info { color: var(--vscode-editorInfo-foreground); }
`;
document.head.appendChild(style);

/** A theme colour, read from VS Code's CSS variables. */
function colour(name: string, fallback: string): string {
  return getComputedStyle(document.body).getPropertyValue(name).trim() || fallback;
}

function worst(f: Findings): string {
  return f.violation ? "violation" : f.warning ? "warning" : f.info ? "info" : "none";
}

function total(f: Findings): number {
  return f.violation + f.warning + f.info;
}

/** The ring grows with the findings: one and forty must not look alike. */
function ring(f: Findings): number {
  return total(f) ? Math.min(6, 1.5 + Math.log2(total(f))) : 0;
}

/** Voltage bands, coloured as grid maps colour them (in the theme's chart colours). */
const BANDS: [string, number, string, string][] = [
  ["v300", 300, "--vscode-charts-red", "#e5534b"],
  ["v200", 200, "--vscode-charts-green", "#57ab5a"],
  ["v100", 100, "--vscode-charts-blue", "#539bf5"],
  ["v30", 30, "--vscode-charts-purple", "#b083f0"],
];

function band(kV: number | null | undefined): string {
  return kV == null ? "none" : BANDS.find(([, min]) => kV >= min)?.[0] ?? "none";
}

/** The legend of the bands, for the side panel. */
function legend(): string {
  const swatch = (c: string) => `<span style="display:inline-block;width:14px;height:3px;margin:0 6px 3px 0;background:${c}"></span>`;
  const rows = BANDS.map(([, min, v, fb], i) => `<div>${swatch(colour(v, fb))}${i ? `${min}–${BANDS[i - 1][1]}` : `≥ ${min}`} kV</div>`);
  return `<div class="sub" style="margin-top:12px">${rows.join("")}<div>${swatch(colour("--vscode-descriptionForeground", "#888"))}lower, or unknown</div></div>`;
}

function gridElements(g: Grid): ElementDefinition[] {
  const nodes = g.nodes.map((n) => {
    const size = Object.values(n.equipment).reduce((a, b) => a + b, 0);
    return {
      group: "nodes" as const,
      data: {
        id: n.id, label: n.label, kind: n.kind === "Substation" ? "substation" : "boundary", sev: worst(n.findings),
        size: Math.min(26, 10 + 2 * Math.sqrt(size)), ring: ring(n.findings), band: band(n.kV[0]),
      },
    };
  });
  const edges = g.edges.map((e) => ({
    group: "edges" as const,
    data: {
      id: `e:${e.id}`, source: e.source, target: e.target, label: e.label, sev: worst(e.findings), band: band(e.kV),
      width: e.kV === null ? 1.2 : e.kV >= 300 ? 2.4 : e.kV >= 200 ? 1.8 : 1.2,
    },
  }));
  return [...nodes, ...edges];
}

/** How an item is drawn: a connectivity node as a dot, a busbar as a bar, the rest as a symbol. */
function kindOf(it: Item): string {
  if (it.junction) return "junction";
  if (!it.far.length && it.class === "BusbarSection") return "busbar";
  return "symbol";
}

/** An item's symbol: by its class, and a switch by its state; equipment reaching another node, a line. */
function symbolFor(it: Item, terminals: number): Symbol {
  if (it.far.length) return "line";
  const s = symbolOf(it.class) ?? (it.open !== null ? "breaker" : "other");
  if (s === "breaker") return it.open ? "breakerOpen" : "breaker";
  if (s === "disconnector") return it.open ? "disconnectorOpen" : "disconnector";
  if (s === "transformer" && terminals >= 3) return "transformer3";
  return s;
}

function insideElements(s: Inside): ElementDefinition[] {
  const kV = new Map(s.groups.map((g) => [g.id, g.kV]));
  const bandOf = new Map(s.items.map((it) => [it.id, it.group ? band(kV.get(it.group)) : "none"]));
  const terminals = new Map<string, number>();
  for (const l of s.links) terminals.set(l.source, (terminals.get(l.source) ?? 0) + 1);
  // Equipment in no voltage level (a line, in its Line): the level of the node it ends at.
  for (const l of s.links) {
    if (bandOf.get(l.source) === "none" && bandOf.get(l.target) !== "none") bandOf.set(l.source, bandOf.get(l.target) as string);
  }
  const groups = s.groups.map((g) => ({
    group: "nodes" as const,
    data: {
      id: `g:${g.id}`, kind: "group", sev: "none", ring: 0, band: band(g.kV),
      // A level is often named by its voltage: "400.0".
      label: g.kV === null ? g.label : Number(g.label) === g.kV ? `${g.kV} kV` : `${g.label} · ${g.kV} kV`,
    },
  }));
  const items = s.items.map((it) => ({
    group: "nodes" as const,
    data: {
      id: it.id, label: it.label, kind: kindOf(it), symbol: symbolFor(it, terminals.get(it.id) ?? 0),
      sev: worst(it.findings), parent: it.group ? `g:${it.group}` : undefined,
      // A transformer joins levels: no one level's colour.
      band: /^transformer/.test(symbolFor(it, terminals.get(it.id) ?? 0)) ? "none" : bandOf.get(it.id),
      ring: ring(it.findings),
    },
    classes: [it.open ? "open" : "", it.inService === false ? "out" : ""].join(" ").trim(),
  }));
  const links = s.links.map((l) => ({
    group: "edges" as const,
    data: { id: `t:${l.id}`, source: l.source, target: l.target, sev: "none", width: 1, band: bandOf.get(l.target) },
    classes: l.connected === false ? "disconnected" : "",
  }));
  return [...groups, ...items, ...links];
}

function stylesheet(): cytoscape.StylesheetJson {
  const sev = {
    violation: colour("--vscode-errorForeground", "#f14c4c"),
    warning: colour("--vscode-editorWarning-foreground", "#cca700"),
    info: colour("--vscode-editorInfo-foreground", "#3794ff"),
    none: colour("--vscode-descriptionForeground", "#888888"),
  };
  const fg = colour("--vscode-foreground", "#cccccc");
  const bg = colour("--vscode-editor-background", "#1e1e1e");
  const fill = colour("--vscode-descriptionForeground", "#8b8b8b");
  const focus = colour("--vscode-focusBorder", "#007fd4");
  const bySeverity = (prop: string) =>
    Object.entries(sev).map(([k, c]) => ({ selector: `[sev = "${k}"]`, style: { [prop]: c } }));
  // Each voltage band's colour, on what `prop` paints of `selector`.
  const byBand = (selector: string, ...props: string[]) =>
    BANDS.map(([b, , v, fb]) => ({
      selector: `${selector}[band = "${b}"]`,
      style: Object.fromEntries(props.map((p) => [p, colour(v, fb)])),
    }));
  const shape = (kind: string, shape: string, width: number, height = width) =>
    ({ selector: `node[kind = "${kind}"]`, style: { shape, width, height, "font-size": 7 } });
  return [
    {
      selector: "node",
      style: {
        label: "data(label)", color: fg, "font-size": 9, "text-valign": "bottom", "text-margin-y": 2,
        // Legible over the lines it sits on.
        "text-outline-color": bg, "text-outline-width": 2, "text-outline-opacity": 0.85,
        width: "data(size)", height: "data(size)", "background-color": fill, "border-width": "data(ring)",
        "min-zoomed-font-size": 7,
      },
    },
    ...byBand('node[kind = "substation"]', "background-color"),
    { selector: 'node[kind = "boundary"]', style: { shape: "diamond", width: 8, height: 8, "font-size": 7 } },
    // Inside a substation: voltage levels as boxes, and a shape for each kind of equipment.
    {
      selector: 'node[kind = "group"]',
      style: {
        shape: "round-rectangle", "background-color": fill, "background-opacity": 0.05, "border-width": 1, "border-color": fill,
        "border-opacity": 0.6, "text-valign": "top", "text-margin-y": -3, "font-size": 10, padding: "12px",
      },
    },
    ...byBand('node[kind = "group"]', "border-color", "background-color", "color"),
    shape("junction", "ellipse", 4),
    shape("busbar", "rectangle", 36, 4),
    { selector: 'node[kind = "junction"]', style: { "font-size": 6, "text-opacity": 0.7 } },
    ...byBand('node[kind = "junction"]', "background-color"),
    ...byBand('node[kind = "busbar"]', "background-color"),
    // Equipment: its symbol, in its voltage's colour, on a transparent disc
    // (which carries the findings' ring, and the selection).
    {
      selector: 'node[kind = "symbol"]',
      style: {
        shape: "ellipse", width: 16, height: 16, "font-size": 7, "background-opacity": 0, "background-fit": "contain",
        "background-clip": "none",
        "background-image": (ele: cytoscape.NodeSingular) => {
          const b = BANDS.find(([name]) => name === ele.data("band"));
          const c = b ? colour(b[2], b[3]) : fill;
          return `data:image/svg+xml;utf8,${encodeURIComponent(symbolSvg(ele.data("symbol") as Symbol, c, 32))}`;
        },
      },
    },
    { selector: 'node[symbol ^= "transformer"]', style: { width: 18, height: 18 } },
    ...bySeverity("border-color").map((s) => ({ ...s, selector: `node[kind != "group"]${s.selector}` })),
    // And a halo, which reads apart from a voltage colour of the same hue.
    ...bySeverity("underlay-color").filter((s) => !s.selector.includes('"none"')).map((s) => ({
      ...s, selector: `node[kind != "group"]${s.selector}`,
      style: { ...s.style, "underlay-opacity": 0.35, "underlay-padding": 4, "underlay-shape": "ellipse" },
    })),

    { selector: "node.out", style: { opacity: 0.45, "border-style": "dashed" } },
    {
      selector: "edge",
      style: { width: "data(width)", "curve-style": "bezier", "line-color": fill, opacity: 0.9 },
    },
    ...byBand("edge", "line-color"),
    // Findings on a line: a halo in the worst one's colour, apart from its voltage colour.
    ...bySeverity("underlay-color").filter((s) => !s.selector.includes('"none"'))
      .map((s) => ({ ...s, selector: `edge${s.selector}`, style: { ...s.style, "underlay-opacity": 0.4, "underlay-padding": 5 } })),
    { selector: "edge.disconnected", style: { "line-style": "dashed" } },
    { selector: "node:selected", style: { "background-color": focus, "background-opacity": 1 } },
    { selector: 'node[kind = "group"]:selected', style: { "background-opacity": 0.15 } },
    { selector: 'node[kind = "symbol"]:selected', style: { "background-opacity": 0.3 } },
    { selector: "edge:selected", style: { "line-color": focus, opacity: 1 } },
    { selector: ".dim", style: { opacity: 0.15 } },
  ] as cytoscape.StylesheetJson;
}

function counts(f: Findings): string {
  return `${f.violation} violation(s), ${f.warning} warning(s), ${f.info} info`;
}

function sum(fs: Findings[], start: Findings): Findings {
  return fs.reduce((a, x) => ({ violation: a.violation + x.violation, warning: a.warning + x.warning, info: a.info + x.info }), { ...start });
}

const NONE: Findings = { violation: 0, warning: 0, info: 0 };

function gridSummary(g: Grid): string {
  const subs = g.nodes.filter((n) => n.kind === "Substation").length;
  const all = sum([...g.nodes, ...g.edges].map((x) => x.findings), g.elsewhere);
  return `${subs} substation(s), ${g.nodes.length - subs} boundary point(s), ${g.edges.length} connection(s) · ` +
    `${counts(all)} · ${total(g.elsewhere)} not on the grid`;
}

function insideSummary(s: Inside): string {
  const equipment = s.items.filter((it) => !it.junction);
  const switches = equipment.filter((it) => it.open !== null);
  const open = switches.filter((it) => it.open).length;
  const all = sum(s.items.map((it) => it.findings), s.findings);
  return `${s.label}: ${equipment.length} equipment, ${s.items.length - equipment.length} node(s), ` +
    `${switches.length} switch(es)${open ? ` (${open} open)` : ""} · ${counts(all)}`;
}

function escape(s: string): string {
  return s.replace(/[&<>"]/g, (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" })[c] as string);
}

function findingsHtml(f: Findings): string {
  if (!total(f)) {
    return "<div>No findings</div>";
  }
  return `<div><span class="violation">${f.violation} violation(s)</span>, <span class="warning">${f.warning} warning(s)</span>, ` +
    `<span class="info">${f.info} info</span></div>`;
}

/** Fill the side panel, with an *Open definition* button for `location` and then `buttons`. */
function panel(html: string, location: Location | null | undefined, buttons: [string, () => void][] = []): void {
  const info = document.getElementById("info") as HTMLElement;
  if (location) {
    buttons = [["Open definition", () => vscode.postMessage({ type: "open", location })], ...buttons];
  }
  info.innerHTML = html + buttons.map(() => "<button></button>").join("");
  info.querySelectorAll("button").forEach((b, i) => {
    b.textContent = buttons[i][0];
    b.addEventListener("click", buttons[i][1]);
  });
}

function showGrid(g: Grid, target: NodeSingular | EdgeSingular | undefined): void {
  if (!target) {
    panel(`<div class="sub">Select a substation or a line. Double-click a substation to see inside it, a line to open where it is written.</div>` + legend(), null);
    return;
  }
  if (target.isNode()) {
    const n = g.nodes.find((x) => x.id === target.id());
    if (!n) return;
    const kv = n.kV.length ? `${n.kV.join(" / ")} kV` : "";
    const equipment = Object.entries(n.equipment)
      .sort((a, b) => b[1] - a[1])
      .map(([k, v]) => `<tr><td>${escape(k)}</td><td class="n">${v}</td></tr>`)
      .join("");
    panel(
      `<h3>${escape(n.label || n.id)}</h3><div class="sub">${escape(n.kind)}${n.region ? ` · ${escape(n.region)}` : ""}${kv ? ` · ${kv}` : ""}</div>` +
        findingsHtml(n.findings) + (equipment ? `<table>${equipment}</table>` : ""),
      n.location,
      n.kind === "Substation" ? [["Show inside", () => enter(n.id)]] : [],
    );
  } else {
    const e = g.edges.find((x) => `e:${x.id}` === target.id());
    if (!e) return;
    const end = (id: string) => escape(g.nodes.find((x) => x.id === id)?.label || id);
    panel(
      `<h3>${escape(e.label)}</h3><div class="sub">${escape(e.class)}${e.kV !== null ? ` · ${e.kV} kV` : ""}</div>` +
        `<div>${end(e.source)} — ${end(e.target)}</div>` + findingsHtml(e.findings),
      e.location,
    );
  }
}

function showInside(s: Inside, target: NodeSingular | EdgeSingular | undefined): void {
  const id = target?.isNode() ? target.id() : undefined;
  const group = s.groups.find((g) => `g:${g.id}` === id);
  if (group) {
    const held = s.items.filter((x) => x.group === group.id);
    panel(
      `<h3>${escape(group.label)}</h3><div class="sub">VoltageLevel${group.kV !== null ? ` · ${group.kV} kV` : ""}</div>` +
        findingsHtml(sum(held.map((x) => x.findings), NONE)) +
        `<div>${held.filter((x) => !x.junction).length} equipment, ${held.filter((x) => x.junction).length} node(s)</div>`,
      null,
    );
    return;
  }
  const it = s.items.find((x) => x.id === id);
  if (!it) {
    const kv = s.kV.length ? ` · ${s.kV.join(" / ")} kV` : "";
    panel(
      `<h3>${escape(s.label)}</h3><div class="sub">Substation${s.region ? ` · ${escape(s.region)}` : ""}${kv}</div>` +
        `<div class="sub">On the substation, its voltage levels and bays:</div>` + findingsHtml(s.findings) +
        `<div class="sub" style="margin-top:8px">Select equipment or a node; double-click to open where it is written, ` +
        `a line to go to its other end. An open switch is drawn open (a hollow breaker, a raised blade); faded: out of service; dashed: a disconnected terminal.</div>` + legend(),
      s.location,
      [["Back to the grid", leave]],
    );
    return;
  }
  const state = [it.open === true ? "open" : it.open === false ? "closed" : "", it.inService === false ? "out of service" : ""]
    .filter((x) => x)
    .join(", ");
  const level = s.groups.find((g) => g.id === it.group)?.label;
  const far = it.far.length ? `<div>To ${it.far.map((f) => escape(f.label || f.id)).join(", ")}</div>` : "";
  panel(
    `<h3>${escape(it.label)}</h3><div class="sub">${escape(it.class)}${level ? ` · ${escape(level)}` : ""}${state ? ` · ${state}` : ""}</div>` +
      far + findingsHtml(it.findings),
    it.location,
    it.far.filter((f) => f.kind === "Substation").map((f): [string, () => void] => [`Show ${f.label || f.id}`, () => enter(f.id)]),
  );
}

function show(target: NodeSingular | EdgeSingular | undefined): void {
  if (inside) showInside(inside, target);
  else if (grid) showGrid(grid, target);
}

function open(target: NodeSingular | EdgeSingular): void {
  if (inside) {
    const it = inside.items.find((x) => x.id === target.id());
    const far = it?.far.filter((f) => f.kind === "Substation") ?? [];
    if (far.length === 1) {
      enter(far[0].id);
    } else if (it?.location) {
      vscode.postMessage({ type: "open", location: it.location });
    }
    return;
  }
  if (target.isNode() && target.data("kind") === "substation") {
    enter(target.id());
    return;
  }
  const location = target.isNode()
    ? grid?.nodes.find((x) => x.id === target.id())?.location
    : grid?.edges.find((x) => `e:${x.id}` === target.id())?.location;
  if (location) {
    vscode.postMessage({ type: "open", location });
  }
}

/** Show what substation `id` holds; the extension answers with an `inside` message. */
function enter(id: string): void {
  if (!inside && cy) {
    gridView = { zoom: cy.zoom(), pan: { ...cy.pan() }, selected: id };
  }
  vscode.postMessage({ type: "enter", id });
}

function leave(): void {
  if (!inside) return;
  vscode.postMessage({ type: "leave" });
  inside = undefined;
  if (grid) draw();
}

/** The scene drawn: its layouts, its nodes' ids, the pairs of nodes its edges join, and its routed edges. */
function scene(): { layouts: Layout[]; auto?: Layout; ids: string[]; pairs: [string, string][]; routed: { id: string; key: string }[] } {
  if (inside) {
    return {
      layouts: inside.layouts,
      ids: inside.items.map((it) => it.id),
      pairs: inside.links.map((l): [string, string] => [l.source, l.target]),
      routed: [],
    };
  }
  const g = grid as Grid;
  return {
    layouts: g.layouts,
    auto: g.auto ?? undefined,
    ids: g.nodes.map((n) => n.id),
    pairs: g.edges.map((e): [string, string] => [e.source, e.target]),
    routed: g.edges.map((e) => ({ id: `e:${e.id}`, key: e.id })),
  };
}

/** The layout to draw: the one picked, else the TSO's diagram, else the map, else automatic. */
function choice(layouts: Layout[]): string {
  const picked = inside ? chosen.inside : chosen.grid;
  if (picked && (picked === "auto" || layouts.some((l) => l.id === picked))) {
    return picked;
  }
  return (layouts.find((l) => l.kind === "diagram") ?? layouts[0])?.id ?? "auto";
}

/** Screen coordinates of a layout's positions and routes, scaled to the scene's size. */
function place(ids: string[], pairs: [string, string][], l: Layout): { nodes: Map<string, Point>; routes: Map<string, Point[]> } {
  const raw = Object.values(l.nodes);
  let project = (p: [number, number]): Point => ({ x: p[0], y: l.flipY ? -p[1] : p[1] });
  if (l.kind === "geographic") {
    // Longitude shrinks with latitude; north is up.
    const lat = raw.reduce((a, p) => a + p[1], 0) / raw.length;
    const k = Math.cos((lat * Math.PI) / 180);
    project = (p) => ({ x: p[0] * k, y: -p[1] });
  }
  const points = raw.map(project);
  const minX = Math.min(...points.map((p) => p.x));
  const minY = Math.min(...points.map((p) => p.y));
  const span = Math.max(Math.max(...points.map((p) => p.x)) - minX, Math.max(...points.map((p) => p.y)) - minY, 1e-9);
  const scale = Math.max(900, 70 * Math.sqrt(ids.length)) / span;
  const screen = (p: [number, number]): Point => {
    const q = project(p);
    return { x: (q.x - minX) * scale, y: (q.y - minY) * scale };
  };
  const nodes = new Map(Object.entries(l.nodes).map(([id, p]) => [id, screen(p)]));
  const routes = new Map(Object.entries(l.edges).map(([id, ps]) => [id, ps.map(screen)]));

  // A node the layout does not place goes among its placed neighbours, else below the rest.
  const neighbours = new Map<string, string[]>();
  for (const [a, b] of pairs) {
    neighbours.set(a, [...(neighbours.get(a) ?? []), b]);
    neighbours.set(b, [...(neighbours.get(b) ?? []), a]);
  }
  let missing = ids.filter((id) => !nodes.has(id));
  for (let round = 0; round < 5 && missing.length; round++) {
    missing.forEach((id, i) => {
      const known = (neighbours.get(id) ?? []).map((n) => nodes.get(n)).filter((p): p is Point => !!p);
      if (known.length) {
        const angle = (i * 2.4) % (2 * Math.PI);
        nodes.set(id, {
          x: known.reduce((a, p) => a + p.x, 0) / known.length + 25 * Math.cos(angle),
          y: known.reduce((a, p) => a + p.y, 0) / known.length + 25 * Math.sin(angle),
        });
      }
    });
    missing = missing.filter((id) => !nodes.has(id));
  }
  const bottom = Math.max(0, ...[...nodes.values()].map((p) => p.y)) + 80;
  missing.forEach((id, i) => nodes.set(id, { x: (i % 20) * 50, y: bottom + Math.floor(i / 20) * 50 }));
  return { nodes, routes };
}

/** Bend an edge along its route: Cytoscape's segment points, relative to the line between its nodes. */
function route(edge: EdgeSingular, points: Point[]): void {
  const s = edge.source().position();
  const t = edge.target().position();
  const dx = t.x - s.x;
  const dy = t.y - s.y;
  const length = Math.hypot(dx, dy);
  const d = (a: Point, b: Point) => Math.hypot(a.x - b.x, a.y - b.y);
  if (points.length < 3 || length < 1e-6) {
    edge.removeStyle();
    return;
  }
  // The route may run from the target's end.
  const ordered = d(points[0], t) < d(points[0], s) ? [...points].reverse() : points;
  const inner = ordered.slice(1, -1);
  const weights = inner.map((p) => ((p.x - s.x) * dx + (p.y - s.y) * dy) / (length * length));
  const distances = inner.map((p) => ((p.y - s.y) * dx - (p.x - s.x) * dy) / length);
  edge.style({
    "curve-style": "segments",
    "edge-distances": "node-position",
    "segment-weights": weights.join(" "),
    "segment-distances": distances.join(" "),
  });
}

function arrange(): void {
  if (!cy) return;
  const { layouts, auto, ids, pairs, routed } = scene();
  const picked = choice(layouts);
  // Automatic: the server's layout for the grid; Cytoscape's inside a substation (it boxes the voltage levels).
  const layout = layouts.find((l) => l.id === picked) ?? (picked === "auto" ? auto : undefined);
  cy.edges().removeStyle();
  if (!layout) {
    // `cose` is quadratic per iteration: past a few hundred nodes, a plain grid rather than a hang.
    cy.layout(ids.length > 1000 ? { name: "grid", animate: false } : {
      name: "cose", animate: false, nodeRepulsion: () => 8000, idealEdgeLength: () => (inside ? 30 : 60),
      numIter: ids.length > 500 ? 400 : 1000,
    }).run();
    return;
  }
  const { nodes, routes } = place(ids, pairs, layout);
  cy.layout({ name: "preset", positions: Object.fromEntries(nodes), fit: true, padding: 30 }).run();
  for (const e of routed) {
    const points = routes.get(e.key);
    if (points) {
      route(cy.getElementById(e.id) as EdgeSingular, points);
    }
  }
}

/** The layout picker: shown when the scene has a diagram or locations. */
function picker(layouts: Layout[]): void {
  const select = document.getElementById("layout") as HTMLSelectElement;
  const options = [...layouts.map((l) => [l.id, l.name]), ["auto", "Automatic"]];
  select.innerHTML = options.map(([v, n]) => `<option value="${escape(v)}">${escape(n)}</option>`).join("");
  select.value = choice(layouts);
  select.style.display = layouts.length ? "" : "none";
}

/** What is drawn: the substation shown, else the grid; and its size, to tell a refresh from a new scene. */
type Shown = { kind: string; id: string; nodes: number; edges: number };

function shown(): Shown {
  return inside
    ? { kind: "inside", id: inside.id, nodes: inside.items.length + inside.groups.length, edges: inside.links.length }
    : { kind: "grid", id: "", nodes: grid?.nodes.length ?? 0, edges: grid?.edges.length ?? 0 };
}

/** Draw the substation shown, else the grid. A refresh of what `previous` drew keeps its layout. */
function draw(previous?: Shown): void {
  const els = inside ? insideElements(inside) : gridElements(grid as Grid);
  (document.getElementById("summary") as HTMLElement).textContent = inside ? insideSummary(inside) : gridSummary(grid as Grid);
  (document.getElementById("back") as HTMLElement).style.display = inside ? "inline-block" : "none";
  (document.getElementById("search") as HTMLInputElement).placeholder = inside ? "Find equipment" : "Find a substation";
  const now = shown();
  const same = cy && previous && previous.kind === now.kind && previous.id === now.id &&
    previous.nodes === now.nodes && previous.edges === now.edges && els.every((el) => cy?.getElementById(el.data.id as string).nonempty());
  if (cy && same) {
    // Only findings and states changed: keep the layout.
    for (const el of els) {
      const at = cy.getElementById(el.data.id as string);
      at.data(el.data);
      if (typeof el.classes === "string") at.classes(el.classes);
    }
    const selected = cy.$(":selected").first();
    show(selected.nonempty() ? (selected as NodeSingular | EdgeSingular) : undefined);
    return;
  }
  cy?.destroy();
  cy = cytoscape({
    container: document.getElementById("graph"),
    elements: els,
    style: stylesheet(),
    layout: { name: "preset" },
    // Large grids: draw a snapshot while panning and zooming.
    textureOnViewport: els.length > 3000,
    hideEdgesOnViewport: els.length > 3000,
    minZoom: 0.05,
    maxZoom: 4,
  });
  picker(scene().layouts);
  arrange();
  if (!inside && gridView) {
    // Back from a substation: the grid as it was, with that substation selected.
    cy.zoom(gridView.zoom);
    cy.pan(gridView.pan);
    cy.getElementById(gridView.selected).select();
    gridView = undefined;
  }
  cy.on("select", "node, edge", (e) => show(e.target));
  cy.on("unselect", () => show(undefined));
  cy.on("dbltap", "node, edge", (e) => open(e.target));
  const selected = cy.$(":selected").first();
  show(selected.nonempty() ? (selected as NodeSingular | EdgeSingular) : undefined);
}

// Restyle when the colour theme changes.
new MutationObserver(() => cy?.style(stylesheet())).observe(document.body, { attributes: true, attributeFilter: ["class"] });

(document.getElementById("layout") as HTMLSelectElement).addEventListener("change", (e) => {
  const value = (e.target as HTMLSelectElement).value;
  if (inside) chosen.inside = value;
  else chosen.grid = value;
  arrange();
});

(document.getElementById("back") as HTMLButtonElement).addEventListener("click", leave);
window.addEventListener("keydown", (e) => {
  if (e.key === "Escape" && document.activeElement?.id !== "search") leave();
});

(document.getElementById("search") as HTMLInputElement).addEventListener("input", (e) => {
  if (!cy) return;
  const q = (e.target as HTMLInputElement).value.trim().toLowerCase();
  cy.elements().removeClass("dim");
  if (!q) return;
  const hits = cy.nodes().filter((n) => String(n.data("label")).toLowerCase().includes(q));
  cy.elements().not(hits).not(hits.ancestors()).addClass("dim");
  if (hits.nonempty()) {
    cy.animate({ fit: { eles: hits, padding: 80 } }, { duration: 200 });
  }
});

window.addEventListener("message", (e) => {
  const previous = cy ? shown() : undefined;
  if (e.data?.type === "grid") {
    grid = e.data.grid as Grid;
    // Behind a substation, the grid is drawn on the way back.
    if (!inside) draw(previous);
  } else if (e.data?.type === "inside") {
    // `null`: the substation is gone from the set.
    inside = (e.data.inside as Inside | null) ?? undefined;
    if (inside || grid) draw(previous);
  } else if (e.data?.type === "leave" && inside) {
    inside = undefined;
    if (grid) draw();
  } else if (e.data?.type === "select") {
    select(String(e.data.id));
  }
});

/** Select `id` and bring it into view with what it joins. */
function select(id: string): void {
  const el = cy?.getElementById(id);
  if (!cy || !el || el.empty()) return;
  cy.$(":selected").unselect();
  el.select();
  const near = el.isEdge() ? el.union(el.connectedNodes()) : (el as NodeSingular).closedNeighborhood();
  cy.animate({ fit: { eles: near, padding: 80 } }, { duration: 200 });
}
vscode.postMessage({ type: "ready" });
