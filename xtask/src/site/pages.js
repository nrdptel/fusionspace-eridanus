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
  // How long the readings of one loaded page may take, in milliseconds. A tab's frames share its
  // main thread, so three frames of print.html, which holds every chapter (about 45,000
  // elements), share it as they are read; the readings took more than a minute each in CI.
  const MEASURE_TIMEOUT_MS = 240000;
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
  // How opaque the action role is in a selection of text (`fusionspace.css`, `::selection`).
  const SELECTION_ALPHA = 0.22;
  // A text of marks alone, spaces aside (#383): arrows, technical symbols, shapes, symbols and
  // dingbats, more arrows, guillemets, pictographs and emoji, and the private-use characters
  // icon fonts draw in.
  const MARK = '\\u00AB\\u00BB\\u2039\\u203A\\u2190-\\u21FF\\u2300-\\u23FF\\u25A0-\\u25FF'
    + '\\u2600-\\u27BF\\u27F0-\\u27FF\\u2900-\\u297F\\u2B00-\\u2BFF\\uE000-\\uF8FF'
    + '\\u{1F000}-\\u{1FAFF}\\u{F0000}-\\u{10FFFD}\\uFE0F';
  const MARKS = new RegExp('^\\s*[' + MARK + '][\\s' + MARK + ']*$', 'u');
  // `:target` in a selector, and the attribute that stands for it while every element with an id
  // is read as the one jumped to.
  const TARGET = /:target(?![-\w])/;
  const TARGETS = /:target(?![-\w])/g;
  const TARGET_ATTR = 'data-check-target';
  // On paper (`web.md`, *Print*): what must not be drawn (navigation and controls), what must
  // stay on one sheet, and the title block's fields that must be drawn.
  const PRINT_HIDDEN = 'nav, [role="navigation"], button, input, select, textarea, '
    + '[role="button"], [role="menuitem"], [role="menu"], [role="search"], #mdbook-sidebar, '
    + '#mdbook-menu-bar';
  const PRINT_WHOLE = 'tr, .fs-note, .fs-titleblock';
  const TITLE_BLOCK_PRINTED = ['Version', 'Date of issue'];
  // The sizes the system draws its icons at (`foundations.md`, *Icons*), square.
  const ICON_SIZES = [16, 20, 24];
  // The page's frame (#384; `web.md`, *Page anatomy*): the lockup's height, the brand's clear
  // space around it as a share of that height, the inks it may be drawn in (Void on light, Paper
  // on dark), the width from which the sidebar stays open with no menu button, and the title
  // block's border.
  const LOCKUP_PX = 24;
  const CLEAR_SPACE = 0.25;
  const INKS = ['rgb(11, 15, 28)', 'rgb(243, 244, 247)'];
  const SIDEBAR_OPEN_FROM_PX = 720;
  const TITLE_BLOCK_BORDER_PX = 2;
  // The intro and the sheets (#384): the tag's type size and border, the sheet's rule on top and
  // its number's type size. The theme switch: its choices, in order, the width from which it
  // stands in the header (`switch.rs`, `HEADER_FROM_PX`), and its buttons' least height and type
  // size (`web.md`, *Components*: buttons 44 px tall in Cascadia Mono 14).
  const TAG_PX = 12;
  const TAG_BORDER_PX = 1;
  const SHEET_RULE_PX = 2;
  const SHEET_NUMBER_PX = 12;
  const SWITCH_CHOICES = ['Auto', 'Light', 'Dark'];
  // mdBook's themes the switch doesn't offer, which a reader may have saved before it.
  const SWITCH_RETIRED = ['coal', 'ayu', 'rust'];
  const SWITCH_HEADER_PX = 960;
  const SWITCH_BUTTON_PX = 44;
  const SWITCH_TYPE_PX = 14;
  // The width a narrow window is widened to, past 720 px, and how long the sidebar's slide
  // (240 ms, the system's slow step) is given to run.
  const SIDEBAR_WIDENED_PX = 800;
  const SIDEBAR_SLIDE_WAIT_MS = 400;
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
  // everything must snap; `as` names the reading in its examples. `printing` reads a page with
  // its print rules applied, where neither motion nor a selection shows, and text marks aren't
  // read.
  //
  // Text marks (#383; `foundations.md`, *Icons*): a text that is nothing but marks (arrows,
  // dingbats, shapes, symbols, guillemets, an icon font's private characters) is an icon drawn
  // as a character. One fails in the page's chrome, outside its `main`, and as a `::before` or
  // `::after` anywhere: mdBook's `❱` fold toggles, its `✓` beside the chosen theme and its `»`
  // before a heading jumped to. A key's legend (`kbd`, as the help popup's `←`) is a key's name,
  // and prose in `main` may use a mark as a word.
  function styleDefaults(win, doc, theme, reduced, found, as, printing) {
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
    const main = doc.querySelector('main');
    const markText = function (el) {
      if (el.closest('kbd') || (main && main.contains(el))) return null;
      const node = Array.from(el.childNodes).find(function (n) {
        return n.nodeType === 3 && MARKS.test(n.textContent);
      });
      return node ? node.textContent.trim() : null;
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
        if (!reduced && !printing && !svg) {
          const generated = pseudo ? /^"(.*)"$/s.exec(s.content) : null;
          const mark = pseudo ? (generated && MARKS.test(generated[1]) ? generated[1] : null)
            : markText(el);
          if (mark !== null) note('mark', el, pseudo, 'content', mark);
        }
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
          // Text selected is drawn on the action role at 22 % (`principles.md`, *One sweep*),
          // read on each element that shows text of its own; a page without the rule gets the
          // browser's own. Paper shows no selection.
          if (!printing && !pseudo && !svg && ownText(el)) {
            const value = win.getComputedStyle(el, '::selection').backgroundColor;
            const h = hex(value);
            if (!h || h.rgb !== ACTION[theme] || Math.abs(h.alpha - SELECTION_ALPHA) > 0.005) {
              note('selection', el, '::selection', 'background-color', value);
            }
          }
        }
        if (printing) continue;
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

  // Every style rule of the page's stylesheets that names `:target`, copied after itself with an
  // attribute in its place, and every element with an id given the attribute: the page as if
  // each of them were the one jumped to, which a stylesheet can draw a mark before (#383). The
  // copy has the same specificity and stands where the rule does, so the cascade is the one a
  // reader who jumps there gets. Returns a function that takes it all out again.
  function applyTarget(win, doc) {
    const added = [];
    const walk = function (holder) {
      let rules;
      try { rules = holder.cssRules; } catch (err) { return; }
      for (let i = rules.length - 1; i >= 0; i--) {
        const rule = rules[i];
        if (rule instanceof win.CSSImportRule) {
          if (rule.styleSheet) walk(rule.styleSheet);
        } else if (rule instanceof win.CSSStyleRule && TARGET.test(rule.selectorText)) {
          const copy = rule.cssText.replace(rule.selectorText,
            rule.selectorText.replace(TARGETS, '[' + TARGET_ATTR + ']'));
          holder.insertRule(copy, i + 1);
          added.push(holder.cssRules[i + 1]);
        } else if (rule.cssRules) {
          // A grouping rule, or a style rule with rules nested in it.
          walk(rule);
        }
      }
    };
    for (const sheet of Array.from(doc.styleSheets)) {
      if (!sheet.disabled) walk(sheet);
    }
    const marked = Array.from(doc.querySelectorAll('[id]'));
    for (const el of marked) el.setAttribute(TARGET_ATTR, '');
    return function () {
      for (const el of marked) el.removeAttribute(TARGET_ATTR);
      for (const rule of added) {
        const holder = rule.parentRule || rule.parentStyleSheet;
        const index = Array.from(holder.cssRules).indexOf(rule);
        if (index >= 0) holder.deleteRule(index);
      }
    };
  }

  // Whether a media query list applies on paper: a query for print or all, or for no type, whose
  // features hold (the frame's width stands in for the paper's); `not` turns it over.
  function onPaper(win, text) {
    return text.split(',').some(function (part) {
      let query = part.trim().toLowerCase().replace(/^only\s+/, '');
      const not = /^not\s+/.test(query);
      if (not) query = query.replace(/^not\s+/, '');
      let type = 'all';
      const typed = /^([a-z-]+)(?:\s+and\s+(.*))?$/.exec(query);
      if (typed) {
        type = typed[1];
        query = typed[2] || '';
      }
      const holds = (type === 'all' || type === 'print')
        && (query === '' || win.matchMedia(query).matches);
      return not ? !holds : holds;
    });
  }

  // The page as it prints (#383): each media list of its stylesheets, their `@media` rules and
  // imports that names `print` or `screen`, and that applies on paper but not on screen or the
  // other way round, set to `all` or `not all`, so the stylesheets' print rules apply (mdBook's
  // `print.css` among them) and their screen rules don't. A headless browser can't be told to
  // print from a page, so the page is read on screen with paper's rules. Returns a function
  // that puts every list back.
  function applyPrint(win, doc) {
    const changed = [];
    const flip = function (list) {
      const text = list.mediaText;
      if (!/\b(print|screen)\b/i.test(text)) return;
      const paper = onPaper(win, text);
      if (win.matchMedia(text).matches === paper) return;
      list.mediaText = paper ? 'all' : 'not all';
      changed.push([list, text]);
    };
    const walk = function (holder) {
      let rules;
      try { rules = holder.cssRules; } catch (err) { return; }
      for (const rule of Array.from(rules)) {
        if (rule.media && rule instanceof win.CSSMediaRule) flip(rule.media);
        if (rule instanceof win.CSSImportRule) {
          flip(rule.media);
          if (rule.styleSheet) walk(rule.styleSheet);
        } else if (rule.cssRules) {
          walk(rule);
        }
      }
    };
    for (const sheet of Array.from(doc.styleSheets)) {
      if (sheet.disabled) continue;
      flip(sheet.media);
      walk(sheet);
    }
    return function () {
      for (let i = changed.length - 1; i >= 0; i--) changed[i][0].mediaText = changed[i][1];
    };
  }

  // The colors an element and its `::before` and `::after` are drawn in, as one string each, for
  // comparing two readings of one page.
  const COLOR_PROPERTIES = ['color', 'background-color', 'border-top-color',
    'border-right-color', 'border-bottom-color', 'border-left-color', 'outline-color',
    'text-decoration-color', 'fill', 'stroke'];
  function colorsOf(win, els) {
    const out = [];
    for (const el of els) {
      for (const pseudo of [null, '::before', '::after']) {
        const s = win.getComputedStyle(el, pseudo);
        if (pseudo && (s.content === 'none' || s.content === 'normal')) {
          out.push(null);
          continue;
        }
        out.push(COLOR_PROPERTIES.map(function (p) { return s.getPropertyValue(p); }));
      }
    }
    return out;
  }

  // The page on paper (#383; `web.md`, *Print*), read with its print rules applied
  // ([`applyPrint`]) in the theme it is in, and on mdBook's pages that switch, in the other too:
  // - the light theme's values whatever the theme: every color one of the light theme's roles,
  //   with the reading's other checks of color, shape and contrast ([`styleDefaults`]), and on a
  //   page that switches, every element drawn in the same colors in both themes;
  // - navigation and the theme's controls hidden: no `nav`, button, text field, menu or search
  //   drawn, nor anything inside one;
  // - each link drawn out of the page (not to a place on it), in `main` or around it, as the
  //   title block's are, followed by its address, as a `::before` or `::after` that holds its
  //   `href`;
  // - table rows, notes and title blocks each kept on one sheet (`break-inside: avoid`);
  // - each title block drawn, with its version and its date of issue.
  async function probePrint(win, doc, found) {
    const note = function (el, pseudo, property, value, as) {
      found.print_count++;
      if (found.print.length < EXAMPLES) {
        found.print.push({
          selector: selector(el) + (pseudo || ''), property: property, value: value, theme: as,
        });
      }
    };
    const root = doc.documentElement;
    const themeClass = function () { return root.classList.contains('navy') ? 'navy' : 'light'; };
    const els = [root].concat(doc.body
      ? [doc.body].concat(Array.from(doc.body.querySelectorAll('*'))) : [])
      .filter(function (el) {
        return ['SCRIPT', 'STYLE', 'NOSCRIPT', 'TEMPLATE'].indexOf(el.tagName) < 0;
      });
    const read = function (as) {
      const sub = {};
      for (const kind of ['color', 'shape', 'contrast', 'motion', 'selection', 'mark']) {
        sub[kind] = [];
        sub[kind + '_count'] = 0;
      }
      styleDefaults(win, doc, 'light', false, sub, as, true);
      for (const kind of Object.keys(sub).filter(function (k) { return !/_count$/.test(k); })) {
        found.print_count += sub[kind + '_count'];
        for (const ex of sub[kind]) {
          if (found.print.length < EXAMPLES) found.print.push(ex);
        }
      }
      // Navigation is read by what it holds as well: mdBook's page arrows are links fixed to the
      // window's sides inside a `nav` with no height of its own.
      for (const el of doc.querySelectorAll(PRINT_HIDDEN)) {
        if (el.type === 'hidden') continue;
        const shown = drawn(win, el) ? el : Array.from(el.querySelectorAll('*')).find(function (c) {
          return drawn(win, c);
        });
        if (shown) note(shown, null, 'display', 'drawn on paper', as);
      }
      for (const a of doc.querySelectorAll('a[href]')) {
        const href = a.getAttribute('href');
        if (href === '' || href.charAt(0) === '#' || !drawn(win, a)) continue;
        const shows = ['::before', '::after'].some(function (pseudo) {
          const content = win.getComputedStyle(a, pseudo).content;
          return /^"/.test(content) && content.indexOf(href) >= 0;
        });
        if (!shows) note(a, '::after', 'content', 'no address (' + href + ')', as);
      }
      for (const el of doc.querySelectorAll(PRINT_WHOLE)) {
        const value = win.getComputedStyle(el).breakInside;
        if (drawn(win, el) && value !== 'avoid' && value !== 'avoid-page') {
          note(el, null, 'break-inside', value, as);
        }
      }
      for (const block of doc.querySelectorAll('.fs-titleblock')) {
        if (!drawn(win, block)) {
          note(block, null, 'display', 'not drawn on paper', as);
          continue;
        }
        for (const name of TITLE_BLOCK_PRINTED) {
          const field = Array.from(block.querySelectorAll('.k')).find(function (k) {
            return k.textContent.trim() === name;
          });
          const entry = field && field.nextElementSibling;
          if (!field || !drawn(win, field) || !entry || !drawn(win, entry)
            || entry.textContent.trim() === '') {
            note(block, null, name, 'not drawn on paper', as);
          }
        }
      }
      return colorsOf(win, els);
    };
    const undo = applyPrint(win, doc);
    try {
      await settle(win, doc);
      const first = themeClass();
      const colors = read('print, from ' + first);
      if (doc.getElementById('mdbook-tomorrow-night-css') && root.classList.contains('js')) {
        const other = first === 'light' ? 'navy' : 'light';
        switchTheme(doc, other);
        await settle(win, doc);
        const others = read('print, from ' + other);
        let i = 0;
        for (const el of els) {
          for (const pseudo of [null, '::before', '::after']) {
            const a = colors[i];
            const b = others[i];
            i++;
            if (a === null && b === null) continue;
            if (a === null || b === null) {
              note(el, pseudo, 'content', 'drawn from one theme only', 'print');
              continue;
            }
            COLOR_PROPERTIES.forEach(function (p, k) {
              if (a[k] !== b[k]) {
                note(el, pseudo, p, a[k] + ' from ' + first + ', ' + b[k] + ' from ' + other,
                  'print');
              }
            });
          }
        }
        switchTheme(doc, first);
        await settle(win, doc);
      }
    } finally {
      undo();
      await settle(win, doc);
    }
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

    // The focus ring, once for each kind of element: its tag, classes and type, and its
    // ancestors' tags and classes, four steps up, ids left out. print.html holds every chapter,
    // thousands of links that differ only in where they go, and each focus lays the page out.
    const kindOf = function (el) {
      const steps = [];
      for (let e = el; e && e.nodeType === 1 && steps.length < 4; e = e.parentElement) {
        steps.unshift(e.tagName + '.' + Array.from(e.classList).sort().join('.')
          + (e.getAttribute('type') ? '[' + e.getAttribute('type') + ']' : ''));
        if (e.tagName === 'BODY') break;
      }
      return steps.join('>');
    };
    const kinds = new Set();
    const before = doc.activeElement;
    for (const el of doc.querySelectorAll(
      'a[href], button, input, select, textarea, summary, [tabindex]')) {
      if (el.tabIndex < 0 || el.disabled) continue;
      const key = kindOf(el);
      if (kinds.has(key)) continue;
      el.focus({ preventScroll: true, focusVisible: true });
      if (doc.activeElement !== el) continue;
      kinds.add(key);
      if (!el.matches(':focus-visible')) {
        throw new Error('the browser did not draw ' + selector(el) + ' as focused by the keyboard');
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

    // One sweep in the page's own colors finds what forced colors and a sticky bar could
    // affect: boxes set apart from what is behind them only by a fill, inline icons, icons
    // painted through a mask, and bars fixed or stuck to the window's top. mdBook's script takes
    // its menu bar's `sticky` class off at load, below 1,080 px, and puts it back when the reader
    // scrolls up: the bar is read as it sticks, on a phone two rows tall.
    const memo = new Map();
    const layers = [];
    const icons = [];
    const masked = [];
    const bars = [];
    const TABLE = ['TABLE', 'THEAD', 'TBODY', 'TFOOT', 'TR', 'TH', 'TD', 'CAPTION'];
    const menuBar = doc.getElementById('mdbook-menu-bar');
    const unstuck = menuBar !== null && !menuBar.classList.contains('sticky');
    if (unstuck) menuBar.classList.add('sticky');
    for (const el of els) {
      if (el instanceof win.SVGElement) {
        if (el.getClientRects().length) icons.push(el);
        continue;
      }
      const s = win.getComputedStyle(el);
      if (s.display === 'none') continue;
      for (const pseudo of [null, '::before', '::after']) {
        const p = pseudo ? win.getComputedStyle(el, pseudo) : s;
        if (pseudo && (p.content === 'none' || p.content === 'normal')) continue;
        const mask = p.getPropertyValue('mask-image') || p.getPropertyValue('-webkit-mask-image');
        if (mask && mask !== 'none') masked.push([el, pseudo]);
      }
      if ((s.position === 'sticky' || s.position === 'fixed') && s.visibility !== 'hidden') {
        const r = boxOf(el);
        if (r && r.width >= view / 2 && r.height < tall / 2) {
          const top = parseFloat(s.top);
          // A sticky bar sticks `top` from the window's top edge; a fixed one is where it is.
          const reach = s.position === 'sticky' ? (Number.isFinite(top) ? top + r.height : null)
            : (r.top <= SLACK_PX ? r.bottom : null);
          if (reach !== null) bars.push([el, reach]);
        }
      }
      if (s.display === 'contents' || /^inline/.test(s.display) || TABLE.indexOf(el.tagName) >= 0) {
        continue;
      }
      const fill = channels(s.backgroundColor);
      if (!fill || fill.a === 0 || !el.parentElement || !boxOf(el)) continue;
      const behind = backdrop(win, doc, el.parentElement, memo).color;
      const own = over(fill, behind, 1);
      const same = Math.abs(own.r - behind.r) < 1 && Math.abs(own.g - behind.g) < 1
        && Math.abs(own.b - behind.b) < 1;
      if (!same) layers.push([el, s.backgroundColor]);
    }
    if (unstuck) menuBar.classList.remove('sticky');

    // The bars, against the padding that keeps what is scrolled to out from under them.
    const scroller = doc.scrollingElement || doc.documentElement;
    const padding = parseFloat(win.getComputedStyle(scroller).scrollPaddingTop) || 0;
    for (const [el, reach] of bars) {
      if (padding + SLACK_PX < reach) {
        note('sticky', el, null, 'scroll-padding-top', padding + 'px, under a bar reaching '
          + Math.round(reach * 10) / 10 + ' px');
      }
    }

    // Forced colors: the stylesheets' rules for them applied, each candidate read again.
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
      for (const el of icons) {
        const s = win.getComputedStyle(el);
        for (const property of ['fill', 'stroke']) {
          const value = s.getPropertyValue(property);
          if (value !== 'none' && !/^url\(/.test(value) && value !== s.color) {
            note('forced', el, null, property, value + ', not its text color ' + s.color);
          }
        }
      }
      for (const [el, pseudo] of masked) {
        if (win.getComputedStyle(el, pseudo).forcedColorAdjust !== 'none') {
          note('forced', el, pseudo, 'mask-image', 'an icon drawn through a mask, its colors forced');
        }
      }
    } finally {
      undo();
    }

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
      // The title is a link to the landing page: the frame stays on this page.
      const stay = function (event) { event.preventDefault(); };
      win.addEventListener('click', stay, true);
      try {
        title.click();
      } finally {
        win.removeEventListener('click', stay, true);
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
    // Read as if every element with an id were the one jumped to, for what a stylesheet draws
    // before a heading jumped to; the readings after it are of the page as it loads.
    const untarget = applyTarget(win, doc);
    styleDefaults(win, doc, first, false, found);
    untarget();
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
  // Each kind of note's signal word, as its head says it (`fusionspace.css`, `.fs-note`; the
  // strip sets it in capitals), and the fill of the kinds whose strip is filled.
  const NOTE_WORDS = {
    trust: 'How far to trust it', note: 'Note', caution: 'Caution', warning: 'Warning',
  };
  const NOTE_FILLS = { caution: '#F5AF20', warning: '#AC001E' };
  // Prose: the blocks whose lines are held to the measure.
  const PROSE = ['P', 'LI', 'DD', 'DT', 'BLOCKQUOTE', 'FIGCAPTION'];

  // Whether two lengths in pixels are the same, to sub-pixel rounding.
  function samePx(a, b) { return Math.abs(a - b) < 0.05; }

  // The page's frame (#384): its type, its measure and its gutters, read once at each width in
  // the theme the page loads in.
  function probeFrame(win, doc) {
    const found = {
      type: [], type_count: 0, measure: [], measure_count: 0, gutter: [], gutter_count: 0,
      squeezed: [], squeezed_count: 0, note: [], note_count: 0, icon: [], icon_count: 0,
      header: [], header_count: 0, title_block: [], title_block_count: 0,
      sheet: [], sheet_count: 0, theme_switch: [], theme_switch_count: 0,
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

      // Notes (#384; `web.md`, *Components*): no quote block in `main`, and each note the
      // system's: a head, then a body, nothing else; its kind's signal word in the head, in the
      // `label` token (Cascadia Mono 12/16, semibold, in capitals), drawn as a strip across the
      // top of the note, set apart from the message by a rule or a fill, inside the note's
      // border on every side. A caution's strip is filled in the caution fill and a warning's
      // in the danger fill, the warning bordered 2 px (`fusionspace.css`, `.fs-note`).
      for (const quote of main.querySelectorAll('blockquote')) {
        note('note', selector(quote), 'a quote block, not a note', snippet(quote.textContent));
      }
      for (const box of main.querySelectorAll('.fs-note')) {
        const sel = selector(box);
        const head = box.firstElementChild;
        const body = head && head.nextElementSibling;
        if (!head || !head.classList.contains('fs-note-head') || !body
          || !body.classList.contains('fs-note-body') || body.nextElementSibling) {
          note('note', sel, 'parts', 'not a head, then a body');
          continue;
        }
        const kind = box.getAttribute('data-kind');
        const word = head.textContent.replace(/\s+/g, ' ').trim();
        if (!Object.prototype.hasOwnProperty.call(NOTE_WORDS, kind) || NOTE_WORDS[kind] !== word) {
          note('note', sel, 'signal word', (kind || 'no kind') + ': ' + snippet(word));
        }
        const hs = win.getComputedStyle(head);
        const bs = win.getComputedStyle(box);
        const family = hs.fontFamily.split(',')[0].trim().replace(/^["']|["']$/g, '');
        if (family !== 'Cascadia Mono' || !samePx(parseFloat(hs.fontSize), 12)
          || !samePx(parseFloat(hs.lineHeight), 16) || Number(hs.fontWeight) < 600
          || hs.textTransform !== 'uppercase') {
          note('note', sel + '>.fs-note-head', 'signal word type', hs.fontWeight + ' '
            + hs.fontSize + '/' + hs.lineHeight + ' ' + family + ', ' + hs.textTransform);
        }
        const fill = hex(hs.backgroundColor);
        const filled = fill !== null && fill.alpha > 0;
        const ruled = parseFloat(hs.borderBottomWidth) >= 1 && hs.borderBottomStyle !== 'none'
          && hs.borderBottomStyle !== 'hidden';
        if (!filled && !ruled) note('note', sel + '>.fs-note-head', 'strip', 'no rule or fill');
        const want = NOTE_FILLS[kind];
        if (want && (!filled || fill.rgb !== want)) {
          note('note', sel + '>.fs-note-head', 'fill', (fill ? fill.rgb : 'none') + ' for '
            + kind);
        }
        const least = kind === 'warning' ? 2 : 1;
        for (const side of ['Top', 'Right', 'Bottom', 'Left']) {
          if (parseFloat(bs['border' + side + 'Width']) < least - 0.05
            || bs['border' + side + 'Style'] !== 'solid') {
            note('note', sel, 'border-' + side.toLowerCase(), bs['border' + side + 'Width']
              + ' ' + bs['border' + side + 'Style']);
          }
        }
        // The strip spans the note's top, inside its border, above the message.
        const b = box.getBoundingClientRect();
        const h = head.getBoundingClientRect();
        const m = body.getBoundingClientRect();
        const inner = {
          left: b.left + parseFloat(bs.borderLeftWidth),
          right: b.right - parseFloat(bs.borderRightWidth),
          top: b.top + parseFloat(bs.borderTopWidth),
        };
        if (h.height <= HIDDEN_BOX_PX || Math.abs(h.top - inner.top) > SLACK_PX
          || Math.abs(h.left - inner.left) > SLACK_PX
          || Math.abs(h.right - inner.right) > SLACK_PX || m.top < h.bottom - SLACK_PX) {
          note('note', sel + '>.fs-note-head', 'strip', 'not across the top, above the message');
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

    // Icons (#383; `foundations.md`, *Icons*): each one in the page's chrome, the spans mdBook
    // draws its buttons, arrows and spinner in, is drawn square at one of the system's sizes.
    // One not drawn at this width, or until asked for (`display: none` on it or around it), has
    // no box and isn't read; one drawn at 0 px is read, and fails. An icon turning (a fold
    // toggle's chevron, as its heading opens when the page loads) is read where its turn ends,
    // as a reader sees it a moment later: a square turned partway has a wider box.
    for (const a of doc.getAnimations()) {
      if (Number.isFinite(a.effect.getComputedTiming().endTime)) a.finish();
    }
    for (const svg of Array.from(doc.querySelectorAll('.fa-svg svg'))) {
      if (svg.getClientRects().length === 0) continue;
      const r = svg.getBoundingClientRect();
      const ok = samePx(r.width, r.height)
        && ICON_SIZES.some(function (px) { return samePx(r.width, px); });
      if (!ok) {
        note('icon', selector(svg), 'size', Math.round(r.width * 10) / 10 + ' by '
          + Math.round(r.height * 10) / 10 + ' px');
      }
    }
    probeHeader(win, doc, note);
    probeNoScript(win, doc, note);
    probeTitleBlock(win, doc, note);
    probeSheets(win, doc, note);
    probeSwitch(win, doc, note);
    return found;
  }

  // Whether `el` is drawn: it has a box, more than a screen reader's 2 px, not hidden.
  function drawn(win, el) {
    if (el.getClientRects().length === 0) return false;
    const r = el.getBoundingClientRect();
    return r.width > HIDDEN_BOX_PX && r.height > HIDDEN_BOX_PX
      && win.getComputedStyle(el).visibility !== 'hidden';
  }

  // The header (#384; `web.md`, *Page anatomy*). Each lockup (`.fs-lockup`, which `xtask site`
  // draws in the menu bar; the file check makes sure it is there) must be 24 px tall, every path
  // filled in the ink role of the page's theme (the body's text color, Void or Paper), wholly
  // drawn inside every box around it that clips and inside the window, and kept a quarter of
  // its height from the window's sides and from every icon and text drawn in its header. From
  // 720 px no menu button (`#mdbook-sidebar-toggle`) is drawn, and under it one is. The sidebar
  // (`#mdbook-sidebar`) is drawn from 720 px, and wherever it is drawn a screen reader and the
  // keyboard reach it (not `aria-hidden`, each link in the tab order); wherever it is put away
  // (no box, hidden, or wholly off the left edge) neither does.
  function probeHeader(win, doc, note) {
    const width = win.innerWidth;
    const right = doc.documentElement.clientWidth;
    const ink = win.getComputedStyle(doc.body).color;
    const round = function (v) { return Math.round(v * 10) / 10; };
    for (const svg of Array.from(doc.querySelectorAll('.fs-lockup'))) {
      const sel = selector(svg);
      if (!drawn(win, svg)) {
        note('header', sel, 'display', 'not drawn');
        continue;
      }
      const r = svg.getBoundingClientRect();
      if (!samePx(r.height, LOCKUP_PX)) note('header', sel, 'height', round(r.height) + ' px');
      for (const path of Array.from(svg.querySelectorAll('path'))) {
        const fill = win.getComputedStyle(path).fill;
        if (fill !== ink || INKS.indexOf(fill) < 0) {
          note('header', sel, 'fill', fill + ', where the ink is ' + ink);
          break;
        }
      }
      let clip = { left: 0, top: -Infinity, right: right, bottom: Infinity };
      for (let a = svg.parentElement; a && a !== doc.documentElement; a = a.parentElement) {
        const s = win.getComputedStyle(a);
        if (s.overflowX === 'visible' && s.overflowY === 'visible') continue;
        const b = a.getBoundingClientRect();
        clip = {
          left: Math.max(clip.left, s.overflowX === 'visible' ? -Infinity : b.left),
          right: Math.min(clip.right, s.overflowX === 'visible' ? Infinity : b.right),
          top: Math.max(clip.top, s.overflowY === 'visible' ? -Infinity : b.top),
          bottom: Math.min(clip.bottom, s.overflowY === 'visible' ? Infinity : b.bottom),
        };
      }
      if (r.left < clip.left - SLACK_PX || r.right > clip.right + SLACK_PX
        || r.top < clip.top - SLACK_PX || r.bottom > clip.bottom + SLACK_PX) {
        note('header', sel, 'cut', 'drawn ' + round(r.left) + ' to ' + round(r.right)
          + ' px, shown ' + round(clip.left) + ' to ' + round(clip.right) + ' px');
      }
      const space = r.height * CLEAR_SPACE;
      if (r.left - space < -SLACK_PX || r.right + space > right + SLACK_PX) {
        note('header', sel, 'clear space', round(r.left) + ' px from the left and '
          + round(right - r.right) + ' px from the right, against ' + round(space) + ' px');
      }
      const header = svg.closest('header') || doc.body;
      for (const el of Array.from(header.querySelectorAll('*'))) {
        if (el === svg || svg.contains(el) || el.contains(svg)) continue;
        const own = el instanceof win.SVGSVGElement || Array.from(el.childNodes).some(function (n) {
          return n.nodeType === 3 && /\S/.test(n.data);
        });
        if (!own || !drawn(win, el)) continue;
        const b = el.getBoundingClientRect();
        if (b.left < r.right + space - SLACK_PX && b.right > r.left - space + SLACK_PX
          && b.top < r.bottom + space - SLACK_PX && b.bottom > r.top - space + SLACK_PX) {
          note('header', sel, 'clear space', selector(el) + ' within ' + round(space) + ' px');
        }
      }
    }
    const toggle = doc.getElementById('mdbook-sidebar-toggle');
    const sidebarEl = doc.getElementById('mdbook-sidebar');
    // A page with the site's header has the sidebar and its menu button too: neither rule
    // below may pass for want of them.
    if (doc.querySelector('header#mdbook-menu-bar') && (!toggle || !sidebarEl)) {
      note('header', 'header#mdbook-menu-bar', 'sidebar', (toggle ? '' : 'no menu button ')
        + (sidebarEl ? '' : 'no sidebar'));
    }
    if (toggle) {
      const shown = drawn(win, toggle);
      if (width >= SIDEBAR_OPEN_FROM_PX && shown) {
        note('header', selector(toggle), 'display', 'a menu button from '
          + SIDEBAR_OPEN_FROM_PX + ' px');
      } else if (width < SIDEBAR_OPEN_FROM_PX && !shown) {
        note('header', selector(toggle), 'display', 'no menu button under '
          + SIDEBAR_OPEN_FROM_PX + ' px');
      }
    }
    const sidebar = doc.getElementById('mdbook-sidebar');
    if (sidebar) {
      const sel = selector(sidebar);
      const shown = drawn(win, sidebar) && sidebar.getBoundingClientRect().right > SLACK_PX;
      if (width >= SIDEBAR_OPEN_FROM_PX && !shown) {
        note('header', sel, 'display', 'put away from ' + SIDEBAR_OPEN_FROM_PX + ' px');
      }
      const hidden = sidebar.getAttribute('aria-hidden') === 'true';
      const links = Array.from(sidebar.querySelectorAll('a[href]'));
      const reached = links.filter(function (a) { return a.tabIndex >= 0; }).length;
      if (shown && (hidden || reached < links.length)) {
        note('header', sel, 'aria-hidden', (hidden ? 'hidden from screen readers' : 'read')
          + ' and ' + reached + ' of ' + links.length + ' links in the tab order, drawn');
      } else if (!shown && (!hidden || reached > 0)) {
        note('header', sel, 'aria-hidden', (hidden ? 'hidden from screen readers' : 'read')
          + ' and ' + reached + ' of ' + links.length + ' links in the tab order, put away');
      }
    }
  }

  // Whether the sidebar `el` is drawn on the page: a box, not hidden, not wholly off the left.
  function sidebarShown(win, el) {
    return drawn(win, el) && el.getBoundingClientRect().right > SLACK_PX;
  }

  // The sidebar with scripts off (#384): read as the page's markup leaves it before any script
  // runs (the root without its `js` class, mdBook's sidebar box unticked, the sidebar without
  // the `display` mdBook's script gives it), with transitions held, then put back. From 720 px
  // the sidebar is drawn and no menu button is; under 720 px the menu button is, as only it can
  // open the sidebar without a script. Read only on a page whose script marks the root `js`.
  function probeNoScript(win, doc, note) {
    const root = doc.documentElement;
    const box = doc.getElementById('mdbook-sidebar-toggle-anchor');
    const sidebar = doc.getElementById('mdbook-sidebar');
    const toggle = doc.getElementById('mdbook-sidebar-toggle');
    if (!box || !sidebar || !toggle || !root.classList.contains('js')) return;
    const width = win.innerWidth;
    const held = doc.createElement('style');
    held.textContent = '*, *::before, *::after { transition: none !important; }';
    const was = { checked: box.checked, display: sidebar.style.display };
    doc.head.appendChild(held);
    try {
      root.classList.remove('js');
      box.checked = false;
      sidebar.style.display = '';
      const shown = sidebarShown(win, sidebar);
      const button = drawn(win, toggle);
      if (width >= SIDEBAR_OPEN_FROM_PX && (!shown || button)) {
        note('header', selector(sidebar), 'scripts off', (shown ? 'drawn' : 'put away')
          + (button ? ', with a menu button' : '') + ' from ' + SIDEBAR_OPEN_FROM_PX + ' px');
      } else if (width < SIDEBAR_OPEN_FROM_PX && !button) {
        note('header', selector(toggle), 'scripts off', 'no menu button under '
          + SIDEBAR_OPEN_FROM_PX + ' px');
      }
    } finally {
      root.classList.add('js');
      box.checked = was.checked;
      sidebar.style.display = was.display;
      held.remove();
    }
  }

  // The sidebar after a window under 720 px widens past it (#384): the frame is widened to
  // `SIDEBAR_WIDENED_PX` and, once the sidebar's slide has run, the sidebar must be drawn and
  // reached, and no menu button drawn. Returns what is wrong, as the header's examples.
  async function probeWidened(frame, win, doc) {
    const found = [];
    const sidebar = doc.getElementById('mdbook-sidebar');
    const toggle = doc.getElementById('mdbook-sidebar-toggle');
    if (!sidebar || !toggle || win.innerWidth >= SIDEBAR_OPEN_FROM_PX) return found;
    const from = win.innerWidth;
    frame.style.width = SIDEBAR_WIDENED_PX + 'px';
    await new Promise(function (resolve) { setTimeout(resolve, SIDEBAR_SLIDE_WAIT_MS); });
    await nextFrames(win);
    const what = 'widened from ' + from + ' to ' + SIDEBAR_WIDENED_PX + ' px';
    const hidden = sidebar.getAttribute('aria-hidden') === 'true';
    const links = Array.from(sidebar.querySelectorAll('a[href]'));
    const reached = links.filter(function (a) { return a.tabIndex >= 0; }).length;
    if (!sidebarShown(win, sidebar) || hidden || reached < links.length) {
      found.push({ selector: selector(sidebar), property: 'widened', theme: from + ' px',
        value: (sidebarShown(win, sidebar) ? 'drawn' : 'put away') + (hidden ? ', hidden' : '')
          + ', ' + reached + ' of ' + links.length + ' links in the tab order, ' + what });
    }
    if (drawn(win, toggle)) {
      found.push({ selector: selector(toggle), property: 'widened', theme: from + ' px',
        value: 'a menu button, ' + what });
    }
    return found;
  }

  // The title block (#384; `web.md`, *Page anatomy*; `fusionspace.css`, `.fs-titleblock`). Each
  // one (`footer.fs-titleblock`; the file check makes sure every page ends with one) must be
  // drawn inside a 2 px solid border in the ink role on every side, its fields' names and entries
  // in Cascadia Mono, and nothing may be drawn below its top: no text, picture or field outside
  // it, other than in a box fixed to the window (the sidebar, the wide page arrows).
  function probeTitleBlock(win, doc, note) {
    const ink = win.getComputedStyle(doc.body).color;
    for (const block of Array.from(doc.querySelectorAll('footer.fs-titleblock'))) {
      const sel = selector(block);
      if (!drawn(win, block)) {
        note('title_block', sel, 'display', 'not drawn');
        continue;
      }
      const s = win.getComputedStyle(block);
      for (const side of ['Top', 'Right', 'Bottom', 'Left']) {
        const w = parseFloat(s['border' + side + 'Width']);
        const style = s['border' + side + 'Style'];
        const color = s['border' + side + 'Color'];
        if (!samePx(w, TITLE_BLOCK_BORDER_PX) || style !== 'solid' || color !== ink) {
          note('title_block', sel, 'border-' + side.toLowerCase(), w + 'px ' + style + ' '
            + color + ', where the ink is ' + ink);
        }
      }
      for (const cell of Array.from(block.querySelectorAll('.k, .v'))) {
        const family = win.getComputedStyle(cell).fontFamily.split(',')[0].trim()
          .replace(/^["']|["']$/g, '');
        if (family !== 'Cascadia Mono') {
          note('title_block', selector(cell), 'font-family', family);
          break;
        }
      }
      const top = block.getBoundingClientRect().top;
      for (const el of Array.from(doc.body.querySelectorAll('*'))) {
        // The box first: on a long page nearly everything ends above the block.
        const b = el.getBoundingClientRect();
        if (b.bottom <= top + SLACK_PX || block.contains(el) || el.contains(block)) continue;
        const tag = el.tagName.toLowerCase();
        if (tag === 'script' || tag === 'style' || tag === 'template' || tag === 'noscript') continue;
        const own = el instanceof win.SVGSVGElement || tag === 'img' || tag === 'input'
          || Array.from(el.childNodes).some(function (n) {
            return n.nodeType === 3 && /\S/.test(n.data);
          });
        if (!own || !drawn(win, el)) continue;
        let fixed = false;
        for (let a = el; a && a !== doc.body; a = a.parentElement) {
          if (win.getComputedStyle(a).position === 'fixed') { fixed = true; break; }
        }
        if (fixed) continue;
        note('title_block', selector(el), 'position', 'drawn to ' + Math.round(b.bottom)
          + ' px, below the title block\'s top at ' + Math.round(top) + ' px');
        break;
      }
    }
  }

  // The first family of `el`'s font, unquoted.
  function familyOf(win, el) {
    return win.getComputedStyle(el).fontFamily.split(',')[0].trim().replace(/^["']|["']$/g, '');
  }

  // The intro and the sheets (#384; `web.md`, *Page anatomy*; `fusionspace.css`, `.fs-tag` and
  // `.fs-sheet`). Each designation tag (`.fs-intro > .fs-tag`; the file check makes sure every
  // title has one) must be drawn above its title, in Cascadia Mono 12 px, inside a 1 px solid
  // border, one corner chamfered (a `clip-path` polygon). Each `h2` in `main` must be a sheet's
  // name, in its rail (`section.fs-sheet > .fs-sheet-rail > h2`); each sheet must be drawn under
  // a 2 px solid rule in the ink role, with its number in Cascadia Mono 12 px drawn above its
  // name and reading `SHEET n / N`, counted from each intro (a chapter, on the print page).
  function probeSheets(win, doc, note) {
    const ink = win.getComputedStyle(doc.body).color;
    for (const tag of Array.from(doc.querySelectorAll('.fs-intro > .fs-tag'))) {
      const sel = selector(tag);
      if (!drawn(win, tag)) {
        note('sheet', sel, 'display', 'a designation tag not drawn');
        continue;
      }
      const s = win.getComputedStyle(tag);
      if (familyOf(win, tag) !== 'Cascadia Mono' || !samePx(parseFloat(s.fontSize), TAG_PX)) {
        note('sheet', sel, 'font', s.fontSize + ' ' + familyOf(win, tag));
      }
      if (!/^polygon\(/.test(s.clipPath)) {
        note('sheet', sel, 'clip-path', s.clipPath + ': a tag with no chamfered corner');
      }
      for (const side of ['Top', 'Right', 'Bottom', 'Left']) {
        const w = parseFloat(s['border' + side + 'Width']);
        if (!samePx(w, TAG_BORDER_PX) || s['border' + side + 'Style'] !== 'solid') {
          note('sheet', sel, 'border-' + side.toLowerCase(), w + 'px '
            + s['border' + side + 'Style']);
          break;
        }
      }
      const title = tag.parentElement.querySelector(':scope > h1');
      if (!title || tag.getBoundingClientRect().bottom > title.getBoundingClientRect().top
        + SLACK_PX) {
        note('sheet', sel, 'position', 'a designation tag not above its title');
      }
    }
    const main = doc.querySelector('main');
    if (!main) return;
    for (const h2 of Array.from(main.querySelectorAll('h2'))) {
      if (!h2.parentElement || !h2.parentElement.matches('section.fs-sheet > .fs-sheet-rail')) {
        note('sheet', selector(h2), 'parent', 'a section\'s name outside a sheet\'s rail');
      }
    }
    // The sheets, in runs from each intro.
    const runs = [];
    for (const el of Array.from(main.querySelectorAll('.fs-intro, section.fs-sheet'))) {
      if (el.matches('.fs-intro') || runs.length === 0) runs.push([]);
      if (el.matches('section.fs-sheet')) runs[runs.length - 1].push(el);
    }
    for (const run of runs) {
      run.forEach(function (sheet, i) {
        const sel = selector(sheet);
        const s = win.getComputedStyle(sheet);
        const w = parseFloat(s.borderTopWidth);
        if (!samePx(w, SHEET_RULE_PX) || s.borderTopStyle !== 'solid' || s.borderTopColor !== ink) {
          note('sheet', sel, 'border-top', w + 'px ' + s.borderTopStyle + ' ' + s.borderTopColor
            + ', where the ink is ' + ink);
        }
        const number = sheet.querySelector(':scope > .fs-sheet-rail > .fs-sheet-no');
        const name = sheet.querySelector(':scope > .fs-sheet-rail > h2');
        if (!number || !drawn(win, number)) {
          note('sheet', sel, 'display', 'a sheet without its number drawn');
          return;
        }
        const words = number.textContent.trim();
        const expected = 'SHEET ' + (i + 1) + ' / ' + run.length;
        if (words !== expected) {
          note('sheet', selector(number), 'text', '"' + words + '", not "' + expected + '"');
        }
        const n = win.getComputedStyle(number);
        if (familyOf(win, number) !== 'Cascadia Mono'
          || !samePx(parseFloat(n.fontSize), SHEET_NUMBER_PX)) {
          note('sheet', selector(number), 'font', n.fontSize + ' ' + familyOf(win, number));
        }
        if (!name || number.getBoundingClientRect().bottom > name.getBoundingClientRect().top
          + SLACK_PX) {
          note('sheet', selector(number), 'position', 'a sheet\'s number not above its name');
        }
      });
    }
  }

  // The theme switch (#384, #429; `web.md`, *Page anatomy*, *Components* and *Theme*;
  // `fusionspace.css`, `.fs-seg`). Where a page has one (`#mdbook-theme-list.fs-seg`; the file
  // check makes sure every page does) and scripts run, it must stand in the `header` from
  // SWITCH_HEADER_PX and in the sidebar under it; hold one button for each of Auto, Light and
  // Dark, in that order, at least 44 px tall in Cascadia Mono 14 px where drawn; have exactly
  // one pressed (`aria-pressed`), Auto, as a first-time reader remembers no theme, filled in the
  // ink role with the others unfilled; and mdBook's theme button must not be drawn. With scripts
  // off it must not be drawn: nothing would work it.
  function probeSwitch(win, doc, note) {
    const group = doc.getElementById('mdbook-theme-list');
    if (!group || !group.classList.contains('fs-seg')) return;
    const sel = selector(group);
    const root = doc.documentElement;
    if (!root.classList.contains('js')) {
      if (drawn(win, group)) note('theme_switch', sel, 'scripts off', 'drawn, with nothing to work it');
      return;
    }
    // With scripts off, read as `probeNoScript` reads the sidebar: the root without its `js`
    // class, transitions held, then put back.
    const held = doc.createElement('style');
    held.textContent = '*, *::before, *::after { transition: none !important; }';
    doc.head.appendChild(held);
    try {
      root.classList.remove('js');
      if (drawn(win, group)) note('theme_switch', sel, 'scripts off', 'drawn, with nothing to work it');
    } finally {
      root.classList.add('js');
      held.remove();
    }
    const width = win.innerWidth;
    const where = group.closest('header') ? 'the header'
      : group.closest('#mdbook-sidebar') ? 'the sidebar' : 'neither the header nor the sidebar';
    const wanted = width >= SWITCH_HEADER_PX ? 'the header' : 'the sidebar';
    if (where !== wanted) {
      note('theme_switch', sel, 'position', 'in ' + where + ' at ' + width + ' px, not in '
        + wanted);
    }
    const toggle = doc.getElementById('mdbook-theme-toggle');
    if (toggle && drawn(win, toggle)) {
      note('theme_switch', selector(toggle), 'display', 'mdBook\'s theme menu button drawn');
    }
    const buttons = Array.from(group.querySelectorAll('button'));
    const names = buttons.map(function (b) { return b.textContent.trim(); });
    if (names.join('|') !== SWITCH_CHOICES.join('|')) {
      note('theme_switch', sel, 'buttons', names.join(', ') + ', not '
        + SWITCH_CHOICES.join(', '));
    }
    const pressed = buttons.filter(function (b) { return b.getAttribute('aria-pressed') === 'true'; });
    if (pressed.length !== 1 || pressed[0].textContent.trim() !== SWITCH_CHOICES[0]) {
      note('theme_switch', sel, 'aria-pressed', 'pressed: ' + (pressed.map(function (b) {
        return b.textContent.trim();
      }).join(', ') || 'none') + ', not ' + SWITCH_CHOICES[0]);
    }
    const retired = SWITCH_RETIRED.filter(function (t) { return root.classList.contains(t); });
    if (retired.length) {
      note('theme_switch', 'html', 'class', 'in mdBook\'s ' + retired.join(', ')
        + ', which the switch doesn\'t offer');
    }
    // Under 720 px the sidebar is put away until the menu button opens it: read opened, as
    // mdBook's script opens it (its box ticked, the root marked `sidebar-visible`, the sidebar
    // without the `display` that put it away), with transitions held, then put back, the styles
    // worked out again before the transitions are let go so that none runs.
    const box = doc.getElementById('mdbook-sidebar-toggle-anchor');
    const sidebar = doc.getElementById('mdbook-sidebar');
    const open = where === 'the sidebar' && width < SIDEBAR_OPEN_FROM_PX
      && !root.classList.contains('sidebar-visible');
    const was = { checked: box ? box.checked : false, display: sidebar ? sidebar.style.display : '' };
    if (open) {
      doc.head.appendChild(held);
      root.classList.add('sidebar-visible');
      if (box) box.checked = true;
      if (sidebar) sidebar.style.display = '';
    }
    try {
      probeSwitchDrawn(win, doc, group, buttons, note);
    } finally {
      if (open) {
        root.classList.remove('sidebar-visible');
        if (box) box.checked = was.checked;
        if (sidebar) sidebar.style.display = was.display;
        void root.getBoundingClientRect();
        held.remove();
      }
    }
  }

  // The switch's buttons as drawn: 44 px tall in Cascadia Mono 14, the pressed one, and only
  // it, filled in ink.
  function probeSwitchDrawn(win, doc, group, buttons, note) {
    if (!drawn(win, group)) {
      note('theme_switch', selector(group), 'display', 'not drawn');
      return;
    }
    const ink = win.getComputedStyle(doc.body).color;
    for (const button of buttons) {
      const s = win.getComputedStyle(button);
      const h = button.getBoundingClientRect().height;
      if (h < SWITCH_BUTTON_PX - SLACK_PX) {
        note('theme_switch', selector(button), 'height', Math.round(h * 10) / 10 + ' px');
      }
      if (familyOf(win, button) !== 'Cascadia Mono'
        || !samePx(parseFloat(s.fontSize), SWITCH_TYPE_PX)) {
        note('theme_switch', selector(button), 'font', s.fontSize + ' ' + familyOf(win, button));
      }
      const on = button.getAttribute('aria-pressed') === 'true';
      const filled = s.backgroundColor === ink;
      if (on !== filled) {
        note('theme_switch', selector(button), 'background-color', s.backgroundColor + ' '
          + (on ? 'pressed' : 'not pressed') + ', where the ink is ' + ink);
      }
    }
  }

  // The theme switch pressed: Dark must put the page in mdBook's dark theme (Navy) and press
  // Dark, and Auto must put it back in the theme the reader's system asks for and press Auto.
  // Pressing works the same at any width, and restyling a long page twice is slow, so it is
  // read once a run, at the width that reads print, with transitions held. mdBook remembers a
  // choice in `localStorage`, which the other frames loading in parallel share, and reads it back
  // to mark the chosen button; while the switch is pressed, this frame's storage is a store of
  // its own in memory, so nothing is left for them.
  async function probeSwitchPress(win, doc, result) {
    const group = doc.getElementById('mdbook-theme-list');
    if (!group || !group.classList.contains('fs-seg')
      || !doc.documentElement.classList.contains('js')) return;
    const note = function (value) {
      result.theme_switch_count++;
      if (result.theme_switch.length < EXAMPLES) {
        result.theme_switch.push({ selector: selector(group), property: 'pressed', value: value,
          theme: win.innerWidth + ' px' });
      }
    };
    const button = function (name) {
      return Array.from(group.querySelectorAll('button')).find(function (b) {
        return b.textContent.trim() === name;
      });
    };
    const dark = button('Dark');
    const auto = button('Auto');
    if (!dark || !auto) return;
    const root = doc.documentElement;
    const held = doc.createElement('style');
    held.textContent = '*, *::before, *::after { transition: none !important; }';
    doc.head.appendChild(held);
    const store = win.Storage.prototype;
    const saved = { get: store.getItem, set: store.setItem, remove: store.removeItem };
    const memory = new Map();
    store.getItem = function (key) {
      return memory.has(String(key)) ? memory.get(String(key)) : null;
    };
    store.setItem = function (key, value) { memory.set(String(key), String(value)); };
    store.removeItem = function (key) { memory.delete(String(key)); };
    try {
      dark.click();
      await nextFrames(win);
      if (!root.classList.contains('navy') || root.classList.contains('light')
        || dark.getAttribute('aria-pressed') !== 'true') {
        note('Dark pressed leaves the page in `' + root.className + '`, Dark pressed: '
          + dark.getAttribute('aria-pressed'));
      }
      auto.click();
      await nextFrames(win);
      const asked = win.matchMedia('(prefers-color-scheme: dark)').matches ? 'navy' : 'light';
      const other = asked === 'navy' ? 'light' : 'navy';
      if (!root.classList.contains(asked) || root.classList.contains(other)
        || auto.getAttribute('aria-pressed') !== 'true') {
        note('Auto pressed leaves the page in `' + root.className + '`, not `' + asked
          + '`, Auto pressed: ' + auto.getAttribute('aria-pressed'));
      }
    } finally {
      store.getItem = saved.get;
      store.setItem = saved.set;
      store.removeItem = saved.remove;
      held.remove();
    }
  }

  // A theme saved from mdBook's six-theme menu before the switch: loaded with Coal saved, the
  // page must forget it, stand in the theme the reader's system asks for with no Coal class,
  // and press Auto. The page is loaded again in a frame of its own at this width, from its text
  // with a `base` keeping its addresses and, before any of its scripts, a store in memory holding
  // Coal, so the frames loading in parallel see nothing. Read with the press, once a run.
  async function probeSwitchRetired(win, doc, result) {
    const group = doc.getElementById('mdbook-theme-list');
    if (!group || !group.classList.contains('fs-seg')
      || !doc.documentElement.classList.contains('js')) return;
    const note = function (value) {
      result.theme_switch_count++;
      if (result.theme_switch.length < EXAMPLES) {
        result.theme_switch.push({ selector: 'html', property: 'saved theme', value: value,
          theme: win.innerWidth + ' px' });
      }
    };
    const address = win.location.href.split('#')[0];
    const text = await (await fetch(address)).text();
    const store = '<base href="' + address + '"><script>(function () { const m = new Map(['
      + '["mdbook-theme", "coal"]]); const s = Storage.prototype; s.getItem = function (k) { '
      + 'k = String(k); return m.has(k) ? m.get(k) : null; }; s.setItem = function (k, v) { '
      + 'm.set(String(k), String(v)); }; s.removeItem = function (k) { m.delete(String(k)); }; '
      + 'window.hprCheckStore = m; })();</script>';
    const frame = document.createElement('iframe');
    frame.style.cssText = 'position:absolute;left:0;top:0;border:0;margin:0;padding:0;'
      + 'width:' + win.innerWidth + 'px;height:' + win.innerHeight + 'px';
    let timer = null;
    const loaded = new Promise(function (resolve) {
      frame.addEventListener('load', function () { resolve(true); }, { once: true });
      timer = setTimeout(function () { resolve(false); }, LOAD_TIMEOUT_MS);
    });
    frame.srcdoc = text.replace(/^(\s*<!doctype[^>]*>)?/i, function (doctype) {
      return doctype + store;
    });
    document.body.appendChild(frame);
    try {
      if (!(await loaded)) {
        note('did not load with Coal saved in ' + LOAD_TIMEOUT_MS / 1000 + ' s');
        return;
      }
      const w = frame.contentWindow;
      const d = frame.contentDocument;
      await nextFrames(w);
      const root = d.documentElement;
      const asked = w.matchMedia('(prefers-color-scheme: dark)').matches ? 'navy' : 'light';
      const list = d.getElementById('mdbook-theme-list');
      const auto = list && Array.from(list.querySelectorAll('button')).find(function (b) {
        return b.textContent.trim() === SWITCH_CHOICES[0];
      });
      const kept = w.hprCheckStore ? w.hprCheckStore.get('mdbook-theme') : 'unread';
      if (root.classList.contains('coal') || !root.classList.contains(asked) || !auto
        || auto.getAttribute('aria-pressed') !== 'true' || kept !== undefined) {
        note('with Coal saved, the page in `' + root.className + '` where the system asks for `'
          + asked + '`, Auto pressed: ' + (auto ? auto.getAttribute('aria-pressed') : 'no Auto')
          + ', still saved: ' + (kept === undefined ? 'nothing' : kept));
      }
    } finally {
      clearTimeout(timer);
      frame.remove();
    }
  }

  // `printed`: whether this width reads the page on paper too.
  function check(page, width, height, printed) {
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
      let timer = setTimeout(function () {
        finish({ width: width, error: 'did not load in ' + LOAD_TIMEOUT_MS / 1000 + ' s' });
      }, LOAD_TIMEOUT_MS);
      frame.addEventListener('load', async function () {
        clearTimeout(timer);
        timer = setTimeout(function () {
          finish({ width: width, error: 'was not read in ' + MEASURE_TIMEOUT_MS / 1000 + ' s' });
        }, MEASURE_TIMEOUT_MS);
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
            forced_count: 0, sticky: [], sticky_count: 0, selection: [], selection_count: 0,
            mark: [], mark_count: 0, print: [], print_count: 0,
          };
          // Before the defaults' readings, which leave the page in its other theme.
          probeAccess(win, doc, found);
          await probeDefaults(win, doc, found);
          if (printed) await probePrint(win, doc, found);
          Object.assign(result, found);
          if (printed) await probeSwitchPress(win, doc, result);
          if (printed) await probeSwitchRetired(win, doc, result);
          result.printed = printed;
          // Last, as it changes the frame's width.
          for (const widened of await probeWidened(frame, win, doc)) {
            result.header_count++;
            if (result.header.length < EXAMPLES) result.header.push(widened);
          }
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
          results[i] = await check(page, plan.widths[i], plan.height,
            plan.widths[i] === plan.print_width);
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
