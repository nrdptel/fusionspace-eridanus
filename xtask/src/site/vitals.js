// SPDX-License-Identifier: MIT OR Apache-2.0
// The vitals measurement's half in the page (`vitals.rs`, #443). The browser runs this before
// any of the page's own scripts (DevTools' `Page.addScriptToEvaluateOnNewDocument`), so its
// observers, with `buffered: true`, see every entry from the start of the page's load:
//
// - Largest Contentful Paint: the last `largest-contentful-paint` entry. The browser stops
//   reporting them at the first input, and the measurement reads LCP before it taps.
// - Layout shifts: each `layout-shift` entry, with its `hadRecentInput` (a shift within 500 ms
//   of a tap or a key is the reader's doing, not the page's). Headless Chrome also flags shifts
//   in the half second after a page loads, before any input, so `vitals.rs` counts a flagged
//   shift that came before the measurement's first input, and groups them into session windows.
// - Interactions: each `event` entry with an `interactionId`, at the lowest threshold Event
//   Timing allows (16 ms), and the `first-input` entry, which comes whatever its duration.
// - Inputs: when the first trusted tap or key began, and a count of those that finished, so
//   the measurement knows each of its inputs arrived before it reads their timing.
//
// `window.__hprVitals` gives `vitals.rs` four calls: `benchmark()` measures the host's CPU
// (below); `settle()` waits for the page to load and then for a second with no new paint or
// shift; `inputs(n)` waits for the n-th input to finish and its timing to be reported; `read()`
// returns what was seen, as JSON.
'use strict';

(function () {
  const state = {
    lcp: null,
    lcpWhat: '',
    shifts: [],
    events: [],
    firstInput: false,
    inputAt: null,
    inputs: 0,
    last: 0,
    unsupported: [],
  };
  const describe = function (node) {
    if (!node || !node.tagName) return '';
    const text = (node.textContent || '').replace(/\s+/g, ' ').trim().slice(0, 40);
    return node.tagName.toLowerCase() + (text ? ' "' + text + '"' : '');
  };
  const observe = function (type, options, each) {
    try {
      new PerformanceObserver(function (list) {
        list.getEntries().forEach(each);
      }).observe(Object.assign({ type: type, buffered: true }, options));
    } catch (err) {
      state.unsupported.push(type);
    }
  };
  observe('largest-contentful-paint', {}, function (entry) {
    state.lcp = entry.startTime;
    state.lcpWhat = describe(entry.element) || entry.url || '';
    state.last = performance.now();
  });
  observe('layout-shift', {}, function (entry) {
    const moved = (entry.sources || []).map(function (source) {
      return describe(source.node);
    }).filter(Boolean).slice(0, 2);
    state.shifts.push([entry.startTime, entry.value, moved.join(', '), entry.hadRecentInput]);
    state.last = performance.now();
  });
  observe('event', { durationThreshold: 16 }, function (entry) {
    if (entry.interactionId) state.events.push([entry.interactionId, entry.name, entry.duration]);
  });
  observe('first-input', {}, function (entry) {
    state.firstInput = true;
    state.events.push([entry.interactionId || 0, entry.name, entry.duration]);
  });
  // An interaction has ended once its last event has run: `click` after a tap, `keyup` after a
  // key. Counted at the window in the capture phase, before the page's own listeners can stop
  // it, and only when trusted, as Event Timing counts only trusted events.
  ['click', 'keyup'].forEach(function (type) {
    window.addEventListener(type, function (event) {
      if (event.isTrusted) state.inputs += 1;
    }, { capture: true, passive: true });
  });
  ['pointerdown', 'keydown'].forEach(function (type) {
    window.addEventListener(type, function (event) {
      if (event.isTrusted && state.inputAt === null) state.inputAt = event.timeStamp;
    }, { capture: true, passive: true });
  });

  const frames = function (n) {
    return new Promise(function (resolve) {
      const next = function () {
        if (n-- <= 0) resolve();
        else requestAnimationFrame(next);
      };
      next();
    });
  };
  const until = function (ready, most) {
    return new Promise(function (resolve) {
      const started = performance.now();
      const poll = function () {
        if (ready() || performance.now() - started > most) resolve(ready());
        else setTimeout(poll, 50);
      };
      poll();
    });
  };

  // The host's CPU, measured as Lighthouse measures it, to choose the CPU slowdown that makes it
  // a phone (`vitals.rs`, `cpu_slowdown`). Ported from Lighthouse's `computeBenchmarkIndex`
  // (core/lib/page-functions.js at 2ac69abc, Copyright Google LLC, Apache-2.0; see
  // THIRD-PARTY-NOTICES.md): the mean of two half-second loops, one building a 10,000-character
  // string (heavy on garbage collection) and one copying a 100,000-entry array, each counted in
  // tens of iterations a second.
  function hprBenchmarkIndex() {
    function withGarbage() {
      const start = Date.now();
      let iterations = 0;
      while (Date.now() - start < 500) {
        let s = '';
        for (let j = 0; j < 10000; j++) s += 'a';
        if (s.length === 1) throw new Error('never: keeps the loop from being optimized away');
        iterations++;
      }
      const seconds = (Date.now() - start) / 1000;
      return Math.round(iterations / 10 / seconds);
    }
    function withoutGarbage() {
      const a = [];
      const b = [];
      for (let i = 0; i < 100000; i++) a[i] = b[i] = i;
      const start = Date.now();
      let iterations = 0;
      // The clock read on every tenth pass only, as Lighthouse does (a performance cliff on some
      // Intel CPUs otherwise: https://bugs.chromium.org/p/v8/issues/detail?id=10954#c1).
      while (iterations % 10 !== 0 || Date.now() - start < 500) {
        const from = iterations % 2 === 0 ? a : b;
        const to = iterations % 2 === 0 ? b : a;
        for (let j = 0; j < from.length; j++) to[j] = from[j];
        iterations++;
      }
      const seconds = (Date.now() - start) / 1000;
      return Math.round(iterations / 10 / seconds);
    }
    return (withGarbage() + withoutGarbage()) / 2;
  }

  Object.defineProperty(window, '__hprVitals', {
    value: Object.freeze({
      // Resolves to whether the page settled: loaded, then a second with no new paint or shift
      // after the load and after the last of them.
      settle: function (quietMs, mostMs) {
        return until(function () {
          const navigation = performance.getEntriesByType('navigation')[0];
          const loaded = navigation ? navigation.loadEventEnd : 0;
          return document.readyState === 'complete' && loaded > 0 &&
            performance.now() - Math.max(state.last, loaded) >= quietMs;
        }, mostMs);
      },
      // Resolves to whether `n` inputs reached the page, after the frames that report their
      // timing have been drawn.
      inputs: function (n, mostMs) {
        return until(function () { return state.inputs >= n; }, mostMs).then(function (arrived) {
          return frames(3).then(function () {
            return new Promise(function (resolve) { setTimeout(function () { resolve(arrived); }, 250); });
          });
        });
      },
      benchmark: hprBenchmarkIndex,
      read: function () {
        const navigation = performance.getEntriesByType('navigation')[0];
        return JSON.stringify({
          visible: document.visibilityState === 'visible',
          lcp: state.lcp,
          lcpWhat: state.lcpWhat,
          shifts: state.shifts,
          events: state.events,
          firstInput: state.firstInput,
          inputAt: state.inputAt,
          inputs: state.inputs,
          unsupported: state.unsupported,
          waitedMs: navigation ? navigation.responseEnd - navigation.startTime : null,
        });
      },
    }),
  });
})();
