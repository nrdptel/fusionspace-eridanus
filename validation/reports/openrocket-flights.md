# hpr against OpenRocket 24.12's flights of the public designs

Written by `cargo xtask ork-flights` ([M2.2d2][m2-2d2], hpr's flights against OpenRocket's; decision [ADR-069][adr-069]) from OpenRocket 24.12's calm-air flights in `validation/fixtures/ork/openrocket-flights.json` ([M2.2d1][m2-2d1]). OR is OpenRocket. Each metric is taken by the definition hpr holds for OpenRocket 24.12: the apogee is the highest point above the launch position; the largest speed is the peak speed (hpr's up to its apogee, since the flight compared for the climb flies no parachute: its descent is compared apart, under *Descents*); the margin, in calibres, is OpenRocket's stability column at its rod-clearance step, which hpr takes at that step's time and Mach number with the air along the axis. Differences (Δ) are hpr less OpenRocket. Motors are OpenRocket's configuration names, one bracketed group per stage. The explanation is on the [documentation site][site].

[m2-2d2]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-2d2
[m2-2d1]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-2d1
[m2-2e1]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-2e1
[adr-069]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/DECISIONS.md#adr-069-hprs-flights-of-the-public-designs-against-openrockets-2026-09-25
[adr-070]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/DECISIONS.md#adr-070-m22e-split-mass-and-centre-of-mass-first-then-the-corpus-2026-09-25
[site]: https://hpr.fusionspace.co/format/ork.html#hprs-flights-against-openrockets

- configurations flown: 54 (3 the record holds are not flown by hpr); apogee more than 5% from OpenRocket's: 8
- fastest: Dual parachute deployment [J570W-P], OpenRocket's largest Mach number 1.147
- apogee, hpr's own drag coefficient: 1 scored, median +6.95%, mean absolute 6.95%, from +6.95% to +6.95%
- apogee, no named cause: 36 scored, median -0.18%, mean absolute 1.03%, from -4.34% to +1.95%
- apogee, reference parachute open before apogee: 15 scored, median +0.10%, mean absolute 5.17%, from -1.24% to +13.80%
- apogee, the rocket turns over before apogee in both tools: 1 scored, median -37.93%, mean absolute 37.93%, from -37.93% to -37.93%
- largest speed, hpr's own drag coefficient: 1 scored, median +6.40%, mean absolute 6.40%, from +6.40% to +6.40%
- largest speed, no named cause: 51 scored, median +0.46%, mean absolute 0.73%, from -0.69% to +6.10%
- largest speed, the rocket turns over before apogee in both tools: 1 scored, median -15.76%, mean absolute 15.76%, from -15.76% to -15.76%
- margin at rod clearance, no named cause: 53 scored, median -0.0003 cal, mean absolute 0.0370 cal, from -1.0768 cal to +0.0764 cal
- apogee against OpenRocket's own flights with the named causes removed, over the flights with a named cause, each counting its largest difference from them: 15 scored, median -0.72%, mean absolute 1.73%, from -1.37% to +8.92%
- apogees more than 5% off: 8, 6 with their named causes sized; within 5% of every flight of OpenRocket's without the causes: 4 of 8
- of those still more than 5% off with their causes removed, within 5% of OpenRocket's flight with nothing deployed when hpr flies OpenRocket's drag, and so put down to the drag coefficient: 2
- apogees more than 5% off sized instead by hpr flying OpenRocket's drag, within 5% on it, and so put down to hpr's own drag coefficient: 1

hpr's mass and center of mass less OpenRocket's ([M2.2e1][m2-2e1], decision [ADR-070][adr-070]), over the flights not aborted: the masses in per cent of OpenRocket's, the center of mass in OpenRocket's calibres, positive when hpr's is further aft, which shortens the margin by as much.

- mass at launch: 53 compared, median +0.008%, mean absolute 0.024%, from -0.016% to +0.209%
- mass at rod clearance: 53 compared, median +0.012%, mean absolute 0.186%, from -0.030% to +3.257%
- center of mass at rod clearance: 53 compared, median +0.0007 cal, mean absolute 0.0059 cal, from -0.0028 cal to +0.0624 cal

