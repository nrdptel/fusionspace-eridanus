# Awareness practice: how the docs site should show its banner and footer line

**Bottom line:** show one banner at a time, chosen in the reader's browser from a dated table with
day-level windows; the narrowest window wins, international before regional, and solemn days stand
alone. Name the region in the text instead of looking up the reader's location. Treat mental health
with the WHO's 2023 media guidance: lead with hope and where to get help, link a small, checked set
of crisis lines, and never a method. This note is for the later milestone that builds the banner;
the dates are in [awareness calendar](awareness-calendar.md). Everything here was read on
2026-10-14. Marks: **P** read on the organizer's or a government page, **S** only on a secondary
page, **U** not verified. The selection rule is a recommendation, not yet built or tested.

## How Motor Finder did it

[Motor Finder][mf] is the maintainer's own MIT project, so its code can be ported with a note in
`THIRD-PARTY-NOTICES.md`, as for Loft and Debrief. Read on 2026-10-14: `lib/observances.ts` (236
lines) and its test, `app/layout.tsx`, `app/page.tsx`, `next.config.ts`, the deploy workflow.

- **What it showed:** 20 entries keyed to the calendar month, all 12 months filled, US-centric by
  its own comment. Each had an id, an emoji, a one-line message, a link and a `bar`: a CSS
  gradient in the cause's ribbon or flag colors with a `title` tooltip.
- **How it chose:** `OBSERVANCES[date.getMonth()]`. The layout stacked one thin full-width bar per
  active entry, `aria-hidden`, with only a `title`; the home page footer printed the emoji, the
  message and an outbound link. The site is a static export, so the month was fixed when the site
  was built, in the build runner's time zone (normally UTC), and rolled over only because the site
  redeploys after each hourly scrape.
- **Tests:** all 12 months filled, links https, ids unique, June listing Pride first. No dismissal.

Its defects:

1. **Stale comments:** two files say the pages revalidate every 60 s; the config says static
   export. If scraping stops, the month freezes.
2. **Month granularity:** Hispanic Heritage Month runs September 15 to October 15
   ([36 U.S.C. 126][usc126]) but showed September 1 to 30; Overdose Awareness Day (August 31) and
   World AIDS Day (December 1) are single days shown all month.
3. **Build-time UTC:** near midnight on the 1st, readers west of UTC see next month early, east of it late.
4. **Meaning hidden:** the bar's words sit in a `title` on an `aria-hidden` element, out of reach of
   keyboard, touch and screen reader; off the home page the bar has no words at all.
5. **Two stacked bars** most months read as a glitch; a rainbow beside a green bar has no label.
6. **US-only help:** 988, SAMHSA, and ada.gov (a legal-information site, not a disability-community
   source). A reader in crisis in the UK, Canada or Australia gets a US number.
7. **Tone:** sexual assault and domestic violence got one slogan line on a motor-shopping page;
   breast cancer linked a fundraising charity, not a public-health source.
8. **Emoji:** a skin-tone emoji chosen for Black History Month; a neutral one, or none, is better.

Worth keeping: a data table with a test that every month is covered, every link is https and ids
are unique; a short warm line plus a link; ribbon colors as an accent, when a label is visible.

## The selection rule

1. **Data.** Each row has a start and an end (a month-day, or a rule such as "third Thursday of
   May" computed in code), a region, a group, a priority, the banner text, the footer text and a
   link. Year-specific dates (Dark Sky Week, Observe the Moon Night, Engineers Week, meteor peaks)
   carry a year; a year without a row simply shows nothing, so a stale table fails quiet, not wrong.
2. **Windows.** A month runs its month and a week its days; a single day shows from 2 days before
   through the day itself ("Thursday is World Sight Day"), never after it.
3. **Choice, in order.** (a) A safety row flagged urgent (none by default; kept for something like a
   motor recall the maintainer adds by hand). (b) The narrowest window wins: a day beats a week
   beats a month. (c) On a tie, international beats one region. (d) Then the group closest to the
   project: safety, STEM and space, access, health, heritage, environment. (e) Then table order.
   The month item that loses the banner moves to the footer, so a heritage month is never hidden,
   only quieter for a day or two.
4. **Solemn days alone.** A solemn row (suicide prevention, remembrance days, World AIDS Day) takes
   the banner, and nothing celebratory shows in the footer that day. On 2027-09-10, World Suicide
   Prevention Day takes the banner and the Observe the Moon Night line moves to September 9.
5. **The reader's clock.** The date comes from `new Date()` in local time at page load, not from the
   build: the docs site is rebuilt only on merges, so a build-time month would freeze for weeks.
   With JavaScript off, show nothing, or a static year-round footer line.
6. **No location lookup.** No IP geolocation and no `navigator.geolocation`: it costs privacy and
   gets VPN users wrong. Name the region in the text instead. Dismissal is per item, remembered in
   `localStorage` inside try/catch.
7. **A source date and a stale-after date** on the help page, as on the project's regulatory
   templates, and a review at least yearly: the WHO asks for contact details checked regularly.

## Regions and seasons

- **Say where:** "In the US, ...", "...in the US and Canada", "October is Black History Month in
  the UK". It reads as information, not an assumption about the reader, and lets UK and Canadian
  readers see their own months (UK Black History Month, Canada's Women's History Month: October).
- **Pair regions** where an observance is shared: AANHPI Heritage Month in the US with Asian
  Heritage Month in Canada.
- **Cite the observance's own source** or a scholarly one (the Library of Congress consortium
  sites, the Smithsonian, the organizer) over a presidential proclamation, which changes with each
  administration and pulls a banner into politics it doesn't need.
