use std::cell::Cell;

use hpr_aero::{AeroModel, DragConditions, Flow};
use hpr_core::interp::{Extrapolation, Interpolation, Table1D};
use serde_json::json;

use super::*;
use crate::gap_split::split;
use crate::histories::own::OWN_HISTORY_BOUND;
use crate::histories::tests::{Public, public_flights};

fn fly(simulation: &Simulation) -> Result<f64, String> {
    crate::fixture_flights::climbed_m(simulation)
}

/// An edge is in the faster band, and a Mach number that isn't a number is in none.
#[test]
fn a_mach_number_s_band() {
    for (mach, expected) in [
        (0.0, Some(0)),
        (0.799_999_9, Some(0)),
        (0.8, Some(1)),
        (1.0, Some(1)),
        (1.199_999_9, Some(1)),
        (1.2, Some(2)),
        (2.5, Some(2)),
        (f64::INFINITY, Some(2)),
        (f64::NAN, None),
    ] {
        assert_eq!(band(mach), expected, "Mach {mach}");
    }
    const { assert!(BANDS.len() == EDGES.len() + 1 && EDGES[0] < EDGES[1]) };
}

/// Two flat curves no buildup gives: 30 coasting and 20 under thrust, on a circle `diameter_m`
/// across.
fn flat_table(diameter_m: Option<f64>) -> DragTable {
    let flat = |value: f64| {
        Table1D::new(
            vec![0.0, 5.0],
            vec![value, value],
            Interpolation::Linear,
            Extrapolation::Clamp,
        )
        .unwrap()
    };
    let table = DragTable::new(flat(30.0), Some(flat(20.0)));
    match diameter_m {
        Some(diameter_m) => table.with_reference_diameter_m(diameter_m).unwrap(),
        None => table,
    }
}

/// In a band swapped the coefficient is the table's, as a rocket flown on the table alone reads
/// it: the power-on curve under thrust, the power-off one coasting, put on the rocket's
/// reference area. In every other band it is HPR Sim's own buildup, bit for bit. Each of the
/// eight choices of bands is tried at Mach numbers inside each band and on each edge.
#[test]
fn the_swap_reads_openrockets_table_in_its_bands_only() {
    let flights = public_flights();
    let simulation = flights[0].build().unwrap();
    let layout = &simulation.assembly().layout;
    let own = AeroModel::new(layout).unwrap();
    let area = own.reference_area_m2();
    // A table on a circle of twice the rocket's reference area.
    let diameter_m = (8.0 * area / std::f64::consts::PI).sqrt();
    for table in [flat_table(None), flat_table(Some(diameter_m))] {
        let scale = if table.reference_diameter_m.is_some() {
            2.0
        } else {
            1.0
        };
        let on_table = AeroModel::new(layout)
            .unwrap()
            .with_drag_table(table.clone());
        for choice in 0..8_u8 {
            let openrockets = [choice & 1 != 0, choice & 2 != 0, choice & 4 != 0];
            let swapped = AeroModel::new(layout)
                .unwrap()
                .with_drag_model(BandSwap::new(table.clone(), openrockets));
            for (mach, in_band) in [
                (0.3, 0),
                (0.79, 0),
                (0.8, 1),
                (1.0, 1),
                (1.19, 1),
                (1.2, 2),
                (1.9, 2),
            ] {
                for conditions in [
                    DragConditions::coasting(1.0e7),
                    DragConditions::thrusting(1.0e7, 0.0),
                ] {
                    let drag = |model: &AeroModel| {
                        model
                            .drag(&Flow::new(mach, 0.0, 0.0), &conditions)
                            .unwrap()
                            .zero_lift_coefficient
                    };
                    let got = drag(&swapped);
                    if openrockets[in_band] {
                        let curve = if conditions.thrusting { 20.0 } else { 30.0 };
                        assert_eq!(got, drag(&on_table), "Mach {mach}, {openrockets:?}");
                        assert!((got - scale * curve).abs() < 1e-9, "Mach {mach}: {got}");
                    } else {
                        assert_eq!(got, drag(&own), "Mach {mach}, {openrockets:?}");
                        // The buildup's drag is far from the table's flat curves.
                        assert!(got > 0.0 && got < 10.0, "Mach {mach}: {got}");
                    }
                }
            }
        }
    }
    // A table that goes negative is refused in a band swapped, as on the table alone.
    let negative = Table1D::new(
        vec![0.0, 5.0],
        vec![0.5, -0.5],
        Interpolation::Linear,
        Extrapolation::Clamp,
    )
    .unwrap();
    let swapped = AeroModel::new(layout)
        .unwrap()
        .with_drag_model(BandSwap::new(
            DragTable::new(negative, None),
            [false, false, true],
        ));
    let at = |mach: f64| swapped.drag(&Flow::new(mach, 0.0, 0.0), &DragConditions::coasting(1.0e7));
    assert!(at(4.0).is_err());
    assert_eq!(
        at(1.0).unwrap().zero_lift_coefficient,
        own.drag(&Flow::new(1.0, 0.0, 0.0), &DragConditions::coasting(1.0e7))
            .unwrap()
            .zero_lift_coefficient
    );
}

/// A public flight split against OpenRocket's own flight of it, and then by band.
fn banded_public(public: &Public, flight: &Value) -> (SplitFlight, Banded) {
    let own_m = fly(&public.build().unwrap()).unwrap();
    let openrocket_m = flight["apogee_m"].as_f64().unwrap();
    let split = split(
        &|_air| public.build(),
        &fly,
        flight,
        (own_m, openrocket_m),
        OWN_HISTORY_BOUND,
    )
    .unwrap();
    let banded = super::flight(&|_air| public.build(), &fly, flight, &split, openrocket_m)
        .unwrap()
        .unwrap()
        .unwrap();
    (split.unwrap(), banded)
}