| design | motors | apogee OR (m) | hpr (m) | Δ | max speed OR (m/s) | hpr (m/s) | Δ | max Mach OR | margin OR (cal) | hpr (cal) | Δ (cal) |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 3D printable nose cone and fins | [A8-3] | 39.0 | 39.0 | -0.06% | 24.85 | 24.89 | +0.17% | 0.073 | 1.139 | 1.139 | -0.0006 |
| 3D printable nose cone and fins | [B6-4] | 112.9 | 112.5 | -0.32% (chute 0.10 s early) | 48.64 | 48.77 | +0.26% | 0.143 | 1.071 | 1.071 | -0.0006 |
| 3D printable nose cone and fins | [C6-3] | 244.5 | 274.2 | +12.18% (chute 2.59 s early) | 83.29 | 84.00 | +0.85% | 0.245 | 0.903 | 0.903 | -0.0003 |
| 3D printable nose cone and fins | [C6-5] | 274.0 | 274.2 | +0.10% (chute 0.59 s early) | 83.29 | 84.00 | +0.85% | 0.245 | 0.903 | 0.903 | -0.0003 |
| 3D printable nose cone and fins | [C6-7] | 274.7 | 274.2 | -0.16% | 83.29 | 84.00 | +0.85% | 0.245 | 0.903 | 0.903 | -0.0003 |
| A simple model rocket | [A8-3] | 51.1 | 50.9 | -0.27% | 29.34 | 29.39 | +0.18% | 0.086 | 2.982 | 2.981 | -0.0008 |
| A simple model rocket | [B4-4] | 136.4 | 135.5 | -0.64% (chute 0.37 s early) | 53.35 | 53.49 | +0.27% | 0.157 | 2.798 | 2.797 | -0.0008 |
| A simple model rocket | [C6-3] | 280.2 | 318.9 | +13.80% (chute 2.99 s early) | 95.43 | 96.18 | +0.79% | 0.281 | 2.492 | 2.492 | -0.0007 |
| A simple model rocket | [C6-5] | 319.1 | 318.9 | -0.08% (chute 0.99 s early) | 95.43 | 96.18 | +0.79% | 0.281 | 2.492 | 2.492 | -0.0007 |
| A simple model rocket | [C6-7] | 322.4 | 318.9 | -1.07% | 95.43 | 96.18 | +0.79% | 0.281 | 2.492 | 2.492 | -0.0007 |
| ARC payload rocket | [None; F50T-9] | 452.2 | 457.0 | +1.05% | 150.32 | 151.78 | +0.97% | 0.442 | 2.672 | 2.673 | +0.0007 |
| Airstart timing | [3× I211W-P, K550W-P] | 1317.5 | 1329.9 | +0.94% | 186.35 | 187.87 | +0.81% | 0.549 | 1.516 | 1.515 | -0.0010 |
| Airstart timing | Airstart @2s | 1296.0 | 1306.9 | +0.84% | 177.64 | 178.91 | +0.72% | 0.524 | 1.496 | 1.495 | -0.0011 |
| Airstart timing | Airstart @1s | 1292.5 | 1305.8 | +1.03% | 190.14 | 191.59 | +0.76% | 0.560 | 1.496 | 1.495 | -0.0011 |
| Airstart timing | airstart @4s | 1303.7 | 1311.4 | +0.59% | 155.76 | 156.56 | +0.51% | 0.460 | 1.496 | 1.495 | -0.0011 |
| Airstart timing | airstart @6s | 1274.2 | 1280.3 | +0.48% | 136.55 | 137.18 | +0.46% | 0.404 | 1.496 | 1.495 | -0.0011 |
| Base drag hack (short-wide) | [C11-5] | 98.8 | 100.7 | +1.95% | 47.25 | 47.35 | +0.23% | 0.139 | 1.044 | 1.048 | +0.0037 |
| Base drag hack (short-wide) | [D12-3] | 199.2 | 225.0 | +12.96% (chute 1.80 s early) (-0.10% on OpenRocket's drag) (+5.84% with the whole base under power) | 73.82 | 75.71 | +2.56% (-0.13% on OpenRocket's drag) (+2.56% with the whole base under power) | 0.217 | 1.012 | 1.016 | +0.0036 |
| Base drag hack (short-wide) | [E12-4] | 316.3 | 350.5 | +10.79% (chute 1.24 s early) (-0.03% on OpenRocket's drag) (+8.92% with the whole base under power) | 91.55 | 97.13 | +6.10% (-0.08% on OpenRocket's drag) (+6.10% with the whole base under power) | 0.269 | 0.996 | 1.000 | +0.0037 |
| Chute release | [G40W-7] | 310.0 | 306.6 | -1.10% | 72.18 | 72.06 | -0.17% | 0.212 | 5.521 | 5.520 | -0.0014 |
| Chute release | [G80T-10] | 490.3 | 481.3 | -1.83% | 106.40 | 106.23 | -0.16% | 0.313 | 5.480 | 5.481 | +0.0014 |
| Clustered motors | [4× A8-3] | 58.2 | 58.0 | -0.35% | 32.47 | 32.54 | +0.20% | 0.095 | 1.587 | 1.590 | +0.0032 |
| Clustered motors | [4× B4-4] | 143.0 | 141.9 | -0.71% (chute 0.37 s early) | 57.76 | 57.94 | +0.31% | 0.170 | 1.418 | 1.421 | +0.0031 |
| Clustered motors | [4× C6-3] | 280.0 | 306.3 | +9.43% (chute 2.59 s early) | 98.59 | 99.50 | +0.92% | 0.290 | 1.134 | 1.138 | +0.0033 |
| Clustered motors | [4× C6-5] | 308.0 | 306.3 | -0.55% (chute 0.59 s early) | 98.59 | 99.50 | +0.92% | 0.290 | 1.134 | 1.138 | +0.0033 |
| Clustered motors | [4× C6-7] | 308.6 | 306.3 | -0.72% | 98.59 | 99.50 | +0.92% | 0.290 | 1.134 | 1.138 | +0.0033 |
| Deployable payload | [None; A8-3] | 32.2 | 32.2 | +0.09% | 21.93 | 21.97 | +0.19% | 0.064 | 4.798 | 4.802 | +0.0041 |
| Deployable payload | [None; B4-4] | 96.7 | 96.5 | -0.20% | 41.10 | 41.20 | +0.24% | 0.121 | 4.572 | 4.576 | +0.0038 |
| Deployable payload | [None; C6-3] | 233.2 | 265.3 | +13.75% (chute 2.12 s early) | 77.42 | 77.92 | +0.64% | 0.228 | 4.186 | 4.191 | +0.0045 |
| Deployable payload | [None; C6-5] | 265.3 | 265.3 | +0.00% (chute 0.69 s early) | 77.42 | 77.92 | +0.64% | 0.228 | 4.186 | 4.191 | +0.0045 |
| Deployable payload | [None; C6-7] | 266.6 | 265.3 | -0.48% | 77.42 | 77.92 | +0.64% | 0.228 | 4.186 | 4.191 | +0.0045 |
| Dual parachute deployment | [H669N-P] | 593.1 | 579.9 | -2.24% | 135.42 | 135.44 | +0.02% | 0.398 | 4.460 | 4.446 | -0.0134 |
| Dual parachute deployment | [H242T-P] | 698.4 | 684.9 | -1.93% | 140.98 | 141.07 | +0.06% | 0.415 | 4.317 | 4.306 | -0.0114 |
| Dual parachute deployment | [J570W-P] | 2224.7 | 2128.2 | -4.34% | 388.59 | 385.90 | -0.69% | 1.147 | 3.276 | 3.266 | -0.0097 |
| Dual parachute deployment | [H999N-P] | 897.8 | 871.7 | -2.91% | 190.83 | 190.91 | +0.04% | 0.561 | 4.206 | 4.193 | -0.0133 |
| Dual parachute deployment | [I1299N-P] | 1159.0 | 1120.7 | -3.30% | 242.76 | 242.89 | +0.06% | 0.714 | 3.934 | 3.921 | -0.0130 |
| Dual parachute deployment | [G64W-P] | 227.4 | 226.0 | -0.63% | 58.54 | 58.48 | -0.11% | 0.172 | 4.877 | 4.862 | -0.0151 |
| Parallel booster staging | [I115W-10; 2× E12-0] | 1123.6 | 1137.5 | +1.23% | 210.91 | 217.75 | +3.25% | 0.622 | 2.462 | 2.461 | -0.0013 |
| Parallel booster staging | [I115W-10; None] | 851.8 | 857.6 | +0.68% | 188.89 | 191.84 | +1.56% | 0.557 | 2.910 | 2.907 | -0.0025 |
| Pods--airframes and winglets | [A8-3] | 32.3 | 32.3 | -0.13% | 24.92 | 24.91 | -0.02% | 0.073 | 2.876 | 2.952 | +0.0764 |
| Pods--airframes and winglets | [B6-4] | 90.5 | 90.7 | +0.18% | 45.76 | 45.90 | +0.29% | 0.134 | 2.563 | 2.639 | +0.0757 |
| Pods--airframes and winglets | [C6-5] | 199.7 | 200.7 | +0.48% | 72.08 | 72.69 | +0.84% | 0.212 | 2.281 | 2.357 | +0.0758 |
| Pods--airframes and winglets | [C12-6] | 207.5 | 207.7 | +0.10% | 102.77 | 103.21 | +0.43% | 0.302 | 2.267 | 2.342 | +0.0752 |
| Pods--airframes and winglets | [D16-6] | 245.7 | 246.0 | +0.10% | 127.04 | 127.73 | +0.55% | 0.373 | 2.180 | 2.251 | +0.0706 |
| Pods--powered with recovery deployment | [C6-7; 2× A3-4, B6-0] | n/a | 121.3 | withheld | n/a | 80.63 | withheld | 0.228 | n/a | 2.930 | n/a |
| Pods--powered with recovery deployment | [C6-7; B6-0] | 152.7 | 94.8 | -37.93% | 84.71 | 71.36 | -15.76% | 0.249 | 3.672 | 3.742 | +0.0702 |
| Pods--powered with recovery deployment | [C6-5; None] | 131.0 | 130.5 | -0.36% | 50.57 | 50.65 | +0.15% | 0.149 | 4.777 | 4.846 | +0.0695 |
| Pods--powered with recovery deployment | [None; 2× A10-3, B6-4] | 115.6 | 113.5 | -1.86% | 52.67 | 52.73 | +0.12% | 0.155 | 3.857 | 3.927 | +0.0697 |
| Three stage low power rocket | [A8-5; B6-0; B6-0] | 276.7 | 273.3 | -1.24% (chute 0.45 s early) | 78.37 | 78.57 | +0.25% | 0.230 | 2.873 | 2.815 | -0.0580 |
| Three stage low power rocket | [C6-5; B6-0; B6-0] | 495.7 | 500.6 | +0.98% (chute 1.57 s early) | 119.69 | 121.51 | +1.51% | 0.353 | 2.860 | 2.804 | -0.0561 |
| Three stage low power rocket | [C6-7; C6-0; C6-0] | 689.5 | 679.4 | -1.48% | 129.85 | 131.97 | +1.64% | 0.383 | 2.339 | 2.301 | -0.0387 |
| Tube fin rocket | [D12-7] | 282.8 | 302.5 | +6.95% (+0.03% on OpenRocket's drag) (+4.61% with the whole base under power) | 119.42 | 127.07 | +6.40% (+0.03% on OpenRocket's drag) (+3.01% with the whole base under power) | 0.351 | 1.871 | 0.795 | -1.0768 |
| Two stage high power rocket | [H148R-0; H148R-0] | 678.5 | 666.3 | -1.79% | 158.96 | 158.10 | -0.54% | 0.468 | 2.173 | 2.171 | -0.0028 |
| Two stage high power rocket | [I59WN-P; I357T-14] | 1384.2 | 1382.4 | -0.13% | 175.88 | 175.81 | -0.04% | 0.519 | 2.166 | 2.167 | +0.0004 |

