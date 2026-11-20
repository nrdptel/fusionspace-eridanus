use hpr_aero::{AeroModel, DragConditions, Flow};
use serde_json::json;

use super::*;
use crate::drag_bands::BandSwap;
use crate::gap_split::split;
use crate::histories::own::OWN_HISTORY_BOUND;
use crate::histories::tests::{Public, public_flights};

fn fly(simulation: &Simulation) -> Result<f64, String> {
    crate::fixture_flights::climbed_m(simulation)
}

/// `drag_terms.py`'s record: on the Mach probe, a launch lug and rail buttons move OpenRocket's
/// pressure and leave its base and its rest; a stated coefficient is in none of its three terms,
/// so in the rest, once per fin of a fin set. That is why HPR Sim's parasitic drag is set with
/// its pressure and its stated coefficients are its rest ([`hpr_terms`]).
#[test]
fn openrocket_books_lugs_and_buttons_as_pressure_and_stated_drag_in_the_rest() {
    let root = crate::ork::root().unwrap();
    let text =
        std::fs::read_to_string(root.join("validation/fixtures/ork/openrocket-drag-terms.json"))
            .unwrap();
    let record: Value = serde_json::from_str(&text).unwrap();
    assert_eq!(
        record["design"],
        "validation/fixtures/ork/history-flights/mach-probe.ork"
    );
    assert_eq!(record["openrocket"], "24.12");
    let at = |variant: &str, i: usize| -> [f64; 5] {
        let row = &record["variants"][variant]["at_mach"][i];
        let n = |key: &str| row[key].as_f64().unwrap();
        let drag = n("drag_coefficient");
        let [f, p, b] = [n("friction"), n("pressure"), n("base")];
        [f, p, b, drag - f - p - b, drag]
    };
    for (i, mach) in [0.3, 0.6].into_iter().enumerate() {
        assert_eq!(record["variants"]["as_is"]["at_mach"][i]["mach"], mach);
        let base = at("as_is", i);
        assert!(base[3].abs() < 1e-12, "the rest as is: {}", base[3]);
        let moved = |variant: &str| -> [f64; 5] {
            let got = at(variant, i);
            std::array::from_fn(|k| got[k] - base[k])
        };
        // Each flight is read between its own rows at the same Mach number, and the rows fall
        // at Mach numbers a little apart: a term that doesn't move reads within a few parts in
        // a million, against the 0.012 the part adds.
        let between_rows = 1e-5;
        for variant in ["launch_lug", "rail_buttons"] {
            let [f, p, b, rest, drag] = moved(variant);
            // The part's own drag is in the pressure: more than 0.01 on the probe.
            assert!(p > 0.01, "{variant} at Mach {mach}: pressure {p}");
            assert!(
                b.abs() < between_rows && rest.abs() < 1e-12,
                "{variant}: {b} {rest}"
            );
            // Buttons cover a little of the tube's wetted area: friction falls by under 0.001.
            assert!(f.abs() < 1e-3, "{variant}: {f}");
            assert!(
                (p + f - drag).abs() < between_rows,
                "{variant}: {p} {f} {drag}"
            );
        }
        // Stated on the body, 0.3 is in the rest, and the tube's own friction and base are gone.
        let stated = at("stated_body", i);
        assert!((stated[3] - 0.3).abs() < 1e-9, "{}", stated[3]);
        assert_eq!(stated[2], 0.0);
        // Stated on the probe's fin set of three fins, three times 0.3.
        let stated = at("stated_fins", i);
        assert!((stated[3] - 0.9).abs() < 1e-9, "{}", stated[3]);
    }
}

/// HPR Sim's four terms: its parasitic drag with its pressure, its stated coefficients the rest.
#[test]
fn hpr_sims_parasitic_drag_is_with_its_pressure() {
    let mut drag = Drag::default();
    drag.friction = 0.25;
    drag.pressure = 0.125;
    drag.base = 0.0625;
    drag.parasitic = 0.5;
    drag.stated = 2.0;
    assert_eq!(hpr_terms(&drag), [0.25, 0.625, 0.0625, 2.0]);
}

