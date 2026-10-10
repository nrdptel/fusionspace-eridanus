// SPDX-License-Identifier: MIT OR Apache-2.0
// FusionSpace HPR's additions to mdBook's script (#383, #384).
//
// The menu bar's title is the system's lockup, a link to the landing page (`cargo xtask site`
// draws it). mdBook's `book.js` also scrolls to the top smoothly when the title is clicked, and
// the product system has no smooth scroll (`foundations.md`, *Motion*), so the click is taken
// first, on its way down to the title, and the link is simply followed. Taking it also keeps it
// from mdBook's own listener that closes an open theme menu on a click outside it, so that menu
// is closed here, through its button. `cargo xtask site`'s page check clicks the title and fails
// any smooth scroll a script asks for.
//
// From 720 px the sidebar stays open and the menu button is gone (`web.md`, *Page anatomy*):
// the build has mdBook show the sidebar there when a page loads, and a window that widens past
// 720 px opens it here, through mdBook's own checkbox, so its `aria-hidden` and its links' focus
// follow as mdBook sets them. mdBook sets its sidebar links' focus once, before its script adds
// the current page's headings to the sidebar, and again only for the links it found then; so
// each link here follows the sidebar's `aria-hidden` whenever either changes: out of the tab
// order while the sidebar is put away, in it while it is shown.
'use strict';

document.addEventListener('click', function (event) {
    const title = event.target instanceof Element ? event.target.closest('.menu-title') : null;
    if (!title) return;
    event.stopPropagation();
    const themes = document.getElementById('mdbook-theme-list');
    const toggle = document.getElementById('mdbook-theme-toggle');
    if (themes && toggle && themes.style.display === 'block') toggle.click();
}, true);

(function () {
    const wide = window.matchMedia('(min-width: 720px)');
    const open = function () {
        const box = document.getElementById('mdbook-sidebar-toggle-anchor');
        if (wide.matches && box && !box.checked) {
            box.checked = true;
            box.dispatchEvent(new Event('change'));
        }
    };
    wide.addEventListener('change', open);
})();

(function () {
    const sidebar = document.getElementById('mdbook-sidebar');
    if (!sidebar) return;
    const follow = function () {
        const away = sidebar.getAttribute('aria-hidden') === 'true';
        sidebar.querySelectorAll('a').forEach(function (link) {
            link.tabIndex = away ? -1 : 0;
        });
    };
    new MutationObserver(follow).observe(sidebar, {
        attributes: true, attributeFilter: ['aria-hidden'], childList: true, subtree: true,
    });
    follow();
})();