/// What the documentation quotes for the two public designs, per cent of OpenRocket's apogee:
/// the three bands' parts, slowest first, each held within [`PUBLIC_WITHIN`]. `pods-none` stays
/// below Mach 0.8, so its drag part is all in the slowest band; `mach-probe` passes Mach 1.2 and
/// has a part in each. On `pods-none` the three are 0.79%, 0% and 0%; on `mach-probe`, −0.08%,
/// +0.26% and +2.85%, which sum to its drag part of 3.03%.
const PUBLIC_BANDS: [(&str, [f64; 3]); 2] = [
    ("pods-none.ork", [0.79, 0.0, 0.0]),
    ("mach-probe.ork", [-0.08, 0.26, 2.85]),
];
const PUBLIC_WITHIN: f64 = 0.01;

/// The two public designs split by band: the bands sum to the drag part, a band the flight
/// never reaches has a part of exactly zero, and swapped alone on HPR Sim's own thrust and mass
/// a band comes close to its part, since on these designs thrust and mass move the apogee by
/// less than 0.005%.
#[test]
fn the_public_flights_split_by_band() {
    let flights = public_flights();
    assert_eq!(flights.len(), PUBLIC_BANDS.len());
    for (public, (file, expected)) in flights.iter().zip(PUBLIC_BANDS) {
        assert!(public.file.ends_with(file), "{}", public.file);
        let (split, banded) = banded_public(public, &public.flight);
        eprintln!(
            "{file}: bands {:+.3}% {:+.3}% {:+.3}% = drag {:+.3}%; alone {:+.3}% {:+.3}% {:+.3}%; \
             drag first {:+.3}%",
            banded.parts[0],
            banded.parts[1],
            banded.parts[2],
            banded.drag,
            banded.alone[0],
            banded.alone[1],
            banded.alone[2],
            banded.drag_first
        );
        let sum: f64 = banded.parts.iter().sum();
        assert!((sum - banded.drag).abs() < 1e-9, "{file}: {sum}");
        assert_eq!(banded.drag, split.parts.drag);
        assert_eq!(banded.drag_first, split.parts.drag_first);
        for (i, (part, expected)) in banded.parts.iter().zip(expected).enumerate() {
            assert!((part - expected).abs() <= PUBLIC_WITHIN, "{file}: band {i}");
            // A band the flight never reaches: the same flight before and after its swap.
            if expected == 0.0 {
                assert_eq!(*part, 0.0, "{file}: band {i}");
                assert_eq!(banded.alone[i], 0.0, "{file}: band {i}");
            } else {
                assert!(part.abs() > 0.05, "{file}: band {i}");
                assert!((banded.alone[i] - part).abs() < 0.05, "{file}: band {i}");
            }
        }
        let heights = banded.heights_m;
        assert_eq!(heights.own_m, split.apogees.own_m);
        assert_eq!(heights.drag_first_m, split.apogees.drag_first_m);
        assert_eq!(
            heights.openrocket_m,
            public.flight["apogee_m"].as_f64().unwrap()
        );
    }
}

/// With no band swapped the flight is HPR Sim's own, and with every band swapped it is the
/// flight on the table alone, each bit for bit, on a flight that passes through all three bands.
/// Swapping only the bands the flight reaches is the same flight as swapping every band.
#[test]
fn no_band_is_the_own_flight_and_every_band_the_tables() {
    let flights = public_flights();
    for public in &flights {
        let inputs = OpenRocketInputs::of(&public.flight).unwrap().unwrap();
        let with = |openrockets: [bool; 3]| {
            fly(&public
                .build()
                .unwrap()
                .with_drag_model(BandSwap::new(inputs.drag.clone(), openrockets)))
            .unwrap()
        };
        let own = fly(&public.build().unwrap()).unwrap();
        let on_table = fly(&public.build().unwrap().with_drag_table(inputs.drag.clone())).unwrap();
        assert_eq!(with([false, false, false]), own, "{}", public.file);
        assert_eq!(with([true, true, true]), on_table, "{}", public.file);
        assert_ne!(own, on_table, "{}", public.file);
    }
    // `pods-none` stays in the slowest band.
    let public = &flights[0];
    let inputs = OpenRocketInputs::of(&public.flight).unwrap().unwrap();
    let with = |openrockets: [bool; 3]| {
        fly(&public
            .build()
            .unwrap()
            .with_drag_model(BandSwap::new(inputs.drag.clone(), openrockets)))
        .unwrap()
    };
    assert_eq!(with([true, false, false]), with([true, true, true]));
    assert_eq!(with([false, true, true]), with([false, false, false]));
}

