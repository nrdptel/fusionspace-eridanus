// SPDX-License-Identifier: MIT OR Apache-2.0
// FusionSpace HPR's additions to mdBook's script (#383, #384).
//
// The menu bar's title is the system's lockup, a link to the landing page (`cargo xtask site`
// draws it). mdBook's `book.js` also scrolls to the top smoothly when the title is clicked, and
// the product system has no smooth scroll (`foundations.md`, *Motion*), so the click is taken
// first, on its way down to the title, and the link is simply followed. `cargo xtask site`'s
// page check clicks the title and fails any smooth scroll a script asks for.
//
// From 720 px the sidebar is open unless the reader put it away, with the menu button or by
// dragging its edge to the window's side (`web.md`, *Site patterns*, Docs): the build has mdBook
// show it there when a page loads unless mdBook stored `hidden`, and a window that widens past
// 720 px opens it here, unless the reader put it away, through mdBook's own box, after undoing
// the `display: none` mdBook gives a sidebar put away (only its menu button undoes it). The box, ticked or not, is what draws the
// sidebar, so the sidebar's `aria-hidden` and its links' focus follow the box here whenever
// either changes: mdBook sets the links' focus once, before its script adds the current page's
// headings, and a swipe or a drag can hide the sidebar from screen readers with the box still
// ticked. The page check reads all of this at every width, after widening a narrow window too.
//
// The theme switch (`web.md`, *Page anatomy* and *Components*): `cargo xtask site` makes mdBook's
// theme menu the system's segmented control, whose buttons mdBook's script still works through.
// It marks the chosen one with a class, which each button's `aria-pressed` follows here. From
// 960 px the switch stands in the header's row, before its buttons; on a narrower window the
// header has no room for it, and it stands at the top of the sidebar, above the chapters'
// scrolling list, which a menu button opens under 720 px. A script the build writes just after
// the header puts it there before the page is first drawn, so a phone's first screen doesn't
// jump (#443); this file moves it when the window's width crosses 960 px. The page check reads
// where it stands and which button is pressed at every width, presses Dark and then Auto to see
// the theme follow, and loads the page with Coal saved to see it forgotten.
//
// The print page (#443): `cargo xtask site` puts its chapters, every chapter of the guide, in a
// template behind a cover, as a phone drawing them all on one screen page was slow to load and
// to answer a tap, and takes out mdBook's call to print as the page loads. The cover's button
// prints. When the browser is about to print, from the button or its own Print command, a copy
// of the chapters is placed here after the cover, which hides on paper, and their code
// highlighted as mdBook's script highlights a page's as it loads: each `code` but a heading's,
// then marked `hljs` for its colors. The copy goes when printing ends. The page check places
// them so before it reads the page on paper.
//
// A jump to a heading: each sheet off screen skips its layout until it nears the window, and one
// never drawn stands in at 1,200 px (`hpr.css`), far short of a sheet on a phone, which at
// 320 px is often tens of thousands of pixels tall. Under 520 px a heading jumped to stands
// 190 px down the window, below the bar's two rows, so the window's top falls in the sheet
// before it: the browser draws that sheet as it nears, keeps its top where it is, and the
// heading moves down by all the sheet grew. At 320 px a link to *Accuracy so far* on the landing
// page stopped about 11,000 px short of it. So before a jump, every sheet from the page's top
// through the target's is drawn, and the jump lands where it would with nothing skipped. That's
// on a click on a link into this page, before the browser follows it; on a page loaded with an
// address that names a heading; and on a change of the address's heading without a click, such
// as one typed in, when the heading is then out of the window. Two frames later the sheets skip
// again, each at the height it was drawn at, so the page doesn't move and a tap still redraws
// only what is on screen. The page check follows a link to each sheet's heading from the top of
// each page at its narrowest width and fails a heading that doesn't land where it would with
// nothing skipped.
//
// The sidebar's list at rest: mdBook scrolls it to put the current chapter in its middle, or to
// keep a link just clicked where it was, and under 960 px the switch then takes the top of the
// sidebar, which shortens the list. Wherever that leaves the list, an entry can sit cut by its
// top edge, or from 960 px under the 4 px strip along the window's top. So as the page loads,
// once its fonts are in and when the switch moves, an entry cut there is brought down to stand
// whole, 4 px below the edge. The reader's own scrolling is left alone. The page check fails text
// cut by the strip, or by the top of a box the page leaves scrolled as it loads.
'use strict';