*Chute s early*: OpenRocket's parachute opened that long before the apogee of the same flight with nothing deployed, which the record also holds; the flight compared for the climb flies no parachute.

*On OpenRocket's drag*: the same flight by hpr on OpenRocket's drag coefficient along OpenRocket's own flight (power on while a motor burns, power off after). *With the whole base under power*: hpr's own drag, but with OpenRocket's base drag under power, the whole base's while a motor burns, where hpr takes the motor's cross-section off it. hpr flies both for an apogee more than 5% off with no other named cause. Within 5% on OpenRocket's drag, the metric's cause is *hpr's own drag coefficient*: the net gap is in the drag, though parts of it could cancel, and it does not say which drag is right, so each such flight has a written breakdown ([ADR-097][adr-097]). hpr flies both too for an apogee more than 5% off whose early parachute, held, still leaves more than 5% ([ADR-167][adr-167]); both fly nothing deployed, so that flight's apogee brackets are against OpenRocket's flight with nothing deployed, and they name no cause: the table of named causes below sizes what the parachute leaves with them.

[adr-097]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/DECISIONS.md#adr-097-a-cause-in-the-drag-sized-by-hpr-flying-openrockets-drag-2026-09-28
[adr-167]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0167-a-part-s-drag-override-as-openrocket-flies-it.md

The named causes, sized ([M2.2e4][m2-2e4], decision [ADR-073][adr-073]). OpenRocket flew each flight with a named cause again without it. *Nothing deployed*: the same flight with no parachute opening. *Drag settings cleared too*: also with every stated drag coefficient cleared, so each part has the drag of its shape, for a design whose rocket states its own coefficient, which hpr does not apply; a part's and a stage's fly ([ADR-167][adr-167-sized]). Each Δ is hpr's apogee less that flight's, in per cent of it. *Within 5% after*, for an apogee more than 5% off: whether OpenRocket's flight with all its causes taken out is within 5% of hpr's. *hpr on OR's drag, nothing deployed*: where an early parachute, held, still leaves more than 5%, hpr's apogee flown on OpenRocket's drag coefficient along OpenRocket's own flight, nothing deployed in either, and its Δ from OpenRocket's flight with nothing deployed; within 5%, what the parachute leaves is in the drag coefficient, not which drag is right ([ADR-097][adr-097-sized]).

| design | motors | Δ apogee | chute early (s) | OR, nothing deployed (m) | Δ | OR, drag settings cleared too (m) | Δ | within 5% after | hpr on OR's drag, nothing deployed (m) | Δ |
|---|---|---:|---:|---:|---:|---:|---:|---|---:|---:|
| 3D printable nose cone and fins | [B6-4] | -0.32% | 0.10 | 112.9 | -0.32% | n/a | n/a | n/a | n/a | n/a |
| 3D printable nose cone and fins | [C6-3] | +12.18% | 2.59 | 274.7 | -0.16% | n/a | n/a | yes | n/a | n/a |
| 3D printable nose cone and fins | [C6-5] | +0.10% | 0.59 | 274.7 | -0.16% | n/a | n/a | n/a | n/a | n/a |
| A simple model rocket | [B4-4] | -0.64% | 0.37 | 136.6 | -0.78% | n/a | n/a | n/a | n/a | n/a |
| A simple model rocket | [C6-3] | +13.80% | 2.99 | 322.4 | -1.07% | n/a | n/a | yes | n/a | n/a |
| A simple model rocket | [C6-5] | -0.08% | 0.99 | 322.4 | -1.07% | n/a | n/a | n/a | n/a | n/a |
| Base drag hack (short-wide) | [D12-3] | +12.96% | 1.80 | 212.6 | +5.84% | n/a | n/a | no | 212.4 | -0.10% |
| Base drag hack (short-wide) | [E12-4] | +10.79% | 1.24 | 321.8 | +8.92% | n/a | n/a | no | 321.6 | -0.03% |
| Clustered motors | [4× B4-4] | -0.71% | 0.37 | 143.1 | -0.79% | n/a | n/a | n/a | n/a | n/a |
| Clustered motors | [4× C6-3] | +9.43% | 2.59 | 308.6 | -0.72% | n/a | n/a | yes | n/a | n/a |
| Clustered motors | [4× C6-5] | -0.55% | 0.59 | 308.6 | -0.72% | n/a | n/a | n/a | n/a | n/a |
| Deployable payload | [None; C6-3] | +13.75% | 2.12 | 258.5 | +2.63% | n/a | n/a | yes | n/a | n/a |
| Deployable payload | [None; C6-5] | +0.00% | 0.69 | 266.6 | -0.48% | n/a | n/a | n/a | n/a | n/a |
| Three stage low power rocket | [A8-5; B6-0; B6-0] | -1.24% | 0.45 | 277.1 | -1.37% | n/a | n/a | n/a | n/a | n/a |
| Three stage low power rocket | [C6-5; B6-0; B6-0] | +0.98% | 1.57 | 505.4 | -0.95% | n/a | n/a | n/a | n/a | n/a |