/// Each band's part is its own swap, added to the slower ones, and each band alone is flown on
/// HPR Sim's own thrust and mass. OpenRocket's drag is scaled by 1.3 and its thrust by 1.05, so
/// each swap moves the apogee by far more than a slip could hide in, and the flights are flown
/// here one by one: the parts must be theirs, bit for bit.
#[test]
fn each_band_is_its_own_swap() {
    let flights = public_flights();
    let public = &flights[1];
    let mut scaled = public.flight.clone();
    for row in scaled["rows"].as_array_mut().unwrap() {
        for (column, factor) in [(1, 1.05), (4, 1.3)] {
            if let Some(value) = row[column].as_f64() {
                row[column] = json!(factor * value);
            }
        }
    }
    assert_eq!(crate::gap_split::COLUMNS[1], "thrust_n");
    assert_eq!(crate::gap_split::COLUMNS[4], "drag_coefficient");
    let (split, banded) = banded_public(public, &scaled);
    let openrocket_m = scaled["apogee_m"].as_f64().unwrap();
    let inputs = OpenRocketInputs::of(&scaled).unwrap().unwrap();
    let on_histories = |openrockets: [bool; 3]| {
        fly(&public
            .build()
            .unwrap()
            .with_thrust_history(inputs.thrust.clone())
            .unwrap()
            .with_mass_history(inputs.mass.clone())
            .unwrap()
            .with_drag_model(BandSwap::new(inputs.drag.clone(), openrockets)))
        .unwrap()
    };
    let on_own = |openrockets: [bool; 3]| {
        fly(&public
            .build()
            .unwrap()
            .with_drag_model(BandSwap::new(inputs.drag.clone(), openrockets)))
        .unwrap()
    };
    let share = |before_m: f64, after_m: f64| 100.0 * (before_m - after_m) / openrocket_m;
    let (slow_m, slow_mid_m) = (
        on_histories([true, false, false]),
        on_histories([true, true, false]),
    );
    assert_eq!(
        banded.parts,
        [
            share(split.apogees.mass_m, slow_m),
            share(slow_m, slow_mid_m),
            share(slow_mid_m, split.apogees.drag_m),
        ]
    );
    let alone_m = [
        on_own([true, false, false]),
        on_own([false, true, false]),
        on_own([false, false, true]),
    ];
    assert_eq!(banded.heights_m.alone_m, alone_m);
    assert_eq!(banded.alone, alone_m.map(|m| share(split.apogees.own_m, m)));
    eprintln!(
        "scaled {}: bands {:+.2}% {:+.2}% {:+.2}%; alone {:+.2}% {:+.2}% {:+.2}%",
        public.file,
        banded.parts[0],
        banded.parts[1],
        banded.parts[2],
        banded.alone[0],
        banded.alone[1],
        banded.alone[2]
    );
    // 30% more drag in a band lowers the apogee, so each part is positive, and by more than
    // 0.5% of OpenRocket's apogee in every band of this flight.
    for (i, (part, alone)) in banded.parts.iter().zip(banded.alone).enumerate() {
        assert!(*part > 0.5, "band {i}: {part}");
        assert!(alone > 0.5, "band {i}: {alone}");
    }
    // On 5% more thrust the flight is faster, so a band's part is not the band alone.
    assert!((banded.parts[2] - banded.alone[2]).abs() > 0.01);
}

/// A flight that is not split has no bands; a flight HPR Sim fails with one or two bands swapped
/// is counted with why; and the run stops when the flight on every band swapped fails or is not
/// the split's flight on OpenRocket's whole drag.
#[test]
fn a_flight_without_bands_says_why() {
    let flights = public_flights();
    let public = &flights[0];
    let own_m = fly(&public.build().unwrap()).unwrap();
    let openrocket_m = public.flight["apogee_m"].as_f64().unwrap();
    let build = |_air: Air| public.build();
    let whole = split(
        &build,
        &fly,
        &public.flight,
        (own_m, openrocket_m),
        OWN_HISTORY_BOUND,
    )
    .unwrap();
    assert!(whole.is_ok());
    let not_split = split(
        &build,
        &fly,
        &json!({"lacks": "no_apogee"}),
        (own_m, openrocket_m),
        OWN_HISTORY_BOUND,
    )
    .unwrap();
    assert!(not_split.is_err());
    assert_eq!(
        super::flight(&build, &fly, &public.flight, &not_split, openrocket_m),
        Ok(None)
    );
    // The first flight flown is the one on every band swapped: the split's own flight on
    // OpenRocket's whole drag, which flew. Its failure is the swap disagreeing with the table,
    // and stops the run.
    let calls = Cell::new(0);
    let failing = |simulation: &Simulation| {
        calls.set(calls.get() + 1);
        if calls.get() == 1 {
            Err("the integrator gave up".to_owned())
        } else {
            fly(simulation)
        }
    };
    let error = super::flight(&build, &failing, &public.flight, &whole, openrocket_m).unwrap_err();
    assert!(
        error.contains("fails the flight it flew on OpenRocket's whole drag")
            && error.contains("the integrator gave up"),
        "{error}"
    );
    assert_eq!(calls.get(), 1);
    // So does that flight climbing a millimeter more than the split's.
    let calls = Cell::new(0);
    let off = |simulation: &Simulation| {
        calls.set(calls.get() + 1);
        fly(simulation).map(|m| if calls.get() == 1 { m + 0.001 } else { m })
    };
    let error = super::flight(&build, &off, &public.flight, &whole, openrocket_m).unwrap_err();
    assert!(error.contains("don't sum to the drag part"), "{error}");
    // A later flight that fails is a flight counted, with why.
    let calls = Cell::new(0);
    let failing = |simulation: &Simulation| {
        calls.set(calls.get() + 1);
        if calls.get() == 3 {
            Err("the integrator gave up".to_owned())
        } else {
            fly(simulation)
        }
    };
    assert_eq!(
        super::flight(&build, &failing, &public.flight, &whole, openrocket_m),
        Ok(Some(Err("the integrator gave up".to_owned())))
    );
    assert_eq!(calls.get(), 3);
    // So is one whose height is not a number: it would count as a part in every band.
    let calls = Cell::new(0);
    let lost = |simulation: &Simulation| {
        calls.set(calls.get() + 1);
        fly(simulation).map(|m| if calls.get() == 5 { f64::NAN } else { m })
    };
    assert_eq!(
        super::flight(&build, &lost, &public.flight, &whole, openrocket_m),
        Ok(Some(Err(
            "a flight with a band swapped climbs NaN m".to_owned()
        )))
    );
    assert_eq!(calls.get(), 6);
    // A split flight whose record has lost its histories stops the run too.
    let error =
        super::flight(&build, &fly, &json!({"lacks": "x"}), &whole, openrocket_m).unwrap_err();
    assert!(error.contains("no histories"), "{error}");
}

