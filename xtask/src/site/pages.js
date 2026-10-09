// SPDX-License-Identifier: MIT OR Apache-2.0
// The page check's harness (xtask/src/site/pages.rs). The check's server serves it, at
// /__check/pages.js, beside the built site, so the pages it loads in frames are on its own origin
// and it can measure them. Each browser tab takes the pages one at a time from /__check/next,
// loads each fresh at every width, measures it, and posts what it found to /__check/result.

'use strict';

(function () {
  const query = new URLSearchParams(location.search);
  const tab = Number(query.get('tab') || 0);

  // How long one load may take before it counts as a failure, in milliseconds.
  const LOAD_TIMEOUT_MS = 60000;
  // Text rectangles closer than this, sideways, don't collide: touching runs of one line.
  const OVERLAP_MIN_WIDTH_PX = 2;
  // ... and vertically they must share more than this part of the shorter one's height, so two
  // lines of a tight line height don't collide and a label drawn over another does.
  const OVERLAP_MIN_HEIGHT_SHARE = 0.25;
  // How far, in pixels, a box may pass an edge before it counts: sub-pixel rounding.
  const SLACK_PX = 0.5;
  // A box this small or smaller hides its contents on purpose (text for screen readers only).
  const HIDDEN_BOX_PX = 2;
  // How many examples of each problem a width reports.
  const EXAMPLES = 3;
  // The product system's widest content, in pixels (`foundations.md`, *Width*): a figure needn't
  // be shown wider than this to be shown at its size.
  const CONTENT_MAX_PX = 1120;

  async function post(path, value) {
    await fetch(path, { method: 'POST', body: JSON.stringify(value) });
  }

  function fail(message) {
    post('/__check/error', { tab: tab, message: String(message) }).catch(function () {});
  }
  window.addEventListener('error', function (event) { fail(event.message); });
  window.addEventListener('unhandledrejection', function (event) { fail(event.reason); });

  function snippet(text) {
    const words = String(text).replace(/\s+/g, ' ').trim();
    return words.length > 40 ? words.slice(0, 39) + '…' : words;
  }

  // A short CSS-like path to an element: up to four steps, stopping at an id.
  function selector(el) {
    const steps = [];
    for (let e = el; e && e.nodeType === 1 && steps.length < 4; e = e.parentElement) {
      let step = e.tagName.toLowerCase();
      if (e.id) {
        steps.unshift(step + '#' + e.id);
        break;
      }
      const classes = Array.from(e.classList).slice(0, 2);
      if (classes.length) step += '.' + classes.join('.');
      steps.unshift(step);
      if (e.tagName === 'BODY') break;
    }
    return steps.join('>');
  }

  function nextFrames(win) {
    // Two animation frames, so the page has laid out and painted what its scripts set at load.
    // A frame that never comes (a throttled frame) gives way to a timer.
    return new Promise(function (resolve) {
      let done = false;
      const finish = function () { if (!done) { done = true; resolve(); } };
      win.requestAnimationFrame(function () { win.requestAnimationFrame(finish); });
      setTimeout(finish, 500);
    });
  }

  const INFINITE = { x0: -Infinity, x1: Infinity, y0: -Infinity, y1: Infinity };

  function intersect(a, b) {
    return {
      x0: Math.max(a.x0, b.x0), x1: Math.min(a.x1, b.x1),
      y0: Math.max(a.y0, b.y0), y1: Math.min(a.y1, b.y1),
    };
  }

  function measure(win, doc) {
    const root = doc.documentElement;
    const body = doc.body;
    const view = root.clientWidth;
    // The boxes that hold the page's main content, as mdBook's `.content` does, stand for the
    // window: a box that scrolls sideways around the whole page is the page scrolling sideways.
    const main = doc.querySelector('main') || body;
    const holdsPage = function (el) { return el === root || el === body || el.contains(main); };
    const styles = new Map();
    const style = function (el) {
      let s = styles.get(el);
      if (!s) { s = win.getComputedStyle(el); styles.set(el, s); }
      return s;
    };

    // Whether an element and all its ancestors are drawn (opacity above zero).
    const opaque = new Map();
    function drawn(el) {
      if (!el || el.nodeType !== 1) return true;
      let value = opaque.get(el);
      if (value === undefined) {
        value = style(el).opacity !== '0' && drawn(el.parentElement);
        opaque.set(el, value);
      }
      return value;
    }

    // Whether an element is in a drawer put away: its nearest fixed box lies wholly off the
    // window's left edge, as mdBook's sidebar (`nav#mdbook-sidebar`) does on a narrow window.
    const stowed = new Map();
    function putAway(el) {
      if (!el || el.nodeType !== 1) return false;
      let value = stowed.get(el);
      if (value === undefined) {
        value = style(el).position === 'fixed'
          ? el.getBoundingClientRect().right <= 0
          : putAway(el.parentElement);
        stowed.set(el, value);
      }
      return value;
    }

    // The nearest ancestor an absolutely positioned element is placed in.
    function containingBlock(el) {
      for (let e = el.parentElement; e; e = e.parentElement) {
        const s = style(e);
        if (s.position !== 'static' || s.transform !== 'none') return e;
      }
      return null;
    }

    // What clips the contents of an element: `all`, every box with overflow other than visible
    // (what a reader sees without scrolling a box), `scroll`, only the boxes that scroll (what a
    // reader can reach), and whether any box that scrolls sideways is among them.
    const clips = new Map();
    const NONE = { all: INFINITE, scroll: INFINITE, sideways: false };
    function clipOf(el) {
      if (!el || el.nodeType !== 1) return NONE;
      let value = clips.get(el);
      if (value) return value;
      const s = style(el);
      let base;
      if (holdsPage(el)) {
        // Their overflow is the window's, not a box's.
        base = NONE;
      } else if (s.position === 'fixed') {
        base = NONE;
      } else if (s.position === 'absolute') {
        base = clipOf(containingBlock(el));
      } else {
        base = clipOf(el.parentElement);
      }
      value = base;
      if (!holdsPage(el)) {
        const x = s.overflowX, y = s.overflowY;
        const clipsX = x !== 'visible', clipsY = y !== 'visible';
        const cut = s.clip && s.clip !== 'auto';
        if (clipsX || clipsY || cut) {
          const r = el.getBoundingClientRect();
          const left = r.left + el.clientLeft, top = r.top + el.clientTop;
          const box = {
            x0: clipsX || cut ? left : -Infinity,
            x1: clipsX || cut ? left + (cut ? r.width : el.clientWidth) : Infinity,
            y0: clipsY || cut ? top : -Infinity,
            y1: clipsY || cut ? top + (cut ? r.height : el.clientHeight) : Infinity,
          };
          const tiny = (clipsX || cut) && box.x1 - box.x0 <= HIDDEN_BOX_PX
            || (clipsY || cut) && box.y1 - box.y0 <= HIDDEN_BOX_PX;
          const scrollsX = !cut && (x === 'auto' || x === 'scroll');
          const scrollsY = !cut && (y === 'auto' || y === 'scroll');
          const reach = {
            x0: scrollsX ? box.x0 : -Infinity, x1: scrollsX ? box.x1 : Infinity,
            y0: scrollsY ? box.y0 : -Infinity, y1: scrollsY ? box.y1 : Infinity,
          };
          value = {
            all: tiny ? { x0: 0, x1: 0, y0: 0, y1: 0 } : intersect(base.all, box),
            scroll: intersect(base.scroll, reach),
            sideways: base.sideways || scrollsX,
          };
        }
      }
      clips.set(el, value);
      return value;
    }

    const skip = new Set(['SCRIPT', 'STYLE', 'NOSCRIPT', 'TEMPLATE']);
    const past = [];
    const cut = [];
    const rects = [];
    const walker = doc.createTreeWalker(body, NodeFilter.SHOW_TEXT);
    const range = doc.createRange();
    let index = 0;
    for (let node = walker.nextNode(); node; node = walker.nextNode()) {
      if (!/\S/.test(node.data)) continue;
      const parent = node.parentElement;
      if (!parent || skip.has(parent.tagName)) continue;
      if (style(parent).visibility !== 'visible' || !drawn(parent)) continue;
      const clip = clipOf(parent);
      // Inside a box too small to show anything: hidden on purpose (for screen readers only).
      if (clip.all.x1 - clip.all.x0 <= HIDDEN_BOX_PX || clip.all.y1 - clip.all.y0 <= HIDDEN_BOX_PX) continue;
      range.selectNodeContents(node);
      const list = range.getClientRects();
      index += 1;
      for (let i = 0; i < list.length; i++) {
        const r = list[i];
        if (r.width <= 0 || r.height <= 0) continue;
        // What a reader can reach by scrolling the boxes that scroll.
        const reach = intersect({ x0: r.left, x1: r.right, y0: r.top, y1: r.bottom }, clip.scroll);
        if (reach.x1 - reach.x0 <= SLACK_PX || reach.y1 - reach.y0 <= SLACK_PX) continue;
        // Wholly off to the left in a drawer put away: the sidebar on a narrow window.
        if (reach.x1 <= 0 && putAway(parent)) continue;
        if (reach.x1 > view + SLACK_PX) {
          past.push({ selector: selector(parent), text: snippet(node.data), right: Math.round(reach.x1) });
        } else if (reach.x0 < -SLACK_PX) {
          past.push({ selector: selector(parent), text: snippet(node.data), right: Math.round(reach.x0) });
        }
        // Cut by a box that doesn't scroll: text the reader can never see.
        const seen = intersect(reach, clip.all);
        const lost = Math.max(seen.x0 - reach.x0, reach.x1 - seen.x1, seen.y0 - reach.y0, reach.y1 - seen.y1);
        if (lost > 1) {
          cut.push({ selector: selector(parent), text: snippet(node.data), lost: Math.round(lost) });
        }
        const shown = intersect(seen, { x0: 0, x1: view, y0: -Infinity, y1: Infinity });
        if (shown.x1 - shown.x0 <= SLACK_PX || shown.y1 - shown.y0 <= SLACK_PX) continue;
        rects.push({ box: shown, top: r.top, height: r.height, node: index, text: node.data });
      }
    }

    // Boxes past the right or the left edge, outside any box that scrolls sideways and any
    // drawer put away: the first of each chain (whose parent is inside), farthest out first.
    const boxes = [];
    const beyond = new Set();
    for (const el of body.querySelectorAll('*')) {
      if (skip.has(el.tagName)) continue;
      const r = el.getBoundingClientRect();
      if (r.width <= 0 && r.height <= 0) continue;
      const right = r.right > view + SLACK_PX;
      if (!right && r.left >= -SLACK_PX) continue;
      if (style(el).visibility !== 'visible' || !drawn(el)) continue;
      if (clipOf(el.parentElement).sideways) continue;
      if (r.right <= 0 && putAway(el)) continue;
      beyond.add(el);
      if (!beyond.has(el.parentElement)) {
        const reach = right ? r.right : r.left;
        boxes.push({ selector: selector(el), text: snippet(el.textContent || ''), right: Math.round(reach) });
      }
    }
    // A box holding the page that is wider inside than out: the page scrolls, or is cut,
    // sideways, wherever that box's edge is. A box whose overflow is visible neither scrolls nor
    // cuts: what reaches past it (a figure wider than the prose column) is measured by the box
    // around it.
    for (let el = main; el && el !== body && el !== root; el = el.parentElement) {
      if (style(el).overflowX !== 'visible' && el.scrollWidth > el.clientWidth + SLACK_PX) {
        const r = el.getBoundingClientRect();
        boxes.push({
          selector: selector(el),
          text: 'holds the page, ' + (el.scrollWidth - el.clientWidth) + ' px wider inside',
          right: Math.round(r.left + el.clientLeft + el.scrollWidth),
        });
      }
    }
    // Farthest past either edge first.
    const out = function (a) { return a.right > 0 ? a.right - view : -a.right; };
    boxes.sort(function (a, b) { return out(b) - out(a); });
    past.sort(function (a, b) { return out(b) - out(a); });
    cut.sort(function (a, b) { return b.lost - a.lost; });

    // Text over text: a sweep down the page, each rectangle against those still open above it.
    rects.sort(function (a, b) { return a.box.y0 - b.box.y0; });
    const overlaps = [];
    let open = [];
    let count = 0;
    for (const r of rects) {
      open = open.filter(function (o) { return o.box.y1 > r.box.y0; });
      for (const o of open) {
        // Pieces of one text on one line (an ellipsis is a piece of its own) are one run.
        if (o.node === r.node && Math.abs(o.top - r.top) < 1) continue;
        const wide = Math.min(o.box.x1, r.box.x1) - Math.max(o.box.x0, r.box.x0);
        const tall = Math.min(o.box.y1, r.box.y1) - Math.max(o.box.y0, r.box.y0);
        if (wide > OVERLAP_MIN_WIDTH_PX && tall > OVERLAP_MIN_HEIGHT_SHARE * Math.min(o.height, r.height)) {
          count += 1;
          if (overlaps.length < EXAMPLES) {
            overlaps.push({ a: snippet(o.text), b: snippet(r.text), width: Math.round(wide), height: Math.round(tall) });
          }
        }
      }
      open.push(r);
    }

    // Figures shown smaller than drawn. A figure's text is drawn at its size, so shown smaller
    // it reads smaller (12 px text in a 960 px figure shown 750 px wide reads at 9.4 px). Where
    // the box holding the page has room for a figure, up to the system's widest content, it must
    // be shown at its size; where it hasn't, the reader must be able to open it at its size:
    // mdBook's zoom, a checkbox in the figure's label that shows it over the page.
    const shrunk = [];
    const holder = main.parentElement && main.parentElement !== root ? main.parentElement : body;
    const hs = style(holder);
    const room = Math.min(CONTENT_MAX_PX,
      holder.clientWidth - parseFloat(hs.paddingLeft) - parseFloat(hs.paddingRight));
    for (const img of main.querySelectorAll('img')) {
      const r = img.getBoundingClientRect();
      if (r.width <= 0 || r.height <= 0) continue;
      if (style(img).visibility !== 'visible' || !drawn(img)) continue;
      const natural = img.naturalWidth;
      if (!(natural > 0) || r.width >= natural - SLACK_PX) continue;
      const src = img.getAttribute('src') || '';
      let why = null;
      // A pixel's slack: `clientWidth` is a whole number of pixels, and Chrome sizes a scaled
      // image from its height snapped to 1/64 px, which a wide figure's aspect ratio multiplies.
      if (r.width < Math.min(natural, room) - 1) {
        why = 'room for ' + Math.round(Math.min(natural, room)) + ' px';
      } else {
        const label = img.closest('label');
        const box = label && label.querySelector('input[type=checkbox]');
        const big = label && Array.from(label.querySelectorAll('img')).some(function (other) {
          return other !== img && other.getAttribute('src') === src;
        });
        if (!box || !big) why = 'no zoom to see it at its size';
      }
      if (why) {
        shrunk.push({
          selector: selector(img),
          text: snippet(src),
          natural: Math.round(natural),
          shown: Math.round(r.width),
          why: why,
        });
      }
    }

    return {
      client_width: view,
      scroll_width: Math.max(root.scrollWidth, body.scrollWidth),
      past: past.slice(0, EXAMPLES).concat(boxes.slice(0, EXAMPLES)),
      past_count: past.length + boxes.length,
      cut: cut.slice(0, EXAMPLES),
      cut_count: cut.length,
      overlaps: overlaps,
      overlap_count: count,
      shrunk: shrunk.slice(0, EXAMPLES),
      shrunk_count: shrunk.length,
    };
  }

  function check(page, width, height) {
    return new Promise(function (resolve) {
      const frame = document.createElement('iframe');
      frame.style.cssText = 'position:absolute;left:0;top:0;border:0;margin:0;padding:0;'
        + 'width:' + width + 'px;height:' + height + 'px';
      let done = false;
      const finish = function (result) {
        if (done) return;
        done = true;
        clearTimeout(timer);
        frame.remove();
        resolve(result);
      };
      const timer = setTimeout(function () {
        finish({ width: width, error: 'did not load in ' + LOAD_TIMEOUT_MS / 1000 + ' s' });
      }, LOAD_TIMEOUT_MS);
      frame.addEventListener('load', async function () {
        try {
          const win = frame.contentWindow;
          const doc = frame.contentDocument;
          if (!doc || !doc.body) throw new Error('the frame has no document');
          // Every face the page declares, loaded before measuring: `font-display: swap` draws
          // a fallback until a face is asked for and arrives, and `fonts.ready` right after the
          // load can resolve before layout has asked.
          const faces = Array.from(doc.fonts);
          await Promise.all(faces.map(function (f) { return f.load().catch(function () {}); }));
          await doc.fonts.ready;
          await nextFrames(win);
          const failed = faces.filter(function (f) { return f.status === 'error'; });
          if (failed.length) {
            const names = Array.from(new Set(failed.map(function (f) { return f.family; })));
            throw new Error('its fonts ' + names.join(', ') + ' did not load, so it was not '
              + 'measured in its own fonts');
          }
          const result = measure(win, doc);
          result.width = width;
          result.error = null;
          finish(result);
        } catch (err) {
          finish({ width: width, error: String((err && err.message) || err) });
        }
      }, { once: true });
      // What a first-time reader gets: no sidebar or theme remembered from an earlier load.
      // mdBook writes these only when the reader acts, never at load, so loads in parallel can't
      // leave anything for each other.
      try { localStorage.clear(); sessionStorage.clear(); } catch (err) { /* none to clear */ }
      frame.src = '/' + page;
      document.body.appendChild(frame);
    });
  }

  async function run() {
    const plan = await (await fetch('/__check/pages.json')).json();
    for (;;) {
      // The next page nobody has taken, so the tabs share the work however long each page takes.
      const taken = await (await fetch('/__check/next', { method: 'POST', body: '{}' })).json();
      const page = taken.page;
      if (page === null) break;
      const results = new Array(plan.widths.length);
      let next = 0;
      const worker = async function () {
        while (next < plan.widths.length) {
          const i = next++;
          results[i] = await check(page, plan.widths[i], plan.height);
        }
      };
      const workers = [];
      for (let i = 0; i < plan.parallel; i++) workers.push(worker());
      await Promise.all(workers);
      await post('/__check/result', { page: page, widths: results });
    }
    await post('/__check/done', { tab: tab });
  }

  run().catch(fail);
})();
