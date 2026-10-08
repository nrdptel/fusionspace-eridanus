# Known issues and risks

**What this is.** The project's standing caveats about its models and data, kept by the work
that found them. Each caveat with a GitHub issue is tracked there too; the open ones by priority are under
[issues labelled P-critical](https://github.com/nrdptel/fusionspace-eridanus/issues?q=is%3Aopen+label%3AP-critical)
and [P-high](https://github.com/nrdptel/fusionspace-eridanus/issues?q=is%3Aopen+label%3AP-high). The user-facing
account of each model's accuracy is on the site's model pages and *Accuracy*.

- Two M1.2 sources are pinned from third-party mirrors (MIL-F-8785C, WMO-No. 8). Dryden turbulence
  is an aircraft model, unvalidated for rockets, and no flight uses it (#39). Only 32 motor curves
  are bundled (none in class A); the rest wait for M5's cache, whose checks ran on unpinned
  `refs/samples/`. Wall and fin mass may differ from OpenRocket's (M2.2);
- `.CDX1` has no public spec, ERA5 `.nc` may be netCDF4. A new RustSec notice can turn CI red with
  no code change. Barrowman 1966, TIR-33, Galejs, the `.rse` spec and Knacke: never redistribute.
- Aero (M1.5a) is small-angle only; the Recruiter's six fins miss the printed slope by +3.42%
  (ADR-008). Body lift (Jorgensen, M1.8e6) reads 1–16% high where the crossflow is supersonic. The
  normal force misses the tunnel between Mach 0.8 and 1.2; fins off, the body reads 14–38% high from
  Mach 1.5 to 2.96 (M1.8e9's bullet).
- A blunt tip's cap (M1.8e7) is checked only on a sphere-cone (#101); a boattail past 16° is worth
  0.67 to 1.35 calibres of doubt.
- A marched flare (M1.8e18) reads +51.5% and +50.4% at Mach 3.95 and 4.63; a near-flat flare leaves
  the crossing's pole: +0.129% on the tests' rocket, +4.3% on a short shoulder (#108); a step in
  radius takes the body off the method past 2.7e-11 m tube to tube or 1.3e-13 m at a boattail:
  −8.65% to −11.34% (#87).
- Four switches (a step #87, a long flare behind a boattail #120, a tip past 30° #121, a vertical
  tip past the cap) read more stable: −7.0% to −27.5%, 0.29 to 1.03 cal aft at Mach 3; M1.14d, h.
- `.ork` (M3.1): `hpr sim` flies 136 of 170 configurations offline after one fetch, 145 with
  `--accept-design-errors` (M4.5j's report); powered splits, several in turn (ADR-160), unpowered
  before apogee when the part keeping the nose opens a device by the split (ADR-165); recovery flown as OR
  flies it (ADR-153), `hpr sim` too (ADR-159, dropped stages' descents unvalidated, #179);
  parallel stages only on one axial stage (ADR-171); a fin on a nose cone flies with its root along the surface (ADR-166), its
  supersonic tip cone approximate there; tube fins fly, drag likely low (#228); screw heads read
  simpler, warned; supersonic pressure drag twice OR's on `C06` (#222);
- `polished` 2 µm may be 0.5 µm in a newer OR (ADR-061). Pods (ADR-092) fly without pod–body
  interference, a single pod's moments dropped (#213); two motor pod sets refused (#214).
- Drag: against RASAero II's Calisto hpr reads −14.9% to −5.1% supersonic (ADR-030); against
  MIL-HDBK-762 the body reads 6–10% low past Mach 1.6 and high through Mach 1 (#67, #68); against
  the Arcas Robin it reads high at every row (#70, #72, #73); a cylinder's base drag is unmeasured
  past Mach 0.3.
- In wind, a slow rocket's drift rests on body lift: Juno III's apogee drift is 245 m in hpr, 240 to
  194 m over Galejs's `K` 1.0 to 1.5 (oracle corrections, #1196).
- Flight: no tip-off, turbulence or thrust misalignment; small-angle aero at every `α` (a fall with
  no recovery glides tail-first, #241; its ascent peaks unshown, #243); two apogees on a near-flat
  rail (#242). Recovery omits canopy overshoot, opening-load factor, added mass and airframe drag;
  attitude freezes at deployment, streamer pleats unmodeled (+58% on Kidwell's), tumble reads +19%.
- Reports pinned to six decimals or 1e-7 relative; no oracle runs in CI.
- Export: State's Category IV rewrite (RIN 1400-AE73, interim final rule listed for 2026-09) may
  redefine the hobby-rocket exclusion; re-read it when published (`export-control.md`, ADR-192).
