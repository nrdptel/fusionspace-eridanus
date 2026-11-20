use super::*;

fn read(root: &Path, name: &str) -> Value {
    serde_json::from_str(&fs::read_to_string(root.join(name)).unwrap()).unwrap()
}

#[test]
fn committed_fixture_is_current() {
    let root = crate::designs::root().unwrap();
    let committed = read(&root, FIXTURE);
    let fresh = generate(&root).unwrap();
    assert!(
        crate::designs::same(&committed, &fresh),
        "{FIXTURE} differs from `cargo xtask aero`: {}",
        crate::designs::difference(&committed, &fresh).unwrap_or_default()
    );
}

/// Below Mach 0.8 HPR Sim's base term for the drawn model is its base rule on the base's area
/// over the maximum cross-section, `(0.12 + 0.13 M²) A/A_ref` (Niskanen 2009, eq. 3.94), times
/// `d_B/d` behind the boattail (NASA TR R-100, 1961, p. 11; M1.14i3, ADR-362), each written out
/// here so that a change to the coefficient or the relief fails. `A` is the whole base coasting,
/// the base less the nozzle exit under power, and nothing with the motor case off, the case
/// being wider than the base. The rule before M1.14i3 is the same without `d_B/d`.
#[test]
fn the_base_term_is_the_rule_on_the_base_less_the_jet() {
    let root = crate::designs::root().unwrap();
    let reference = read(&root, REFERENCE);
    let fixture = read(&root, FIXTURE);
    let geometry = &reference["geometry"];
    let area =
        |key: &str| 0.25 * std::f64::consts::PI * (geometry[key].as_f64().unwrap() * IN).powi(2);
    let (reference_m2, base_m2, exit_m2, case_m2) = (
        area("diameter_in"),
        area("base_diameter_in"),
        area("nozzle_exit_diameter_in"),
        area("motor_case_diameter_in"),
    );
    assert!(exit_m2 < base_m2 && base_m2 < reference_m2 && base_m2 < case_m2);
    // d_B/d, from the diameters as given.
    let relief = geometry["base_diameter_in"].as_f64().unwrap()
        / geometry["diameter_in"].as_f64().unwrap();
    assert!((relief - 0.6).abs() < 1e-12, "{relief}");
    let rows = fixture["rows"].as_array().unwrap();
    assert!(!rows.is_empty());
    for row in rows {
        let mach = row["mach"].as_f64().unwrap();
        assert!(mach < 0.8, "Mach {mach}");
        let rule = 0.12 + 0.13 * mach * mach;
        let close = |key: &str, expected: f64| {
            let got = row[key].as_f64().unwrap();
            assert!(
                (got - expected).abs() < 1e-12,
                "Mach {mach} {key}: {got} {expected}"
            );
        };
        close("hpr_power_off", relief * rule * base_m2 / reference_m2);
        close(
            "hpr_power_on",
            relief * rule * (base_m2 - exit_m2) / reference_m2,
        );
        close("hpr_power_on_case", 0.0);
        close("whole_base_under_power", relief * rule * base_m2 / reference_m2);
        close("flat_base_rule_power_off", rule * base_m2 / reference_m2);
        // Hoerner 1965, p. 3-20, eq. 35, written out: 0.029 (d_B/d)^3 / sqrt(C_Df), C_Df the
        // body's drag less its base.
        let forebody = row["hpr_forebody_drag"].as_f64().unwrap();
        assert!(forebody > 0.0, "Mach {mach}: {forebody}");
        close(
            "hoerner_power_off",
            0.029 * (base_m2 / reference_m2).powf(1.5) / forebody.sqrt(),
        );
    }
}

/// What the reference shows, row by row below Mach 0.8, which the documentation quotes:
///
/// - power off, HPR Sim's base drag (M1.14i3's rule) reads high on every row, past the stated
///   error and the reading, by 0.025 to 0.035: 3.0 to 5.1 times the measured. Too much drag, so
///   too low an apogee. The rule before M1.14i3 read higher on every row, Hoerner's lower; no row's
///   error grows, and the mean absolute error falls by about half;
/// - under power, HPR Sim's rule as it flies, the motor case off the base, leaves no base drag
///   and reads low past the stated error and the reading: too little drag, so too high an apogee;
/// - given only the nozzle exit, the same rule is within 0.007 of the measured, and the whole
///   base reads high, past the stated error and the reading;
/// - the jet more than doubles the measured base drag, where HPR Sim's rule lowers it to the
///   annulus's share, about 0.61 of power off, and the whole base keeps it.
///
/// Mutation probes (ADR-362): HPR Sim's relief set back to 1, or squared (`a_b` for `√a_b`),
/// fails this test or the one above.
#[test]
fn the_power_off_rule_reads_high_the_flown_power_on_rule_low() {
    let root = crate::designs::root().unwrap();
    let fixture = read(&root, FIXTURE);
    let rows = fixture["rows"].as_array().unwrap();
    assert_eq!(rows.len(), 4);
    let margin = fixture["stated_error"].as_f64().unwrap()
        + fixture["reading_uncertainty"].as_f64().unwrap();
    let (mut before, mut after) = (0.0, 0.0);
    for row in rows {
        let n = |key: &str| row[key].as_f64().unwrap();
        let mach = n("mach");
        let hpr = n("hpr_power_off_difference");
        let flat = n("flat_base_rule_power_off_difference");
        let hoerner = n("hoerner_power_off_difference");
        // Which way it errs: high, past the stated error and the reading.
        assert!(hpr > margin && hpr < 0.036, "Mach {mach}: {hpr}");
        let times = n("hpr_power_off") / n("measured_power_off");
        assert!((2.95..5.1).contains(&times), "Mach {mach}: {times}");
        // Between Hoerner's rule and the rule before M1.14i3, each high too.
        assert!(hoerner > margin && hoerner < hpr && hpr < flat, "Mach {mach}");
        assert!(hpr.abs() < flat.abs(), "Mach {mach}: the error grows");
        before += flat.abs() / 4.0;
        after += hpr.abs() / 4.0;
        assert_eq!(n("hpr_power_on_case"), 0.0, "Mach {mach}");
        assert!(n("hpr_power_on_case_difference") < -margin, "Mach {mach}");
        assert!(n("hpr_power_on_difference").abs() < 0.007, "Mach {mach}");
        assert!(
            n("whole_base_under_power_difference") > margin.max(n("hpr_power_on_difference")),
            "Mach {mach}"
        );
        assert!(
            n("measured_power_on") > 2.0 * n("measured_power_off"),
            "Mach {mach}"
        );
        let ratio = n("hpr_power_on") / n("hpr_power_off");
        assert!((ratio - 0.609).abs() < 0.001, "Mach {mach}: {ratio}");
        for key in [
            "hpr_power_off",
            "hpr_power_on_case",
            "whole_base_under_power",
            "hoerner_power_off",
            "flat_base_rule_power_off",
        ] {
            assert_eq!(row[format!("{key}_within_stated_and_reading")], false);
        }
    }
    // The mean absolute error over the four rows: 0.0574 before, 0.0303 after.
    assert!((before - 0.0574).abs() < 5e-5, "{before}");
    assert!((after - 0.0303).abs() < 5e-5, "{after}");
}