[m2-2e4]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-2e4
[adr-073]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/DECISIONS.md#adr-073-each-named-cause-sized-by-openrockets-own-flight-without-it-2026-09-25
[adr-097-sized]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/DECISIONS.md#adr-097-a-cause-in-the-drag-sized-by-hpr-flying-openrockets-drag-2026-09-28
[adr-167-sized]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0167-a-part-s-drag-override-as-openrocket-flies-it.md

At the rod-clearance step, the parts of the margin (m from the nose tip, and kg):

| design | motors | CG OR | CG hpr | CP OR | CP hpr | reference OR | reference hpr | mass OR | mass hpr |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 3D printable nose cone and fins | [A8-3] | 0.2323 | 0.2323 | 0.2605 | 0.2605 | 0.0248 | 0.0248 | 0.0721 | 0.0721 |
| 3D printable nose cone and fins | [B6-4] | 0.2340 | 0.2340 | 0.2605 | 0.2605 | 0.0248 | 0.0248 | 0.0739 | 0.0739 |
| 3D printable nose cone and fins | [C6-3] | 0.2381 | 0.2381 | 0.2605 | 0.2605 | 0.0248 | 0.0248 | 0.0787 | 0.0788 |
| 3D printable nose cone and fins | [C6-5] | 0.2381 | 0.2381 | 0.2605 | 0.2605 | 0.0248 | 0.0248 | 0.0787 | 0.0788 |
| 3D printable nose cone and fins | [C6-7] | 0.2381 | 0.2381 | 0.2605 | 0.2605 | 0.0248 | 0.0248 | 0.0787 | 0.0788 |
| A simple model rocket | [A8-3] | 0.2448 | 0.2448 | 0.3193 | 0.3193 | 0.0250 | 0.0250 | 0.0629 | 0.0629 |
| A simple model rocket | [B4-4] | 0.2494 | 0.2494 | 0.3193 | 0.3193 | 0.0250 | 0.0250 | 0.0653 | 0.0653 |
| A simple model rocket | [C6-3] | 0.2570 | 0.2571 | 0.3193 | 0.3193 | 0.0250 | 0.0250 | 0.0696 | 0.0696 |
| A simple model rocket | [C6-5] | 0.2570 | 0.2571 | 0.3193 | 0.3193 | 0.0250 | 0.0250 | 0.0696 | 0.0696 |
| A simple model rocket | [C6-7] | 0.2570 | 0.2571 | 0.3193 | 0.3193 | 0.0250 | 0.0250 | 0.0696 | 0.0696 |
| ARC payload rocket | [None; F50T-9] | 0.6334 | 0.6334 | 0.7839 | 0.7839 | 0.0563 | 0.0563 | 0.3537 | 0.3537 |
| Airstart timing | [3× I211W-P, K550W-P] | 1.2437 | 1.2441 | 1.5390 | 1.5393 | 0.1949 | 0.1949 | 12.6233 | 12.6236 |
| Airstart timing | Airstart @2s | 1.2475 | 1.2479 | 1.5390 | 1.5392 | 0.1949 | 0.1949 | 12.6610 | 12.6614 |
| Airstart timing | Airstart @1s | 1.2475 | 1.2479 | 1.5390 | 1.5392 | 0.1949 | 0.1949 | 12.6610 | 12.6614 |
| Airstart timing | airstart @4s | 1.2475 | 1.2479 | 1.5390 | 1.5392 | 0.1949 | 0.1949 | 12.6610 | 12.6614 |
| Airstart timing | airstart @6s | 1.2475 | 1.2479 | 1.5390 | 1.5392 | 0.1949 | 0.1949 | 12.6610 | 12.6614 |
| Base drag hack (short-wide) | [C11-5] | 0.0841 | 0.0839 | 0.1663 | 0.1663 | 0.0787 | 0.0787 | 0.1521 | 0.1525 |
| Base drag hack (short-wide) | [D12-3] | 0.0866 | 0.0864 | 0.1663 | 0.1663 | 0.0787 | 0.0787 | 0.1592 | 0.1595 |
| Base drag hack (short-wide) | [E12-4] | 0.0878 | 0.0876 | 0.1663 | 0.1663 | 0.0787 | 0.0787 | 0.1751 | 0.1754 |
| Chute release | [G40W-7] | 0.4821 | 0.4822 | 0.8327 | 0.8327 | 0.0635 | 0.0635 | 1.0215 | 1.0217 |
| Chute release | [G80T-10] | 0.4848 | 0.4847 | 0.8328 | 0.8328 | 0.0635 | 0.0635 | 1.0266 | 1.0269 |
| Clustered motors | [4× A8-3] | 0.4338 | 0.4337 | 0.5211 | 0.5212 | 0.0550 | 0.0550 | 0.2292 | 0.2293 |
| Clustered motors | [4× B4-4] | 0.4431 | 0.4430 | 0.5211 | 0.5212 | 0.0550 | 0.0550 | 0.2385 | 0.2386 |
| Clustered motors | [4× C6-3] | 0.4587 | 0.4586 | 0.5211 | 0.5212 | 0.0550 | 0.0550 | 0.2559 | 0.2559 |
| Clustered motors | [4× C6-5] | 0.4587 | 0.4586 | 0.5211 | 0.5212 | 0.0550 | 0.0550 | 0.2559 | 0.2559 |
| Clustered motors | [4× C6-7] | 0.4587 | 0.4586 | 0.5211 | 0.5212 | 0.0550 | 0.0550 | 0.2559 | 0.2559 |
| Deployable payload | [None; A8-3] | 0.2753 | 0.2752 | 0.3952 | 0.3953 | 0.0250 | 0.0250 | 0.0799 | 0.0799 |
| Deployable payload | [None; B4-4] | 0.2809 | 0.2809 | 0.3952 | 0.3953 | 0.0250 | 0.0250 | 0.0823 | 0.0823 |
| Deployable payload | [None; C6-3] | 0.2906 | 0.2905 | 0.3952 | 0.3953 | 0.0250 | 0.0250 | 0.0866 | 0.0866 |
| Deployable payload | [None; C6-5] | 0.2906 | 0.2905 | 0.3952 | 0.3953 | 0.0250 | 0.0250 | 0.0866 | 0.0866 |
| Deployable payload | [None; C6-7] | 0.2906 | 0.2905 | 0.3952 | 0.3953 | 0.0250 | 0.0250 | 0.0866 | 0.0866 |
| Dual parachute deployment | [H669N-P] | 0.9513 | 0.9521 | 1.2039 | 1.2039 | 0.0566 | 0.0566 | 1.5751 | 1.5750 |
| Dual parachute deployment | [H242T-P] | 0.9592 | 0.9599 | 1.2037 | 1.2037 | 0.0566 | 0.0566 | 1.6019 | 1.6019 |
| Dual parachute deployment | [J570W-P] | 1.0184 | 1.0190 | 1.2039 | 1.2040 | 0.0566 | 0.0566 | 2.1821 | 2.1818 |
| Dual parachute deployment | [H999N-P] | 0.9658 | 0.9666 | 1.2040 | 1.2041 | 0.0566 | 0.0566 | 1.6422 | 1.6421 |
| Dual parachute deployment | [I1299N-P] | 0.9813 | 0.9821 | 1.2041 | 1.2042 | 0.0566 | 0.0566 | 1.7253 | 1.7253 |
| Dual parachute deployment | [G64W-P] | 0.9274 | 0.9283 | 1.2037 | 1.2037 | 0.0566 | 0.0566 | 1.4940 | 1.4940 |
| Parallel booster staging | [I115W-10; 2× E12-0] | 0.8363 | 0.8364 | 0.9776 | 0.9776 | 0.0574 | 0.0574 | 1.2902 | 1.2907 |
| Parallel booster staging | [I115W-10; None] | 0.8106 | 0.8107 | 0.9776 | 0.9776 | 0.0574 | 0.0574 | 1.1774 | 1.1781 |
| Pods--airframes and winglets | [A8-3] | 0.2453 | 0.2453 | 0.3421 | 0.3446 | 0.0337 | 0.0337 | 0.0703 | 0.0703 |
| Pods--airframes and winglets | [B6-4] | 0.2558 | 0.2558 | 0.3421 | 0.3446 | 0.0337 | 0.0337 | 0.0750 | 0.0750 |
| Pods--airframes and winglets | [C6-5] | 0.2653 | 0.2653 | 0.3421 | 0.3446 | 0.0337 | 0.0337 | 0.0799 | 0.0799 |
| Pods--airframes and winglets | [C12-6] | 0.2658 | 0.2658 | 0.3421 | 0.3446 | 0.0337 | 0.0337 | 0.0808 | 0.0808 |
| Pods--airframes and winglets | [D16-6] | 0.2687 | 0.2689 | 0.3421 | 0.3446 | 0.0337 | 0.0337 | 0.0826 | 0.0826 |
| Pods--powered with recovery deployment | [C6-7; 2× A3-4, B6-0] | 0.5689 | 0.5690 | 0.6663 | 0.6687 | 0.0340 | 0.0340 | 0.1464 | 0.1464 |
| Pods--powered with recovery deployment | [C6-7; B6-0] | 0.5413 | 0.5413 | 0.6663 | 0.6687 | 0.0340 | 0.0340 | 0.1298 | 0.1298 |
| Pods--powered with recovery deployment | [C6-5; None] | 0.5037 | 0.5037 | 0.6663 | 0.6687 | 0.0340 | 0.0340 | 0.1117 | 0.1117 |
| Pods--powered with recovery deployment | [None; 2× A10-3, B6-4] | 0.5350 | 0.5350 | 0.6663 | 0.6687 | 0.0340 | 0.0340 | 0.1169 | 0.1169 |
| Three stage low power rocket | [A8-5; B6-0; B6-0] | 0.3596 | 0.3612 | 0.4314 | 0.4316 | 0.0250 | 0.0250 | 0.1230 | 0.1253 |
| Three stage low power rocket | [C6-5; B6-0; B6-0] | 0.3599 | 0.3614 | 0.4314 | 0.4316 | 0.0250 | 0.0250 | 0.1296 | 0.1320 |
| Three stage low power rocket | [C6-7; C6-0; C6-0] | 0.3730 | 0.3740 | 0.4314 | 0.4316 | 0.0250 | 0.0250 | 0.1423 | 0.1470 |
| Tube fin rocket | [D12-7] | 0.4503 | 0.4503 | 0.4966 | 0.4700 | 0.0248 | 0.0248 | 0.0744 | 0.0744 |
| Two stage high power rocket | [H148R-0; H148R-0] | 1.3194 | 1.3198 | 1.5403 | 1.5403 | 0.1016 | 0.1016 | 2.4920 | 2.5332 |
| Two stage high power rocket | [I59WN-P; I357T-14] | 1.3203 | 1.3203 | 1.5403 | 1.5404 | 0.1016 | 0.1016 | 2.7492 | 2.7492 |

