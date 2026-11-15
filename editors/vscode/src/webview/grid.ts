// The grid view's page: draws `cimoxide/grid` with Cytoscape.js, coloured by
// each substation's and line's worst finding, in VS Code's theme colours; and,
// double-clicking a substation, what it holds (`cimoxide/substation`).

import cytoscape, { Core, ElementDefinition, NodeSingular, EdgeSingular } from "cytoscape";
import { Symbol, symbolOf, symbolSvg } from "../symbols";
import { keepDecluttered } from "./declutter";
import { panFromNodes, panWithOtherButtons } from "./pan";

type ValuesShown = "all" | "exceptions" | "off";
declare function acquireVsCodeApi(): { postMessage(msg: unknown): void; getState(): { valuesShown?: ValuesShown } | undefined; setState(state: { valuesShown?: ValuesShown }): void };
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
  /** Node id → its width and height, where the layout sizes it (a busbar across its bays). */
  sizes?: Record<string, [number, number]>;
  /** Pixels per unit, for positions drawn as they are; else they are scaled to the scene's size. */
  unit?: number | null;
  /** Edge id → how many line spacings aside it runs from its route, left of its way (negative: right). */
  tracks?: Record<string, number>;
}

/** How far apart, in pixels, lines sharing a stretch run: the widest line and a gap. */
const TRACK = 5;

/**
 * `points` (on the screen) drawn `by` pixels to the left of their way (negative: right), corners
 * mitred so that each piece stays parallel to its own; the ends joined back to where they were.
 */
function aside(points: Point[], by: number): Point[] {
  if (!by || points.length < 2) return points;
  const normals = points.slice(1).map((p, i) => {
    const [dx, dy] = [p.x - points[i].x, p.y - points[i].y];
    const l = Math.hypot(dx, dy) || 1;
    return { x: -dy / l, y: dx / l };
  });
  const shifted = points.map((p, i) => {
    const [a, b] = [normals[Math.max(0, i - 1)], normals[Math.min(normals.length - 1, i)]];
    const k = by / (1 + a.x * b.x + a.y * b.y);
    return { x: p.x + (a.x + b.x) * k, y: p.y + (a.y + b.y) * k };
  });
  return [points[0], ...shifted, points[points.length - 1]];
}

interface Grid {
  files: string[];
  nodes: GridNode[];
  edges: GridEdge[];
  elsewhere: Findings;
  layouts: Layout[];
  /** The automatic layout, computed by the server. */
  auto: Layout | null;
  /** The state the set's SV publishes, over the grid; `null` without one. */
  state?: Overlay | null;
}

/** A state of the grid, by what the grid draws: as a power flow overlays it. */
interface Overlay {
  /** Each grid node's lowest and highest voltage, p.u. */
  nodes: Record<string, { min: number; max: number }>;
  /** By topological and connectivity node. */
  voltages: Record<string, { pu: number; kV: number; deg: number }>;
  /** By equipment: each terminal's flow into it (MW, Mvar), its current, and the highest loading against a PATL, %. */
  flows: Record<string, { ends: { terminal: string; p: number; q: number; amps: number | null; loading: number | null }[]; loading: number | null; towards?: string | null }>;
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
  /** The automatic layout, a single-line diagram, computed by the server. */
  auto: Layout | null;
}

type Point = { x: number; y: number };

/** The layout picked in this panel for the grid, and for substations: a layout id, or "auto". */
const chosen: { grid?: string; inside?: string } = {};

let cy: Core | undefined;
let grid: Grid | undefined;
/** The substation shown, if any; else the grid. */
let inside: Inside | undefined;
/** What the scene is coloured by: voltage levels, or the SV's state where the set has one. */
let colourBy: "voltage" | "state" = "voltage";
/** The SV's values beside what they describe, on or off apart from the colouring. */
/** The SV's values beside what they describe: all, only those that need attention (loaded from 80 %, 5 % off nominal), or none. */
let valuesShown: ValuesShown = vscode.getState()?.valuesShown ?? "exceptions";
/** The grid's viewport when a substation was opened, to return to. */
let gridView: { zoom: number; pan: Point; selected: string } | undefined;