document.addEventListener('click', function (event) {
    const title = event.target instanceof Element ? event.target.closest('.menu-title') : null;
    if (!title) return;
    event.stopPropagation();
}, true);

(function () {
    const sidebar = document.getElementById('mdbook-sidebar');
    const box = document.getElementById('mdbook-sidebar-toggle-anchor');
    if (!sidebar || !box) return;
    const wide = window.matchMedia('(min-width: 720px)');
    wide.addEventListener('change', function () {
        let kept = null;
        try {
            kept = localStorage.getItem('mdbook-sidebar');
        } catch (e) {
            // Storage blocked: nothing was kept.
        }
        if (wide.matches && !box.checked && kept !== 'hidden') {
            sidebar.style.display = '';
            box.checked = true;
            box.dispatchEvent(new Event('change'));
        }
    });
    // Sets only what differs, so the observer, which sees every write, stops.
    const follow = function () {
        const hidden = String(!box.checked);
        if (sidebar.getAttribute('aria-hidden') !== hidden) sidebar.setAttribute('aria-hidden', hidden);
        const focus = box.checked ? 0 : -1;
        sidebar.querySelectorAll('a, button').forEach(function (link) {
            if (link.tabIndex !== focus) link.tabIndex = focus;
        });
    };
    new MutationObserver(follow).observe(sidebar, {
        attributes: true, attributeFilter: ['aria-hidden'], childList: true, subtree: true,
    });
    box.addEventListener('change', follow);
    follow();
})();

// Dragging the sidebar's edge to the window's side puts it away (`web.md`, *Site patterns*,
// Docs). mdBook's drag handle marks the sidebar hidden once the edge is within 20 px of the side,
// but leaves its box ticked, so the sidebar stays drawn as a strip with its links reachable. When
// such a drag ends, the box is unticked here, as the menu button unticks it, which has mdBook put
// the sidebar away and store the choice; its width goes back to what it was before the drag,
// for when the menu button brings it back.
(function () {
    const handle = document.getElementById('mdbook-sidebar-resize-handle');
    const box = document.getElementById('mdbook-sidebar-toggle-anchor');
    if (!handle || !box) return;
    const root = document.documentElement;
    handle.addEventListener('mousedown', function () {
        const before = root.style.getPropertyValue('--sidebar-target-width');
        window.addEventListener('mouseup', function () {
            if (root.classList.contains('sidebar-visible') || !box.checked) return;
            if (before) {
                root.style.setProperty('--sidebar-target-width', before);
            } else {
                root.style.removeProperty('--sidebar-target-width');
            }
            box.checked = false;
            box.dispatchEvent(new Event('change'));
        }, { once: true });
    });
})();

(function () {
    const themes = document.getElementById('mdbook-theme-list');
    if (!themes) return;
    const buttons = Array.from(themes.querySelectorAll('button.theme'));
    // Sets only what differs, so the observer, which sees every write, stops.
    const pressed = function () {
        buttons.forEach(function (button) {
            const on = String(button.classList.contains('theme-selected'));
            if (button.getAttribute('aria-pressed') !== on) button.setAttribute('aria-pressed', on);
        });
    };
    new MutationObserver(pressed).observe(themes, {
        attributes: true, attributeFilter: ['class'], subtree: true,
    });
    pressed();
    // A theme saved from mdBook's six-theme menu before the switch, Coal, Ayu or Rust, leaves its
    // class on the page, drawn in neither of the site's looks, and no button for it to press: it
    // is forgotten and Auto pressed, as a first-time reader has it.
    let saved = null;
    try {
        saved = localStorage.getItem('mdbook-theme');
    } catch (e) {
        // Storage blocked: mdBook saved nothing either.
    }
    if (saved !== null && saved !== 'light' && saved !== 'navy') {
        document.documentElement.classList.remove('coal', 'ayu', 'rust');
        const auto = document.getElementById('mdbook-theme-default_theme');
        if (auto) auto.click();
    }
    const header = document.querySelector('#mdbook-menu-bar .right-buttons');
    const sidebar = document.getElementById('mdbook-sidebar');
    const list = sidebar && sidebar.querySelector('.sidebar-scrollbox');
    if (!header || !list) return;
    const wide = window.matchMedia('(min-width: 960px)');
    const place = function () {
        if (wide.matches) {
            if (themes.parentElement === header) return;
            header.prepend(themes);
            // The sidebar's focus rule above may have taken the buttons out of the tab order.
            buttons.forEach(function (button) { button.removeAttribute('tabindex'); });
        } else if (themes.parentElement !== sidebar) {
            sidebar.insertBefore(themes, list);
        }
    };
    wide.addEventListener('change', place);
    place();
})();

