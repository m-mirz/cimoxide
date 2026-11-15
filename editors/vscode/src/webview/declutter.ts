// Fewer labels where they would crowd: each label shown only if it overlaps none shown before it,
// taken by priority (voltage, then findings, then size). Labels too small to read at the zoom are
// left to Cytoscape, which hides them itself. As cimlens has it (web/src/grid/declutter.ts).

import type { Core, SingularElementArgument } from "cytoscape";

export interface Box {
  x1: number;
  y1: number;
  x2: number;
  y2: number;
}

export interface Label {
  id: string;
  /** Higher first. */
  priority: number;
  box: Box;
  /** Always shown, and in the way of the rest: a voltage level's name, what is selected. */
  kept?: boolean;
  /** Whose: a label is not in the way of its own element's shape. */
  owner?: string;
  /** A shape, not a label: in the way only of labels that matter less than it. */
  shape?: boolean;
}

/** The ids of `labels` to hide: each that overlaps one shown before it, by priority. */
export function crowded(labels: Label[], gap = 2): Set<string> {
  const order = [...labels].sort((a, b) => Number(!!b.kept) - Number(!!a.kept) || b.priority - a.priority || (a.id < b.id ? -1 : 1));
  // Shown boxes in buckets of a grid, so each label is checked against its neighbours only.
  const size = 64;
  const buckets = new Map<string, (Box & { owner?: string; shape?: number })[]>();
  const cells = (b: Box) => {
    const out: string[] = [];
    for (let i = Math.floor(b.x1 / size); i <= Math.floor(b.x2 / size); i++)
      for (let j = Math.floor(b.y1 / size); j <= Math.floor(b.y2 / size); j++) out.push(`${i},${j}`);
    return out;
  };
  const hidden = new Set<string>();
  for (const label of order) {
    const b = { x1: label.box.x1 - gap, y1: label.box.y1 - gap, x2: label.box.x2 + gap, y2: label.box.y2 + gap };
    const keys = cells(b);
    const hits =
      !label.kept &&
      keys.some((k) =>
        buckets.get(k)?.some(
          (o) => (!o.owner || o.owner !== label.owner) && (o.shape === undefined || label.priority < o.shape) && o.x1 < b.x2 && b.x1 < o.x2 && o.y1 < b.y2 && b.y1 < o.y2,
        ),
      );
    if (hits) {
      hidden.add(label.id);
      continue;
    }
    const held = { ...b, owner: label.owner, shape: label.shape ? label.priority : undefined };
    for (const k of keys) {
      const list = buckets.get(k);
      if (list) list.push(held);
      else buckets.set(k, [held]);
    }
  }
  return hidden;
}

const BAND_RANK: Record<string, number> = { v300: 4, v200: 3, v100: 2, v30: 1 };
const SEV_RANK: Record<string, number> = { violation: 3, warning: 2, info: 1 };
const KIND_RANK: Record<string, number> = { substation: 3, busbar: 3, symbol: 2, boundary: 1, junction: 0 };
const ROLE_RANK: Record<string, number> = { main: 3, equipment: 2, switch: 1, node: 0 };

/** How much a label matters: its voltage, its findings, a state's warning, its kind, its size. */
export function priority(el: SingularElementArgument): number {
  const d = el.data();
  const pf = d.pf === 'bad' ? 2 : d.pf === 'warn' ? 1 : 0;
  // Inside a substation, what an item is weighs more than its voltage: a switch's name never
  // displaces a line's.
  const role = d.role === undefined ? 0 : (ROLE_RANK[d.role] ?? 0) * 10000;
  return role + (BAND_RANK[d.band] ?? 0) * 1000 + (SEV_RANK[d.sev] ?? 0) * 100 + pf * 100 + (KIND_RANK[d.kind] ?? 0) * 10 + (Number(d.size) || 0) / 10;
}

/**
 * Hides, by the class `crowded`, the labels of `cy` in view that overlap ones that matter more;
 * shows the rest. A voltage level's name and what is selected are always shown.
 */
export function declutter(cy: Core): void {
  const zoom = cy.zoom();
  const view = cy.extent();
  const labels: Label[] = [];
  const onScreen = (b: { x1: number; y1: number; x2: number; y2: number }) => ({ x1: b.x1 * zoom, y1: b.y1 * zoom, x2: b.x2 * zoom, y2: b.y2 * zoom });
  // What is drawn is in the way of the labels that matter less than it: each node's shape but a
  // bar's (a name across a thin bar still reads), and never its own label's.
  cy.nodes().forEach((n) => {
    if (n.data('kind') === 'group' || n.data('kind') === 'busbar' || n.hasClass('onbar') || n.width() > 4 * n.height() || n.height() > 4 * n.width()) return;
    const box = n.boundingBox({ includeLabels: false, includeOverlays: false } as never);
    if (box.x2 < view.x1 || box.x1 > view.x2 || box.y2 < view.y1 || box.y1 > view.y2) return;
    labels.push({ id: `${n.id()}#shape`, priority: priority(n), box: onScreen(box), kept: true, owner: n.id(), shape: true });
  });
  cy.elements().forEach((found) => {
    const el = found as unknown as SingularElementArgument & { pstyle(name: string): { pfValue: number } };
    const text = el.isEdge() ? el.data('flowLabel') : el.data('label');
    if (!text) return;
    // Far out of view: not measured, which is the costly part on a large grid.
    if (el.isNode()) {
      const p = (el as unknown as { position(): { x: number; y: number } }).position();
      const margin = 200 / zoom;
      if (p.x < view.x1 - margin || p.x > view.x2 + margin || p.y < view.y1 - margin || p.y > view.y2 + margin) return;
    }
    // Too small to read here: Cytoscape does not draw it, and it is in no one's way.
    const font = el.pstyle('font-size').pfValue;
    const least = el.pstyle('min-zoomed-font-size').pfValue;
    if (font * zoom < least) return;
    const box = el.boundingBox({ includeNodes: false, includeEdges: false, includeLabels: true, includeOverlays: false } as never);
    if (!(box.w > 0) || box.x2 < view.x1 || box.x1 > view.x2 || box.y2 < view.y1 || box.y1 > view.y2) return;
    // On the screen, so the gap between labels is the same at every zoom.
    labels.push({ id: el.id(), priority: priority(el), box: onScreen(box), kept: el.data('kind') === 'group' || el.selected(), owner: el.id() });
  });
  const hidden = crowded(labels);
  cy.batch(() => {
    cy.elements('.crowded').forEach((el) => {
      if (!hidden.has(el.id())) el.removeClass('crowded');
    });
    for (const id of hidden) cy.getElementById(id).addClass('crowded');
  });
}

/**
 * Declutters `cy` now, after each change at most once a frame, and once a pan or zoom has come to
 * rest: a pass on a large grid takes longer than a frame, which would make zooming stutter.
 */
export function keepDecluttered(cy: Core, rest = 120): () => void {
  let frame = 0;
  let timer: ReturnType<typeof setTimeout> | undefined;
  const run = () => {
    if (!cy.destroyed()) declutter(cy);
  };
  const soon = () => {
    if (frame) return;
    frame = requestAnimationFrame(() => {
      frame = 0;
      run();
    });
  };
  const settled = () => {
    clearTimeout(timer);
    timer = setTimeout(run, rest);
  };
  cy.on('viewport', settled);
  cy.on('add remove data select unselect position', soon);
  soon();
  return () => {
    cancelAnimationFrame(frame);
    clearTimeout(timer);
    cy.off('viewport', settled);
    cy.off('add remove data select unselect position', soon);
  };
}
