# ADR-017: Model pages open with *In short*; Accuracy traces its numbers; the records stay files (2026-09-18)

- **Status:** accepted
- **Summary:** Model pages open with *In short*; Accuracy traces its numbers; the records stay files

**Context.** M0.4b asks that every model page open with *In short* (what it models, its source,
how well it is validated, what it leaves out), that a page without it fail CI, that an *Accuracy*
page give every validation result from the committed report, and that the decisions and the
roadmap be reachable from the site. ADR-016 left open whether the last two become pages. On
2026-09-18 there were 16 model pages under `docs/physics/`, one validation report with 5 cases and
30 metrics, 17 decision records and a roadmap of 806 lines.

**Decision.**

- **The form of *In short* is fixed, so a check can hold every page to it.** Right under the
  page's title, a `## In short` section holds only a bulleted list of four items, which open with
  the bold labels `What it models:`, `Sources:`, `How well it is validated:` and
  `What it leaves out:`, in that order, each followed by its answer. The labels repeat the
  milestone's own words. What pages used to open with (code paths, decisions, the source list)
  moves under a heading of its own, usually `## Code and sources`. A model page is any page under
  `docs/physics/`; the format pages under `docs/format/` describe files, not models, and are
  exempt. The check (`in_short` in `xtask/src/site.rs`) reports the first departure and its line.
- **Its answers follow the page, and say how far the evidence goes.** *How well it is validated*
  names which of the four kinds of evidence in `VALIDATION.md` the model has (analytic, published
  source, another code, real flights), with the page's own numbers, and says plainly when a model
  has not been compared with another code or a real flight. The check can't judge that; the
  physics review does.
- ***Accuracy* traces every number it quotes.** Each number in a paragraph, list item or table row
  (outside code) must appear in a repository file that the same paragraph, item or row links to:
  a model page (less its own *In short*), the report, a case file. The pages at the top of the
  site don't count, so a page can't vouch for itself. A number is anything with a decimal point,
  an exponent (`1e-6`, `10⁻¹²`), a percent sign or two digits; "Level 2", "6-DOF" and "3 fins"
  are words. It matches only a number written the same way: the same digits as a whole number,
  and the same sign and percent sign where the page writes them. So a reader checks it in one
  click, and a number that moves at its source fails the site check until the page follows. The
  check can't tell whether a number is quoted in the right context; reviews do that.
- **The report's results are checked cell by cell.** A table on *Accuracy* whose header opens
  with `case`, and names a report metric in code in each other column, must give each case's
  difference exactly as the report writes it, and all the report's results must be in such a
  table. So the page gives every validation result, and each one right. Numbers are checked, not
  generated: a generated page would read worse than one written for people, and the check gives
  the same guarantee. The page must also link every model page.
- ***In short* traces its numbers too.** Each number in a model page's *In short* must appear in
  the rest of that page, or in a file its item links to, so the summary can't claim what the page
  doesn't show.
- **The decisions and the roadmap stay files, with a page that indexes them.** Rendered as pages,
  `DECISIONS.md` and `ROADMAP.md` would carry hundreds of labels at the places that define them,
  which the label check would have to learn to accept, and they are working files that change
  with every milestone. Instead, *Decisions and the roadmap* explains the three kinds of label and
  links every decision record and every phase of the roadmap on GitHub, each with a line of
  meaning. The check fails if a record or a phase is missing from it.

**Consequences.**

- A new model page fails CI until it opens with *In short*, and until *Accuracy* links it.
- A regenerated report that moves a quoted number, or adds a case or a metric, fails CI until
  *Accuracy* follows. So does a model page whose quoted number changes.
- A new decision record fails CI until the records page links it.
- The check can't tell a true *In short* from a false one. Reviews do that, against the page's
  body, `VALIDATION.md` and the report.