## Descents

Each configuration flown again with the file's parachutes and streamers, as `hpr::ork::recovery` maps them and OpenRocket flies them (decision [ADR-153][adr-153]): the drag of the open devices only, each opening at once. A flight hpr separates is compared by the part that keeps the nose, the branch OpenRocket's record keeps; one OpenRocket separates where hpr does not is not compared. Each deployment is matched to OpenRocket's in time order; the landing is where hpr's center of mass comes back through the height it started at, OpenRocket's zero. The targets, set before measuring: each deployment within 0.1 s of OpenRocket's or 2% of its time, whichever is larger; the landing speed and the flight time within 5%. A deployment timed from the apogee or a height carries the apogee's difference in it, and OpenRocket opens a device set to a height at the end of its step, below that height: the *left* column is the difference less those two, computed from the record and hpr's flight. Times are seconds from launch, OpenRocket's then hpr's.

[adr-153]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0153-a-orks-recovery-flown-as-openrocket-flies-it.md

- descents compared: 53; not compared: OpenRocket's flight is aborted or does not land: 1
- 51 with no named cause for the apogee but an early parachute: landing speed median +0.01%, mean absolute 0.01%, from +0.00% to +0.12%; flight time median -0.02%, mean absolute 0.89%, from -1.57% to +7.07%; every deployment within its target in 43; meeting every target 42, and 50 once each deployment that misses is held by what is left of it
- 2 with a named cause for the apogee: landing speed median +0.00%, mean absolute 0.00%, from +0.00% to +0.00%; flight time median -23.43%, mean absolute 29.21%, from -52.64% to +5.78%; every deployment within its target in 2; meeting every target 0, and 0 once each deployment that misses is held by what is left of it