/// A flight by hand: its three parts, each band alone, and its heights, with a logged apogee.
/// OpenRocket climbs 1,000 m, so a part of 1% is 10 m.
fn by_hand(parts: [f64; 3], alone: [f64; 3], logged_m: Option<f64>) -> BandFlight {
    let own_m = 1100.0;
    let alone_m = alone.map(|percent| own_m - 10.0 * percent);
    let drag_first: f64 = alone.iter().sum();
    BandFlight {
        banded: Ok(Banded {
            parts,
            alone,
            drag: parts.iter().sum(),
            drag_first,
            heights_m: Heights {
                own_m,
                alone_m,
                drag_first_m: own_m - 10.0 * drag_first,
                openrocket_m: 1000.0,
            },
        }),
        logged_m,
    }
}

/// Seven flights by hand. The first two stay in the slowest band, three reach the fastest, and
/// the first six are compared with a log of 1,000 m, so each of those climbs 10% above its log
/// on its own drag.
fn section() -> Vec<BandFlight> {
    vec![
        by_hand([2.0, 0.0, 0.0], [2.0, 0.0, 0.0], Some(1000.0)),
        by_hand([-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], Some(1000.0)),
        by_hand([0.5, 1.5, 0.0], [0.5, 1.4, 0.0], Some(1000.0)),
        by_hand([0.5, -0.5, 3.0], [0.5, -0.6, 3.2], Some(1000.0)),
        by_hand([1.0, 1.0, -2.0], [1.0, 1.1, -2.0], Some(1000.0)),
        by_hand([0.3, 0.2, 0.3], [0.3, 0.2, 0.3], Some(1000.0)),
        by_hand([0.1, 0.4, 0.0], [0.1, 0.4, 0.0], None),
    ]
}

/// The section over `flights`, where the report compares no flight but those of them that are
/// split by band and carry a log.
fn of(flights: &[BandFlight]) -> DragBands {
    let compared = flights
        .iter()
        .filter(|flight| flight.banded.is_ok() && flight.logged_m.is_some())
        .count();
    DragBands::of(flights, compared)
}

/// A mean, a median and a mean absolute value, per cent.
fn size(mean_percent: f64, median_percent: f64, mean_absolute_percent: f64) -> Option<Size> {
    Some(Size {
        mean_percent,
        median_percent,
        mean_absolute_percent,
    })
}

/// The report's own errors over the flights compared in [`section`]: HPR Sim's are all +10%,
/// OpenRocket's all 0%.
const REPORTS: ([f64; 3], [f64; 3]) = ([10.0, 10.0, 10.0], [0.0, 0.0, 0.0]);

