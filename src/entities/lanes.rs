#[derive(Debug)]
enum Lane {
    T { num: usize },
    Corol,
    N { num: usize },
    Citadine { num: usize },
}

impl Lane {
    pub fn new_num_lane(num: usize) -> Option<Self> {
        if matches!(num,
            10..=18 |
            20..=24 |
            30 |
            32..=33 |
            50..=67
        ) {
            return Some(Lane::N { num });
        }
        None
    }

    pub fn new_tempo_lane(num: usize) -> Option<Self> {
        if num > 0 && num < 6 {
            return Some(Lane::T { num });
        }
        None
    }

    pub fn new_citadine_lane(num: usize) -> Option<Self> {
        if num > 0 && num < 3 {
            return Some(Lane::Citadine { num });
        }
        None
    }

    pub fn from_text(text: &str) -> Option<Self> {
        match text {
            "Corol" => Some(Lane::Corol),

            t if t.starts_with("T") => {
                t[1..].parse().ok().and_then(Lane::new_tempo_lane)
            }

            t if t.starts_with("Citadine ") => {
                t["Citadine ".len()..].parse().ok().and_then(Lane::new_citadine_lane)
            }

            _ => text.parse().ok().and_then(Lane::new_num_lane),
        }
    }

    pub fn to_text(&self) -> String {
        match self {
            Lane::T { num } => {
                format!("T{}", num)
            }
            Lane::Corol => "Corol".into(),
            Lane::N { num } => format!("{}", num),
            Lane::Citadine { num } => {
                format!("Citadine {}", num)
            }
        }
    }
}

// Yes, this is ugly. But this is done for performance: it is statically defined to be of type &'static[Lane]
impl Lane {
    pub fn all() -> &'static [Lane] {
        const ALL_LANES: &[Lane] = &[
            // T lanes
            Lane::T { num: 1 },
            Lane::T { num: 2 },
            Lane::T { num: 3 },
            Lane::T { num: 4 },
            Lane::T { num: 5 },

            // Corol
            Lane::Corol,

            // N lanes
            Lane::N { num: 10 },
            Lane::N { num: 11 },
            Lane::N { num: 12 },
            Lane::N { num: 13 },
            Lane::N { num: 14 },
            Lane::N { num: 15 },
            Lane::N { num: 16 },
            Lane::N { num: 17 },
            Lane::N { num: 18 },
            Lane::N { num: 20 },
            Lane::N { num: 21 },
            Lane::N { num: 22 },
            Lane::N { num: 23 },
            Lane::N { num: 24 },
            Lane::N { num: 30 },
            Lane::N { num: 32 },
            Lane::N { num: 33 },
            Lane::N { num: 50 },
            Lane::N { num: 51 },
            Lane::N { num: 52 },
            Lane::N { num: 53 },
            Lane::N { num: 54 },
            Lane::N { num: 55 },
            Lane::N { num: 56 },
            Lane::N { num: 57 },
            Lane::N { num: 58 },
            Lane::N { num: 59 },
            Lane::N { num: 60 },
            Lane::N { num: 61 },
            Lane::N { num: 62 },
            Lane::N { num: 63 },
            Lane::N { num: 64 },
            Lane::N { num: 65 },
            Lane::N { num: 66 },
            Lane::N { num: 67 },

            // Citadine lanes
            Lane::Citadine { num: 1 },
            Lane::Citadine { num: 2 },
        ];

        ALL_LANES
    }
}


#[cfg(test)]
mod tests {
    use super::Lane;

    // Helper that encodes the documented valid numeric lanes.
    fn is_valid_num_lane(n: usize) -> bool {
        matches!(n,
            10..=18 |
            20..=24 |
            30 |
            32 |
            33 |
            50..=67
        )
    }

    #[test]
    fn test_new_num_lane_exhaustive() {
        // test a little beyond the documented range to ensure edge behaviour
        for n in 0..=70 {
            let expect = is_valid_num_lane(n);
            let got = Lane::new_num_lane(n).is_some();
            assert_eq!(got, expect, "new_num_lane({}) => {}, expected {}", n, got, expect);
        }
    }

    #[test]
    fn test_new_tempo_lane_exhaustive() {
        // Tempo lanes are valid from T1 to T5, others invalid
        for n in 0..=10 {
            let expect = (1..=5).contains(&n);
            let got = Lane::new_tempo_lane(n).is_some();
            assert_eq!(got, expect, "new_tempo_lane({}) => {}, expected {}", n, got, expect);
        }
    }

    #[test]
    fn test_new_citadine_lane_exhaustive() {
        // Citadine are valid from Citadine 1 to Citadine 2, others invalid
        for n in 0..=5 {
            let expect = (1..=2).contains(&n);
            let got = Lane::new_citadine_lane(n).is_some();
            assert_eq!(got, expect, "new_citadine_lane({}) => {}, expected {}", n, got, expect);
        }
    }

    #[test]
    fn test_from_text_numbers_and_roundtrip() {
        // check all numeric strings in range and roundtrip via to_text
        for n in 0..=70 {
            let text = n.to_string();
            let parsed = Lane::from_text(&text);
            let expect = is_valid_num_lane(n);

            assert_eq!(parsed.is_some(), expect, "from_text(\"{}\") existence mismatch", text);

            if expect {
                // verify it becomes an N { num } and round-trips to the same textual representation
                let lane = parsed.expect("expected Some(Lane::N)");
                match lane {
                    Lane::N { num } => {
                        assert_eq!(num, n);
                        let out = lane.to_text();
                        assert_eq!(out, text, "roundtrip to_text mismatch for N {}", n);
                    }
                    other => panic!("expected Lane::N for '{}', got {:?}", text, other),
                }
            }
        }
    }