const style = document.createElement("style");
style.textContent = `
  html, body { height: 100%; margin: 0; padding: 0; overflow: hidden; }
  body { display: flex; flex-direction: column; color: var(--vscode-foreground); background: var(--vscode-editor-background);
         font-family: var(--vscode-font-family); font-size: var(--vscode-font-size); }
  #bar { display: flex; gap: 12px; align-items: center; padding: 6px 10px; border-bottom: 1px solid var(--vscode-panel-border); }
  #hover { position: fixed; z-index: 5; max-width: 280px; padding: 4px 8px; pointer-events: none; display: none;
           color: var(--vscode-editorHoverWidget-foreground); background: var(--vscode-editorHoverWidget-background);
           border: 1px solid var(--vscode-editorHoverWidget-border); }
  #hover div { color: var(--vscode-descriptionForeground); }
  #layout, #colour, #values { color: var(--vscode-dropdown-foreground); background: var(--vscode-dropdown-background);
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
  #edit, #palette { display: flex; gap: 6px; align-items: center; flex-wrap: wrap; }
  #palette { padding: 4px 10px; border-bottom: 1px solid var(--vscode-panel-border); overflow-x: auto; flex-wrap: nowrap; }
  #edit button, #palette button { color: var(--vscode-button-secondaryForeground); background: var(--vscode-button-secondaryBackground);
          border: none; padding: 3px 10px; cursor: pointer; flex: none; }
  #edit button:hover, #palette button:hover { background: var(--vscode-button-secondaryHoverBackground); }
  #palette button[aria-pressed="true"] { outline: 1px solid var(--vscode-focusBorder); }
  #palette select { color: var(--vscode-dropdown-foreground); background: var(--vscode-dropdown-background); flex: none; }
  #edit .hint { color: var(--vscode-descriptionForeground); }
  #edit .failed { color: var(--vscode-errorForeground); }
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

/** A voltage's band: within 5 % of nominal, within 10 %, or beyond. */
function voltageBand(min: number, max: number): "ok" | "warn" | "bad" {
  const off = Math.max(Math.abs(min - 1), Math.abs(max - 1));
  return off <= 0.05 ? "ok" : off <= 0.1 ? "warn" : "bad";
}

/** A loading's band: below 80 % of its PATL, up to it, or past it; `unknown` without a limit. */
function loadingBand(loading: number | null): "ok" | "warn" | "bad" | "unknown" {
  return loading === null ? "unknown" : loading < 80 ? "ok" : loading <= 100 ? "warn" : "bad";
}

const STATE_COLOURS: Record<string, [string, string]> = {
  ok: ["--vscode-testing-iconPassed", "#73c991"],
  warn: ["--vscode-editorWarning-foreground", "#cca700"],
  bad: ["--vscode-errorForeground", "#f14c4c"],
};

/** How the state colours the scene: by `pf`, which only a state sets. */
function stateBands(): cytoscape.StylesheetJson {
  const fill = colour("--vscode-descriptionForeground", "#8b8b8b");
  return [
    ...Object.entries(STATE_COLOURS).flatMap(([b, [v, fb]]) => {
      const c = colour(v, fb);
      return [
        { selector: `node[kind != "symbol"][kind != "group"][pf = "${b}"]`, style: { "background-color": c } },
        { selector: `node[kind = "symbol"][pf = "${b}"]`, style: { "underlay-color": c, "underlay-opacity": 0.45, "underlay-padding": 4, "underlay-shape": "ellipse" } },
        { selector: `edge[pf = "${b}"]`, style: { "line-color": c, "mid-target-arrow-color": c, "mid-source-arrow-color": c } },
      ];
    }),
    { selector: '[pf = "none"], [pf = "unknown"]', style: { "background-color": fill, "line-color": fill } },
    { selector: '[pf = "none"]', style: { opacity: 0.5 } },
    {
      selector: "edge[flowLabel]",
      style: {
        label: "data(flowLabel)", "font-size": 7, color: colour("--vscode-foreground", "#cccccc"), "text-rotation": "autorotate",
        "text-outline-color": colour("--vscode-editor-background", "#1e1e1e"), "text-outline-width": 2, "min-zoomed-font-size": 6,
      },
    },
  ] as cytoscape.StylesheetJson;
}

/** What a scene's element shows of a state: its band, and a line of text. `null` if nothing. */
function stateOf(id: string, kind: string | undefined, o: Overlay): { band: string; text: string } | null {
  const pu = (x: number) => x.toFixed(3);
  const flow = (f: Overlay["flows"][string] | undefined) => {
    if (!f) return null;
    const p = Math.max(...f.ends.map((e) => Math.abs(e.p)));
    return { band: loadingBand(f.loading), text: f.loading === null ? `${p.toFixed(0)} MW` : `${f.loading.toFixed(0)} %` };
  };
  if (id.startsWith("e:")) return flow(o.flows[id.slice(2).split("#")[0]]);
  if (kind === "substation" || kind === "boundary") {
    const v = o.nodes[id];
    return v ? { band: voltageBand(v.min, v.max), text: v.min === v.max ? `${pu(v.min)} pu` : `${pu(v.min)}–${pu(v.max)} pu` } : null;
  }
  if (kind === "junction" || kind === "busbar") {
    const v = o.voltages[id];
    return v ? { band: voltageBand(v.pu, v.pu), text: `${pu(v.pu)} pu` } : null;
  }
  if (kind === "symbol") return flow(o.flows[id]);
  return null;
}

/** An element's state, for the side panel. */
function stateHtml(id: string): string {
  const lines = stateLines(id);
  return lines.length ? `<div class="sub" style="margin-top:8px">From the SV:</div>${lines.map((l) => `<div>${escape(l)}</div>`).join("")}` : "";
}

/** An element's state, as lines of text. */
function stateLines(id: string): string[] {
  const o = grid?.state;
  if (!o) return [];
  const lines: string[] = [];
  const v = o.nodes[id] ?? (o.voltages[id] ? { min: o.voltages[id].pu, max: o.voltages[id].pu } : undefined);
  if (v) lines.push(v.min === v.max ? `Voltage ${v.min.toFixed(4)} p.u.` : `Voltage ${v.min.toFixed(4)} to ${v.max.toFixed(4)} p.u.`);
  const kv = o.voltages[id];
  if (kv) lines.push(`${kv.kV.toFixed(2)} kV, ${kv.deg.toFixed(2)}°`);
  for (const [i, e] of (o.flows[id]?.ends ?? []).entries()) {
    const amps = e.amps === null ? "" : `, ${e.amps.toFixed(0)} A`;
    const loading = e.loading === null ? "" : `, ${e.loading.toFixed(1)} % of its PATL`;
    lines.push(`End ${i + 1}: ${e.p.toFixed(1)} MW, ${e.q.toFixed(1)} Mvar into it${amps}${loading}`);
  }
  return lines;
}

/** The legend of the state's colours, for the side panel. */
function stateLegend(): string {
  const swatch = (b: string) => `<span style="display:inline-block;width:10px;height:10px;border-radius:5px;margin:0 6px -1px 0;background:${colour(...STATE_COLOURS[b])}"></span>`;
  return `<div class="sub" style="margin-top:12px">Coloured by the SV's state:` +
    `<div>${swatch("ok")}within 5 % of nominal, loaded below 80 %</div>` +
    `<div>${swatch("warn")}within 10 %, up to its PATL</div>` +
    `<div>${swatch("bad")}beyond</div>` +
    `<div>Grey: not in the SV, or no limit</div></div>`;
}

/** The state shown: the SV's, if the set has one and it is chosen. */
function shownState(): Overlay | null {
  return colourBy === "state" ? grid?.state ?? null : null;
}

/**
 * Colours the scene by the SV's state, or as drawn; and, apart from that, adds its values to the
 * labels: each element keeps its own label as `base`, a value adds a line to it; on an edge, a
 * label of its own.
 */
function paintState(): void {
  if (!cy) return;
  const on = shownState();
  const values = valuesShown === "off" ? null : grid?.state ?? null;
  cy.batch(() => {
    cy?.elements().forEach((el) => {
      if (el.data("base") === undefined) el.data("base", el.data("label") ?? "");
      const base = el.data("base") as string;
      const kind = el.data("kind") as string | undefined;
      const band = on ? stateOf(el.id(), kind, on) : null;
      if (on && kind !== "group" && !el.id().startsWith("t:")) el.data("pf", band?.band ?? "none");
      else el.removeData("pf");
      const value = values ? stateOf(el.id(), kind, values) : null;
      const found = value && (valuesShown === "all" || value.band === "warn" || value.band === "bad") ? value : null;
      if (el.group() === "edges") {
        if (found) el.data("flowLabel", found.text);
        else el.removeData("flowLabel");
        // With its value, which way its power flows.
        const arrow = found && values && el.id().startsWith("e:") ? arrowOf(el.id(), el.data("source"), el.data("target"), values) : null;
        if (arrow) el.data("arrow", arrow);
        else el.removeData("arrow");
      } else el.data("label", found ? (base ? `${base}\n${found.text}` : found.text) : base);
    });
  });
  // Data removed does not restyle what was matched on it (`edge[flowLabel]`, `[pf]`).
  cy.style().update();
  const pick = document.getElementById("colour") as HTMLSelectElement;
  pick.style.display = grid?.state ? "" : "none";
  pick.value = colourBy;
  const shown = document.getElementById("values") as HTMLSelectElement;
  shown.style.display = grid?.state ? "" : "none";
  shown.value = valuesShown;
}

