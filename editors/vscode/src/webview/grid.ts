// The grid view's page: draws `cimoxide/grid` with Cytoscape.js, coloured by
// each substation's and line's worst finding, in VS Code's theme colours.

import cytoscape, { Core, ElementDefinition, NodeSingular, EdgeSingular } from "cytoscape";

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
}

type Point = { x: number; y: number };

/** The layout picked in this panel: a layout id, or "auto". */
let chosen: string | undefined;

let cy: Core | undefined;
let grid: Grid | undefined;

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
  #summary { color: var(--vscode-descriptionForeground); }
  #main { flex: 1; display: flex; min-height: 0; }
  #graph { flex: 1; min-width: 0; }
  #info { width: 280px; overflow: auto; padding: 8px 12px; border-left: 1px solid var(--vscode-panel-border); }
  #info h3 { margin: 4px 0 2px; font-size: 1.1em; }
  #info .sub { color: var(--vscode-descriptionForeground); margin-bottom: 8px; }
  #info table { border-collapse: collapse; width: 100%; }
  #info td { padding: 1px 4px 1px 0; vertical-align: top; }
  #info td.n { text-align: right; }
  #info button { margin-top: 10px; color: var(--vscode-button-foreground); background: var(--vscode-button-background);
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

function elements(g: Grid): ElementDefinition[] {
  const nodes = g.nodes.map((n) => {
    const size = Object.values(n.equipment).reduce((a, b) => a + b, 0);
    return {
      group: "nodes" as const,
      data: {
        id: n.id, label: n.label, kind: n.kind, sev: worst(n.findings), size: Math.min(48, 16 + 4 * Math.sqrt(size)),
        // The ring grows with the findings: one and forty must not look alike.
        ring: total(n.findings) ? Math.min(8, 1.5 + 1.5 * Math.log2(total(n.findings))) : 0,
      },
    };
  });
  const edges = g.edges.map((e) => ({
    group: "edges" as const,
    data: {
      id: `e:${e.id}`, source: e.source, target: e.target, label: e.label, sev: worst(e.findings),
      width: e.kV === null ? 1.5 : e.kV >= 300 ? 3.5 : e.kV >= 200 ? 2.5 : 1.5,
    },
  }));
  return [...nodes, ...edges];
}

function stylesheet(): cytoscape.StylesheetJson {
  const sev = {
    violation: colour("--vscode-errorForeground", "#f14c4c"),
    warning: colour("--vscode-editorWarning-foreground", "#cca700"),
    info: colour("--vscode-editorInfo-foreground", "#3794ff"),
    none: colour("--vscode-descriptionForeground", "#888888"),
  };
  const fg = colour("--vscode-foreground", "#cccccc");
  const fill = colour("--vscode-editorLineNumber-foreground", "#6e7681");
  const focus = colour("--vscode-focusBorder", "#007fd4");
  const bySeverity = (prop: string) =>
    Object.entries(sev).map(([k, c]) => ({ selector: `[sev = "${k}"]`, style: { [prop]: c } }));
  return [
    {
      selector: "node",
      style: {
        label: "data(label)", color: fg, "font-size": 10, "text-valign": "bottom", "text-margin-y": 3,
        width: "data(size)", height: "data(size)", "background-color": fill, "border-width": "data(ring)",
      },
    },
    { selector: 'node[kind != "Substation"]', style: { shape: "diamond", width: 12, height: 12, "font-size": 8 } },
    ...bySeverity("border-color").map((s) => ({ ...s, selector: `node${s.selector}` })),
    {
      selector: "edge",
      style: { width: "data(width)", "curve-style": "bezier", opacity: 0.85 },
    },
    ...bySeverity("line-color").map((s) => ({ ...s, selector: `edge${s.selector}` })),
    { selector: "node:selected", style: { "background-color": focus } },
    { selector: "edge:selected", style: { "line-color": focus, opacity: 1 } },
    { selector: ".dim", style: { opacity: 0.15 } },
  ] as cytoscape.StylesheetJson;
}

