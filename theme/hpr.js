// SPDX-License-Identifier: MIT OR Apache-2.0
// FusionSpace HPR's additions to mdBook's script (#383, #384).
//
// The menu bar's title is the system's lockup, a link to the landing page (`cargo xtask site`
// draws it). mdBook's `book.js` also scrolls to the top smoothly when the title is clicked, and
// the product system has no smooth scroll (`foundations.md`, *Motion*), so the click is taken
// first, on its way down to the title, and the link is simply followed. `cargo xtask site`'s
// page check clicks the title and fails any smooth scroll a script asks for.
//
// From 720 px the sidebar stays open and the menu button is gone (`web.md`, *Page anatomy*):
// the build has mdBook show the sidebar there when a page loads, and a window that widens past
// 720 px opens it here, through mdBook's own box, after undoing the `display: none` mdBook gives
// a sidebar put away (only its menu button undoes it). The box, ticked or not, is what draws the
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
// scrolling list, which a menu button opens under 720 px. The page check reads where it stands
// and which button is pressed at every width, presses Dark and then Auto to see the theme
// follow, and loads the page with Coal saved to see it forgotten.
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
        if (wide.matches && !box.checked) {
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