/// The coefficient at `mach` of a design's buildup, coasting or thrusting with no motor area.
fn buildup(model: &AeroModel, mach: f64, thrusting: bool) -> Drag {
    let conditions = if thrusting {
        DragConditions::thrusting(1.0e7, 0.0)
    } else {
        DragConditions::coasting(1.0e7)
    };
    model
        .buildup_drag(&Flow::new(mach, 0.0, 0.0), &conditions)
        .unwrap()
}

/// Each code's four terms sum to its whole coefficient, power on and power off, at Mach numbers
/// across the band: OpenRocket's tables to the split's whole table, HPR Sim's to its buildup.
#[test]
fn each_codes_four_terms_sum_to_its_whole() {
    for public in &public_flights() {
        let inputs = OpenRocketInputs::of(&public.flight).unwrap().unwrap();
        let tables = term_tables(&public.flight).unwrap().unwrap();
        let simulation = public.build().unwrap();
        let model = AeroModel::new(&simulation.assembly().layout).unwrap();
        for mach in [0.1, 0.3, 0.6, 0.79] {
            for thrusting in [false, true] {
                let whole = inputs.drag.lookup(mach, thrusting).unwrap().value;
                let sum: f64 = tables
                    .iter()
                    .map(|t| t.lookup(mach, thrusting).unwrap().value)
                    .sum();
                assert!(
                    (sum - whole).abs() < 1e-12,
                    "{}: {sum} {whole}",
                    public.file
                );
                let own = buildup(&model, mach, thrusting);
                let sum: f64 = hpr_terms(&own).iter().sum();
                assert!(
                    (sum - own.zero_lift_coefficient).abs() < 1e-15,
                    "{}: {sum} {}",
                    public.file,
                    own.zero_lift_coefficient
                );
            }
        }
    }
}

/// Each code's four terms on the two public designs, coasting, at Mach 0.3 and 0.6, each held
/// within [`PIN_WITHIN`]: `(design, Mach, HPR Sim's, OpenRocket's)` in [`TERMS`]' order, each on
/// the code's own reference area. Neither design states a coefficient, so both rests are zero.
///
/// The two designs share an airframe and differ in their motors, so coasting their terms are
/// the same but for OpenRocket's pressure at Mach 0.6, read along two flights. HPR Sim's
/// friction is about 1.3% above OpenRocket's; pressure and base agree to 0.2%.
const PINS: [(&str, f64, [f64; 4], [f64; 4]); 4] = [
    (
        "pods-none.ork",
        0.3,
        [0.30690, 0.19281, 0.13170, 0.0],
        [0.30310, 0.19282, 0.13170, 0.0],
    ),
    (
        "pods-none.ork",
        0.6,
        [0.29853, 0.21184, 0.16680, 0.0],
        [0.29484, 0.21221, 0.16680, 0.0],
    ),
    (
        "mach-probe.ork",
        0.3,
        [0.30690, 0.19281, 0.13170, 0.0],
        [0.30310, 0.19282, 0.13170, 0.0],
    ),
    (
        "mach-probe.ork",
        0.6,
        [0.29853, 0.21184, 0.16680, 0.0],
        [0.29484, 0.21220, 0.16680, 0.0],
    ),
];
const PIN_WITHIN: f64 = 5e-5;

#[test]
fn both_codes_terms_are_pinned_at_mach_0_3_and_0_6() {
    let flights = public_flights();
    let read: Vec<([f64; 4], [f64; 4])> = PINS
        .iter()
        .map(|(file, mach, _, _)| {
            let public = flights.iter().find(|p| p.file.ends_with(file)).unwrap();
            let tables = term_tables(&public.flight).unwrap().unwrap();
            let simulation = public.build().unwrap();
            let model = AeroModel::new(&simulation.assembly().layout).unwrap();
            let own = hpr_terms(&buildup(&model, *mach, false));
            let theirs = tables.map(|t| t.lookup(*mach, false).unwrap().value);
            eprintln!("{file} Mach {mach}: HPR Sim {own:.5?}; OpenRocket {theirs:.5?}");
            (own, theirs)
        })
        .collect();
    for ((file, mach, hpr, openrocket), (own, theirs)) in PINS.into_iter().zip(read) {
        for (got, expected) in own.iter().zip(hpr).chain(theirs.iter().zip(openrocket)) {
            assert!(
                (got - expected).abs() <= PIN_WITHIN,
                "{file} Mach {mach}: {own:?} {theirs:?}"
            );
        }
        assert_eq!((own[3], theirs[3]), (0.0, 0.0), "{file}");
    }
}

