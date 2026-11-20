use hpr_aero::drag::base_drag_coefficient;

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
/// over the maximum cross-section, `(0.12 + 0.13 M²) A/A_ref`, whatever the body ahead (the
/// boattail's factor on the base is 1 there), with `A` the whole base coasting and the base
/// less the nozzle exit under power.
#[test]
fn the_base_term_is_the_rule_on_the_base_less_the_jet() {
    let root = crate::designs::root().unwrap();
    let reference = read(&root, REFERENCE);
    let fixture = read(&root, FIXTURE);
    let geometry = &reference["geometry"];
    let area =
        |key: &str| 0.25 * std::f64::consts::PI * (geometry[key].as_f64().unwrap() * IN).powi(2);
    let (reference_m2, base_m2, exit_m2) = (
        area("diameter_in"),
        area("base_diameter_in"),
        area("nozzle_exit_diameter_in"),
    );
    assert!(exit_m2 < base_m2 && base_m2 < reference_m2);
    let rows = fixture["rows"].as_array().unwrap();
    assert!(!rows.is_empty());
    for row in rows {
        let mach = row["mach"].as_f64().unwrap();
        assert!(mach < 0.8, "Mach {mach}");
        let rule = base_drag_coefficient(mach).unwrap();
        let close = |key: &str, expected: f64| {
            let got = row[key].as_f64().unwrap();
            assert!(
                (got - expected).abs() < 1e-12,
                "Mach {mach} {key}: {got} {expected}"
            );
        };
        close("hpr_power_off", rule * base_m2 / reference_m2);
        close("hpr_power_on", rule * (base_m2 - exit_m2) / reference_m2);
        close("whole_base_under_power", rule * base_m2 / reference_m2);
    }
}

/// What the reference shows, row by row below Mach 0.8, which the documentation quotes:
///
/// - power off, HPR Sim's base drag is high by more than 0.04, past the stated error and the
///   reading: on this long-boattailed base the measured drag is about a sixth of its rule's;
/// - under power, both rules are high past the stated error and the reading, HPR Sim's (the
///   nozzle exit off the base) by less than the whole base's;
/// - the jet more than doubles the measured base drag, where HPR Sim's rule lowers it to the
///   annulus's share, about 0.61 of power off, and the whole base keeps it.
#[test]
fn every_rule_reads_high_and_the_jet_raises_the_measured_base_drag() {
    let root = crate::designs::root().unwrap();
    let fixture = read(&root, FIXTURE);
    let rows = fixture["rows"].as_array().unwrap();
    assert_eq!(rows.len(), 4);
    let margin = fixture["stated_error"].as_f64().unwrap()
        + fixture["reading_uncertainty"].as_f64().unwrap();
    for row in rows {
        let n = |key: &str| row[key].as_f64().unwrap();
        let mach = n("mach");
        assert!(n("hpr_power_off_difference") > 0.04, "Mach {mach}");
        assert!(n("hpr_power_on_difference") > margin, "Mach {mach}");
        assert!(
            n("whole_base_under_power_difference") > n("hpr_power_on_difference"),
            "Mach {mach}"
        );
        assert!(
            n("measured_power_on") > 2.0 * n("measured_power_off"),
            "Mach {mach}"
        );
        let ratio = n("hpr_power_on") / n("hpr_power_off");
        assert!((ratio - 0.609).abs() < 0.001, "Mach {mach}: {ratio}");
        for key in ["hpr_power_off", "hpr_power_on", "whole_base_under_power"] {
            assert_eq!(row[format!("{key}_within_stated_and_reading")], false);
        }
    }
}
