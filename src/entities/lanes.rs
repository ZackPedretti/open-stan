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