/// A stated coefficient, OpenRocket's rest, is the one term the public designs leave at zero, so
/// here OpenRocket's rest and its whole coefficient are both raised by 0.2. With friction,
/// pressure and base swapped and the rest not, the coefficient drops OpenRocket's rest for HPR
/// Sim's own (zero on this design); with the rest swapped too, it keeps OpenRocket's.
#[test]
fn the_rest_is_openrockets_only_when_swapped() {
    let flights = public_flights();
    let public = &flights[1];
    let inputs = OpenRocketInputs::of(&public.flight).unwrap().unwrap();
    let tables = term_tables(&public.flight).unwrap().unwrap();
    let simulation = public.build().unwrap();
    let layout = &simulation.assembly().layout;
    let own = AeroModel::new(layout).unwrap();
    let diameter = tables[0].reference_diameter_m.unwrap();
    let scale = 0.25 * std::f64::consts::PI * diameter * diameter / own.reference_area_m2();
    let raise = |table: &DragTable| -> DragTable {
        let mut value = serde_json::to_value(table).unwrap();
        for key in ["power_off", "power_on"] {
            for y in value[key]["y"].as_array_mut().unwrap() {
                *y = json!(y.as_f64().unwrap() + 0.2);
            }
        }
        serde_json::from_value(value).unwrap()
    };
    let mut raised = tables.clone();
    raised[3] = raise(&tables[3]);
    let whole = raise(&inputs.drag);
    let every = AeroModel::new(layout)
        .unwrap()
        .with_drag_model(TermSwap::new(inputs.drag.clone(), tables, [true; 4]));
    for rest in [false, true] {
        let swapped = AeroModel::new(layout)
            .unwrap()
            .with_drag_model(TermSwap::new(
                whole.clone(),
                raised.clone(),
                [true, true, true, rest],
            ));
        for mach in [0.3, 0.6] {
            for conditions in [
                DragConditions::coasting(1.0e7),
                DragConditions::thrusting(1.0e7, 0.0),
            ] {
                let flow = Flow::new(mach, 0.0, 0.0);
                let drag = |model: &AeroModel| {
                    model
                        .drag(&flow, &conditions)
                        .unwrap()
                        .zero_lift_coefficient
                };
                assert_eq!(own.buildup_drag(&flow, &conditions).unwrap().stated, 0.0);
                let expected = drag(&every) + if rest { 0.2 * scale } else { 0.0 };
                let got = drag(&swapped);
                assert!(
                    (got - expected).abs() < 1e-12,
                    "Mach {mach}, rest {rest}: {got} {expected}"
                );
            }
        }
    }
}