- **Hemisphere-neutral safety lines:** "Fire season where you fly?", not July as summer. Australia,
  New Zealand, South Africa and South America fly in the opposite season.

## Include, footer only, leave out

- **Include:** every UN, WHO or UNESCO day that ties to science, space, access or health; the
  heritage months, regions named; Pride, with its Library of Congress history link, not a slogan;
  Disability Pride Month; suicide prevention and mental health, with the help page; the rocketry
  safety reminders, the most useful thing a launch tool can say, and true in every country.
- **Footer only:** disease-awareness days and months (cancer, heart, diabetes, AIDS), PTSD
  Awareness Month, national preparedness, environmental days; Blood Donor Month and American Heart
  Month (US-only) at most. Worthy, but far from the tool, and a banner every few days becomes noise.
- **Leave out:**
  - Sexual assault and domestic violence months: real and serious, but a one-line banner can't
    carry the care they need, and the right line differs by country. List such services on the
    help page through [Find A Helpline][fah], which covers them by country (P).
  - A single charity's donation page as a link, unless the campaign is the observance (Movember);
    prefer public-health sources such as the WHO and national health agencies.
  - Commercial or novelty days, and anything tied to an election, a court case, a military
    operation or a living politician.
  - Contested observances, such as Columbus Day against Indigenous Peoples' Day on the second
    Monday of October ([5 U.S.C. 6103][usc6103] lists Columbus Day, P). The UN's International Day
    of the World's Indigenous Peoples (August 9) carries the same care without the dispute.
- **Tone:** warm, specific, short, one idea; no exclamation marks; no "Happy" on a solemn day; no
  emoji in the banner (footer only, `aria-hidden`); no skin-tone or flag emoji chosen for a group.

## Mental health and suicide prevention