    #[test]
    fn test_from_text_tempo_and_roundtrip() {
        for n in 0..=10 {
            let text = format!("T{}", n);
            let parsed = Lane::from_text(&text);
            let expect = (1..=5).contains(&n);
            assert_eq!(parsed.is_some(), expect, "from_text(\"{}\") existence mismatch", text);

            if expect {
                let lane = parsed.expect("expected Some(Lane::T)");
                match lane {
                    Lane::T { num } => {
                        assert_eq!(num, n);
                        let out = lane.to_text();
                        assert_eq!(out, text, "roundtrip to_text mismatch for T{}", n);
                    }
                    other => panic!("expected Lane::T for '{}', got {:?}", text, other),
                }
            }
        }

        // non-numeric suffix after T should fail
        assert!(Lane::from_text("Tabc").is_none());
        assert!(Lane::from_text("T").is_none()); // empty number
    }

    #[test]
    fn test_from_text_citadine_and_roundtrip() {
        for n in 0..=5 {
            let text = format!("Citadine {}", n);
            let parsed = Lane::from_text(&text);
            let expect = (1..=2).contains(&n);
            assert_eq!(parsed.is_some(), expect, "from_text(\"{}\") existence mismatch", text);

            if expect {
                let lane = parsed.expect("expected Some(Lane::Citadine)");
                match lane {
                    Lane::Citadine { num } => {
                        assert_eq!(num, n);
                        let out = lane.to_text();
                        assert_eq!(out, text, "roundtrip to_text mismatch for Citadine {}", n);
                    }
                    other => panic!("expected Lane::Citadine for '{}', got {:?}", text, other),
                }
            }
        }

        assert!(Lane::from_text("Citadine").is_none()); // missing number
        assert!(Lane::from_text("Citadine X").is_none()); // invalid number
    }

    #[test]
    fn test_corol_text() {
        // "Corol" must parse and roundtrip
        let parsed = Lane::from_text("Corol");
        assert!(parsed.is_some(), "Corol should parse");
        let lane = parsed.unwrap();
        match lane {
            Lane::Corol => {
                let out = lane.to_text();
                assert_eq!(out, String::from("Corol"), "Corol roundtrip to_text mismatch");
            }
            other => panic!("expected Lane::Corol, got {:?}", other),
        }
    }

    #[test]
    fn test_invalid_names() {
        // a few strings that should definitely fail
        let invalids = [
            "", " ", "foo", "42x", "T0", "T6", "Citadine 0", "Citadine 3", "19", "25", "31", "68", "1000"
        ];
        for &s in &invalids {
            assert!(Lane::from_text(s).is_none(), "expected from_text(\"{}\") to be None", s);
        }
    }

    #[test]
    fn test_to_text_formats() {
        // Construct lanes via constructors (where applicable) and verify to_text output
        if let Some(l) = Lane::new_tempo_lane(1) {
            assert_eq!(l.to_text(), String::from("T1"));
        } else {
            panic!("expected new_tempo_lane(1) to be Some");
        }

        if let Some(l) = Lane::new_citadine_lane(2) {
            assert_eq!(l.to_text(), String::from("Citadine 2"));
        } else {
            panic!("expected new_citadine_lane(2) to be Some");
        }

        if let Some(l) = Lane::new_num_lane(10) {
            assert_eq!(l.to_text(), String::from("10"));
        } else {
            panic!("expected new_num_lane(10) to be Some");
        }
    }
}

#[test]
fn test_all_lanes_complete_and_consistent() {
    let all = Lane::all();

    // --- 1️⃣ Check total count ---
    // From docs:
    // T1-5 (5) + Corol (1) + N(10–18=9, 20–24=5, 30,32,33=3, 50–67=18) + Citadine(2)
    // Total = 5 + 1 + (9+5+3+18) + 2 = 43 + 8 = 51
    let expected_count = 5 + 1 + 9 + 5 + 3 + 18 + 2;
    assert_eq!(
        all.len(),
        expected_count,
        "Unexpected number of lanes in Lane::all()"
    );

    // --- 2️⃣ Check no duplicates ---
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    for lane in all {
        let key = lane.to_text(); // String
        assert!(
            seen.insert(key.clone()),
            "Duplicate lane found: {}",
            key
        );
    }

    // --- 3️⃣ Check all known lanes are present ---
    // Build expected lane names according to spec
    let mut expected_names = Vec::new();

    // T1..=5
    for n in 1..=5 {
        expected_names.push(format!("T{}", n));
    }
    // Corol
    expected_names.push("Corol".to_string());

    // N lanes
    for n in 10..=18 {
        expected_names.push(n.to_string());
    }
    for n in 20..=24 {
        expected_names.push(n.to_string());
    }
    for n in [30, 32, 33] {
        expected_names.push(n.to_string());
    }
    for n in 50..=67 {
        expected_names.push(n.to_string());
    }

    // Citadine
    for n in 1..=2 {
        expected_names.push(format!("Citadine {}", n));
    }

    let all_names: Vec<_> = all.iter().map(|l| l.to_text()).collect();

    for expected in &expected_names {
        assert!(
            all_names.contains(expected),
            "Missing expected lane '{}'",
            expected
        );
    }

    // --- 4️⃣ Roundtrip consistency ---
    for lane in all {
        let text = lane.to_text();
        let parsed = Lane::from_text(&text)
            .unwrap_or_else(|| panic!("Lane::from_text failed for '{}'", text));
        let re_text = parsed.to_text();
        assert_eq!(
            text, re_text,
            "Roundtrip mismatch: '{}' → {:?} → '{}'",
            text, parsed, re_text
        );
    }
}