/// Every choice of terms swapped, at Mach numbers inside the band and past it, power on and
/// off: inside, the coefficient is HPR Sim's own less its terms swapped plus OpenRocket's,
/// scaled to HPR Sim's area; past it, and with no term swapped, HPR Sim's own bit for bit.
/// With every term swapped it is the band swap's below Mach 0.8.
#[test]
fn each_swap_replaces_its_own_term_and_nothing_else() {
    let flights = public_flights();
    let public = &flights[1];
    let inputs = OpenRocketInputs::of(&public.flight).unwrap().unwrap();
    let tables = term_tables(&public.flight).unwrap().unwrap();
    let simulation = public.build().unwrap();
    let layout = &simulation.assembly().layout;
    let own = AeroModel::new(layout).unwrap();
    let diameter = tables[0].reference_diameter_m.unwrap();
    let scale = 0.25 * std::f64::consts::PI * diameter * diameter / own.reference_area_m2();
    let band_swap = AeroModel::new(layout)
        .unwrap()
        .with_drag_model(BandSwap::new(inputs.drag.clone(), [true, false, false]));
    for choice in 0..16_u8 {
        let openrockets: [bool; 4] = std::array::from_fn(|i| choice & (1 << i) != 0);
        let swapped = AeroModel::new(layout)
            .unwrap()
            .with_drag_model(TermSwap::new(
                inputs.drag.clone(),
                tables.clone(),
                openrockets,
            ));
        for mach in [0.05, 0.3, 0.6, 0.79, 0.8, 1.0, 1.5] {
            for conditions in [
                DragConditions::coasting(1.0e7),
                DragConditions::thrusting(1.0e7, 0.0),
            ] {
                let flow = Flow::new(mach, 0.0, 0.0);
                let drag = |model: &AeroModel| {
                    model
                        .drag(&flow, &conditions)
                        .unwrap()
                        .zero_lift_coefficient
                };
                let got = drag(&swapped);
                let mine = own.buildup_drag(&flow, &conditions).unwrap();
                if mach >= 0.8 || choice == 0 {
                    assert_eq!(got, mine.zero_lift_coefficient, "Mach {mach}, {choice}");
                    continue;
                }
                let mut expected = mine.zero_lift_coefficient;
                for ((table, term), swapped) in tables.iter().zip(hpr_terms(&mine)).zip(openrockets)
                {
                    if swapped {
                        let theirs = table.lookup(mach, conditions.thrusting).unwrap().value;
                        expected += scale * theirs - term;
                    }
                }
                assert!(
                    (got - expected).abs() < 1e-12,
                    "Mach {mach}, {choice}: {got} {expected}"
                );
                // With friction, pressure and base swapped, the rest unswapped too on these
                // designs, it is the band swap's, bit for bit.
                if choice & 7 == 7 {
                    assert_eq!(got, drag(&band_swap), "Mach {mach}, {choice}");
                }
            }
        }
    }
}

/// A public flight split, by band and then by term.
fn termed_public(public: &Public) -> (Banded, Termed) {
    let flight = &public.flight;
    let own_m = fly(&public.build().unwrap()).unwrap();
    let openrocket_m = flight["apogee_m"].as_f64().unwrap();
    let build = |_air| public.build();
    let split = split(
        &build,
        &fly,
        flight,
        (own_m, openrocket_m),
        OWN_HISTORY_BOUND,
    )
    .unwrap();
    let banded = crate::drag_bands::flight(&build, &fly, flight, &split, openrocket_m)
        .unwrap()
        .unwrap();
    let termed = super::flight(&build, &fly, flight, &split, Some(&banded), openrocket_m)
        .unwrap()
        .unwrap()
        .unwrap();
    (banded.unwrap(), termed)
}

/// What the two public designs' parts below Mach 0.8 come to by term, per cent of OpenRocket's
/// apogee, in [`TERMS`]' order, each held within 0.01. On `pods-none`, which stays below Mach
/// 0.8 and burns there, they are −0.37% friction, +0.04% pressure, +1.12% base and 0% rest,
/// which sum to the band's +0.79%; the base's is the largest: coasting the two codes' base drag
/// agree ([`PINS`]), so it is the base under thrust. `mach-probe` burns out past Mach 1.2, so
/// below Mach 0.8 it burns only on the way up.
const PUBLIC_TERMS: [(&str, [f64; 4]); 2] = [
    ("pods-none.ork", [-0.374, 0.042, 1.122, 0.0]),
    ("mach-probe.ork", [-0.162, 0.023, 0.057, 0.0]),
];