(function () {
    const book = document.getElementById('hpr-book');
    if (!book || !book.content) return;
    const button = document.getElementById('hpr-print');
    if (button) button.addEventListener('click', function () { window.print(); });
    let placed = [];
    window.addEventListener('beforeprint', function () {
        if (placed.length) return;
        const copy = book.content.cloneNode(true);
        const code = Array.from(copy.querySelectorAll('code')).filter(function (block) {
            return !block.parentElement.classList.contains('header');
        });
        code.forEach(function (block) {
            if (window.hljs) window.hljs.highlightBlock(block);
            block.classList.add('hljs');
        });
        placed = Array.from(copy.childNodes);
        book.parentNode.insertBefore(copy, book);
    });
    window.addEventListener('afterprint', function () {
        placed.forEach(function (node) { node.remove(); });
        placed = [];
    });
})();

(function () {
    const main = document.querySelector('.content main');
    const sheets = main ? Array.from(main.querySelectorAll('.fs-sheet')) : [];
    if (!sheets.length) return;
    const targetOf = function (hash) {
        if (!hash || hash === '#') return null;
        let id = hash.slice(1);
        try {
            id = decodeURIComponent(id);
        } catch (e) {
            // Not percent-encoded as an address would be: the id as written.
        }
        return document.getElementById(id) || document.getElementsByName(id)[0] || null;
    };
    // Draws each sheet that holds `target` or comes before it, until two frames have recorded
    // each one's drawn height as the height it skips at.
    const draw = function (target) {
        const reach = sheets.filter(function (sheet) {
            return sheet.contains(target)
                || (sheet.compareDocumentPosition(target) & Node.DOCUMENT_POSITION_FOLLOWING) !== 0;
        });
        // Drawn with the containment `auto` keeps, so each is as tall as when it draws itself.
        reach.forEach(function (sheet) {
            sheet.style.contentVisibility = 'visible';
            sheet.style.contain = 'layout style paint';
        });
        requestAnimationFrame(function () {
            requestAnimationFrame(function () {
                reach.forEach(function (sheet) {
                    sheet.style.removeProperty('content-visibility');
                    sheet.style.removeProperty('contain');
                });
            });
        });
    };
    let clicked = null;
    document.addEventListener('click', function (event) {
        const link = event.target instanceof Element ? event.target.closest('a[href]') : null;
        if (!link) return;
        const url = new URL(link.getAttribute('href'), document.baseURI);
        if (url.origin !== location.origin || url.pathname !== location.pathname
            || url.search !== location.search) return;
        const target = targetOf(url.hash);
        if (!target) return;
        draw(target);
        // The same heading again changes no address, so no `hashchange` follows to clear this.
        clicked = url.hash === location.hash ? null : url.hash;
    }, true);
    window.addEventListener('hashchange', function () {
        const target = targetOf(location.hash);
        const followed = clicked === location.hash;
        clicked = null;
        if (!target || followed) return;
        draw(target);
        const top = target.getBoundingClientRect().top;
        if (top < 0 || top >= window.innerHeight) target.scrollIntoView({ block: 'start' });
    });
    const first = targetOf(location.hash);
    if (first) {
        draw(first);
        first.scrollIntoView({ block: 'start' });
    }
})();

(function () {
    const list = document.querySelector('#mdbook-sidebar .sidebar-scrollbox');
    if (!list) return;
    // The strip the system's theme fixes along the window's top, as the body's `::before`.
    const strip = function () {
        const before = window.getComputedStyle(document.body, '::before');
        if (before.position !== 'fixed' || parseFloat(before.top) !== 0) return 0;
        return parseFloat(before.height) || 0;
    };
    const settle = function () {
        if (list.scrollTop <= 0) return;
        const edge = Math.max(list.getBoundingClientRect().top, strip());
        const cut = Array.from(list.querySelectorAll('a, .part-title')).find(function (entry) {
            const box = entry.getBoundingClientRect();
            return box.top < edge && box.bottom > edge;
        });
        if (cut) list.scrollTop -= edge + 4 - cut.getBoundingClientRect().top;
    };
    settle();
    if (document.fonts) document.fonts.ready.then(settle);
    window.matchMedia('(min-width: 960px)').addEventListener('change', settle);
})();