function summary(g: Grid): string {
  const subs = g.nodes.filter((n) => n.kind === "Substation").length;
  const sum = [...g.nodes, ...g.edges].reduce(
    (a, x) => ({ violation: a.violation + x.findings.violation, warning: a.warning + x.findings.warning, info: a.info + x.findings.info }),
    { ...g.elsewhere },
  );
  return `${subs} substation(s), ${g.nodes.length - subs} boundary point(s), ${g.edges.length} connection(s) · ` +
    `${sum.violation} violation(s), ${sum.warning} warning(s), ${sum.info} info · ${total(g.elsewhere)} not on the grid`;
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

function show(target: NodeSingular | EdgeSingular | undefined): void {
  const info = document.getElementById("info") as HTMLElement;
  if (!grid || !target) {
    info.innerHTML = `<div class="sub">Select a substation or a line; double-click to open where it is written.</div>`;
    return;
  }
  let html: string;
  let location: Location | null;
  if (target.isNode()) {
    const n = grid.nodes.find((x) => x.id === target.id());
    if (!n) return;
    location = n.location;
    const kv = n.kV.length ? `${n.kV.join(" / ")} kV` : "";
    const equipment = Object.entries(n.equipment)
      .sort((a, b) => b[1] - a[1])
      .map(([k, v]) => `<tr><td>${escape(k)}</td><td class="n">${v}</td></tr>`)
      .join("");
    html = `<h3>${escape(n.label || n.id)}</h3><div class="sub">${escape(n.kind)}${n.region ? ` · ${escape(n.region)}` : ""}${kv ? ` · ${kv}` : ""}</div>` +
      findingsHtml(n.findings) + (equipment ? `<table>${equipment}</table>` : "");
  } else {
    const e = grid.edges.find((x) => `e:${x.id}` === target.id());
    if (!e) return;
    location = e.location;
    const end = (id: string) => escape(grid?.nodes.find((x) => x.id === id)?.label || id);
    html = `<h3>${escape(e.label)}</h3><div class="sub">${escape(e.class)}${e.kV !== null ? ` · ${e.kV} kV` : ""}</div>` +
      `<div>${end(e.source)} — ${end(e.target)}</div>` + findingsHtml(e.findings);
  }
  if (location) {
    html += `<button id="open">Open definition</button>`;
  }
  info.innerHTML = html;
  document.getElementById("open")?.addEventListener("click", () => vscode.postMessage({ type: "open", location }));
}

function open(target: NodeSingular | EdgeSingular): void {
  const location = target.isNode()
    ? grid?.nodes.find((x) => x.id === target.id())?.location
    : grid?.edges.find((x) => `e:${x.id}` === target.id())?.location;
  if (location) {
    vscode.postMessage({ type: "open", location });
  }
}

/** The layout to draw: the one picked, else the TSO's diagram, else the map, else automatic. */
function choice(g: Grid): string {
  if (chosen && (chosen === "auto" || g.layouts.some((l) => l.id === chosen))) {
    return chosen;
  }
  return (g.layouts.find((l) => l.kind === "diagram") ?? g.layouts[0])?.id ?? "auto";
}

/** Screen coordinates of a layout's positions and routes, scaled to the grid's size. */
function place(g: Grid, l: Layout): { nodes: Map<string, Point>; routes: Map<string, Point[]> } {
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
  const scale = Math.max(900, 70 * Math.sqrt(g.nodes.length)) / span;
  const screen = (p: [number, number]): Point => {
    const q = project(p);
    return { x: (q.x - minX) * scale, y: (q.y - minY) * scale };
  };
  const nodes = new Map(Object.entries(l.nodes).map(([id, p]) => [id, screen(p)]));
  const routes = new Map(Object.entries(l.edges).map(([id, ps]) => [id, ps.map(screen)]));

  // A node the layout does not place goes among its placed neighbours, else below the rest.
  const neighbours = new Map<string, string[]>();
  for (const e of g.edges) {
    neighbours.set(e.source, [...(neighbours.get(e.source) ?? []), e.target]);
    neighbours.set(e.target, [...(neighbours.get(e.target) ?? []), e.source]);
  }
  let missing = g.nodes.map((n) => n.id).filter((id) => !nodes.has(id));
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

function arrange(g: Grid): void {
  if (!cy) return;
  const id = choice(g);
  const layout = g.layouts.find((l) => l.id === id);
  cy.edges().removeStyle();
  if (!layout) {
    cy.layout({ name: "cose", animate: false, nodeRepulsion: () => 8000, idealEdgeLength: () => 60, numIter: g.nodes.length > 500 ? 400 : 1000 }).run();
    return;
  }
  const { nodes, routes } = place(g, layout);
  cy.layout({ name: "preset", positions: Object.fromEntries(nodes), fit: true, padding: 30 }).run();
  for (const e of g.edges) {
    const points = routes.get(e.id);
    if (points) {
      route(cy.getElementById(`e:${e.id}`) as EdgeSingular, points);
    }
  }
}

/** The layout picker: shown when the set has a diagram or locations. */
function picker(g: Grid): void {
  const select = document.getElementById("layout") as HTMLSelectElement;
  const options = [...g.layouts.map((l) => [l.id, l.name]), ["auto", "Automatic"]];
  select.innerHTML = options.map(([v, n]) => `<option value="${escape(v)}">${escape(n)}</option>`).join("");
  select.value = choice(g);
  select.style.display = g.layouts.length ? "" : "none";
}

function draw(g: Grid): void {
  const previous = grid;
  grid = g;
  (document.getElementById("summary") as HTMLElement).textContent = summary(g);
  const sameShape = cy && previous &&
    previous.nodes.length === g.nodes.length && previous.edges.length === g.edges.length &&
    g.nodes.every((n) => cy?.getElementById(n.id).nonempty()) && g.edges.every((e) => cy?.getElementById(`e:${e.id}`).nonempty());
  if (cy && sameShape) {
    // Only the findings changed: keep the layout.
    for (const el of elements(g)) {
      cy.getElementById(el.data.id as string).data(el.data);
    }
    const selected = cy.$(":selected").first();
    show(selected.nonempty() ? (selected as NodeSingular | EdgeSingular) : undefined);
    return;
  }
  cy?.destroy();
  cy = cytoscape({
    container: document.getElementById("graph"),
    elements: elements(g),
    style: stylesheet(),
    layout: { name: "preset" },
    wheelSensitivity: 0.3,
    minZoom: 0.05,
    maxZoom: 4,
  });
  picker(g);
  arrange(g);
  cy.on("select", "node, edge", (e) => show(e.target));
  cy.on("unselect", () => show(undefined));
  cy.on("dbltap", "node, edge", (e) => open(e.target));
  show(undefined);
}

// Restyle when the colour theme changes.
new MutationObserver(() => cy?.style(stylesheet())).observe(document.body, { attributes: true, attributeFilter: ["class"] });

(document.getElementById("layout") as HTMLSelectElement).addEventListener("change", (e) => {
  chosen = (e.target as HTMLSelectElement).value;
  if (grid) arrange(grid);
});

(document.getElementById("search") as HTMLInputElement).addEventListener("input", (e) => {
  if (!cy) return;
  const q = (e.target as HTMLInputElement).value.trim().toLowerCase();
  cy.elements().removeClass("dim");
  if (!q) return;
  const hits = cy.nodes().filter((n) => String(n.data("label")).toLowerCase().includes(q));
  cy.elements().not(hits).addClass("dim");
  if (hits.nonempty()) {
    cy.animate({ fit: { eles: hits, padding: 80 } }, { duration: 200 });
  }
});

window.addEventListener("message", (e) => {
  if (e.data?.type === "grid") {
    draw(e.data.grid as Grid);
  }
});
vscode.postMessage({ type: "ready" });