/// The two public designs split by term: the four sum to the band's part, the rest is zero on
/// designs that state no coefficient (both codes' rests are zero, so the swap leaves the flight
/// as it was, bit for bit), and each term alone is set beside the band alone.
#[test]
fn the_public_flights_split_by_term() {
    let flights = public_flights();
    let split: Vec<(Banded, Termed)> = flights.iter().map(termed_public).collect();
    for (public, (_, termed)) in flights.iter().zip(&split) {
        eprintln!(
            "{}: terms {:+.3}% {:+.3}% {:+.3}% {:+.3}% = band {:+.3}%; alone {:+.3}% {:+.3}% \
             {:+.3}% {:+.3}%; band alone {:+.3}%",
            public.file,
            termed.parts[0],
            termed.parts[1],
            termed.parts[2],
            termed.parts[3],
            termed.band,
            termed.alone[0],
            termed.alone[1],
            termed.alone[2],
            termed.alone[3],
            termed.band_alone
        );
    }
    for ((public, (file, expected)), (banded, termed)) in
        flights.iter().zip(PUBLIC_TERMS).zip(split)
    {
        assert!(public.file.ends_with(file), "{}", public.file);
        let sum: f64 = termed.parts.iter().sum();
        assert!((sum - termed.band).abs() < 1e-9, "{file}: {sum}");
        assert_eq!(termed.band, banded.parts[0]);
        assert_eq!(termed.band_alone, banded.alone[0]);
        assert_eq!(termed.heights_m.band_m, banded.heights_m.alone_m[0]);
        assert_eq!(termed.heights_m.own_m, banded.heights_m.own_m);
        assert_eq!((termed.parts[3], termed.alone[3]), (0.0, 0.0), "{file}");
        // The base under thrust alone is nearly all of the base alone.
        assert!(
            (termed.base_powered_alone - termed.alone[2]).abs() < 0.01,
            "{file}: {} {}",
            termed.base_powered_alone,
            termed.alone[2]
        );
        for (i, (part, expected)) in termed.parts.iter().zip(expected).enumerate() {
            assert!((part - expected).abs() <= 0.01, "{file}: term {i}: {part}");
        }
    }
}

/// A flight that isn't split, isn't split by band, or whose record lacks a term, says so; the
/// first is no flight here, the others are counted under their reasons.
#[test]
fn a_flight_without_terms_says_why() {
    let flights = public_flights();
    let public = &flights[0];
    let build = |_air| public.build();
    let not_split: Result<SplitFlight, NotSplit> = Err(NotSplit {
        reason: 0,
        detail: String::new(),
    });
    let none = super::flight(&build, &fly, &public.flight, &not_split, None, 1.0).unwrap();
    assert_eq!(none, None);
    let own_m = fly(&public.build().unwrap()).unwrap();
    let openrocket_m = public.flight["apogee_m"].as_f64().unwrap();
    let split = split(
        &build,
        &fly,
        &public.flight,
        (own_m, openrocket_m),
        OWN_HISTORY_BOUND,
    )
    .unwrap();
    let failed: Result<Banded, String> = Err("a band failed".to_owned());
    let got = super::flight(
        &build,
        &fly,
        &public.flight,
        &split,
        Some(&failed),
        openrocket_m,
    );
    assert_eq!(got, Ok(Some(Err((0, "a band failed".to_owned())))));
    // A split flight with no split by band is the report's own error.
    let got = super::flight(&build, &fly, &public.flight, &split, None, openrocket_m);
    assert!(got.unwrap_err().contains("no split by band"));
    // A term OpenRocket gives as not a number where its drag coefficient is a number.
    let banded = crate::drag_bands::flight(&build, &fly, &public.flight, &split, openrocket_m)
        .unwrap()
        .unwrap();
    let mut lacking = public.flight.clone();
    let row = lacking["rows"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|row| row[DRAG].is_number() && row[MACH].as_f64().unwrap() > 0.1)
        .unwrap();
    row[TERM_COLUMNS[1]] = json!(null);
    let got = super::flight(&build, &fly, &lacking, &split, Some(&banded), openrocket_m)
        .unwrap()
        .unwrap();
    assert!(
        matches!(&got, Err((1, why)) if why.contains("pressure_coefficient")),
        "{got:?}"
    );
}

