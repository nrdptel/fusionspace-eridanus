// SPDX-License-Identifier: MIT OR Apache-2.0
// FusionSpace HPR's additions to mdBook's script (#383). mdBook's `book.js` scrolls to the top
// smoothly when the menu bar's title is clicked, and no stylesheet can turn a script's smooth
// scroll off. The product system has no smooth scroll (`foundations.md`, *Motion*), so the click
// is taken first, on its way down to the title, and the page jumps to the top. Taking it also
// keeps it from mdBook's own listener that closes an open theme menu on a click outside it, so
// that menu is closed here, through its button. `cargo xtask site`'s page check clicks the title
// and fails any smooth scroll a script asks for.
'use strict';

document.addEventListener('click', function (event) {
    const title = event.target instanceof Element ? event.target.closest('.menu-title') : null;
    if (!title) return;
    event.stopPropagation();
    const themes = document.getElementById('mdbook-theme-list');
    const toggle = document.getElementById('mdbook-theme-toggle');
    if (themes && toggle && themes.style.display === 'block') toggle.click();
    document.scrollingElement.scrollTo({ top: 0, behavior: 'instant' });
}, true);
