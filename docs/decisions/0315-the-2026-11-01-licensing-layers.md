# ADR-315: The 2026-11-01 licensing layers: the engine stays open, hosted services carry their own terms, and M9.0 sets the app's license (2026-11-01)

- **Status:** accepted; keeps [ADR-001][adr-001]'s license for the engine; adds the app's license to what [M9.0, the UI architecture decision][m9-0] settles; no done-when changes
- **Summary:** The code that computes and checks FusionSpace HPR's numbers stays `MIT OR Apache-2.0`. Hosted services, starting with the motor stock API at motor.fusionspace.co, carry their own terms: free for personal use and for FusionSpace's own tools, with written permission needed for any other site, app or service and for any commercial use. The app's license is chosen in M9.0's decision record, before any app code is public; the leading option is the Functional Source License 1.1 with Apache-2.0 as its future license. Registering a FUSIONSPACE trademark stays deferred.

[adr-001]: 0001-license-and-workspace-layout.md
[adr-287]: 0287-thrustcurve-orgs-motor-records-used-as-published-facts.md
[adr-288]: 0288-every-solid-motor-ships-in-the-package.md
[adr-192]: 0192-the-2026-10-06-follow-up-best-over-first-and-export-control.md
[m9-0]: ../decisions-and-roadmap.md#m9-0
[m9-4]: ../decisions-and-roadmap.md#m9-4
[m9-5]: ../decisions-and-roadmap.md#m9-5
[fsl]: https://fsl.software/
[mf-api]: https://motor.fusionspace.co/api
[i1097]: https://github.com/nrdptel/fusionspace-eridanus/issues/1097

**Context.** hpr has been `MIT OR Apache-2.0` since ADR-001, like the maintainer's other
FusionSpace projects (VISION V14). Release 0.1.0 shipped under those licenses, so that code stays
available under them whatever comes next. Every commit is the maintainer's, so later versions can
be licensed differently without asking anyone.

Two things prompted a second look in the autumn of 2026. Several new browser-based rocket
simulators appeared, most free and some closed. And another company's website and paid simulator
began reading [motor.fusionspace.co's stock API][mf-api] from their visitors' browsers. Its
published terms allowed that: any site could call it, with no key, and its data was under CC BY
4.0. The maintainer pays to run the service, which its API page says refreshes the data hourly.

A code license and a service's terms are separate questions. Closing hpr's source would not have
stopped that use: the API is a running service, governed by its own terms and by who may call it.

**Decision.**

1. **The engine stays open.** The crates, the `hpr` command, the Python package, the file formats,
   and the validation method and reports stay `MIT OR Apache-2.0` (ADR-001). This is the part whose
   value is trust and reach. A number hpr prints is a claim anyone can check against its source,
   its tests and its validation report, and teams, competitions and package channels can adopt it
   without asking.
2. **Hosted services carry their own terms.** A service FusionSpace runs is governed by its terms,
   not by the code's license. From 2026-11-01 the motor stock API's data is free for personal,
   non-commercial use and for FusionSpace's own tools, hpr included. Any other website, app or
   service, and any commercial use, needs the maintainer's written permission first, and shows the
   credit "Motor stock data from motor.fusionspace.co" with a link. Browsers may call the API only
   from FusionSpace's own sites and from sites the maintainer has allowed. Requests that don't come
   from a browser page are served as before, so hpr's reader keeps working. The server doesn't
   check what those programs are, and the terms still apply to them: a program someone writes for
   themselves is personal use, and a published app or service built on `hpr_net::motor_finder`
   needs permission like any other. What the terms cover is the service's own compilation: which
   vendor lists which motor, at what price, and when. The motor figures in it are ThrustCurve.org's
   and the listings are the vendors' own, credited to them ([ADR-287][adr-287]). Copies obtained
   before 2026-11-01 keep the CC BY 4.0 license they came with. The same rule covers the services still to
   come: hpr's own stock service ([ADR-192][adr-192]), accounts and cloud saves
   ([M9.5][m9-5]), and any paid team workspace.
3. **M9.0 sets the app's license.** The app (M9.1 to M9.4) is the part a reseller would rebrand, so
   its license is chosen in M9.0's decision record, with its reasons, before any app code is
   public. The leading option is the [Functional Source License][fsl], version 1.1, with Apache-2.0
   as its future license (`FSL-1.1-ALv2`). Anyone may read, run and change the code for any purpose
   except a commercial product or service that competes with it, and each release becomes
   Apache-2.0 two years after it ships. Until then a release is source-available, not open source
   in the Open Source Initiative's sense. The engine's license doesn't change with it, and
   ADR-001's dependency policy is unaffected.
4. **The trademark waits.** Registering FUSIONSPACE stays deferred by the maintainer. Until then
   the name is protected only by its use. It comes up again before the app is published
   ([M9.4][m9-4]).

**What changes for a user.**

- **The `hpr` command and the Python package:** nothing. They keep fetching stock data, which is
  free for personal use. Their credit line still says CC BY 4.0 until [#1097][i1097] fixes it.
- **The motor data in the package** ([ADR-288][adr-288]): unchanged. It is ThrustCurve.org's,
  used as published facts.
- **A website, app or service that shows the stock data:** needs the maintainer's permission
  from 2026-11-01, and a browser on another site gets a refusal from the API.

**Alternatives considered.**

- **Close the engine's source.** Rejected. It would cost the trust the project rests on and the
  reach of an open library, and it would stop neither new simulators nor reuse of the hosted data.
  Release 0.1.0 would stay open anyway.
- **Move the engine to AGPL-3.0.** Rejected. It puts off the teams and companies that would build
  on the library, and it doesn't govern the data a service hands out.
- **Leave the service as it was.** Rejected. Others would keep building paid products on a
  service the maintainer pays to run, with no say for him.
- **Choose the app's license now.** Deferred to M9.0. No app code exists yet, and the architecture
  chosen there decides which parts are app and which are engine.

**Consequences.** VISION V14 points here: open source for the engine, services under their own
terms, the app's license set in M9.0. M9.0's roadmap entry lists the app's license among what its
record decides. THIRD-PARTY-NOTICES.md notes that the recorded API answers in hpr's tests keep CC
BY 4.0 while the live API's data follows the new terms. Motor Finder's terms and its list of
allowed sites change in its own repository. hpr's built-in credit line for the stock data still
names CC BY 4.0; [#1097][i1097] updates it, with the command reference and the stock guide
that quote it. Release 0.1.0 prints the old line, and that can't change.