/// A flight by hand: its four parts and each term alone, with a logged apogee. OpenRocket climbs
/// 1,000 m and HPR Sim 1,100 m on its own, so a part of 1% is 10 m.
fn by_hand(parts: [f64; 4], alone: [f64; 4], logged_m: Option<f64>) -> TermFlight {
    let own_m = 1100.0;
    let band_alone: f64 = alone.iter().sum();
    TermFlight {
        termed: Ok(Termed {
            parts,
            alone,
            base_powered_alone: alone[2],
            band: parts.iter().sum(),
            band_alone,
            heights_m: TermHeights {
                own_m,
                alone_m: alone.map(|percent| own_m - 10.0 * percent),
                band_m: own_m - 10.0 * band_alone,
            },
        }),
        logged_m,
    }
}

/// Four flights by hand, the first three compared with a log of 1,000 m, so each climbs 10%
/// above its log on its own drag. Neither code states a coefficient: the rest is zero.
fn section() -> Vec<TermFlight> {
    vec![
        by_hand([1.0, -0.5, 0.2, 0.0], [1.0, -0.5, 0.2, 0.0], Some(1000.0)),
        by_hand([0.4, 0.6, -0.1, 0.0], [0.4, 0.6, -0.1, 0.0], Some(1000.0)),
        by_hand([-0.2, 0.1, 0.9, 0.0], [-0.2, 0.1, 1.1, 0.0], Some(1000.0)),
        by_hand([0.6, 0.4, 0.2, 0.0], [0.6, 0.4, 0.2, 0.0], None),
    ]
}

fn size(mean_percent: f64, median_percent: f64, mean_absolute_percent: f64) -> Option<Size> {
    Some(Size {
        mean_percent,
        median_percent,
        mean_absolute_percent,
    })
}

/// Every count and size of the section, worked by hand over [`section`]'s four flights.
#[test]
fn the_section_counts_and_averages() {
    let terms = DragTerms::of(&section(), 3);
    assert!(terms.published());
    assert_eq!(terms.check_consistent(3), Ok(()));
    let term = |name: &str, flights, sizes: [Option<Size>; 2], counts: [usize; 2]| Term {
        name: name.to_owned(),
        flights,
        part: sizes[0],
        alone_flights: flights,
        alone: sizes[1],
        dominates: Some(counts[0]),
        large: Some(counts[1]),
    };
    let row = |name: &str, moved, nearer, farther, error| LogRow {
        name: name.to_owned(),
        moved,
        nearer: Some(nearer),
        farther: Some(farther),
        error,
    };
    let expected = DragTerms {
        split: 4,
        termed: 4,
        not_termed: NOT_TERMED
            .iter()
            .map(|(reason, _)| NotTermed {
                reason: (*reason).to_owned(),
                flights: 0,
            })
            .collect(),
        terms: vec![
            // 1 + 0.4 − 0.2 + 0.6 = 1.8, over 4 is 0.45; the middle of −0.2, 0.4, 0.6, 1 is
            // 0.5; absolute, 2.2 / 4 = 0.55. It dominates the first and the last flights; none
            // is over 1% (1 is not over).
            term("friction", 4, [size(0.45, 0.5, 0.55); 2], [2, 0]),
            // −0.5 + 0.6 + 0.1 + 0.4 = 0.6, over 4 is 0.15; the middle of −0.5, 0.1, 0.4, 0.6
            // is 0.25; absolute, 1.6 / 4 = 0.4. It dominates the second flight.
            term("pressure", 4, [size(0.15, 0.25, 0.4); 2], [1, 0]),
            // 0.2 − 0.1 + 0.9 + 0.2 = 1.2, over 4 is 0.3; the middle of −0.1, 0.2, 0.2, 0.9 is
            // 0.2; absolute, 1.4 / 4 = 0.35. Alone, the third flight's 1.1: 1.4 / 4 = 0.35, 0.2
            // and 1.6 / 4 = 0.4. It dominates the third flight.
            term(
                "base",
                4,
                [size(0.3, 0.2, 0.35), size(0.35, 0.2, 0.4)],
                [1, 0],
            ),
            term("rest", 0, [size(0.0, 0.0, 0.0); 2], [0, 0]),
        ],
        // The band: 0.7, 0.9, 0.8 and 1.2, over 4 is 0.9, the middle is 0.85. Alone: 0.7, 0.9,
        // 1.0 and 1.2: 0.95 and 0.95.
        band: size(0.9, 0.85, 0.9),
        band_alone: size(0.95, 0.95, 0.95),
        // By hand, the base under thrust alone is each flight's base alone.
        base_powered_flights: 4,
        base_powered_alone: size(0.35, 0.2, 0.4),
        logs: Logs {
            flights: 3,
            left_out: 0,
            rows: vec![
                row("own", 0, 0, 0, size(10.0, 10.0, 10.0)),
                // 1,090, 1,096 and 1,102 m: 9%, 9.6% and 10.2%; two nearer, one farther.
                row("friction", 3, 2, 1, size(9.6, 9.6, 9.6)),
                // 1,105, 1,094 and 1,099 m: 10.5%, 9.4% and 9.9%: 29.8 / 3 = 9.93.
                row("pressure", 3, 2, 1, size(9.93, 9.9, 9.93)),
                // 1,098, 1,101 and 1,089 m: 9.8%, 10.1% and 8.9%: 28.8 / 3 = 9.6.
                row("base", 3, 2, 1, size(9.6, 9.8, 9.6)),
                row("rest", 0, 0, 0, size(10.0, 10.0, 10.0)),
                // 1,093, 1,091 and 1,090 m: 9.3%, 9.1% and 9%: 27.4 / 3 = 9.13.
                row("subsonic", 3, 3, 0, size(9.13, 9.1, 9.13)),
            ],
        },
        // The run adds the base's measured reference after `of`.
        reference: Vec::new(),
    };
    assert_eq!(terms, expected);
    assert_eq!(terms.largest_measurable(), Some(0));
    let page = terms.to_markdown();
    assert!(
        page.contains("| friction | 4 | +0.45% | +0.50% | 0.55% | 2 | 0 |"),
        "{page}"
    );
    assert!(page.contains("| the part below Mach 0.8 | 4 | +0.90% | +0.85% | 0.90% |"));
}

