// Panning the drawing with the middle or right mouse button held, over anything in it: the left
// button can then move what it grabs (while editing) without a pan ever doing so by accident. As
// cimlens has it (web/src/grid/pan.ts).

import type { Core } from "cytoscape";

/** Pans whatever `cy()` draws in `container` while the middle or right button is held. */
export function panWithOtherButtons(container: HTMLElement, cy: () => Core | undefined): () => void {
  let from: { x: number; y: number } | null = null;
  let dragged = false;
  const down = (e: PointerEvent) => {
    if (e.button !== 1 && e.button !== 2) return;
    // Before Cytoscape sees it: no grab, no box selection, no context tap.
    e.preventDefault();
    e.stopPropagation();
    from = { x: e.clientX, y: e.clientY };
    dragged = false;
    container.setPointerCapture(e.pointerId);
    container.style.cursor = 'grabbing';
  };
  const move = (e: PointerEvent) => {
    if (!from) return;
    e.preventDefault();
    e.stopPropagation();
    const by = { x: e.clientX - from.x, y: e.clientY - from.y };
    if (by.x || by.y) {
      dragged = true;
      cy()?.panBy(by);
    }
    from = { x: e.clientX, y: e.clientY };
  };
  const up = (e: PointerEvent) => {
    if (!from) return;
    e.stopPropagation();
    from = null;
    container.releasePointerCapture?.(e.pointerId);
    container.style.cursor = '';
  };
  // The browser's menu after a right-button pan would be in the way; the drawing offers none.
  const menu = (e: MouseEvent) => {
    if (dragged || e.button === 2) e.preventDefault();
    dragged = false;
  };
  // Middle-button autoscroll, which some browsers start on mousedown.
  const mouse = (e: MouseEvent) => {
    if (e.button === 1 || e.button === 2) {
      e.preventDefault();
      e.stopPropagation();
    }
  };
  container.addEventListener('pointerdown', down, true);
  container.addEventListener('pointermove', move, true);
  container.addEventListener('pointerup', up, true);
  container.addEventListener('pointercancel', up, true);
  container.addEventListener('mousedown', mouse, true);
  container.addEventListener('contextmenu', menu);
  return () => {
    container.removeEventListener('pointerdown', down, true);
    container.removeEventListener('pointermove', move, true);
    container.removeEventListener('pointerup', up, true);
    container.removeEventListener('pointercancel', up, true);
    container.removeEventListener('mousedown', mouse, true);
    container.removeEventListener('contextmenu', menu);
  };
}

/**
 * Pans `cy` when a drag with the left button starts on something that cannot be moved (`ungrabbed`
 * says when nothing can): as a drag that starts on the background does.
 */
export function panFromNodes(cy: Core, ungrabbed: () => boolean): void {
  let last: { x: number; y: number } | null = null;
  cy.on('tapstart', 'node', (e) => {
    last = ungrabbed() ? { ...e.renderedPosition } : null;
  });
  cy.on('tapdrag', (e) => {
    if (!last) return;
    const p = e.renderedPosition;
    cy.panBy({ x: p.x - last.x, y: p.y - last.y });
    last = { ...p };
  });
  cy.on('tapend', () => {
    last = null;
  });
}