/// Every count and size of the section, worked by hand over [`section`]'s seven flights.
#[test]
fn the_section_counts_and_averages() {
    let bands = of(&section());
    assert!(bands.published());
    assert_eq!(bands.check_consistent(7, 6, REPORTS), Ok(()));
    let band =
        |name: &str, flights, alone_flights, sizes: [Option<Size>; 3], counts: [usize; 2]| {
            let [part, part_on_its_flights, alone] = sizes;
            Band {
                name: name.to_owned(),
                flights,
                part,
                part_on_its_flights,
                alone_flights,
                alone,
                dominates: Some(counts[0]),
                large: Some(counts[1]),
            }
        };
    let row = |name: &str, moved, nearer, farther, error| LogRow {
        name: name.to_owned(),
        moved,
        nearer: Some(nearer),
        farther: Some(farther),
        error,
    };
    let expected = DragBands {
        split: 7,
        banded: 7,
        failed: 0,
        bands: vec![
            // The slowest band, in every flight: 2, −1, 0.5, 0.5, 1, 0.3 and 0.1 sum to 3.4,
            // and 3.4 / 7 = 0.49; the middle of −1, 0.1, 0.3, 0.5, 0.5, 1, 2 is 0.5; absolute,
            // 5.4 / 7 = 0.77. Alone it is the same seven numbers. It dominates the first two
            // flights and the sixth, where it ties the fastest: the slower of a tie. Over 1%:
            // the 2 only (−1 and 1 are not over).
            band("subsonic", 7, 7, [size(0.49, 0.5, 0.77); 3], [3, 1]),
            // The middle band: 0, 0, 1.5, −0.5, 1, 0.2 and 0.4 sum to 2.6, and 2.6 / 7 = 0.37;
            // the middle of −0.5, 0, 0, 0.2, 0.4, 1, 1.5 is 0.2; absolute, 3.6 / 7 = 0.51.
            // Over its own five flights: 2.6 / 5 = 0.52, the middle of −0.5, 0.2, 0.4, 1, 1.5
            // is 0.4, and 3.6 / 5 = 0.72. Alone: 0, 0, 1.4, −0.6, 1.1, 0.2 and 0.4 sum to 2.5,
            // 2.5 / 7 = 0.36; the middle is 0.2; absolute, 3.7 / 7 = 0.53. It dominates the
            // third flight and the last. Over 1%: the 1.5.
            band(
                "transonic",
                5,
                5,
                [
                    size(0.37, 0.2, 0.51),
                    size(0.52, 0.4, 0.72),
                    size(0.36, 0.2, 0.53),
                ],
                [2, 1],
            ),
            // The fastest: (3 − 2 + 0.3) / 7 = 0.19, the middle of seven with four zeros is 0,
            // and 5.3 / 7 = 0.76 absolute; over its own three flights, 1.3 / 3 = 0.43, 0.3 and
            // 5.3 / 3 = 1.77. Alone: (3.2 − 2 + 0.3) / 7 = 0.21, 0, and 5.5 / 7 = 0.79. It
            // dominates the fourth and fifth flights. Over 1%: the 3 and the −2.
            band(
                "supersonic",
                3,
                3,
                [
                    size(0.19, 0.0, 0.76),
                    size(0.43, 0.3, 1.77),
                    size(0.21, 0.0, 0.79),
                ],
                [2, 2],
            ),
        ],
        // The drag part, the sum of each flight's parts: 2, −1, 2, 3, 0, 0.8 and 0.5 are
        // 7.3 / 7 = 1.04; the middle is 0.8; and 9.3 / 7 = 1.33 absolute.
        drag: size(1.04, 0.8, 1.33),
        // Swapped first, the sum of each flight's bands alone: 2, −1, 1.9, 3.1, 0.1, 0.8 and
        // 0.5 are 7.4 / 7 = 1.06; the middle is 0.8; and 9.4 / 7 = 1.34 absolute.
        drag_first: size(1.06, 0.8, 1.34),
        // Against the logs, over the first six flights. A band alone that is 2% of
        // OpenRocket's 1,000 m takes 20 m off HPR Sim's 1,100 m: an error of +8% of the 1,000 m
        // logged, where HPR Sim's own is +10%.
        logs: Logs {
            flights: 6,
            left_out: 0,
            rows: vec![
                row("own", 0, 0, 0, size(10.0, 10.0, 10.0)),
                // 8, 11, 9.5, 9.5, 9 and 9.7: 56.7 / 6 = 9.45, a median of 9.5; five nearer
                // the log than 10, and the 11 farther.
                row("subsonic", 6, 5, 1, size(9.45, 9.5, 9.45)),
                // 10, 10, 8.6, 10.6, 8.9 and 9.8: 57.9 / 6 = 9.65; the middle two of 8.6, 8.9,
                // 9.8, 10, 10, 10.6 are 9.8 and 10; four moved, three nearer and the 10.6
                // farther.
                row("transonic", 4, 3, 1, size(9.65, 9.9, 9.65)),
                // 10, 10, 10, 6.8, 12 and 9.7: 58.5 / 6 = 9.75, a median of 10; three moved,
                // two nearer and the 12 farther.
                row("supersonic", 3, 2, 1, size(9.75, 10.0, 9.75)),
                // The whole drag, 10 less each flight's sum alone: 8, 11, 8.1, 6.9, 9.9 and
                // 9.2 are 53.1 / 6 = 8.85; the middle two of 6.9, 8, 8.1, 9.2, 9.9, 11 are 8.1
                // and 9.2; five nearer and the 11 farther.
                row("every", 6, 5, 1, size(8.85, 8.65, 8.85)),
                // OpenRocket's own flights: 1,000 m against 1,000 m logged.
                row("openrocket", 6, 6, 0, size(0.0, 0.0, 0.0)),
            ],
        },
    };
    assert_eq!(bands, expected);

    let page = bands.to_markdown();
    for line in [
        "### The drag part by Mach band",
        "Of the 7 flights split, 7 are split by band; on 0 HPR Sim failed a flight",
        "| below Mach 0.8 | 7 | +0.49% | +0.50% | 0.77% | 0.77% | 3 | 1 |",
        "| above Mach 1.2 | 3 | +0.19% | +0.00% | 0.76% | 1.77% | 2 | 2 |",
        "| the drag part | 7 | +1.04% | +0.80% | 1.33% |  |  |  |",
        "| Mach 0.8 to 1.2 | 5 | +0.37% | +0.20% | 0.51% | 0.72% | 2 | 1 |",
        "| Mach 0.8 to 1.2 | 5 | +0.36% | +0.20% | 0.53% |",
        "| above Mach 1.2 | 3 | +0.21% | +0.00% | 0.79% |",
        "| every band: the drag part, swapped first |  | +1.06% | +0.80% | 1.34% |",
        "| OpenRocket's at every Mach number | 6 | +8.85% | +8.65% | 8.85% | 5 | 1 |",
        "| its own |  | +10.00% | +10.00% | 10.00% |  |  |",
        "| OpenRocket's below Mach 0.8, its own above | 6 | +9.45% | +9.50% | 9.45% | 5 | 1 |",
        "| OpenRocket's own flight, for comparison | 6 | +0.00% | +0.00% | 0.00% | 6 | 0 |",
    ] {
        assert!(page.contains(line), "{line}\n{page}");
    }
}

/// Holds that a section gives no size, on the page or in its fields; its counts stay.
fn no_sizes(bands: &DragBands) {
    assert!(!bands.published());
    assert_eq!((bands.drag, bands.drag_first), (None, None));
    for band in &bands.bands {
        assert_eq!(
            (band.part, band.part_on_its_flights, band.alone),
            (None, None, None)
        );
        assert_eq!((band.dominates, band.large), (None, None));
    }
    for row in &bands.logs.rows {
        assert_eq!((row.error, row.nearer, row.farther), (None, None, None));
    }
    let page = bands.to_markdown();
    assert!(page.contains("The bands' sizes are not given"), "{page}");
    assert!(!page.contains('%'), "{page}");
}