/** The legend of the bands, for the side panel. */
function legend(): string {
  const swatch = (c: string) => `<span style="display:inline-block;width:14px;height:3px;margin:0 6px 3px 0;background:${c}"></span>`;
  const rows = BANDS.map(([, min, v, fb], i) => `<div>${swatch(colour(v, fb))}${i ? `${min}–${BANDS[i - 1][1]}` : `≥ ${min}`} kV</div>`);
  if (shownState()) return stateLegend();
  return `<div class="sub" style="margin-top:12px">${rows.join("")}<div>${swatch(colour("--vscode-descriptionForeground", "#888"))}lower, or unknown</div></div>`;
}

/** A line's width by its voltage band: the backbone stands out, the distribution recedes. */
const LINE_WIDTH: Record<string, number> = { v300: 3.2, v200: 2.2, v100: 1.4, v30: 0.9, none: 0.8 };

/** Where a state's arrow on grid edge `id` (`e:…`) points: to its `target`, its `source`, or nowhere. */
function arrowOf(id: string, source: string, target: string, o: Overlay): "source" | "target" | null {
  const towards = o.flows[id.slice(2).split("#")[0]]?.towards;
  return towards === target ? "target" : towards === source ? "source" : null;
}

function gridElements(g: Grid): ElementDefinition[] {
  const nodes = g.nodes.map((n) => {
    const size = Object.values(n.equipment).reduce((a, b) => a + b, 0);
    // A substation as a box, as tall as it holds, a little wider for each voltage level more.
    const height = Math.min(16, 7 + 1.1 * Math.sqrt(size));
    return {
      group: "nodes" as const,
      data: {
        id: n.id, label: n.label, kind: n.kind === "Substation" ? "substation" : "boundary", sev: worst(n.findings),
        size: height, width: height + 5 * Math.max(0, Math.min(3, n.kV.length - 1)), band: band(n.kV[0]),
      },
    };
  });
  const edges = g.edges.map((e) => ({
    group: "edges" as const,
    data: {
      id: `e:${e.id}`, source: e.source, target: e.target, label: e.label, sev: worst(e.findings), band: band(e.kV),
      width: LINE_WIDTH[band(e.kV)],
    },
  }));
  return [...nodes, ...edges];
}

/** How an item is drawn: a connectivity node as a dot, a busbar as a bar, the rest as a symbol. */
/**
 * How much an item's name matters inside its substation, so that the more it does, the sooner it
 * is named as one zooms in: a busbar, a line or a transformer (`main`), other equipment, a switch,
 * a connectivity node.
 */