/// One or two flights under a reason, or in a term, give no sizes; a size given anyway, or a
/// sum that doesn't hold, is caught.
#[test]
fn a_small_or_inconsistent_section_is_caught() {
    // One flight failed with a term swapped: no size is given, and the counts still hold.
    let mut flights = section();
    flights.push(TermFlight {
        termed: Err((2, "a flight failed".to_owned())),
        logged_m: Some(1000.0),
    });
    let terms = DragTerms::of(&flights, 4);
    assert!(!terms.published());
    assert_eq!(terms.not_termed[2].flights, 1);
    assert_eq!(terms.check_consistent(4), Ok(()));
    assert!(terms.terms.iter().all(|t| t.part.is_none()));
    assert!(terms.to_markdown().contains("sizes are not given"));
    // A size given while the section isn't published.
    let mut given = terms.clone();
    given.band = size(0.9, 0.85, 0.9);
    assert!(given.check_consistent(4).unwrap_err().contains("given"));
    // One flight with a stated coefficient: the rest's one flight holds every size back.
    let mut flights = section();
    flights[0] = by_hand([1.0, -0.5, 0.2, 0.3], [1.0, -0.5, 0.2, 0.3], Some(1000.0));
    assert!(!DragTerms::of(&flights, 3).published());
    // Sums that don't hold.
    let good = DragTerms::of(&section(), 3);
    let mut bad = good.clone();
    bad.band = size(0.95, 0.85, 0.95);
    assert!(bad.check_consistent(3).unwrap_err().contains("sum to"));
    let mut bad = good.clone();
    bad.terms[1].dominates = Some(2);
    assert!(bad.check_consistent(3).unwrap_err().contains("dominated"));
    let mut bad = good.clone();
    bad.termed = 5;
    assert!(bad.check_consistent(3).is_err());
    let mut bad = good;
    bad.terms.swap(0, 1);
    assert!(
        bad.check_consistent(3)
            .unwrap_err()
            .contains("the terms are")
    );
}