/// An aggregate over one or two flights is a flight's own number, so no size is given when a
/// band, or a row against the logs, holds one or two flights: not that band's, and not the
/// others', which sum with it to the drag part. Counts stay.
#[test]
fn one_or_two_flights_in_a_band_give_no_sizes() {
    // The fastest band with two flights.
    let mut flights = section();
    flights.remove(5);
    let bands = of(&flights);
    assert_eq!(bands.bands[2].flights, 2);
    assert!(!bands.published());
    assert_eq!(bands.check_consistent(6, 5, REPORTS), Ok(()));
    assert_eq!(bands.logs.flights, 5);
    no_sizes(&bands);
    assert!(bands.to_markdown().contains("| above Mach 1.2 | 2 |"));
    // The band's count alone hides them: two flights in the fastest band, neither compared
    // with a log, so no row against the logs moves one or two flights.
    let slow = || by_hand([1.0, 0.0, 0.0], [1.0, 0.0, 0.0], Some(1000.0));
    // Alone, on their own thrust and mass, they stay out of it.
    let fast = || by_hand([0.5, 0.0, 2.0], [0.5, 0.0, 0.0], None);
    let bands = of(&[slow(), slow(), slow(), fast(), fast()]);
    assert_eq!(
        bands
            .bands
            .iter()
            .map(|band| band.flights)
            .collect::<Vec<_>>(),
        [5, 0, 2]
    );
    assert_eq!(bands.logs.flights, 3);
    assert_eq!(
        bands
            .logs
            .rows
            .iter()
            .map(|row| row.moved)
            .collect::<Vec<_>>(),
        [0, 3, 0, 0, 3, 3]
    );
    assert_eq!(
        bands
            .bands
            .iter()
            .map(|band| band.alone_flights)
            .collect::<Vec<_>>(),
        [5, 0, 0]
    );
    no_sizes(&bands);
    assert_eq!(bands.check_consistent(5, 3, REPORTS), Ok(()));
    // The count a band alone changes hides them by itself too: two flights that reach the
    // fastest band only on their own thrust and mass. Their mean alone, times the flights,
    // would be the two flights' sum.
    let fast_alone = || by_hand([0.5, 0.0, 0.0], [0.5, 0.0, 2.0], None);
    let bands = of(&[slow(), slow(), slow(), fast_alone(), fast_alone()]);
    assert_eq!(
        bands
            .bands
            .iter()
            .map(|band| (band.flights, band.alone_flights))
            .collect::<Vec<_>>(),
        [(5, 5), (0, 0), (0, 2)]
    );
    assert_eq!(
        bands
            .logs
            .rows
            .iter()
            .map(|row| row.moved)
            .collect::<Vec<_>>(),
        [0, 3, 0, 0, 3, 3]
    );
    no_sizes(&bands);
    assert_eq!(bands.check_consistent(5, 3, REPORTS), Ok(()));
    // One flight failed, or one compared flight that is not split by band: the split's drag
    // part and the errors' table are over one flight more, and the difference is that flight.
    // Three hide nothing.
    let failed = || BandFlight {
        banded: Err("the integrator gave up".to_owned()),
        logged_m: None,
    };
    let mut flights = section();
    flights.push(failed());
    let bands = of(&flights);
    assert_eq!((bands.failed, bands.logs.left_out), (1, 0));
    no_sizes(&bands);
    assert_eq!(bands.check_consistent(8, 6, REPORTS), Ok(()));
    flights.extend([failed(), failed()]);
    assert!(of(&flights).published());
    let bands = DragBands::of(&section(), 7);
    assert_eq!((bands.failed, bands.logs.left_out), (0, 1));
    no_sizes(&bands);
    assert_eq!(bands.check_consistent(7, 7, REPORTS), Ok(()));
    let bands = DragBands::of(&section(), 9);
    assert_eq!(bands.logs.left_out, 3);
    assert!(bands.published());
    assert_eq!(bands.check_consistent(7, 9, REPORTS), Ok(()));

    // Two flights compared with a log, or a row against the logs that moves two.
    let mut flights = section();
    for flight in &mut flights[2..6] {
        flight.logged_m = None;
    }
    let bands = of(&flights);
    assert_eq!(bands.logs.flights, 2);
    no_sizes(&bands);
    let mut flights = section();
    flights[5].logged_m = None;
    let bands = of(&flights);
    // The row's count alone hides them: every band holds three flights or more.
    assert_eq!((bands.logs.flights, bands.logs.rows[3].moved), (5, 2));
    assert_eq!(
        bands
            .bands
            .iter()
            .map(|band| band.flights)
            .collect::<Vec<_>>(),
        [7, 5, 3]
    );
    no_sizes(&bands);
    assert_eq!(bands.check_consistent(7, 5, REPORTS), Ok(()));
    // The count compared alone hides them: two flights compared that no swap moves, since
    // their drag and OpenRocket's agree, would still give the first row over two flights.
    let unmoved = || {
        let mut flight = by_hand([0.0; 3], [0.0; 3], Some(1000.0));
        if let Ok(banded) = &mut flight.banded {
            banded.heights_m.openrocket_m = banded.heights_m.own_m;
        }
        flight
    };
    let unlogged = || by_hand([1.0, 0.0, 0.0], [1.0, 0.0, 0.0], None);
    let bands = of(&[unlogged(), unlogged(), unlogged(), unmoved(), unmoved()]);
    assert_eq!(
        bands
            .bands
            .iter()
            .map(|band| band.flights)
            .collect::<Vec<_>>(),
        [3, 0, 0]
    );
    assert_eq!(bands.logs.flights, 2);
    assert!(bands.logs.rows.iter().all(|row| row.moved == 0));
    no_sizes(&bands);
    assert_eq!(bands.check_consistent(5, 2, REPORTS), Ok(()));
    // No flight in a band, and none compared, hide nothing.
    let flights: Vec<BandFlight> = section()
        .into_iter()
        .take(2)
        .chain(section().into_iter().take(2))
        .map(|mut flight| {
            flight.logged_m = None;
            flight
        })
        .collect();
    let bands = of(&flights);
    assert_eq!(bands.bands[1].flights, 0);
    assert!(bands.published());
    assert_eq!(bands.check_consistent(4, 0, REPORTS), Ok(()));
    // Fewer than three flights split by band, by itself: two flights with no part anywhere,
    // whose mean would still be theirs.
    let still = || by_hand([0.0; 3], [0.0; 3], None);
    let bands = of(&[still(), still()]);
    assert!(
        bands
            .bands
            .iter()
            .all(|band| band.flights == 0 && band.alone_flights == 0)
    );
    no_sizes(&bands);
    assert_eq!(bands.check_consistent(2, 0, REPORTS), Ok(()));
    assert!(!of(&section()[..2]).published());
}