| design | motors | deployments, OR / hpr (s) | left (s) | landing speed OR (m/s) | hpr | Δ | flight time OR (s) | hpr | Δ | targets |
|---|---|---|---|---:|---:|---:|---:|---:|---:|---|
| 3D printable nose cone and fins | [A8-3] | 3.73 / 3.73 | – | 4.32 | 4.32 | +0.01% | 12.2 | 12.2 | -0.35% | met |
| 3D printable nose cone and fins | [B6-4] | 4.86 / 4.86 | – | 4.31 | 4.31 | +0.01% | 31.3 | 31.3 | -0.18% | met |
| 3D printable nose cone and fins | [C6-3] | 4.86 / 4.86 | – | 4.30 | 4.30 | +0.01% | 62.2 | 62.4 | +0.28% | met |
| 3D printable nose cone and fins | [C6-5] | 6.86 / 6.86 | – | 4.30 | 4.30 | +0.01% | 70.8 | 70.8 | -0.03% | met |
| 3D printable nose cone and fins | [C6-7] | 8.86 / 8.86 | – | 4.30 | 4.30 | +0.01% | 69.9 | 69.7 | -0.31% | met |
| A simple model rocket | [A8-3] | 3.73 / 3.73 | – | 4.16 | 4.16 | +0.01% | 16.0 | 16.0 | -0.06% | met |
| A simple model rocket | [B4-4] | 5.03 / 5.03 | – | 4.16 | 4.16 | +0.01% | 38.3 | 38.1 | -0.51% | met |
| A simple model rocket | [C6-3] | 4.86 / 4.86 | – | 4.14 | 4.14 | +0.01% | 73.0 | 72.8 | -0.20% | met |
| A simple model rocket | [C6-5] | 6.86 / 6.86 | – | 4.14 | 4.14 | +0.01% | 84.1 | 83.5 | -0.71% | met |
| A simple model rocket | [C6-7] | 8.86 / 8.86 | – | 4.14 | 4.14 | +0.01% | 84.8 | 83.7 | -1.28% | met |
| ARC payload rocket | [None; F50T-9] | 10.43 / 10.43 | – | 4.27 | 4.27 | +0.00% | 110.1 | 110.5 | +0.32% | met |
| Airstart timing | [3× I211W-P, K550W-P] | 15.66 / 15.68; 58.74 / 58.88 | -0.00; +0.02 | 4.95 | 4.95 | +0.01% | 92.5 | 94.8 | +2.51% | met |
| Airstart timing | Airstart @2s | 16.54 / 16.57; 58.60 / 58.96 | -0.00; +0.06 | 4.95 | 4.95 | +0.01% | 93.9 | 94.9 | +1.06% | met |
| Airstart timing | Airstart @1s | 16.01 / 16.05; 58.06 / 58.40 | -0.00; +0.05 | 4.95 | 4.95 | +0.01% | 92.7 | 94.3 | +1.76% | met |
| Airstart timing | airstart @4s | 17.76 / 17.76; 59.89 / 60.31 | -0.00; +0.03 | 4.95 | 4.95 | +0.01% | 96.2 | 96.2 | +0.08% | met |
| Airstart timing | airstart @6s | 18.91 / 18.91; 60.05 / 60.36 | -0.00; +0.03 | 4.95 | 4.95 | +0.01% | 96.1 | 96.3 | +0.21% | met |
| Base drag hack (short-wide) | [C11-5] | 5.81 / 5.81 | – | 4.22 | 4.22 | +0.11% | 27.5 | 27.9 | +1.72% | met |
| Base drag hack (short-wide) | [D12-3] | 4.65 / 4.65 | – | 4.19 | 4.20 | +0.12% | 52.7 | 54.9 | +4.11% | met |
| Base drag hack (short-wide) | [E12-4] | 6.44 / 6.44 | – | 4.23 | 4.23 | +0.11% | 81.4 | 87.2 | +7.07% | missed |
| Chute release | [G40W-7] | 9.30 / 9.30; 20.51 / 20.15 | –; +0.07 | 4.75 | 4.75 | +0.02% | 51.8 | 51.7 | -0.27% | met |
| Chute release | [G80T-10] | 11.81 / 11.81; 34.31 / 33.40 (misses) | –; -0.16 | 4.74 | 4.74 | +0.02% | 66.0 | 65.0 | -1.55% | met once sized |
| Clustered motors | [4× A8-3] | 3.73 / 3.73 | – | 5.95 | 5.95 | +0.02% | 13.8 | 13.8 | +0.03% | met |
| Clustered motors | [4× B4-4] | 5.03 / 5.03 | – | 5.94 | 5.94 | +0.02% | 29.7 | 29.5 | -0.49% | met |
| Clustered motors | [4× C6-3] | 4.86 / 4.86 | – | 5.91 | 5.91 | +0.02% | 53.1 | 53.1 | +0.03% | met |
| Clustered motors | [4× C6-5] | 6.86 / 6.86 | – | 5.91 | 5.91 | +0.02% | 59.4 | 59.1 | -0.52% | met |
| Clustered motors | [4× C6-7] | 8.86 / 8.86 | – | 5.91 | 5.91 | +0.02% | 58.9 | 58.2 | -1.15% | met |
| Deployable payload | [None; A8-3] | 3.73 / 3.73 | – | 3.95 | 3.95 | +0.00% | 10.8 | 10.8 | -0.05% | met |
| Deployable payload | [None; B4-4] | 5.03 / 5.03 | – | 3.95 | 3.95 | +0.00% | 29.4 | 29.4 | -0.01% | met |
| Deployable payload | [None; C6-3] | 4.86 / 4.86 | – | 3.95 | 3.95 | +0.00% | 64.4 | 64.5 | +0.17% | met |
| Deployable payload | [None; C6-5] | 6.86 / 6.86 | – | 3.95 | 3.95 | +0.00% | 74.2 | 74.1 | -0.21% | met |
| Deployable payload | [None; C6-7] | 8.86 / 8.86 | – | 3.95 | 3.95 | +0.00% | 73.7 | 73.2 | -0.71% | met |
| Dual parachute deployment | [H669N-P] | 10.35 / 10.17; 30.82 / 29.87 (misses) | -0.00; +0.13 | 4.18 | 4.18 | +0.00% | 64.8 | 65.5 | +1.17% | met once sized |
| Dual parachute deployment | [H242T-P] | 11.47 / 11.30; 36.40 / 35.40 (misses) | -0.00; +0.16 | 4.18 | 4.18 | +0.00% | 70.2 | 71.1 | +1.31% | met once sized |
| Dual parachute deployment | [J570W-P] | 17.40 / 16.90 (misses); 98.79 / 94.43 (misses) | -0.00; +0.33 | 4.43 | 4.43 | +0.00% | 130.1 | 128.1 | -1.57% | met once sized |
| Dual parachute deployment | [H999N-P] | 12.20 / 11.98; 44.86 / 43.46 (misses) | -0.00; +0.14 | 4.23 | 4.23 | +0.00% | 79.3 | 78.7 | -0.66% | met once sized |
| Dual parachute deployment | [I1299N-P] | 13.51 / 13.18 (misses); 56.15 / 54.21 (misses) | -0.00; +0.16 | 4.28 | 4.29 | +0.00% | 89.9 | 89.0 | -1.01% | met once sized |
| Dual parachute deployment | [G64W-P] | 7.34 / 7.30; 12.52 / 12.12 (misses) | -0.00; +0.22 | 4.09 | 4.09 | +0.00% | 47.1 | 48.6 | +3.16% | met once sized |
| Parallel booster staging | [I115W-10; 2× E12-0] | 13.51 / 13.51 | – | 5.17 | 5.17 | +0.00% | 224.9 | 227.3 | +1.08% | met |
| Parallel booster staging | [I115W-10; None] | 13.51 / 13.51 | – | 5.40 | 5.40 | +0.00% | 163.4 | 163.8 | +0.27% | met |
| Pods--airframes and winglets | [A8-3] | 3.53 / 3.53 | – | 4.34 | 4.34 | +0.01% | 10.3 | 10.2 | -0.62% | met |
| Pods--airframes and winglets | [B6-4] | 4.86 / 4.86 | – | 4.42 | 4.42 | +0.01% | 25.0 | 25.0 | +0.08% | met |
| Pods--airframes and winglets | [C6-5] | 6.86 / 6.86 | – | 4.41 | 4.41 | +0.01% | 51.4 | 51.6 | +0.36% | met |
| Pods--airframes and winglets | [C12-6] | 6.89 / 6.89 | – | 4.44 | 4.44 | +0.01% | 51.9 | 52.0 | +0.12% | met |
| Pods--airframes and winglets | [D16-6] | 6.84 / 6.84 | – | 4.44 | 4.44 | +0.01% | 61.0 | 61.0 | -0.02% | met |
| Pods--powered with recovery deployment | [C6-7; B6-0] | 9.72 / 9.72 | – | 4.53 | 4.54 | +0.00% | 33.4 | 15.8 | -52.64% | missed |
| Pods--powered with recovery deployment | [C6-5; None] | 6.86 / 6.86 | – | 5.28 | 5.28 | +0.00% | 29.8 | 30.0 | +0.86% | met |
| Pods--powered with recovery deployment | [None; 2× A10-3, B6-4] | 5.35 / 5.35; 5.35 / 5.35 | –; – | 4.55 | 4.55 | +0.00% | 30.2 | 29.9 | -1.02% | met |
| Three stage low power rocket | [A8-5; B6-0; B6-0] | 7.45 / 7.44 | – | 4.16 | 4.16 | +0.01% | 74.1 | 73.3 | -1.13% | met |
| Three stage low power rocket | [C6-5; B6-0; B6-0] | 8.57 / 8.57 | – | 4.13 | 4.13 | +0.01% | 127.9 | 127.1 | -0.56% | met |
| Three stage low power rocket | [C6-7; C6-0; C6-0] | 12.58 / 12.58 | – | 4.13 | 4.13 | +0.01% | 176.7 | 174.2 | -1.41% | met |
| Tube fin rocket | [D12-7] | 8.65 / 8.65 | – | 3.89 | 3.89 | +0.00% | 75.5 | 79.9 | +5.78% | missed |
| Two stage high power rocket | [H148R-0; H148R-0] | 10.98 / 10.94; 40.25 / 39.16 (misses) | -0.00; +0.06 | 5.94 | 5.94 | +0.00% | 63.9 | 64.2 | +0.32% | met once sized |
| Two stage high power rocket | [I59WN-P; I357T-14] | 16.26 / 16.21; 79.95 / 79.47 | -0.00; +0.07 | 6.06 | 6.06 | +0.00% | 103.2 | 103.9 | +0.73% | met |