function roleOf(it: Item, symbol: string): "main" | "equipment" | "switch" | "node" {
  if (it.junction) return "node";
  if (it.class === "BusbarSection" || it.far.length || /^transformer/.test(symbol)) return "main";
  return it.open !== null ? "switch" : "equipment";
}

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
      role: roleOf(it, symbolFor(it, terminals.get(it.id) ?? 0)),
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
  const fg = colour("--vscode-foreground", "#cccccc");
  const bg = colour("--vscode-editor-background", "#1e1e1e");
  const fill = colour("--vscode-descriptionForeground", "#8b8b8b");
  const focus = colour("--vscode-focusBorder", "#007fd4");
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
        label: "data(label)", color: fg, "font-size": 9, "text-valign": "bottom", "text-margin-y": 2, "text-wrap": "wrap",
        // Legible over the lines it sits on.
        "text-outline-color": bg, "text-outline-width": 2, "text-outline-opacity": 0.85,
        width: "data(size)", height: "data(size)", "background-color": fill, "border-width": 0,
        "min-zoomed-font-size": 7,
      },
    },
    // The grid: a substation as a rounded box in its highest voltage's colour; its name the larger
    // the higher its voltage, a lower one's quieter.
    {
      selector: 'node[kind = "substation"]',
      style: { shape: "round-rectangle", width: "data(width)", "font-size": 7, "text-opacity": 0.75 },
    },
    { selector: 'node[kind = "substation"][band = "v300"]', style: { "font-size": 10, "font-weight": "bold", "text-opacity": 1 } },
    { selector: 'node[kind = "substation"][band = "v200"]', style: { "font-size": 9, "text-opacity": 1 } },
    { selector: 'node[kind = "substation"][band = "v100"]', style: { "font-size": 8, "text-opacity": 0.9 } },
    ...byBand('node[kind = "substation"]', "background-color"),
    { selector: 'node[kind = "boundary"]', style: { shape: "diamond", width: 7, height: 7, "font-size": 6, "text-opacity": 0.7 } },
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
    // Named as one zooms in, what matters most first: a busbar, line or transformer from the
    // start, other equipment soon, a switch or a node close up.
    { selector: 'node[role = "main"]', style: { "font-size": 8, "min-zoomed-font-size": 6 } },
    { selector: 'node[role = "switch"]', style: { "font-size": 6.5, "min-zoomed-font-size": 9 } },
    { selector: 'node[role = "node"]', style: { "min-zoomed-font-size": 10 } },
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
    // Drawn as a single-line diagram: labels beside the bays' columns, a busbar's above it.
    {
      selector: 'node.drawn[kind = "symbol"], node.drawn[kind = "junction"]',
      style: { "text-halign": "right", "text-valign": "center", "text-margin-x": 3, "text-margin-y": 0 },
    },
    { selector: 'node.drawn[kind = "busbar"]', style: { "text-valign": "top", "text-margin-y": -2 } },
    // Bays in rows beside upright busbars: names below, clear of the next one in the row.
    {
      selector: 'node.drawn.across[kind = "symbol"], node.drawn.across[kind = "junction"]',
      style: { "text-halign": "center", "text-valign": "bottom", "text-margin-x": 0, "text-margin-y": 2 },
    },
    { selector: 'node.drawn.across[kind = "busbar"]', style: { "text-valign": "top", "text-halign": "center" } },
    { selector: "node.onbar", style: { "text-opacity": 0 } },
    // Set off from the lines it joins. Findings are the CIM Model view's to show, not the grid's.
    { selector: 'node[kind = "substation"]', style: { "border-width": 1.5, "border-color": bg } },
    { selector: "node.out", style: { opacity: 0.45, "border-style": "dashed" } },
    {
      selector: "edge",
      style: { width: "data(width)", "curve-style": "bezier", "line-color": fill, opacity: 0.9 },
    },
    ...byBand("edge", "line-color", "mid-target-arrow-color", "mid-source-arrow-color"),
    // Distribution recedes behind the backbone.
    { selector: 'edge[band = "v30"], edge[band = "none"]', style: { opacity: 0.6 } },
    // Which way active power flows, where its values are shown.
    { selector: 'edge[arrow = "target"]', style: { "mid-target-arrow-shape": "triangle", "arrow-scale": 1.1 } },
    { selector: 'edge[arrow = "source"]', style: { "mid-source-arrow-shape": "triangle", "arrow-scale": 1.1 } },
    { selector: "edge.disconnected", style: { "line-style": "dashed" } },
    { selector: "node:selected", style: { "background-color": focus, "background-opacity": 1 } },
    { selector: 'node[kind = "group"]:selected', style: { "background-opacity": 0.15 } },
    { selector: 'node[kind = "symbol"]:selected', style: { "background-opacity": 0.3 } },
    { selector: "edge:selected", style: { "line-color": focus, opacity: 1 } },
    ...stateBands(),
    // Where what is dragged would land, while editing: over a state's halos.
    { selector: ".target", style: { "overlay-color": focus, "overlay-opacity": 0.3, "overlay-padding": 6, "overlay-shape": "round-rectangle" } },
    { selector: 'node[kind = "group"].target', style: { "border-color": focus, "border-width": 2, "overlay-opacity": 0 } },
    // A label that would crowd one that matters more (declutter.ts).
    { selector: ".crowded", style: { "text-opacity": 0 } },
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
        findingsHtml(n.findings) + stateHtml(n.id) + (equipment ? `<table>${equipment}</table>` : ""),
      n.location,
      n.kind === "Substation" ? [["Show inside", () => enter(n.id)]] : [],
    );
  } else {
    const e = g.edges.find((x) => `e:${x.id}` === target.id());
    if (!e) return;
    const end = (id: string) => escape(g.nodes.find((x) => x.id === id)?.label || id);
    panel(
      `<h3>${escape(e.label)}</h3><div class="sub">${escape(e.class)}${e.kV !== null ? ` · ${e.kV} kV` : ""}</div>` +
        `<div>${end(e.source)} — ${end(e.target)}</div>` + findingsHtml(e.findings) + stateHtml(e.id.split("#")[0]),
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
      far + findingsHtml(it.findings) + stateHtml(it.id),
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
  stopEditing(false);
  vscode.postMessage({ type: "leave" });
  inside = undefined;
  if (grid) draw();
}

/** The scene drawn: its layouts, its nodes' ids, the pairs of nodes its edges join, and its routed edges. */
function scene(): { layouts: Layout[]; auto?: Layout; ids: string[]; pairs: [string, string][]; routed: { id: string; key: string }[] } {
  if (inside) {
    return {
      layouts: inside.layouts,
      auto: inside.auto ?? undefined,
      ids: inside.items.map((it) => it.id),
      pairs: inside.links.map((l): [string, string] => [l.source, l.target]),
      routed: inside.links.map((l) => ({ id: `t:${l.id}`, key: l.id })),
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
function place(
  ids: string[],
  pairs: [string, string][],
  l: Layout,
): { nodes: Map<string, Point>; routes: Map<string, Point[]>; scale: number; back: (p: Point) => [number, number] } {
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
  const scale = l.unit ?? Math.max(900, 70 * Math.sqrt(ids.length)) / span;
  const screen = (p: [number, number]): Point => {
    const q = project(p);
    return { x: (q.x - minX) * scale, y: (q.y - minY) * scale };
  };
  const nodes = new Map(Object.entries(l.nodes).map(([id, p]) => [id, screen(p)]));
  // Lines sharing a stretch, each on its own track: a line width and a gap apart, whatever the scale.
  const routes = new Map(Object.entries(l.edges).map(([id, ps]) => [id, aside(ps.map(screen), (l.tracks?.[id] ?? 0) * TRACK)]));

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
  const back = (p: Point): [number, number] => {
    const y = p.y / scale + minY;
    return [p.x / scale + minX, l.flipY ? -y : y];
  };
  return { nodes, routes, scale, back };
}

/**
 * Bend an edge along its route: Cytoscape's segment points, relative to the line between its nodes.
 * At a node of `bars` (a busbar), the edge ends where the route does, on the bar, not at its centre.
 */
function route(edge: EdgeSingular, points: Point[], bars: Set<string> = new Set()): void {
  const s = edge.source().position();
  const t = edge.target().position();
  const dx = t.x - s.x;
  const dy = t.y - s.y;
  const length = Math.hypot(dx, dy);
  const d = (a: Point, b: Point) => Math.hypot(a.x - b.x, a.y - b.y);
  edge.removeStyle();
  if (points.length < 2) return;
  // The route may run from the target's end.
  const ordered = d(points[0], t) < d(points[0], s) ? [...points].reverse() : points;
  const style: Record<string, string> = {};
  let [first, last] = [ordered[0], ordered[ordered.length - 1]];
  let inner = ordered.slice(1, -1);
  // A bend within its node's shape (parallel lines stepping aside as they leave it) is where the
  // line starts: Cytoscape draws no edge with a bend inside its end.
  const within = (p: Point, n: NodeSingular) => Math.abs(p.x - n.position().x) <= n.width() / 2 + 1 && Math.abs(p.y - n.position().y) <= n.height() / 2 + 1;
  while (inner.length && within(inner[0], edge.source())) [first, inner] = [inner[0], inner.slice(1)];
  while (inner.length && within(inner[inner.length - 1], edge.target())) [last, inner] = [inner[inner.length - 1], inner.slice(0, -1)];
  const moved = (p: Point, end: Point) => p !== end;
  if ((bars.has(edge.source().id()) || moved(first, ordered[0])) && d(first, s) > 0.5) style["source-endpoint"] = `${first.x - s.x}px ${first.y - s.y}px`;
  if ((bars.has(edge.target().id()) || moved(last, ordered[ordered.length - 1])) && d(last, t) > 0.5) style["target-endpoint"] = `${last.x - t.x}px ${last.y - t.y}px`;
  if (!inner.length || length < 1e-6) {
    if (Object.keys(style).length) edge.style({ ...style, "curve-style": "straight" });
    return;
  }
  const weights = inner.map((p) => ((p.x - s.x) * dx + (p.y - s.y) * dy) / (length * length));
  const distances = inner.map((p) => ((p.y - s.y) * dx - (p.x - s.x) * dy) / length);
  edge.style({
    ...style,
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
  // Automatic: the server's layout, for the grid and inside a substation (a single-line diagram);
  // Cytoscape's where it has none.
  const layout = layouts.find((l) => l.id === picked) ?? (picked === "auto" ? auto : undefined);
  cy.elements().removeStyle();
  cy.nodes().removeClass("drawn onbar across");
  if (!layout) {
    // `cose` is quadratic per iteration: past a few hundred nodes, a plain grid rather than a hang.
    cy.layout(ids.length > 1000 ? { name: "grid", animate: false } : {
      name: "cose", animate: false, nodeRepulsion: () => 8000, idealEdgeLength: () => (inside ? 30 : 60),
      numIter: ids.length > 500 ? 400 : 1000,
    }).run();
    return;
  }
  const { nodes, routes, scale, back } = place(ids, pairs, layout);
  cy.layout({ name: "preset", positions: Object.fromEntries(nodes), fit: true, padding: 30 }).run();
  const pair = (p: Point): [number, number] => [p.x, p.y];
  const screen: Layout = {
    ...layout,
    flipY: false,
    nodes: Object.fromEntries([...nodes].map(([id, p]) => [id, pair(p)])),
    edges: Object.fromEntries([...routes].map(([id, ps]) => [id, ps.map(pair)])),
    sizes: Object.fromEntries(Object.entries(layout.sizes ?? {}).map(([id, [w, h]]) => [id, [w * scale, h * scale]])),
  };
  current = inside && layout.kind === "diagram" ? { screen, from: layout, back, scale } : undefined;
  // Drawn as laid out: busbars as long as the layout makes them, labels beside the bays' columns.
  paint(screen, routed, !!layout.unit);
  if (layout.unit) cy.fit(undefined, 30);
}

/**
 * Draw `l` (on the screen): busbars as long as it makes them, routes as it runs them, and, drawn
 * as laid out, labels beside the bays' columns. With `move`, its items moved to it too.
 */
function paint(l: Layout, routed: { id: string; key: string }[], drawn: boolean, move = false): void {
  if (!cy) return;
  const core = cy;
  core.batch(() => {
    if (move) for (const [id, [x, y]] of Object.entries(l.nodes)) core.getElementById(id).position({ x, y });
    // The busbars, and the nodes they are drawn for: where routes end on a bar.
    const bars = new Set<string>();
    if (drawn) {
      core.nodes().addClass("drawn");
      // Upright busbars: bays in rows.
      const sizes = Object.values(l.sizes ?? {});
      if (sizes.filter(([w, h]) => h > w).length * 2 > sizes.length) core.nodes().addClass("across");
      for (const [id, [width, height]] of Object.entries(l.sizes ?? {})) {
        const bar = core.getElementById(id);
        bar.style({ shape: "rectangle", width, height });
        bars.add(id);
        // A busbar section's node is the bar drawn: its own name would only cross the next one.
        if (bar.data("kind") === "busbar") {
          bar.connectedEdges().connectedNodes('[kind = "junction"]').forEach((j) => {
            j.addClass("onbar");
            bars.add(j.id());
          });
        }
      }
    }
    for (const e of routed) {
      const points = l.edges[e.key];
      if (points) route(core.getElementById(e.id) as EdgeSingular, points.map(([x, y]) => ({ x, y })), bars);
    }
  });
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
      // Its own label, which the state adds to.
      at.removeData("base");
    }
    paintState();
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
  // Only editing moves things: else a drag that starts on one pans, as on the background.
  cy.autoungrabify(!editing);
  panFromNodes(cy, () => !editing);
  keepDecluttered(cy);
  hover(undefined);
  cy.on("mouseover", "node, edge", (e) => hover(e.target as NodeSingular | EdgeSingular));
  // Only its own: leaving the voltage level around it comes after entering it.
  cy.on("mouseout", "node, edge", (e) => {
    if (hovered === (e.target as NodeSingular).id()) hover(undefined);
  });
  // The view moved under it: beside it still.
  cy.on("viewport", () => {
    const el = hovered ? cy?.getElementById(hovered) : undefined;
    if (el?.nonempty()) hoverAt(el as NodeSingular | EdgeSingular);
  });
  cy.on("grab tapstart", () => hover(undefined));
  if (!inside && gridView) {
    // Back from a substation: the grid as it was, with that substation selected.
    cy.zoom(gridView.zoom);
    cy.pan(gridView.pan);
    cy.getElementById(gridView.selected).select();
    gridView = undefined;
  }
  cy.on("select", "node, edge", (e) => {
    const t = e.target as NodeSingular | EdgeSingular;
    picked = !inside ? undefined : t.isEdge() ? { kind: "link", id: t.id().slice(2) } : t.data("kind") === "group" ? undefined : { kind: "item", id: t.id() };
    show(t);
    toolbar();
  });
  cy.on("unselect", () => {
    queueMicrotask(() => {
      if (cy?.$(":selected").empty()) {
        picked = undefined;
        show(undefined);
        toolbar();
      }
    });
  });
  cy.on("dbltap", "node, edge", (e) => {
    const it = editing ? inside?.items.find((x) => x.id === (e.target as NodeSingular).id()) : undefined;
    // Editing, a switch double-clicked opens or closes.
    if (it && it.open !== null && !it.junction) void topology({ op: "switch", id: it.id, open: !it.open });
    else open(e.target);
  });
  // A junction dragged: the busbar or node it would join, if let go there.
  cy.on("drag", "node", (e) => {
    const node = e.target as NodeSingular;
    if (!editing || node.data("kind") !== "junction") return;
    const onto = hit(node.position(), node).node;
    target(onto && isBus(onto) ? onto : undefined);
  });
  // Armed from the palette: where a click would put it.
  cy.on("mousemove", (e) => {
    if (editing && armed) target(targetOf(armed, e.position));
  });
  cy.on("dragfree", "node", (e) => {
    if (editing) released(e.target as NodeSingular);
  });
  cy.on("tap", (e) => {
    if (editing && armed) {
      const entry = armed;
      armed = undefined;
      target();
      toolbar();
      drop(entry, e.position);
    }
  });
  toolbar();
  paintState();
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

(document.getElementById("colour") as HTMLSelectElement).addEventListener("change", (e) => {
  colourBy = (e.target as HTMLSelectElement).value as typeof colourBy;
  paintState();
  const selected = cy?.$(":selected").first();
  show(selected?.nonempty() ? (selected as NodeSingular | EdgeSingular) : undefined);
});

(document.getElementById("values") as HTMLSelectElement).addEventListener("change", (e) => {
  valuesShown = (e.target as HTMLSelectElement).value as ValuesShown;
  vscode.setState({ ...(vscode.getState() ?? {}), valuesShown });
  paintState();
});

/** Hovered: what it is and its state, beside the pointer. */
const hoverBox = document.createElement("div");
hoverBox.id = "hover";
hoverBox.setAttribute("role", "tooltip");
document.body.append(hoverBox);
let hovered: string | undefined;

function hoverAt(el: NodeSingular | EdgeSingular): void {
  const box = (document.getElementById("graph") as HTMLElement).getBoundingClientRect();
  const p = el.isNode() ? el.renderedPosition() : el.renderedMidpoint();
  hoverBox.style.left = `${box.left + p.x + 14}px`;
  hoverBox.style.top = `${box.top + p.y + 14}px`;
}

function hover(el: NodeSingular | EdgeSingular | undefined): void {
  const id = el?.id().replace(/^e:/, "").split("#")[0];
  const heading = el ? String(el.data("base") ?? el.data("label") ?? "") : "";
  if (!el || !id || !heading || el.data("kind") === "group" || el.id().startsWith("t:")) {
    hovered = undefined;
    hoverBox.style.display = "none";
    return;
  }
  hovered = el.id();
  hoverBox.innerHTML = `<strong>${escape(heading)}</strong>` + stateLines(id).slice(0, 4).map((l) => `<div>${escape(l)}</div>`).join("");
  hoverAt(el);
  hoverBox.style.display = "block";
}

(document.getElementById("back") as HTMLButtonElement).addEventListener("click", leave);
window.addEventListener("keydown", (e) => {
  const typing = document.activeElement instanceof HTMLElement && document.activeElement.closest("input, select, textarea");
  if (e.key === "Escape" && !typing) {
    if (armed) {
      armed = undefined;
      toolbar();
    } else if (editing) stopEditing(true);
    else leave();
  }
  if (editing && !typing && (e.key === "Delete" || e.key === "Backspace")) deletePicked();
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
  if (e.data?.type === "reply") {
    const waiting = asked.get(e.data.n as number);
    asked.delete(e.data.n as number);
    if (e.data.error) waiting?.reject(new Error(String(e.data.error)));
    else waiting?.resolve(e.data.value);
    return;
  }
  const previous = cy ? shown() : undefined;
  if (e.data?.type === "grid") {
    grid = e.data.grid as Grid;
    // Behind a substation, the grid is drawn on the way back.
    if (!inside) draw(previous);
  } else if (e.data?.type === "inside") {
    // `null`: the substation is gone from the set.
    inside = (e.data.inside as Inside | null) ?? undefined;
    if (!inside) stopEditing(false);
    // After an edit here, drawn anew: on the diagram it saved, its moves gone.
    const anew = redraw;
    redraw = false;
    changed = false;
    if (inside || grid) draw(anew ? undefined : previous);
    // After a topology edit, the view where it was.
    if (anew && keepView && inside && cy) restore(keepView);
    keepView = undefined;
  } else if (e.data?.type === "leave" && inside) {
    inside = undefined;
    stopEditing(false);
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
  // Editing, the view stays where it is: what was added only brought into it.
  if (inside && editing) {
    if (!inView(el)) cy.animate({ center: { eles: el } }, { duration: 200 });
    return;
  }
  const near = el.isEdge() ? el.union(el.connectedNodes()) : (el as NodeSingular).closedNeighborhood();
  cy.animate({ fit: { eles: near, padding: 80 } }, { duration: 200 });
}
// Editing inside a substation: moving items (the routes follow, the layout saved as a DL
// diagram) and its topology (switches, deleting, joining nodes, adding from the palette), each
// asked of the language server through the extension, which applies the edit it answers.

/** The layout shown inside a substation, on the screen; the one it came from; and the way back. */
let current: { screen: Layout; from: Layout; back: (p: Point) => [number, number]; scale: number } | undefined;
let editing = false;
/** An item moved since editing began. */
let changed = false;
/** Draw anew with the next substation the extension sends. */
let redraw = false;
let busy = false;
let palette: { classes: { group: string; class: string }[]; templates: { id: string; label: string }[] } | undefined;
let armed: { class?: string; template?: string; label: string } | undefined;
let picked: { kind: "item" | "link"; id: string } | undefined;
let failed = "";

/** Requests to the extension waiting for its reply, by number. */
const asked = new Map<number, { resolve: (v: unknown) => void; reject: (e: Error) => void }>();
let asking = 0;

/** Ask the language server `method` (one of the editing requests) with `params`, through the extension. */
function ask<T>(method: string, params: Record<string, unknown>): Promise<T> {
  const n = ++asking;
  return new Promise<T>((resolve, reject) => {
    asked.set(n, { resolve: resolve as (v: unknown) => void, reject });
    vscode.postMessage({ type: "ask", n, method, params });
  });
}

type TopologyOp =
  | { op: "switch"; id: string; open: boolean }
  | { op: "delete"; ids: string[] }
  | { op: "merge"; from: string; into: string }
  | { op: "disconnect"; terminal: string }
  | { op: "add"; class: string; level: string }
  | { op: "insert"; class: string; terminal: string }
  | { op: "bay"; template: string; bus: string };

const MIME = "application/x-cimoxide-add";
const COMMON = ["BusbarSection", "Breaker", "Disconnector", "LoadBreakSwitch", "ConformLoad", "EnergyConsumer", "SynchronousMachine", "LinearShuntCompensator", "PowerTransformer"];

const links = () => inside?.links.map((l) => ({ id: `t:${l.id}`, key: l.id })) ?? [];
const itemOf = (id: string) => inside?.items.find((x) => x.id === id);
const isBus = (n: NodeSingular) => n.data("kind") === "junction" || n.data("kind") === "busbar";

function button(label: string, run: () => void, title = ""): HTMLButtonElement {
  const b = document.createElement("button");
  b.textContent = label;
  b.title = title;
  b.disabled = busy;
  b.addEventListener("click", run);
  return b;
}

/** The editing controls in the bar, and the palette, as things are now. */
function toolbar(): void {
  if (!armed) target();
  const bar = document.getElementById("edit") as HTMLElement;
  const pal = document.getElementById("palette") as HTMLElement;
  bar.replaceChildren();
  pal.replaceChildren();
  pal.style.display = "none";
  if (!inside) return;
  if (!editing) {
    const edit = button("Edit", () => void startEditing(), "Move, wire, add and delete equipment");
    edit.disabled = !current;
    bar.append(edit);
    return;
  }
  bar.append(button("Save as diagram", () => void saveDiagram()), button(changed ? "Discard" : "Done", () => stopEditing(true)));
  const it = picked?.kind === "item" ? itemOf(picked.id) : undefined;
  if (it && !it.junction && it.open !== null) bar.append(button(`${it.open ? "Close" : "Open"} ${it.label || "switch"}`, () => void topology({ op: "switch", id: it.id, open: !it.open })));
  if (it && !it.junction) bar.append(button(`Delete ${it.label || it.class}`, deletePicked));
  if (picked?.kind === "link") {
    const terminal = picked.id;
    bar.append(button("Disconnect", () => void topology({ op: "disconnect", terminal })));
  }
  const hint = document.createElement("span");
  hint.className = failed ? "failed" : "hint";
  hint.textContent = failed
    ? `Not edited: ${failed}`
    : armed
      ? `Click where ${armed.label} goes: a voltage level, a wire for a switch, a busbar for a bay.`
      : changed
        ? "Save or discard the moves before changing the topology."
        : "Drag to move; drop a node onto another to join them; add from below.";
  bar.append(hint);
  if (!palette) return;
  pal.style.display = "";
  const entries = [
    ...COMMON.filter((c) => palette?.classes.some((x) => x.class === c)).map((c) => ({ label: c, add: { class: c } })),
    ...palette.templates.map((t) => ({ label: t.label, add: { template: t.id } })),
  ];
  for (const entry of entries) {
    const b = button(entry.label, () => {
      armed = armed?.label === entry.label ? undefined : { ...entry.add, label: entry.label };
      toolbar();
    });
    b.draggable = true;
    b.disabled = busy || changed;
    b.setAttribute("aria-pressed", String(armed?.label === entry.label));
    b.addEventListener("dragstart", (e) => {
      e.dataTransfer?.setData(MIME, JSON.stringify(entry.add));
      carried = entry.add;
    });
    b.addEventListener("dragend", () => {
      carried = undefined;
      target();
    });
    pal.append(b);
  }
  const more = document.createElement("select");
  more.title = "More equipment";
  more.disabled = busy || changed;
  more.innerHTML = `<option value="">More…</option>` +
    [...new Set(palette.classes.map((c) => c.group))]
      .map((g) => `<optgroup label="${escape(g)}">${palette?.classes.filter((c) => c.group === g).map((c) => `<option>${escape(c.class)}</option>`).join("")}</optgroup>`)
      .join("");
  more.addEventListener("change", () => {
    if (more.value) armed = { class: more.value, label: more.value };
    toolbar();
  });
  pal.append(more);
}

async function startEditing(): Promise<void> {
  editing = true;
  cy?.autoungrabify(false);
  failed = "";
  toolbar();
  const level = inside?.groups[0]?.id;
  palette = level ? await ask<typeof palette>("cimoxide/palette", { level }).catch(() => undefined) : undefined;
  toolbar();
}

function stopEditing(rearrange: boolean): void {
  const moved = changed;
  editing = false;
  cy?.autoungrabify(true);
  changed = false;
  armed = undefined;
  failed = "";
  if (rearrange && moved) arrange();
  toolbar();
}

/** An item moved: snapped to the grid, and the routes follow. */
async function moved(node: NodeSingular): Promise<void> {
  if (!cy || !inside || !current) return;
  const snap = (v: number, step: number) => Math.round(v / step) * step;
  node.union(node.descendants()).forEach((n) => {
    if (n.data("kind") === "group") return;
    const p = n.position();
    n.position({ x: snap(p.x, 4), y: snap(p.y, 14) });
  });
  const core = cy;
  const nodes = Object.fromEntries(inside.items.map((it) => {
    const p = core.getElementById(it.id).position();
    return [it.id, [p.x, p.y]];
  }));
  try {
    const again = await ask<Layout | null>("cimoxide/reroute", { id: inside.id, layout: { ...current.screen, nodes, edges: {} } });
    if (!again || !current) return;
    current = { ...current, screen: { ...again, flipY: false } };
    changed = true;
    paint(current.screen, links(), true, true);
    toolbar();
  } catch (e) {
    failed = String(e instanceof Error ? e.message : e);
    toolbar();
  }
}

/** The layout edited, saved as a DL diagram: into the diagram shown, else as a new one. */
async function saveDiagram(): Promise<void> {
  if (!inside || !current) return;
  const { screen, from, back, scale } = current;
  const at = ([x, y]: [number, number]) => back({ x, y });
  const layout = {
    ...from,
    nodes: Object.fromEntries(Object.entries(screen.nodes).map(([id, p]) => [id, at(p)])),
    edges: Object.fromEntries(Object.entries(screen.edges).map(([id, ps]) => [id, ps.map(at)])),
    sizes: Object.fromEntries(Object.entries(screen.sizes ?? {}).map(([id, [w, h]]) => [id, [w / scale, h / scale]])),
  };
  const existing = from.id.startsWith("dl:") ? from.id.slice(3) : null;
  busy = true;
  failed = "";
  toolbar();
  try {
    const done = await ask<{ id: string }>("cimoxide/diagram", { substation: inside.id, layout, existing, name: `${inside.label} single-line` });
    chosen.inside = `dl:${done.id}`;
    editing = false;
    redraw = true;
  } catch (e) {
    failed = String(e instanceof Error ? e.message : e);
  } finally {
    busy = false;
    toolbar();
  }
}

/** A topology edit inside the substation shown; in a DL diagram, drawn there too, what is new `at`. */
async function topology(op: TopologyOp, at: Point | null = null): Promise<void> {
  if (!inside || busy) return;
  if (changed) {
    failed = "save or discard the moves first";
    toolbar();
    return;
  }
  const diagram = current?.from.id.startsWith("dl:") ? current.from.id.slice(3) : null;
  busy = true;
  failed = "";
  toolbar();
  keepView = anchor();
  try {
    await ask("cimoxide/topology", { substation: inside.id, op, diagram, at: at && current ? current.back(at) : null });
    redraw = true;
  } catch (e) {
    failed = String(e instanceof Error ? e.message : e);
    keepView = undefined;
  } finally {
    busy = false;
    toolbar();
  }
}

function deletePicked(): void {
  const it = picked?.kind === "item" ? itemOf(picked.id) : undefined;
  if (it && !it.junction) void topology({ op: "delete", ids: [it.id] });
}

/** How far `p` is from wire `e`, as drawn. */
function distance(e: EdgeSingular, p: Point): number {
  const points = [e.sourceEndpoint(), ...(e.segmentPoints() ?? []), e.targetEndpoint()];
  let best = Infinity;
  for (let k = 1; k < points.length; k++) {
    const [a, b] = [points[k - 1], points[k]];
    const [dx, dy] = [b.x - a.x, b.y - a.y];
    const t = dx || dy ? Math.max(0, Math.min(1, ((p.x - a.x) * dx + (p.y - a.y) * dy) / (dx * dx + dy * dy))) : 0;
    best = Math.min(best, Math.hypot(p.x - (a.x + t * dx), p.y - (a.y + t * dy)));
  }
  return best;
}

/** What is at `p` (in the scene's coordinates): an item, a wire, the voltage level around it. */
function hit(p: Point, except?: NodeSingular): { node?: NodeSingular; edge?: EdgeSingular; group?: string } {
  const core = cy as Core;
  const pad = 4 / core.zoom();
  const within = (n: NodeSingular, by = pad) => {
    const bb = n.boundingBox({ includeLabels: false });
    return p.x >= bb.x1 - by && p.x <= bb.x2 + by && p.y >= bb.y1 - by && p.y <= bb.y2 + by;
  };
  const node = core.nodes().filter((n) => n.data("kind") !== "group" && (!except || n.id() !== except.id()) && within(n));
  const edge = node.empty() ? core.edges().filter((e) => distance(e, p) < 6 / core.zoom()) : core.collection();
  const group = core.nodes('[kind = "group"]').filter((g) => within(g, 0));
  return {
    node: node.nonempty() ? (node.first() as NodeSingular) : undefined,
    edge: edge.nonempty() ? (edge.first() as EdgeSingular) : undefined,
    group: group.nonempty() ? group.last().id().slice(2) : inside?.groups.length === 1 ? inside.groups[0].id : undefined,
  };
}

/** What a palette entry dropped at `p` would go into: a busbar, a wire, a voltage level; as `drop` decides. */
function targetOf(entry: { class?: string; template?: string }, p: Point): cytoscape.Collection | NodeSingular | EdgeSingular | undefined {
  const core = cy as Core;
  const at = hit(p);
  if (entry.template) return at.node && isBus(at.node) ? at.node : undefined;
  const isSwitch = palette?.classes.some((c) => c.class === entry.class && c.group === "Switches");
  if (isSwitch && at.edge) return at.edge;
  const level = (at.node && itemOf(at.node.id())?.group) || at.group;
  const group = level ? core.getElementById(`g:${level}`) : undefined;
  return group?.nonempty() ? group : undefined;
}

/** Marks where a drop would land, while something is dragged; nothing, when it would land nowhere. */
function target(el?: cytoscape.Collection | NodeSingular | EdgeSingular): void {
  cy?.$(".target").removeClass("target");
  el?.addClass("target");
}

/** What is being dragged from the palette: `dragover` cannot read what is carried. */
let carried: { class?: string; template?: string } | undefined;

/**
 * Where the view was before a topology edit: drawn again, it stays there, not fitted anew. The
 * drawing may shift as a whole, so the view keeps an item near its middle where it was.
 */
let keepView: { zoom: number; id: string; at: Point } | undefined;

function anchor(): typeof keepView {
  if (!cy) return undefined;
  const core = cy;
  const middle = { x: core.width() / 2, y: core.height() / 2 };
  const items = core.nodes().filter((n) => n.data("kind") !== "group");
  if (items.empty()) return undefined;
  const off = (n: NodeSingular) => Math.hypot(n.renderedPosition().x - middle.x, n.renderedPosition().y - middle.y);
  const near = items.min((n) => off(n as NodeSingular)).ele as NodeSingular;
  return { zoom: core.zoom(), id: near.id(), at: { ...near.renderedPosition() } };
}

/** The view as `keep` had it, if its item is still drawn and the drawing in view. */
function restore(keep: NonNullable<typeof keepView>): void {
  const core = cy as Core;
  const el = core.getElementById(keep.id);
  if (el.empty() || !el.isNode()) return;
  const fitted = { zoom: core.zoom(), pan: { ...core.pan() } };
  core.zoom(keep.zoom);
  const now = el.renderedPosition();
  core.panBy({ x: keep.at.x - now.x, y: keep.at.y - now.y });
  const bb = core.elements().renderedBoundingBox({});
  const [w, h] = [core.width(), core.height()];
  // Nothing of it in view any more: as fitted.
  if (bb.x2 < 0 || bb.y2 < 0 || bb.x1 > w || bb.y1 > h) {
    core.viewport(fitted);
    return;
  }
  // A drawing that fits at this zoom is moved wholly into view, as little as it takes.
  const pad = 20;
  const into = (lo: number, hi: number, size: number) => (hi - lo > size - 2 * pad ? 0 : lo < pad ? pad - lo : hi > size - pad ? size - pad - hi : 0);
  core.panBy({ x: into(bb.x1, bb.x2, w), y: into(bb.y1, bb.y2, h) });
}

/** `el` is wholly in view. */
function inView(el: cytoscape.SingularElementArgument): boolean {
  const core = cy as Core;
  const bb = el.renderedBoundingBox({ includeLabels: false });
  return bb.x1 >= 0 && bb.y1 >= 0 && bb.x2 <= core.width() && bb.y2 <= core.height();
}

/** A palette entry dropped at `p`: a bay onto a busbar, a switch into a wire, else equipment in the level there. */
function drop(entry: { class?: string; template?: string }, p: Point): void {
  const at = hit(p);
  const fail = (why: string) => {
    failed = why;
    toolbar();
  };
  if (entry.template) {
    if (!at.node || !isBus(at.node)) return fail("drop a bay onto a busbar");
    void topology({ op: "bay", template: entry.template, bus: at.node.id() }, p);
    return;
  }
  const cls = entry.class as string;
  const isSwitch = palette?.classes.some((c) => c.class === cls && c.group === "Switches");
  if (isSwitch && at.edge) {
    void topology({ op: "insert", class: cls, terminal: at.edge.id().slice(2) }, p);
    return;
  }
  const level = (at.node && itemOf(at.node.id())?.group) || at.group;
  if (!level) return fail("drop it into a voltage level");
  void topology({ op: "add", class: cls, level }, p);
}

/** A node let go of, while editing: a junction onto another joins them, else a move. */
function released(node: NodeSingular): void {
  target();
  if (node.data("kind") === "junction") {
    const onto = hit(node.position(), node).node;
    if (onto && isBus(onto)) {
      void topology({ op: "merge", from: node.id(), into: onto.id() });
      return;
    }
  }
  void moved(node);
}

const graphBox = document.getElementById("graph") as HTMLElement;
// The middle or right button pans, over anything.
panWithOtherButtons(graphBox, () => cy);
/** The scene's coordinates of a point on the screen. */
function sceneAt(clientX: number, clientY: number): Point {
  const core = cy as Core;
  const box = graphBox.getBoundingClientRect();
  const pan = core.pan();
  return { x: (clientX - box.left - pan.x) / core.zoom(), y: (clientY - box.top - pan.y) / core.zoom() };
}
graphBox.addEventListener("dragover", (e) => {
  if (!editing || !e.dataTransfer?.types.includes(MIME)) return;
  e.preventDefault();
  if (carried && cy) target(targetOf(carried, sceneAt(e.clientX, e.clientY)));
});
graphBox.addEventListener("dragleave", () => target());
graphBox.addEventListener("drop", (e) => {
  target();
  const data = e.dataTransfer?.getData(MIME);
  if (!editing || !data || !cy) return;
  e.preventDefault();
  drop(JSON.parse(data), sceneAt(e.clientX, e.clientY));
});

vscode.postMessage({ type: "ready" });