/// A failed flight is counted, and the section fails when fewer than half the flights split are
/// split by band.
#[test]
fn failed_flights_are_counted_and_half_must_be_banded() {
    let failed = || BandFlight {
        banded: Err("the integrator gave up".to_owned()),
        logged_m: Some(1000.0),
    };
    let mut flights = section();
    flights.push(failed());
    let bands = DragBands::of(&flights, 7);
    assert_eq!((bands.split, bands.banded, bands.failed), (8, 7, 1));
    // The failed flight is compared with a log, and not among the rows against the logs.
    assert_eq!((bands.logs.flights, bands.logs.left_out), (6, 1));
    assert_eq!(bands.check_consistent(8, 7, REPORTS), Ok(()));
    assert!(
        bands
            .to_markdown()
            .contains("Of the 8 flights split, 7 are split by band; on 1 HPR Sim failed")
    );
    // Three failed: counted, and the sizes over the other seven are given.
    flights.extend([failed(), failed()]);
    let bands = DragBands::of(&flights, 9);
    assert_eq!((bands.split, bands.banded, bands.failed), (10, 7, 3));
    assert!(bands.published());
    assert_eq!(bands.check_consistent(10, 9, REPORTS), Ok(()));
    assert!(
        bands
            .to_markdown()
            .contains("| the drag part | 7 | +1.04% | +0.80% | 1.33% |  |  |  |")
    );
    flights.extend((0..5).map(|_| failed()));
    let bands = DragBands::of(&flights, 14);
    assert_eq!((bands.split, bands.banded), (15, 7));
    let error = bands.check_consistent(15, 14, REPORTS).unwrap_err();
    assert!(error.contains("fewer than half"), "{error}");
    // No flight at all fails it too: no section.
    assert!(
        DragBands::default()
            .check_consistent(0, 0, REPORTS)
            .is_err()
    );
    assert!(of(&[]).check_consistent(0, 0, REPORTS).is_ok());
    assert!(of(&[]).check_consistent(1, 0, REPORTS).is_err());
}