## Separations with nothing left to burn

A configuration whose stack comes apart with nothing ahead of the split left to burn, such as a payload dropped at its booster's ejection charge, flies its climb whole in the first table. Flown again with its devices, each part flies on from the split as a point with only its own devices' drag, as OpenRocket flies a descent, and only when the part that keeps the nose has a device of its own open by then (decision [ADR-165][adr-165]). OpenRocket's record holds that part's branch, so its apogee is compared here, from where each tool's flight starts, and its descent in the table above. *Coasting*: the same part flown on from the split with no drag until its own apogee, the flight `hpr sim` refuses, against OpenRocket's flight of the configuration with no device deployed, where the part flies on its own airframe; shown where OpenRocket's split came before its apogee.

[adr-165]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0165-an-unpowered-separation-in-hpr-sim.md

| design | motors | split, OR / hpr (s) | before OR's apogee | apogee OR (m) | hpr | Δ | coasting: OR, nothing deployed (m) | hpr, no drag | Δ |
|---|---|---|---|---:|---:|---:|---:|---:|---:|
| ARC payload rocket | [None; F50T-9] | 10.43 / 10.43 | no | 452.2 | 457.0 | +1.05% | – | – | – |
| Deployable payload | [None; A8-3] | 3.73 / 3.73 | no | 32.2 | 32.2 | +0.09% | – | – | – |
| Deployable payload | [None; B4-4] | 5.03 / 5.03 | no | 96.7 | 96.5 | -0.20% | – | – | – |
| Deployable payload | [None; C6-3] | 4.86 / 4.86 | yes | 233.2 | 233.3 | +0.04% | 258.5 | 268.8 | +4.00% |
| Deployable payload | [None; C6-5] | 6.86 / 6.86 | yes | 265.3 | 264.4 | -0.32% | 266.6 | 265.4 | -0.43% |
| Deployable payload | [None; C6-7] | 8.86 / 8.86 | no | 266.6 | 265.3 | -0.48% | – | – | – |

## Flights OpenRocket aborted

OpenRocket stops a flight it can't carry on, such as a stage that tumbles while its motor still burns, and its record then has no apogee, so the first table withholds the comparison. Up to the abort the record is a flight like any other: hpr's height above its start and its speed are compared at OpenRocket's separation (OpenRocket's row just after it, and hpr's sample at its separation event, the whole stack's center of mass at the split) and at OpenRocket's last row, and are met within 5% of OpenRocket's with the separation at the same time (decision [ADR-172][adr-172]). *Turns over*: when hpr's angle of attack first passes 90°.

