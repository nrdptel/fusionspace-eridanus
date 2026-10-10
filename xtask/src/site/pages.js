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
      const src = img.getAttribute('src') || '';
      // A figure with no size to read (broken, or an SVG without one) fails: the check can't
      // say how it is shown.
      if (!(natural > 0)) {
        shrunk.push({
          selector: selector(img), text: snippet(src), natural: 0, shown: Math.round(r.width),
          why: 'no size the check can read',
        });
        continue;
      }
      if (r.width >= natural - SLACK_PX) continue;
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

  // The product system's color roles, in each theme (`foundations.md`, *Semantic roles*, the
  // light and dark columns, and the signal fills under the table). A color the page computes
  // must be one of its theme's, or transparent.
  const ROLES = {
    light: ['#F3F4F7', '#FFFFFF', '#D6DAE4', '#566079', '#0B0F1C', '#98A1B8', '#3350D6',
      '#AC001E', '#B34F0C', '#0A6355', '#A22488', '#F5AF20'],
    navy: ['#0B0F1C', '#141A2B', '#2A3248', '#6D7790', '#F3F4F7', '#98A1B8', '#566079',
      '#768DF5', '#FB8083', '#F5AF20', '#6AD5B6', '#ED89D2', '#AC001E', '#FFFFFF', '#0A6355',
      '#3350D6'],
  };
  // The action role in each theme, a selection's fill.
  const ACTION = { light: '#3350D6', navy: '#768DF5' };
  // The system's motion (`foundations.md`, *Motion*): no motion, quick, base and slow, as
  // computed styles write them, and its two easings.
  const DURATIONS = ['0s', '0.1s', '0.16s', '0.24s'];
  const EASINGS = ['cubic-bezier(0.2, 0, 0, 1)', 'cubic-bezier(0.3, 0, 1, 1)'];

  // `rgb(r, g, b)` or `rgba(r, g, b, a)` as `#RRGGBB` and its opacity, or null for a color that
  // isn't written that way.
  function hex(value) {
    const m = /^rgba?\(([\d.]+),\s*([\d.]+),\s*([\d.]+)(?:,\s*([\d.]+))?\)$/.exec(value);
    if (!m) return null;
    return {
      rgb: '#' + [m[1], m[2], m[3]].map(function (c) {
        return Number(c).toString(16).toUpperCase().padStart(2, '0');
      }).join(''),
      alpha: m[4] === undefined ? 1 : Number(m[4]),
    };
  }

  // A computed `rgb()` or `rgba()` color as its channels, 0 to 255, and its opacity, 0 to 1, or
  // null for a color that isn't written that way.
  function channels(value) {
    const m = /^rgba?\(([\d.]+),\s*([\d.]+),\s*([\d.]+)(?:,\s*([\d.]+))?\)$/.exec(value);
    if (!m) return null;
    return { r: Number(m[1]), g: Number(m[2]), b: Number(m[3]), a: m[4] === undefined ? 1 : Number(m[4]) };
  }

  // `top` drawn over `under` (opaque) at `top`'s opacity times `alpha`.
  function over(top, under, alpha) {
    const a = top.a * alpha;
    return {
      r: top.r * a + under.r * (1 - a),
      g: top.g * a + under.g * (1 - a),
      b: top.b * a + under.b * (1 - a),
      a: 1,
    };
  }

  // WCAG 2.2's contrast ratio of two opaque colors, from their relative luminance (1.4.3, and
  // its definition of relative luminance in sRGB).
  function contrast(a, b) {
    const lum = function (c) {
      const lin = [c.r, c.g, c.b].map(function (v) {
        const s = v / 255;
        return s <= 0.04045 ? s / 12.92 : Math.pow((s + 0.055) / 1.055, 2.4);
      });
      return 0.2126 * lin[0] + 0.7152 * lin[1] + 0.0722 * lin[2];
    };
    const la = lum(a);
    const lb = lum(b);
    return (Math.max(la, lb) + 0.05) / (Math.min(la, lb) + 0.05);
  }

  // WCAG 2.2 AA's floor (1.4.3), which `foundations.md` (*Contrast*) makes the floor everywhere:
  // 4.5 : 1 for text, 3 : 1 for large text, 24 px and over, or 18.66 px (14 pt) and over in bold.
  const CONTRAST_TEXT = 4.5;
  const CONTRAST_LARGE = 3;
  function contrastFloor(s) {
    const size = parseFloat(s.fontSize);
    const bold = Number(s.fontWeight) >= 700;
    return size >= 24 || (bold && size >= 18.66) ? CONTRAST_LARGE : CONTRAST_TEXT;
  }

  // What an element's own box is drawn on, opaque: its background over its ancestors', down to
  // the first opaque one, or the browser's own canvas, white or, for a dark `color-scheme`, the
  // dark one Chrome paints. Also how much of the element's opacity, and its ancestors', comes
  // between its text and that background. `memo` holds the answers for one reading.
  function backdrop(win, doc, el, memo) {
    if (memo.has(el)) return memo.get(el);
    const s = win.getComputedStyle(el);
    const own = channels(s.backgroundColor);
    const opacity = Number(s.opacity);
    let answer;
    if (own && own.a === 1) {
      answer = { color: own, alpha: opacity };
    } else {
      const parent = el.parentElement;
      let under;
      if (parent) {
        under = backdrop(win, doc, parent, memo);
      } else {
        const dark = /dark/.test(s.colorScheme);
        under = { color: dark ? { r: 18, g: 18, b: 18, a: 1 } : { r: 255, g: 255, b: 255, a: 1 }, alpha: 1 };
      }
      answer = {
        color: own && own.a > 0 ? over(own, under.color, 1) : under.color,
        alpha: under.alpha * opacity,
      };
    }
    memo.set(el, answer);
    return answer;
  }

  // mdBook's defaults, by computed style (#383): colors off the system's roles, rounded corners
  // and shadows, and motion off its durations and easings. Every element of the body is read, its
  // `::before` and `::after` too, whether or not it is drawn now: the help popup and the copy
  // tooltip are in every page, hidden until asked for. A search hit (`mark`) and a search result
  // are made only when asked for, so one of each is added for the reading. Hover and focus aren't
  // read. `reduced` reads motion with the stylesheets' reduced-motion rules applied, where
  // everything must snap; `as` names the reading in its examples.
  function styleDefaults(win, doc, theme, reduced, found, as) {
    const allowed = ROLES[theme];
    const note = function (kind, el, pseudo, property, value) {
      found[kind + '_count']++;
      if (found[kind].length < EXAMPLES) {
        found[kind].push({
          selector: selector(el) + (pseudo || ''),
          property: property,
          value: value,
          theme: as || (theme + (reduced ? ', reduced motion' : '')),
        });
      }
    };
    // A role, opaque; transparent; or, for a background only, a role seen through: a dialog's
    // dimmed canvas (`foundations.md`, *Lines and shape*).
    const color = function (el, pseudo, property, value) {
      const h = hex(value);
      const ok = h !== null && (h.alpha === 0 || (allowed.indexOf(h.rgb) >= 0
        && (h.alpha === 1 || property === 'background-color')));
      if (!ok) note('color', el, pseudo, property, value);
    };
    // Text against what it is drawn on (`foundations.md`, *Contrast*): each element's own text,
    // and text a `::before` or `::after` draws, faded by any opacity between it and its
    // background. An opacity on a group fades its text the same way (#432).
    const memo = new Map();
    const legible = function (el, pseudo, s) {
      const ink = channels(s.color);
      if (!ink) return;
      const under = backdrop(win, doc, el, memo);
      let ground = under.color;
      if (pseudo) {
        const fill = channels(s.backgroundColor);
        if (fill && fill.a > 0) ground = over(fill, ground, 1);
      }
      const ratio = contrast(over(ink, ground, under.alpha), ground);
      const floor = contrastFloor(s);
      if (ratio < floor) {
        note('contrast', el, pseudo, 'color', s.color + ' at ' + ratio.toFixed(2) + ' : 1, under '
          + floor + ' : 1');
      }
    };
    const ownText = function (el) {
      return Array.from(el.childNodes).some(function (n) {
        return n.nodeType === 3 && n.textContent.trim() !== '';
      });
    };
    const els = [doc.documentElement].concat(doc.body
      ? [doc.body].concat(Array.from(doc.body.querySelectorAll('*'))) : []);
    for (const el of els) {
      const tag = el.tagName.toLowerCase();
      if (tag === 'script' || tag === 'style' || tag === 'noscript' || tag === 'template') continue;
      const svg = el instanceof win.SVGElement;
      for (const pseudo of [null, '::before', '::after']) {
        const s = win.getComputedStyle(el, pseudo);
        if (pseudo && (s.content === 'none' || s.content === 'normal')) continue;
        if (!reduced) {
          if (!svg && (pseudo ? /^["']/.test(s.content) && s.content.length > 2 : ownText(el))) {
            legible(el, pseudo, s);
          }
          color(el, pseudo, 'color', s.color);
          color(el, pseudo, 'background-color', s.backgroundColor);
          for (const side of ['top', 'right', 'bottom', 'left']) {
            const style = s.getPropertyValue('border-' + side + '-style');
            const width = parseFloat(s.getPropertyValue('border-' + side + '-width'));
            if (style !== 'none' && style !== 'hidden' && width > 0) {
              color(el, pseudo, 'border-' + side + '-color',
                s.getPropertyValue('border-' + side + '-color'));
            }
          }
          if (s.outlineStyle !== 'none' && parseFloat(s.outlineWidth) > 0) {
            color(el, pseudo, 'outline-color', s.outlineColor);
          }
          if (s.textDecorationLine !== 'none') {
            color(el, pseudo, 'text-decoration-color', s.textDecorationColor);
          }
          if (svg) {
            for (const property of ['fill', 'stroke']) {
              const value = s.getPropertyValue(property);
              if (value !== 'none' && !/^url\(/.test(value)) color(el, pseudo, property, value);
            }
          }
          for (const corner of ['top-left', 'top-right', 'bottom-right', 'bottom-left']) {
            const value = s.getPropertyValue('border-' + corner + '-radius');
            if (value !== '0px') note('shape', el, pseudo, 'border-' + corner + '-radius', value);
          }
          if (s.boxShadow !== 'none') note('shape', el, pseudo, 'box-shadow', s.boxShadow);
          // A filter or an image drawn as content paints colors the check can't read: mdBook
          // tints its copy icon with one.
          if (s.filter !== 'none') note('color', el, pseudo, 'filter', s.filter);
          if (s.backdropFilter && s.backdropFilter !== 'none') {
            note('color', el, pseudo, 'backdrop-filter', s.backdropFilter);
          }
          if (pseudo && /url\(/.test(s.content)) note('color', el, pseudo, 'content', 'an image');
          // A search hit is a selection, in the action fill, not a signal fill (#383).
          if (tag === 'mark') {
            const fill = hex(s.backgroundColor);
            if (!fill || fill.rgb !== ACTION[theme]) {
              note('color', el, pseudo, 'background-color', s.backgroundColor);
            }
          }
          if (s.textShadow !== 'none') note('shape', el, pseudo, 'text-shadow', s.textShadow);
        }
        if (s.scrollBehavior !== 'auto') {
          note('motion', el, pseudo, 'scroll-behavior', s.scrollBehavior);
        }
        for (const kind of ['transition', 'animation']) {
          if (kind === 'animation' && s.animationName === 'none') continue;
          const durations = s.getPropertyValue(kind + '-duration').split(', ');
          const easings = s.getPropertyValue(kind + '-timing-function').split(/,\s*(?![^()]*\))/);
          durations.forEach(function (duration, i) {
            const easing = easings[i % easings.length];
            const ok = reduced
              ? duration === '0s'
              : DURATIONS.indexOf(duration) >= 0
                && (duration === '0s' || EASINGS.indexOf(easing) >= 0);
            if (!ok) note('motion', el, pseudo, kind, duration + ' ' + easing);
          });
          if (kind === 'animation' && s.animationIterationCount.split(', ').indexOf('infinite') >= 0) {
            note('motion', el, pseudo, 'animation-iteration-count', 'infinite');
          }
        }
      }
    }
  }

  // Each top-level media rule of the page's stylesheets whose condition matches `media`, applied
  // as if the reader's system met it (reduced motion, or a dark scheme): its rules inserted
  // unconditionally where it stands, so the cascade is the one that reader gets. Returns a
  // function that takes them out again.
  function applyMedia(doc, media) {
    const added = [];
    for (const sheet of Array.from(doc.styleSheets)) {
      if (sheet.disabled) continue;
      let rules;
      try { rules = sheet.cssRules; } catch (err) { continue; }
      for (let i = rules.length - 1; i >= 0; i--) {
        const rule = rules[i];
        if (!(rule instanceof doc.defaultView.CSSMediaRule)) continue;
        if (!media.test(rule.media.mediaText)) continue;
        const inner = Array.from(rule.cssRules).map(function (r) { return r.cssText; });
        for (let j = inner.length - 1; j >= 0; j--) {
          sheet.insertRule(inner[j], i + 1);
          added.push(sheet.cssRules[i + 1]);
        }
      }
    }
    return function () {
      for (const rule of added) {
        const sheet = rule.parentStyleSheet;
        const index = Array.from(sheet.cssRules).indexOf(rule);
        if (index >= 0) sheet.deleteRule(index);
      }
    };
  }

  // The theme a page is shown in: mdBook gives the dark ones a dark `color-scheme`. It picks
  // navy for a reader whose system is dark, and a headless browser may say it is.
  function themeOf(win, doc) {
    return /dark/.test(win.getComputedStyle(doc.documentElement).colorScheme) ? 'navy' : 'light';
  }

  // mdBook's switch between its light and navy themes, as its `set_theme` makes it, without
  // remembering it: the page's class, and the code colors for that theme.
  function switchTheme(doc, theme) {
    const sheet = function (id, disabled) {
      const link = doc.getElementById(id);
      if (link) link.disabled = disabled;
    };
    sheet('mdbook-highlight-css', theme !== 'light');
    sheet('mdbook-tomorrow-night-css', theme !== 'navy');
    sheet('mdbook-ayu-highlight-css', true);
    doc.documentElement.classList.remove('light', 'navy', 'coal', 'ayu', 'rust');
    doc.documentElement.classList.add(theme);
  }

  // The transitions a change of class starts, run to their end: colors read during one are
  // halfway between two roles. Finishing them, not waiting, needs no frames, which a busy page
  // may not get in time.
  async function settle(win, doc) {
    for (const a of doc.getAnimations()) {
      if (Number.isFinite(a.effect.getComputedTiming().endTime)) a.finish();
    }
    await nextFrames(win);
  }

  // What a reader with a keyboard, a screen in Windows' high-contrast mode, or a sticky bar over
  // the page gets (#383; `web.md`, *Theme* and *Accessibility*), read in the theme the page loads
  // in:
  // - the focus ring: every element a keyboard reaches, focused as by the keyboard, draws a 2 px
  //   solid outline in the action role, 2 px from its edge; a control hidden for its picture (the
  //   zoom's checkbox) draws it on the element after it;
  // - forced colors, which paint every color in the reader's own few and drop backgrounds' tints:
  //   a box set apart from what is behind it only by its fill must have a border or an outline
  //   there, an inline icon must be drawn in its text color, and an icon painted through a mask
  //   (a background shaped by an image) must keep its own colors (`forced-color-adjust: none`);
  // - a bar fixed or stuck to the top of the window must be covered by the page's
  //   `scroll-padding-top`, so what the keyboard focuses never scrolls under it (2.4.11);
  // - mdBook's script scrolls to the top smoothly on a click on the menu bar's title, motion no
  //   stylesheet can turn off: the click must ask for no smooth scroll.
  function probeAccess(win, doc, found) {
    const theme = themeOf(win, doc);
    const note = function (kind, el, pseudo, property, value) {
      found[kind + '_count']++;
      if (found[kind].length < EXAMPLES) {
        found[kind].push({
          selector: selector(el) + (pseudo || ''), property: property, value: value, theme: theme,
        });
      }
    };
    const view = doc.documentElement.clientWidth;
    const tall = win.innerHeight;
    const els = doc.body ? Array.from(doc.body.querySelectorAll('*')) : [];
    const boxOf = function (el) {
      const r = el.getBoundingClientRect();
      return r.width > HIDDEN_BOX_PX && r.height > HIDDEN_BOX_PX ? r : null;
    };

    // The focus ring, once for each kind of element, as its short path names it: print.html
    // holds thousands of links that differ only in where they go.
    const kinds = new Set();
    const before = doc.activeElement;
    for (const el of doc.querySelectorAll(
      'a[href], button, input, select, textarea, summary, [tabindex]')) {
      if (el.tabIndex < 0 || el.disabled) continue;
      const key = selector(el);
      if (kinds.has(key)) continue;
      el.focus({ preventScroll: true, focusVisible: true });
      if (doc.activeElement !== el) continue;
      kinds.add(key);
      if (!el.matches(':focus-visible')) {
        throw new Error('the browser did not draw ' + key + ' as focused by the keyboard');
      }
      const ring = el.getClientRects().length && !boxOf(el) ? el.nextElementSibling : el;
      if (!ring) {
        note('focus', el, null, 'outline', 'none: hidden, with nothing after it to draw on');
        continue;
      }
      const s = win.getComputedStyle(ring);
      const h = hex(s.outlineColor);
      const ok = s.outlineStyle === 'solid' && s.outlineWidth === '2px'
        && s.outlineOffset === '2px' && h !== null && h.alpha === 1 && h.rgb === ACTION[theme];
      if (!ok) {
        note('focus', ring, null, 'outline', s.outlineWidth + ' ' + s.outlineStyle + ' '
          + s.outlineColor + ', offset ' + s.outlineOffset);
      }
    }
    if (doc.activeElement && doc.activeElement !== before) doc.activeElement.blur();

    // Forced colors. The boxes set apart only by a fill, found in the page's own colors.
    const memo = new Map();
    const layers = [];
    const TABLE = ['TABLE', 'THEAD', 'TBODY', 'TFOOT', 'TR', 'TH', 'TD', 'CAPTION'];
    for (const el of els) {
      if (el instanceof win.SVGElement || TABLE.indexOf(el.tagName) >= 0) continue;
      const s = win.getComputedStyle(el);
      if (s.display === 'none' || s.display === 'contents' || /^inline/.test(s.display)) continue;
      const fill = channels(s.backgroundColor);
      if (!fill || fill.a === 0 || !el.parentElement || !boxOf(el)) continue;
      const behind = backdrop(win, doc, el.parentElement, memo).color;
      const own = over(fill, behind, 1);
      const same = Math.abs(own.r - behind.r) < 1 && Math.abs(own.g - behind.g) < 1
        && Math.abs(own.b - behind.b) < 1;
      if (!same) layers.push([el, s.backgroundColor]);
    }
    const undo = applyMedia(doc, /forced-colors:\s*active/);
    try {
      for (const [el, fill] of layers) {
        const s = win.getComputedStyle(el);
        if (s.forcedColorAdjust === 'none') continue;
        const bordered = ['top', 'right', 'bottom', 'left'].some(function (side) {
          const style = s.getPropertyValue('border-' + side + '-style');
          return style !== 'none' && style !== 'hidden'
            && parseFloat(s.getPropertyValue('border-' + side + '-width')) > 0;
        }) || (s.outlineStyle !== 'none' && parseFloat(s.outlineWidth) > 0);
        if (!bordered) note('forced', el, null, 'background-color', fill + ' with no border');
      }
      for (const el of els) {
        if (el instanceof win.SVGElement) {
          if (!el.getClientRects().length) continue;
          const s = win.getComputedStyle(el);
          for (const property of ['fill', 'stroke']) {
            const value = s.getPropertyValue(property);
            if (value !== 'none' && !/^url\(/.test(value) && value !== s.color) {
              note('forced', el, null, property, value + ', not its text color ' + s.color);
            }
          }
          continue;
        }
        for (const pseudo of [null, '::before', '::after']) {
          const s = win.getComputedStyle(el, pseudo);
          if (pseudo && (s.content === 'none' || s.content === 'normal')) continue;
          const mask = s.getPropertyValue('mask-image') || s.getPropertyValue('-webkit-mask-image');
          if (mask && mask !== 'none' && s.forcedColorAdjust !== 'none') {
            note('forced', el, pseudo, 'mask-image', 'an icon drawn through a mask, its colors forced');
          }
        }
      }
    } finally {
      undo();
    }

    // A bar at the top, and the padding that keeps what is scrolled to out from under it.
    // mdBook's script takes its menu bar's `sticky` class off at load, below 1,080 px, and puts
    // it back when the reader scrolls up: the bar is read as it sticks, on a phone two rows tall.
    const scroller = doc.scrollingElement || doc.documentElement;
    const padding = parseFloat(win.getComputedStyle(scroller).scrollPaddingTop) || 0;
    const menuBar = doc.getElementById('mdbook-menu-bar');
    const unstuck = menuBar !== null && !menuBar.classList.contains('sticky');
    if (unstuck) menuBar.classList.add('sticky');
    for (const el of els) {
      const s = win.getComputedStyle(el);
      if (s.position !== 'sticky' && s.position !== 'fixed') continue;
      if (s.display === 'none' || s.visibility === 'hidden') continue;
      const r = boxOf(el);
      if (!r || r.width < view / 2 || r.height >= tall / 2) continue;
      const top = parseFloat(s.top);
      // A sticky bar sticks `top` from the window's top edge; a fixed one is where it is.
      const reach = s.position === 'sticky' ? (Number.isFinite(top) ? top + r.height : null)
        : (r.top <= SLACK_PX ? r.bottom : null);
      if (reach !== null && padding + SLACK_PX < reach) {
        note('sticky', el, null, 'scroll-padding-top', padding + 'px, under a bar reaching '
          + Math.round(reach * 10) / 10 + ' px');
      }
    }
    if (unstuck) menuBar.classList.remove('sticky');

    // The title's click, with every way a script scrolls watched for a smooth scroll.
    const title = doc.querySelector('.menu-title');
    if (title) {
      const asked = [];
      const watched = [
        [win.Element.prototype, 'scrollTo'], [win.Element.prototype, 'scroll'],
        [win.Element.prototype, 'scrollBy'], [win.Element.prototype, 'scrollIntoView'],
        [win, 'scrollTo'], [win, 'scroll'], [win, 'scrollBy'],
      ];
      const saved = watched.map(function (w) {
        return [Object.prototype.hasOwnProperty.call(w[0], w[1]), w[0][w[1]]];
      });
      const at = scroller.scrollTop;
      watched.forEach(function (w, i) {
        w[0][w[1]] = function (options) {
          if (options && typeof options === 'object' && options.behavior === 'smooth') {
            asked.push(w[1]);
          }
          return saved[i][1].apply(this, arguments);
        };
      });
      try {
        title.click();
      } finally {
        watched.forEach(function (w, i) {
          if (saved[i][0]) w[0][w[1]] = saved[i][1]; else delete w[0][w[1]];
        });
        scroller.scrollTop = at;
      }
      for (const how of asked) {
        note('motion', title, null, 'scroll-behavior', 'smooth, asked by a script with ' + how
          + ' on a click');
      }
    }
  }

  async function probeDefaults(win, doc, found) {
    // A search hit and a search result, as mdBook's search makes them.
    const p = doc.querySelector('main p');
    if (p) {
      const hit = doc.createElement('mark');
      hit.setAttribute('data-markjs', 'true');
      hit.textContent = 'hit';
      p.appendChild(hit);
    }
    const results = doc.getElementById('mdbook-searchresults');
    if (results) {
      const li = doc.createElement('li');
      li.className = 'focus';
      li.innerHTML = '<a href="#">A result</a><span class="teaser">a teaser</span>';
      results.appendChild(li);
    }
    const first = themeOf(win, doc);
    styleDefaults(win, doc, first, false, found);
    const undo = applyMedia(doc, /prefers-reduced-motion:\s*reduce/);
    styleDefaults(win, doc, first, true, found);
    undo();
    // As a reader with scripts off gets the page: mdBook's script adds `js` to the root before
    // the page draws, and without it mdBook's stylesheets give `html:not(.js)` their own colors
    // and a 0.3 s slide (#383). The class names the stylesheets' rules, so taking it off applies
    // them; the page's theme is the one it was saved with, as for that reader.
    if (doc.documentElement.classList.contains('js')) {
      doc.documentElement.classList.remove('js');
      await settle(win, doc);
      const bare = themeOf(win, doc);
      styleDefaults(win, doc, bare, false, found, bare + ', scripts off');
      // And with a dark system, where mdBook's stylesheets pick their own dark colors.
      if (bare === 'light') {
        const light = applyMedia(doc, /prefers-color-scheme:\s*dark/);
        await settle(win, doc);
        styleDefaults(win, doc, 'navy', false, found, 'navy, scripts off, a dark system');
        light();
        await settle(win, doc);
      }
      doc.documentElement.classList.add('js');
      await settle(win, doc);
    }
    // The other theme, on mdBook's pages that switch: a page without mdBook's code stylesheets,
    // or its sidebar for readers without scripts, is read in the theme it loads in.
    if (doc.getElementById('mdbook-tomorrow-night-css') && doc.documentElement.classList.contains('js')) {
      const other = first === 'light' ? 'navy' : 'light';
      switchTheme(doc, other);
      // The colors change over the theme's transitions: read them once those have run.
      await settle(win, doc);
      if (themeOf(win, doc) !== other) throw new Error('the page did not switch to ' + other);
      styleDefaults(win, doc, other, false, found);
    }
  }

  // The product system's type scale (`foundations.md`, *Type*): each token's size and line
  // height in pixels and its family. Archivo is the prose family, Cascadia Mono the data one.
  const SCALE = [
    [12, 16, 'Cascadia Mono'], // label
    [14, 20, 'Cascadia Mono'], // heading, code
    [20, 24, 'Cascadia Mono'], // subtitle
    [28, 32, 'Cascadia Mono'], // title, readout
    [40, 44, 'Cascadia Mono'], // display, readout-l
    [14, 20, 'Archivo'], // small
    [16, 24, 'Archivo'], // body
    [20, 28, 'Archivo'], // lead
  ];
  // The tokens set in capitals: `label` and `heading`.
  const CAPITALS = [[12, 16, 'Cascadia Mono'], [14, 20, 'Cascadia Mono']];
  // No text smaller than this on screen, in pixels (`foundations.md`, *Type*).
  const SMALLEST_PX = 12;
  // The longest a line of prose may be, in characters (`foundations.md`, *Type*).
  const MEASURE_CHARS = 68;
  // The gutters, in pixels: 16 on a phone, 32 from 720 px (`foundations.md`, *Width*).
  const GUTTER_PX = 16;
  const GUTTER_WIDE_PX = 32;
  const GUTTER_WIDE_FROM_PX = 720;
  // Prose: the blocks whose lines are held to the measure.
  const PROSE = ['P', 'LI', 'DD', 'DT', 'BLOCKQUOTE', 'FIGCAPTION'];

  // Whether two lengths in pixels are the same, to sub-pixel rounding.
  function samePx(a, b) { return Math.abs(a - b) < 0.05; }

  // The page's frame (#384): its type, its measure and its gutters, read once at each width in
  // the theme the page loads in.
  function probeFrame(win, doc) {
    const found = {
      type: [], type_count: 0, measure: [], measure_count: 0, gutter: [], gutter_count: 0,
      squeezed: [], squeezed_count: 0,
    };
    const width = win.innerWidth;
    const note = function (kind, sel, property, value) {
      found[kind + '_count']++;
      if (found[kind].length < EXAMPLES) {
        found[kind].push({ selector: sel, property: property, value: value, theme: width + ' px' });
      }
    };

    // Type: every element that shows text, drawn now or hidden until asked for, with its
    // `::before` and `::after` when they show text, and every field.
    const els = doc.body ? [doc.body].concat(Array.from(doc.body.querySelectorAll('*'))) : [];
    for (const el of els) {
      const tag = el.tagName.toLowerCase();
      if (tag === 'script' || tag === 'style' || tag === 'noscript' || tag === 'template') continue;
      if (el instanceof win.SVGElement) continue;
      // A field that shows text: a text box, a list to choose from. A checkbox shows none.
      const field = tag === 'textarea' || tag === 'select' || (tag === 'input'
        && ['text', 'search', 'email', 'url', 'tel', 'number', 'password'].indexOf(el.type) >= 0);
      const ownText = Array.from(el.childNodes).some(function (n) {
        return n.nodeType === 3 && /\S/.test(n.data);
      });
      for (const pseudo of [null, '::before', '::after']) {
        const s = win.getComputedStyle(el, pseudo);
        if (pseudo) {
          const m = /^"([\s\S]*)"$/.exec(s.content);
          if (!m || !/\S/.test(m[1])) continue;
        } else if (!ownText && !field) {
          continue;
        }
        // Text for screen readers only, in a box of 2 px or less.
        if (parseFloat(s.width) <= HIDDEN_BOX_PX && parseFloat(s.height) <= HIDDEN_BOX_PX) continue;
        const sel = selector(el) + (pseudo || '');
        const family = s.fontFamily.split(',')[0].trim().replace(/^["']|["']$/g, '');
        const size = parseFloat(s.fontSize);
        const line = s.lineHeight === 'normal' ? NaN : parseFloat(s.lineHeight);
        const shown = size + 'px/' + s.lineHeight + ' ' + family;
        if (family !== 'Archivo' && family !== 'Cascadia Mono') {
          note('type', sel, 'font-family', s.fontFamily);
        } else {
          // The family's metric-matched fallback second (the system's `fonts.css`), so text
          // drawn before the face arrives takes the same room (#383).
          const second = (s.fontFamily.split(',')[1] || '').trim().replace(/^["']|["']$/g, '');
          if (second !== family + ' Fallback') note('type', sel, 'font-family', s.fontFamily);
        }
        // A superscript or subscript sits outside the line's scale, but never below 12 px.
        if (el.closest('sup, sub')) {
          if (size < SMALLEST_PX - 0.05) note('type', sel, 'font-size', shown);
        } else if (!SCALE.some(function (t) {
          return samePx(t[0], size) && samePx(t[1], line) && t[2] === family;
        })) {
          note('type', sel, 'font', shown);
        }
        const weight = Number(s.fontWeight);
        if (weight < 400) note('type', sel, 'font-weight', s.fontWeight);
        if (s.textTransform === 'uppercase' && !CAPITALS.some(function (t) {
          return samePx(t[0], size) && samePx(t[1], line) && t[2] === family;
        })) {
          note('type', sel, 'text-transform', 'uppercase at ' + shown);
        }
        if (family === 'Archivo' && s.fontVariantNumeric.split(' ').indexOf('tabular-nums') < 0) {
          note('type', sel, 'font-variant-numeric', s.fontVariantNumeric);
        }
        // Navigation in Cascadia Mono 14 (`web.md`, *Page anatomy*), the current page
        // underlined 2 px.
        if (el.closest('.sidebar, ol.chapter')) {
          if (family !== 'Cascadia Mono' || !samePx(size, 14)) {
            note('type', sel, 'navigation font', shown);
          }
          if (!pseudo && tag === 'a' && el.classList.contains('active')
            && (s.textDecorationLine.indexOf('underline') < 0
              || s.textDecorationThickness !== '2px')) {
            note('type', sel, 'current page', s.textDecorationLine + ' '
              + s.textDecorationThickness);
          }
        }
      }
    }

    // The measure: no line of prose in `main` longer than 68 characters, spaces between words
    // counted once, as drawn. Each text node belongs to its nearest box that isn't inline; the
    // lines of a prose block are found from where its characters are drawn: a character more
    // than half a line below the line before starts a new one.
    const main = doc.querySelector('main');
    if (main) {
      const range = doc.createRange();
      const midAt = function (node, k) {
        range.setStart(node, k);
        range.setEnd(node, k + 1);
        const r = range.getClientRects()[0];
        return r ? (r.top + r.bottom) / 2 : null;
      };
      const blockOf = function (node) {
        for (let e = node.parentElement; e && e !== main; e = e.parentElement) {
          const d = win.getComputedStyle(e).display;
          if (d !== 'inline' && d !== 'contents') return e;
        }
        return main;
      };
      const blocks = new Map();
      const walker = doc.createTreeWalker(main, 4);
      for (let n = walker.nextNode(); n; n = walker.nextNode()) {
        if (!/\S/.test(n.data)) {
          // Spaces still part words: note them on the block.
          const b = blocks.get(blockOf(n));
          if (b) b.push(n);
          continue;
        }
        const block = blockOf(n);
        if (PROSE.indexOf(block.tagName) < 0 || block.closest('table, pre')) continue;
        if (!blocks.has(block)) blocks.set(block, []);
        blocks.get(block).push(n);
      }
      for (const [block, nodes] of blocks) {
        const s = win.getComputedStyle(block);
        const half = (s.lineHeight === 'normal' ? 1.2 * parseFloat(s.fontSize)
          : parseFloat(s.lineHeight)) / 2;
        let lineMid = null;
        let chars = 0;
        let text = '';
        let longest = 0;
        let longestText = '';
        let gap = false; // whitespace since the last word
        const close = function () {
          if (chars > longest) { longest = chars; longestText = text; }
        };
        const piece = function (str, mid) {
          if (lineMid === null || mid > lineMid + half) {
            if (lineMid !== null) close();
            lineMid = mid;
            chars = 0;
            text = '';
          } else if (gap) {
            chars += 1;
            text += ' ';
          }
          chars += Array.from(str).length;
          text += str;
          gap = false;
        };
        for (const node of nodes) {
          const data = node.data;
          const words = /\S+/g;
          let last = 0;
          for (let m = words.exec(data); m; m = words.exec(data)) {
            if (m.index > last) gap = true;
            last = m.index + m[0].length;
            let start = m.index;
            const end = last;
            while (start < end) {
              const mid = midAt(node, start);
              if (mid === null) break;
              const endMid = midAt(node, end - 1);
              if (endMid !== null && endMid <= mid + half) {
                piece(data.slice(start, end), mid);
                break;
              }
              // The word wraps: the last character still on this line.
              let lo = start;
              let hi = end - 1;
              while (hi - lo > 1) {
                const k = (lo + hi) >> 1;
                const km = midAt(node, k);
                if (km !== null && km <= mid + half) lo = k; else hi = k;
              }
              piece(data.slice(start, lo + 1), mid);
              start = lo + 1;
            }
          }
          if (last < data.length) gap = true;
        }
        close();
        if (longest > MEASURE_CHARS) {
          note('measure', selector(block), 'a line of ' + longest + ' characters',
            snippet(longestText));
        }
      }

      // A code block or table that scrolls sideways inside a box held to the prose measure: it
      // keeps the column's width (ADR-223), so nothing around it in `main` may cap it.
      for (const box of main.querySelectorAll('pre, .table-wrapper, table')) {
        if (box.scrollWidth <= box.clientWidth + SLACK_PX) continue;
        for (let e = box.parentElement; e && e !== main; e = e.parentElement) {
          const cap = win.getComputedStyle(e).maxWidth;
          if (cap !== 'none') {
            note('squeezed', selector(box), 'a cap of ' + cap + ' from', selector(e));
            break;
          }
        }
      }

      // Gutters: the space between the column and the edges of the pane that holds it, the
      // page's wrapper beside the sidebar.
      const paneEl = doc.querySelector('.page-wrapper') || doc.body;
      const pane = paneEl.getBoundingClientRect();
      const paneRight = Math.min(pane.right, doc.documentElement.clientWidth);
      const col = main.getBoundingClientRect();
      const left = col.left - pane.left;
      const right = paneRight - col.right;
      const gutter = width >= GUTTER_WIDE_FROM_PX ? GUTTER_WIDE_PX : GUTTER_PX;
      const most = parseFloat(win.getComputedStyle(main).maxWidth);
      const full = Number.isFinite(most) && col.width >= most - SLACK_PX;
      const round = function (v) { return Math.round(v * 10) / 10; };
      // Each side the gutter, or more where the column has reached its widest.
      const off = function (side) {
        return side < gutter - SLACK_PX || (!full && side > gutter + SLACK_PX);
      };
      if (off(left) || off(right)) {
        note('gutter', 'main', 'gutters', round(left) + ' px left and ' + round(right)
          + ' px right, against ' + gutter + ' px');
      }
    }
    return found;
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
          // A metric-matched fallback face names a font of the reader's own system (`local()`),
          // which a system without that font lacks; the page's own faces must all load.
          const failed = faces.filter(function (f) {
            return f.status === 'error' && !/ Fallback$/.test(f.family);
          });
          if (failed.length) {
            const names = Array.from(new Set(failed.map(function (f) { return f.family; })));
            throw new Error('its fonts ' + names.join(', ') + ' did not load, so it was not '
              + 'measured in its own fonts');
          }
          const result = measure(win, doc);
          Object.assign(result, probeFrame(win, doc));
          const found = {
            color: [], color_count: 0, shape: [], shape_count: 0, motion: [], motion_count: 0,
            contrast: [], contrast_count: 0, focus: [], focus_count: 0, forced: [],
            forced_count: 0, sticky: [], sticky_count: 0,
          };
          // Before the defaults' readings, which leave the page in its other theme.
          probeAccess(win, doc, found);
          await probeDefaults(win, doc, found);
          Object.assign(result, found);
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