/// Each way a section read from a file can disagree with itself or the report is caught.
#[test]
fn an_inconsistent_section_is_caught() {
    let good = of(&section());
    assert_eq!(good.check_consistent(7, 6, REPORTS), Ok(()));
    let broken = |edit: &dyn Fn(&mut DragBands), expected: &str| {
        let mut bands = good.clone();
        edit(&mut bands);
        let error = bands.check_consistent(7, 6, REPORTS).unwrap_err();
        assert!(error.contains(expected), "{expected}: {error}");
    };
    broken(&|b| b.bands.swap(0, 1), "the bands are");
    broken(&|b| b.logs.rows.swap(0, 1), "the rows against the logs are");
    broken(&|b| b.banded = 6, "are not the 7 flights split");
    broken(&|b| b.failed = 1, "are not the 7 flights split");
    broken(&|b| b.bands[1].flights = 8, "more than the 7 split by band");
    broken(
        &|b| b.bands[1].alone_flights = 8,
        "more than the 7 split by band",
    );
    broken(&|b| b.logs.flights = 7, "flights against the logs");
    broken(&|b| b.logs.left_out = 1, "and 1 left out, of 6 compared");
    broken(&|b| b.logs.rows[2].moved = 7, "row moves 7 flights");
    broken(&|b| b.logs.rows[0].moved = 3, "own row moves a flight");
    broken(&|b| b.drag = None, "a size is missing");
    broken(&|b| b.bands[2].alone = None, "a size is missing");
    broken(&|b| b.logs.rows[4].nearer = None, "a size is missing");
    // Two flights in a band, with the sizes still there.
    broken(&|b| b.bands[2].flights = 2, "a size is given");
    broken(&|b| b.logs.rows[3].moved = 1, "a size is given");
    broken(&|b| b.bands[2].alone_flights = 2, "a size is given");
    // A flight failed, or a compared flight left out, with the sizes still there.
    let mut failed = good.clone();
    (failed.split, failed.failed) = (8, 1);
    let error = failed.check_consistent(8, 6, REPORTS).unwrap_err();
    assert!(error.contains("a size is given"), "{error}");
    let mut left_out = good.clone();
    left_out.logs.left_out = 2;
    let error = left_out.check_consistent(7, 8, REPORTS).unwrap_err();
    assert!(error.contains("a size is given"), "{error}");
    broken(
        &|b| b.bands[1].part.as_mut().unwrap().mean_percent = 5.0,
        "transonic",
    );
    broken(&|b| b.bands[1].dominates = Some(6), "dominates 6 flights");
    broken(&|b| b.bands[2].large = Some(4), "is large on 4");
    broken(
        &|b| {
            b.bands[2]
                .part_on_its_flights
                .as_mut()
                .unwrap()
                .mean_absolute_percent = 0.5
        },
        "below 0.76 over every flight",
    );
    broken(
        &|b| b.bands[0].dominates = Some(4),
        "8 flights are dominated",
    );
    // A band's two mean absolute values are one sum over two counts of flights.
    broken(
        &|b| {
            b.bands[1]
                .part_on_its_flights
                .as_mut()
                .unwrap()
                .mean_absolute_percent = 0.9
        },
        "the transonic band's sizes don't fit its flights",
    );
    // Over every flight, they are one size.
    broken(
        &|b| {
            b.bands[0]
                .part_on_its_flights
                .as_mut()
                .unwrap()
                .median_percent = 0.4
        },
        "the subsonic band's sizes don't fit its flights",
    );
    // A band emptied of its flights, its counts moved to another, keeps a part.
    broken(
        &|b| {
            (b.bands[1].flights, b.bands[1].dominates, b.bands[1].large) = (0, Some(0), Some(0));
            b.bands[0].dominates = Some(5);
        },
        "the transonic band's sizes don't fit its flights",
    );
    // Even a part whose mean absolute value is nothing: with no flights it has no median.
    broken(
        &|b| {
            (b.bands[1].flights, b.bands[1].dominates, b.bands[1].large) = (0, Some(0), Some(0));
            b.bands[0].dominates = Some(5);
            b.bands[1].part = size(0.0, 0.3, 0.0);
        },
        "the transonic band's sizes don't fit its flights",
    );
    broken(
        &|b| b.bands[1].alone_flights = 0,
        "the transonic band's sizes don't fit its flights",
    );
    broken(
        &|b| {
            b.bands[0].part.as_mut().unwrap().mean_percent = 0.6;
            b.bands[0]
                .part_on_its_flights
                .as_mut()
                .unwrap()
                .mean_percent = 0.6;
        },
        "the bands' means sum to",
    );
    broken(
        &|b| b.logs.rows[1].nearer = Some(6),
        "6 flights nearer and 1 farther",
    );
    broken(
        &|b| b.logs.rows[0].error.as_mut().unwrap().mean_percent = 9.0,
        "where the errors' table has",
    );
    broken(
        &|b| b.logs.rows[5].error.as_mut().unwrap().median_percent = 0.01,
        "where the errors' table has",
    );
    // The first and last rows are held to the report's only over every flight compared.
    let partly = DragBands::of(&section(), 9);
    assert_eq!(partly.check_consistent(7, 9, ([9.0; 3], [1.0; 3])), Ok(()));
    assert!(good.check_consistent(7, 6, ([9.0; 3], [1.0; 3])).is_err());
    // With no flight failed, the drag rows are the split's own.
    let (drag, drag_first) = (good.drag.unwrap(), good.drag_first.unwrap());
    assert_eq!(good.check_against_split(drag, drag_first), Ok(()));
    let error = good
        .check_against_split(drag_first, drag_first)
        .unwrap_err();
    assert!(error.contains("where the split has"), "{error}");
    assert!(good.check_against_split(drag, drag).is_err());
    // With flights failed, the split's rows are over more flights, and not held.
    let mut failed = good.clone();
    (failed.split, failed.failed) = (10, 3);
    assert_eq!(failed.check_against_split(drag_first, drag), Ok(()));
    assert!(good.check_consistent(7, 5, REPORTS).is_err());
    assert!(good.check_consistent(6, 6, REPORTS).is_err());
}

/// The committed report's section: every flight split is split by band, and each number the
/// documentation's decision rests on is the one the report holds. The slowest band has the
/// largest mean absolute part, dominates the most flights and, swapped alone, takes the most off
/// HPR Sim's mean absolute error against the logs.
#[test]
fn the_committed_report_puts_the_most_below_mach_0_8() {
    let root = crate::designs::root().unwrap();
    let text = std::fs::read_to_string(root.join(crate::fixture_flights::REPORT_JSON)).unwrap();
    let report: crate::fixture_flights::Report = serde_json::from_str(&text).unwrap();
    let bands = &report.bands;
    assert!(bands.published());
    assert_eq!(bands.banded, report.split.split);
    assert_eq!(bands.failed, 0);
    assert_eq!(bands.logs.flights, report.counts.compared);
    let part = |i: usize| bands.bands[i].part.unwrap().mean_absolute_percent;
    assert!(part(0) > part(1) && part(1) > part(2));
    let dominates = |i: usize| bands.bands[i].dominates.unwrap();
    assert!(dominates(0) > dominates(1) + dominates(2));
    let error = |row: usize| bands.logs.rows[row].error.unwrap().mean_absolute_percent;
    let taken_off = |row: usize| error(0) - error(row);
    assert!(taken_off(1) > taken_off(2) && taken_off(2) > taken_off(3));
    assert!(taken_off(3) > 0.0);
    // On OpenRocket's whole drag HPR Sim's mean absolute error is within 0.05 of OpenRocket's.
    assert!(
        (error(4) - error(5)).abs() < 0.05,
        "{} {}",
        error(4),
        error(5)
    );
}