[adr-172]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0172-a-stage-s-first-burnout.md

| design | motors | OR's cause | split, OR / hpr (s) | height at split, OR / hpr (m) | Δ | speed at split, OR / hpr (m/s) | Δ | last row (s) | height, OR / hpr (m) | Δ | speed, OR / hpr (m/s) | Δ | hpr turns over (s) | met |
|---|---|---|---|---|---:|---|---:|---:|---|---:|---|---:|---:|---|
| Pods--powered with recovery deployment | [C6-7; 2× A3-4, B6-0] | Stage began to tumble under thrust. | 0.86 / 0.86 | 22.44 / 22.18 | -1.17% | 46.08 / 46.06 | -0.04% | 1.81 | 85.0 / 84.0 | -1.18% | 76.8 / 79.9 | +4.02% | 2.04 | met |

## Parts dropped still burning

A stage whose motors sit in several mounts separates at its first burnout, as OpenRocket's does, and drops its other motors still burning (decision [ADR-172][adr-172]). The part flies as a point with no thrust, so its flight leaves out what they had left; the sustainer's flight, compared above, is not touched by it.

- Pods--powered with recovery deployment [C6-7; 2× A3-4, B6-0]: part 1 drops at 0.860 s with 0.509 N·s of its motors' impulse left

Configurations OpenRocket flew that hpr does not fly yet:

| design | motors | why |
|---|---|---|
| demo-payload-separation | [None; F50T-8] | the design's only configuration, which OpenRocket gave a new id: a motor with no curve |
| Simulation extensions | [2800CC172L-L540-P] | a motor with no curve |
| Simulation scripting | [2800CC172L-L540-P] | a motor with no curve |

## Pod probes

Small designs written to test pods ([M1.13c2][m1-13c2]), not counted among the designs above: one airframe on an AeroTech H128W, carrying pods of bodies, fins, a tail cone, winglets or motors, and once with none (`pods-none`, first). *Pods' change* is the apogee's and the margin's change from `pods-none` in each code. `pods-motors-2` flies an H128W in each of its two pods and none in the airframe, with 0.35 kg of nose ballast, not 0.15, so its change is not its pods' alone.

[m1-13c2]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m1-13c2

| probe | apogee OR (m) | hpr (m) | Δ | pods' change OR | hpr | max speed OR (m/s) | hpr (m/s) | Δ | max Mach OR | margin OR (cal) | hpr (cal) | pods' change OR (cal) | hpr (cal) |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| pods-none | 776.1 | 782.4 | +0.81% | +0.00% | +0.00% | 254.00 | 256.78 | +1.09% | 0.747 | 2.757 | 2.759 | +0.0000 | +0.0000 |
| pods-bodies-3 | 663.3 | 666.5 | +0.48% | -14.53% | -14.81% | 226.17 | 227.91 | +0.77% | 0.665 | 2.360 | 2.362 | -0.3970 | -0.3971 |
| pods-fins-2 | 613.7 | 616.7 | +0.49% | -20.93% | -21.18% | 220.78 | 222.39 | +0.73% | 0.650 | 2.653 | 2.654 | -0.1040 | -0.1042 |
| pods-fins-tail-4 | 533.3 | 535.5 | +0.41% | -31.28% | -31.56% | 202.24 | 203.43 | +0.59% | 0.595 | 2.470 | 2.471 | -0.2872 | -0.2875 |
| pods-motors-2 | 876.9 | 881.5 | +0.53% | +12.98% | +12.66% | 274.13 | 277.52 | +1.24% | 0.807 | 2.990 | 2.991 | +0.2326 | +0.2322 |
| pods-winglets-2 | 699.8 | 704.8 | +0.71% | -9.84% | -9.92% | 242.07 | 244.37 | +0.95% | 0.712 | 2.622 | 2.623 | -0.1357 | -0.1359 |

## Tilted-rod probes

The pod probes' airframe with no pods, launched from a 1 m rod tilted from the vertical toward a compass bearing ([M2.2e5][m2-2e5]), in calm air, and not counted among the designs above:

- `pods-none`, first, is the same airframe from OpenRocket's default vertical rod: the control. Its bearing means nothing: from a vertical rod both codes' rockets drift about 0.4 m west, from Earth's rotation.
- *Rod's change* is the apogee's change from `pods-none` in each code.
- *At apogee* is where the rocket is then, meters east and north of where it started, and *bearing* the direction of that place from the pad, clockwise from north: a rod read the wrong way round would send hpr's rocket another way than OpenRocket's.
- *α OR* is the angle of attack OpenRocket's rocket has at its rod-clearance row, which grows with the tilt, where hpr's margin is taken at none; *at OR's α* is hpr's margin at OpenRocket's angle, which says how much of the margins' difference that accounts for.

[m2-2e5]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-2e5

| probe | rod | apogee OR (m) | hpr (m) | Δ | rod's change OR | hpr | max speed OR (m/s) | hpr (m/s) | Δ | α OR (°) | margin OR (cal) | hpr (cal) | at OR's α (cal) | at apogee OR (E, N m) | hpr (E, N m) | bearing OR | hpr |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| pods-none | vertical | 776.1 | 782.4 | +0.81% | +0.00% | +0.00% | 254.00 | 256.78 | +1.09% | 0.000 | 2.757 | 2.759 | 2.759 | -0.4, 0.0 | -0.4, 0.0 | 270.6° | 270.0° |
| rod-10-east | 10° toward 90° (east) | 755.9 | 762.7 | +0.90% | -2.60% | -2.52% | 254.09 | 256.88 | +1.10% | 0.116 | 2.749 | 2.759 | 2.752 | 195.0, 0.0 | 194.3, -0.1 | 90.0° | 90.0° |
| rod-10-southwest | 10° toward 225° (south-west) | 755.9 | 762.5 | +0.87% | -2.60% | -2.54% | 254.10 | 256.88 | +1.09% | 0.116 | 2.749 | 2.759 | 2.752 | -138.5, -138.1 | -138.1, -137.6 | 225.1° | 225.1° |
| rod-20-east | 20° toward 90° (east) | 697.8 | 705.6 | +1.11% | -10.09% | -9.82% | 254.38 | 257.18 | +1.10% | 0.228 | 2.742 | 2.759 | 2.747 | 370.6, 0.1 | 369.6, -0.1 | 90.0° | 90.0° |
| rod-5-north | 5° toward 0° (north) | 771.1 | 777.5 | +0.83% | -0.65% | -0.63% | 254.02 | 256.81 | +1.09% | 0.058 | 2.753 | 2.759 | 2.756 | -0.4, 99.2 | -0.4, 98.5 | 359.8° | 359.8° |