/// The section against the bands': fewer than half the flights split by band split by term is
/// refused, and with every flight split by both the band rows must be the bands'.
#[test]
fn the_section_is_held_to_the_bands() {
    let terms = DragTerms::of(&section(), 3);
    let mut bands = crate::drag_bands::DragBands {
        split: 4,
        banded: 4,
        failed: 0,
        bands: vec![crate::drag_bands::Band {
            part: terms.band,
            alone: terms.band_alone,
            ..Default::default()
        }],
        logs: Logs {
            flights: 3,
            left_out: 0,
            rows: vec![terms.logs.rows[0].clone(), terms.logs.rows[5].clone()],
        },
        ..Default::default()
    };
    assert!(bands.published());
    assert_eq!(terms.check_against_bands(&bands), Ok(()));
    // The band's part, or its row against the logs, not the bands' own.
    let mut other = bands.clone();
    other.bands[0].part = size(0.8, 0.85, 0.9);
    assert!(
        terms
            .check_against_bands(&other)
            .unwrap_err()
            .contains("band's part")
    );
    let mut other = bands.clone();
    other.logs.rows[1].nearer = Some(2);
    assert!(
        terms
            .check_against_bands(&other)
            .unwrap_err()
            .contains("row against")
    );
    bands.banded = 9;
    bands.split = 4;
    assert!(
        terms
            .check_against_bands(&bands)
            .unwrap_err()
            .contains("fewer than half")
    );
    bands.split = 5;
    bands.banded = 4;
    assert!(terms.check_against_bands(&bands).is_err());
}

/// The committed report's section: every flight split by band is split by term, it holds to
/// itself and to the bands, and its page is the report's.
#[test]
fn the_committed_report_splits_the_band_by_term() {
    let root = crate::designs::root().unwrap();
    let text = std::fs::read_to_string(root.join(crate::fixture_flights::REPORT_JSON)).unwrap();
    let report: crate::fixture_flights::Report = serde_json::from_str(&text).unwrap();
    let terms = &report.terms;
    assert!(terms.published());
    assert_eq!(terms.termed, report.bands.banded);
    // The base is the largest of friction, pressure and base: the reference is of it
    // (`crate::aero_base_power`). Its part is nearly all under power.
    assert_eq!(terms.largest_measurable(), Some(2));
    let base = &terms.terms[2];
    let dominates = |i: usize| terms.terms[i].dominates.unwrap();
    assert!(dominates(2) > dominates(0) + dominates(1) + dominates(3));
    let alone = base.alone.unwrap().mean_absolute_percent;
    let powered = terms.base_powered_alone.unwrap().mean_absolute_percent;
    assert!(powered > 0.95 * alone, "{powered} {alone}");
    assert_eq!(terms.check_consistent(report.counts.compared), Ok(()));
    assert_eq!(terms.check_against_bands(&report.bands), Ok(()));
    // The base's measured reference is in the report, row by row below Mach 0.8, as the
    // committed fixture gives it.
    let fixture: Value = serde_json::from_str(
        &std::fs::read_to_string(root.join(crate::aero_base_power::FIXTURE)).unwrap(),
    )
    .unwrap();
    assert_eq!(terms.reference, reference_rows(&fixture).unwrap());
    assert_eq!(terms.reference.len(), 4);
    assert!(terms.reference.iter().all(|r| r.mach < 0.8));
    let page = std::fs::read_to_string(root.join(crate::fixture_flights::REPORT_MD)).unwrap();
    assert!(page.contains(&terms.to_markdown()));
    assert!(page.contains("| 0.70 | 0.0105 | 0.0661 | +0.0556 | 0.0244 | 0.0403 | +0.0159 |"));
    assert!(page.contains("to 2.1 to 2.4 times its power-off value"));
    assert!(page.contains("gives 0.61 of its power-off value"));
    assert!(page.contains("power-on less power-off: 0.0118 to 0.0143."));
}