- **[WHO, *Preventing suicide: a resource for media professionals, update 2023*][who]** (published
  2023-09-12; CC BY-NC-SA 3.0 IGO, so cite, don't copy). Its Dos: accurate information on where to
  seek help, at the start or end; facts; stories of coping and help-seeking. Name only a limited
  number of services, since a long list overwhelms, and check their contact details regularly
  (printed pages 12–13). Its Don'ts: the method or location, sensational or normalizing language,
  a single simple cause, sensational headlines (pages 18–21).
- **[Action Alliance, Framework for Successful Messaging][aa]:** four elements: strategy, safety,
  a positive narrative (actions, solutions, resources) and guidelines.
- **[Recommendations for Reporting on Suicide][save]:** reportingonsuicide.org now redirects to
  SAVE, which hosts the consensus built with AFSP, AAS, CDC, IASP's media task force and others.
- **[Samaritans media guidelines][samg]** (UK): no novel methods, no pointers to sites or forums
  with harmful content, extra care with young audiences.

On the banner: lead with connection and hope; link one help page, not a phone list; never a method,
an "epidemic" framing or a single cause; "died by suicide" if death is mentioned (it never needs to
be). The help page is reachable all year from the footer, not only in September.

| Where | Service | How | Checked |
|---|---|---|---|
| Everywhere | [Find A Helpline][fah] (ThroughLine) | Verified lines in 175+ countries, domestic-violence and sexual-assault lines too | 2026-10-14, P |
| United States | [988 Suicide & Crisis Lifeline][us988] | Call or text 988; chat; 24/7; Spanish text and chat; a Deaf and hard-of-hearing page | 2026-10-14, P |
| US veterans | Veterans Crisis Line | Dial 988 then press 1; text 838255 (confirm at veteranscrisisline.net) | 2026-10-14, S (VA releases, 2022) |
| Canada | [9-8-8: Suicide Crisis Helpline][ca988] | Call or text 9-8-8; 24/7/365; Public Health Agency of Canada | 2026-10-14, P |
| UK and Ireland | [Samaritans][sam] | Call 116 123, free from landlines and mobiles, any time | 2026-10-14, P |
| Australia | [Lifeline][ll] | Call 13 11 14, 24/7; text 0477 13 11 14; chat; 13YARN 13 92 76 for Aboriginal and Torres Strait Islander people | 2026-10-14, S (search summary) |

Put "If you are in immediate danger, call your local emergency number" first; keep the list this
short. The Trevor Project (US, LGBTQ+ young people) is a reasonable June addition if it stays short.

## Beyond banners

- **Accessibility:** meet [WCAG 2.2][wcag] AA, the W3C Recommendation of 2024-12-12.
- **Color and contrast:** never color alone (SC 1.4.1), every ribbon color labeled; SC 1.4.3 and
  1.4.11 for charts and the banner, light and dark ([WCAG 2.2][wcag]).
- **Motion:** no animated banner; any transition behind [`prefers-reduced-motion`][mdn].
- **Plain language:** short sentences, terms defined at first use ([digital.gov guides][pl]).
- **Units:** metric and imperial side by side, as [Tripoli's code][tra] does (rule 5-1).
- **Translation:** strings keyed by id, `lang` set, no idioms such as "fly high" (no outside source).
- **Offline:** the banner is a few hundred bytes of data, not an image or a font (project rule).
- **No tracking:** no analytics or third-party script; dismissals stay in the reader's browser, as
  the [Contributor Covenant][cc] site itself runs no tracking.
- **Code of conduct:** [Contributor Covenant 3.0][cc] (CC BY-SA 4.0, over 40 translations).
- **Youth programs:** [American Rocketry Challenge][arc] (grades 6–12; 1,107 teams in 2026),
  [UKROC][ukroc] (ages 11–18, S), [NARTREK Junior][nartrek] (S), Tripoli mentoring ([code][tra] 6-4).
- **Grants and clubs:** NAR scholarships (ages 17–22, due June 1) and $500 youth grants
  ([NAR][grants], S); NAR's club locator (200+ clubs, S) and Tripoli's prefectures (U).
- **Dark skies:** night-launch banners can point to [DarkSky][ds]'s lighting guidance.
- **First flight welcome:** what to bring and how a club launch works, the [model code][nar] first.
- **Credit contributors:** a contributors page; thank first-time reporters, if they agree (no outside source).
- **Report a barrier:** an accessibility issue template, linked from the GAAD and Disability Pride
  banners.

Licenses: Motor Finder is MIT (port with a note); the WHO resource is CC BY-NC-SA 3.0 IGO (cite
only); Tripoli's code is all rights reserved and NAR's states no terms (link only). The calendar's
dates and names are facts, restated in our own words.

## Open questions for the maintainer

- Transgender Day of Visibility (March 31) and of Remembrance (November 20), dates U: consistent
  with including Pride, but whether they get a banner is a values call.
- Remembrance Day and Veterans Day (November 11): one shared line, two, or footer only.
- Which heavy topics get a banner at all: sexual assault, domestic violence, overdose, cancer.

[mf]: https://github.com/nrdptel/Hobby-Rocket-Motor-Finder
[usc126]: https://www.law.cornell.edu/uscode/text/36/126
[usc6103]: https://www.law.cornell.edu/uscode/text/5/6103
[who]: https://www.who.int/publications/i/item/9789240076846
[aa]: https://suicidepreventionmessaging.org/
[save]: https://www.save.org/media/media-recommendations/
[samg]: https://www.samaritans.org/about-samaritans/media-guidelines/
[fah]: https://findahelpline.com/
[us988]: https://988lifeline.org/
[ca988]: https://988.ca/
[sam]: https://www.samaritans.org/how-we-can-help/contact-samaritan/
[ll]: https://www.lifeline.org.au/131114/
[wcag]: https://www.w3.org/TR/WCAG22/
[mdn]: https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-reduced-motion
[pl]: https://digital.gov/guides/plain-language
[tra]: https://www.tripoli.org/safety
[cc]: https://www.contributor-covenant.org/version/3/0/code_of_conduct/
[arc]: https://www.rocketrychallenge.org/
[ukroc]: https://www.adsgroup.org.uk/events/uk-youth-rocketry-challenge-2026-ukroc/
[nartrek]: https://nar.org/NARTREKJunior
[grants]: https://nar.org/GrantsandScholarships
[nar]: https://www.nar.org/
[ds]: https://darksky.org/
