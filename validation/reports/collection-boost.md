# The private collection's logs, cut at the boost (M7.3d4, ADR-326)

Written by `cargo xtask fixture-flights`; do not edit. This is the first step of predicting the [private collection's](fixture-flights.md) apogees from each log cut just after burnout, as the [public flights' report](from-boost.md) does for 7 public flights. It counts which flights have a log the prediction can start from, over the whole collection. It gives no prediction and no error yet, and nothing for either half of the collection, so no number here can have steered a choice of model. The collection's license allows only aggregates, so no flight is named.

**The halves.** A flight is held out (ADR-209 §4) when the SHA-256 of its id followed by `ADR-209` starts with eight bytes below 2⁶⁴ / 3, about a third of the flights (ADR-322). The held-out ids stay private; the SHA-256 of their sorted list is committed in [`held-out.sha256`](held-out.sha256) before any half's number is reported, so the split can't be changed after the numbers are seen.

**Each flight's log.** A flight may have several logs, from more than one altimeter or file. Each pinned log is read alone, with no sheet chosen and no companion file. Of those whose height track is a barometer's and gives an apogee, the flight's log is the one whose apogee is nearest the logged apogee the collection states. The library's boost finder then cuts it just after burnout, or says why it can't.

| | flights |
|---|---:|
| in the collection's manifest | 83 |
| left out of the apogee errors by the manifest | 11 |
| no log reads alone | 3 |
| a log read, but none gives an apogee on a barometric track | 5 |
| a log chosen | 64 |
| of those, refused a cut | 32 |
| … a log slower than 8 Hz | 2 |
| … the transonic limit before a cut | 30 |
| of those, cut | 32 |

Of the flights not left out, 241 pinned logs were tried, 199 read alone and 117 gave an apogee on a barometric track. Of the chosen logs, 1 read an apogee more than 2% from the logged one: the nearest log is then another altimeter's, or a reading the analyzer takes differently.

**How far to trust it.** The cut is found on the log as it stands. On the public flights' barometers it can fall inside the burn ([#1307](https://github.com/nrdptel/fusionspace-eridanus/issues/1307)), so a count of cuts is not yet a count of cuts at burnout. The finder refuses a log whose climb reaches 227.7 m/s (747 ft/s), where a barometer's reading may be the airflow's, not the height's. A log that needs a stated column mapping or a companion file to read counts as unread here.
