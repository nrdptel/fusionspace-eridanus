# ADR-239: The words to the design: spelling, minus signs, number columns and the READMEs (2026-10-11)

- **Status:** accepted; closes M0.9e
- **Summary:** `xtask spelling` reads the British forms the design audit found and the others a survey of the pages turned up, and the project now writes them in US English, renamed code names included; the site check fails a hyphen-minus written for a minus sign and a table column of numbers that isn't right-aligned; the READMEs follow `writing.md`'s order with a status word and the version, held by a test. What #386 listed beyond M0.9e's *done when* moves to #476 and a new increment, M0.9f, before release 0.2.

**Context.** M0.9e's *done when*: "#386 and #387 are closed: `xtask spelling` fails on the
British forms the audit found, a site check fails on a hyphen-minus for minus and on a number
column not right-aligned, and the READMEs follow `writing.md`'s order with the version; the
audit's rows name the checks." The design audit
([ADR-208](0208-the-design-audit.md)) filed #386 for the number, spelling, date, name and voice
rules of the product system's `writing.md` and `data.md`, and #387 for the READMEs.

**Decisions.**

1. **The spelling list.** `xtask spelling` checked eight word families. It now also reads the
   audit's (calibre, labelled, organisation, optimisation, normalised, fibreglass, grey,
   travelled) and the British forms a survey of every checked file found: -re words (fibre,
   litre, theatre, manoeuvre), doubled l's (labelled, signalled, fuelled), -our words (favour,
   neighbour, vapour), -ence words (defence), single words (aluminium, judgement, sulphur,
   ageing, programme), and -ise, -ising and -isation on a list of whole stems (normalise,
   optimisation, summarising). Whole stems keep "otherwise", "exercise" and "optimistic" out.
   "Programmer" and "programmed" are US forms as they stand and are skipped, as "analyses" was.
   On the first run it found 1,033 forms in the 664 files it reads, 123 of them in code names; `--fix`
   replaced the words and the code names were renamed by hand from the check's own list. Three
   open roadmap entries changed in spelling only ("grays out", "anonymized", "labeled"); no
   criterion changed.
2. **Renamed names and kept keys.** One renamed name was saved to a file: the census's unit key,
   `calibre`, now written `caliber`, with the old key kept as a serde alias, as `writing.md` has
   it for every renamed key. Three public names of `hpr-atmos` were renamed without an alias:
   `saturation_vapor_pressure_pa`, `vapor_pressure_pa` and
   `WATER_VAPOR_MOLECULAR_WEIGHT_KG_PER_KMOL`. Until 1.0 a new minor version may change the
   library's API (the changelog's preamble), so 0.2's notes must list them. The command line
   prints "calibers". Kept as they are: a company's name (Airbus Defence and Space, in the
   elevation data's credit) and SVG's `aria-labelledby`.
3. **A minus sign is a minus sign.** The site check fails a hyphen-minus right before a figure
   (or a decimal point and a figure) at the start of a run of text, after a space, an opening
   bracket or an operator. A hyphen after a letter or a digit joins (`ADR-208`, `2026-10-04`,
   `1e-9`) and passes; code is never read. Seven were found. Four were in the census's table,
   which a program writes, so the census now writes its ranges with U+2212, and the committed
   census was rewritten with no number changed.
4. **A number column is right-aligned.** A column fails when every body cell that isn't blank (an
   empty cell, a dash, "n/a" or "Not used") is a number and its delimiter isn't `---:`. A number
   is a figure, with a sign or comparison before it, followed only by unit symbols, figures,
   signs and brackets: `1,400 m (4,593 ft)`, `−7.28%`, `5,486 ± 227`. A cell in code, a date or a
   version is a label, and a cell with any other word is text: the check fails toward passing a
   column it can't read rather than failing one it can't judge. 338 columns on 25 pages were
   right-aligned.
5. **The READMEs.** The README and the Python package's README now give, in order: what it is;
   a status in a drawing word (`principles.md` §7) with the version; what works and what doesn't,
   with how far to trust it; how to install it; where the details are; and the license with the
   designation and the version. A test reads both, and fails on a heading out of order, a status
   that is not IN PREPARATION, RELEASED or WITHDRAWN, or a version other than the workspace's.
   Every published crate inherits the README, so crates.io shows it.
6. **#386's other items move to #476 and M0.9f.** #386 also listed ungrouped and over-precise
   numbers, ISO dates in running prose, en dashes in joined names, code identifiers as table
   heads, numbers that wrap away from their units, and 60-word warnings. M0.9e's *done when*
   names none of them, and each needs its own rule: a grouping check, for one, needs a list of the
   identifiers written as numbers (NTRS report numbers, station numbers, years). #476 holds them;
   M0.9f, queued before release 0.2 because `review.md`'s *Before release* list asks for the
   writing rules, closes it. "RSO" is spelled out where the README and the validation plan first
   use it.

**Consequences.** The audit counts 37 sections met and 20 not, from 33 and 24: *Pointing a
project here*, *Numbered like parts*, *Space and layout* and *READMEs and docs* are met, and
the rows that stay open cite #476 and M0.9f.
