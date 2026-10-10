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
// 720 px opens it here, through mdBook's own box, after undoing the `display: none` mdBook gives
// a sidebar put away (only its menu button undoes it). The box, ticked or not, is what draws the
// sidebar, so the sidebar's `aria-hidden` and its links' focus follow the box here whenever
// either changes: mdBook sets the links' focus once, before its script adds the current page's
// headings, and a swipe or a drag can hide the sidebar from screen readers with the box still
// ticked. The page check reads all of this at every width, after widening a narrow window too.
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
        sidebar.querySelectorAll('a').forEach(function (link) {
            if (link.tabIndex !== focus) link.tabIndex = focus;
        });
    };
    new MutationObserver(follow).observe(sidebar, {
        attributes: true, attributeFilter: ['aria-hidden'], childList: true, subtree: true,
    });
    box.addEventListener('change', follow);
    follow();
})();
