# ADR-268: Neer's 2026-10-14 plan: awareness and care on the sites (2026-10-14)

- **Status:** accepted; adds M0.13, queued after release 0.2 ([M10.2](../ROADMAP.md))
- **Summary:** The docs site, and then fusionspace.co, get a thin banner under the page's strip and a line in the footer that change through the year: heritage and awareness months, health and mental health, accessibility, science and space days, and rocketry-safety reminders. One banner at a time, chosen in the reader's browser by date from a dated table where every row names its organizer's own page. A region-specific observance names its region, a solemn day stands alone, nothing looks up where the reader is, and with scripts off nothing shows. A help page lists checked crisis lines, as WHO's guidance on suicide prevention asks. The pattern goes into the product system first. A second increment adds what a banner can't do: a code of conduct, a page on getting involved (youth programs, clubs, mentoring) and a way to report a barrier to access.

[adr-162]: 0162-the-suite-and-its-releases.md
[adr-164]: 0164-the-fusionspace-product-system.md
[adr-217]: 0217-the-2026-10-09-fusionspace-co-rebuilt.md

**Context.** On 2026-10-14 Neer decided to use the project's sites to spread awareness, outreach
and care, and asked that the project be considerate of everyone who uses it. His earlier Motor
Finder site (motor.fusionspace.co, the maintainer's own MIT-licensed project) drew thin colored
bars at the top of each page and lines in the home page's footer, changing with the month.

- **What Motor Finder did, and where it fell short** ([practice note](../research/awareness-practice.md)).
  It picked the month when the site was built, on a build machine's UTC clock, and drew one bar per
  observance with its meaning only in a hover tooltip. Hispanic Heritage Month (September 15 to
  October 15) showed for all of September and none of October; single days showed for a whole
  month; every help link was for the US only; and nothing could be dismissed.
- **The calendar** ([calendar note](../research/awareness-calendar.md)). About 85 observances across
  the twelve months, each with its date rule, region and organizer's page, and how sure the date is.
  Rocketry safety (fire season and burn bans, Fire Prevention Week, hearing and eye protection,
  heat, storms, the NAR and Tripoli safety codes) applies in every country and fills the gaps.
- **Where it sits.** The docs site is the project's most-read surface, and fusionspace.co
  ([ADR-217][adr-217]) is its front door. Neither shows anything that changes by date today. The
  banner adds no number and changes no physics, so it serves no release's gate: [ADR-162][adr-162]
  places such ideas behind the release they don't serve.

**Decision.**

1. **One banner, one footer line, chosen by date in the reader's browser.** A dated table, one row
   per observance: its window (a fixed date or a rule such as "second Tuesday of October"), its
   region, its group, the banner text (at most 90 characters), the footer text, its organizer's
   page and the date the row was checked. A single day shows from two days before through the day,
   never after it. The narrowest window wins (a day beats a week beats a month); on a tie an
   international observance beats a regional one, then the group nearest the project (safety,
   then space and science, then access). The month item that loses the banner moves to the footer,
   so it is quieter for a day, never hidden. A row tied to one year (a meteor peak, Observe the
   Moon Night) simply doesn't show in a year without a row.
2. **Considerate of everyone.**
   - A region-specific month names its region ("in the US and Canada"); nothing reads where the
     reader is, by address or by location.
   - Seasonal safety lines don't assume the northern hemisphere ("fire season where you fly?").
   - A solemn day (suicide prevention, World AIDS Day, a day of remembrance) takes the banner alone,
     with nothing celebratory in the footer that day.
   - Links go to the organizer's own page or a public-health agency, not a fundraising page, and
     not to a proclamation that changes with each administration.
   - No observance tied to an election, a court case, a living politician or a disputed name; a
     disputed day gives way to its undisputed international counterpart.
   - Plain, warm, short; no exclamation marks, no "happy" on a solemn day, no emoji in the banner.
3. **Mental health done carefully.** A banner about mental health or suicide prevention links one
   help page, never a phone list in the banner. The page leads with hope and where to get help,
   lists a few lines checked on their official sites (988 in the US, 9-8-8 in Canada, Samaritans
   116 123 in the UK and Ireland, and Find A Helpline for every other country), and shows the date
   they were checked, as WHO's 2023 guidance for media asks. Services for sexual and domestic
   violence are reached through that page's international directory, not a banner.
4. **Accessible and out of the way.** The banner is one line in the system's type, never clipped at
   any width from 320 px, a labelled region for screen readers, still under reduced motion, and
   dismissible per row (remembered in the browser where it can be, never required). Meaning never
   rides on color alone. With scripts off, nothing shows; the footer may keep a year-round line.
   Nothing is sent anywhere: no tracking, no counting views.
5. **Kept true.** A test fails a row without an organizer's page, past its stale-after date (a year
   after it was checked), with a window that doesn't parse, or with text over 90 characters; the
   page check opens the banner on a date from each month and fails a clipped or overlapping one.
6. **The pattern goes into the product system first** ([ADR-164][adr-164]): a notice strip under
   the brand's strip and a footer line, with their rules, in `fusionspace-design`; then this docs
   site; then fusionspace.co, from the same table.
7. **What a banner can't do comes next** (M0.13b): a code of conduct (Contributor Covenant 3.0)
   with a way to reach the maintainer; a page on getting involved, linking youth programs (the
   American Rocketry Challenge, the UK Youth Rocketry Challenge, the NAR's junior and scholarship
   programs, Tripoli's mentoring program) and club finders; and an issue template for reporting a
   barrier to access, answered like a bug.
8. **Open for the maintainer.** Whether Transgender Day of Visibility and Transgender Day of
   Remembrance, and Remembrance Day and Veterans Day, take the banner, the footer or neither, and
   which heavy topics take a banner at all. The table ships with each such row marked, off until
   he decides.

**Consequences.** M0.13 joins the queue after release 0.2. Its first increment changes three
repositories (the product system, this one and fusionspace.co's), and the table needs a yearly
look: the stale-after test makes that visible rather than silent. Some organizer pages refused
automated reads when the calendar was researched; those rows say so and are checked by hand
before they ship.
