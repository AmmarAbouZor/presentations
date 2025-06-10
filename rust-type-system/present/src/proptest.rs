mod example {
    fn parse_time(s: &str) -> Option<(u32, u32, u32)> {
        if s.len() != 8 {
            return None;
        }
        if &s[2..3] != ":" || &s[5..6] != ":" {
            return None;
        }

        let hour = &s[0..2];
        let minute = &s[3..5];
        let second = &s[6..8];

        hour.parse::<u32>().ok().and_then(|h| {
            minute
                .parse::<u32>()
                .ok()
                .and_then(|m| second.parse::<u32>().ok().map(|s| (h, m, s)))
        })
    }

    // #[test]
    // fn test_inputs() {
    //     assert_eq!(parse_time("01:20:30"), Some((1, 20, 30)));
    //     assert!(parse_time("012020").is_none());
    //     assert!(parse_time("").is_none());
    //     assert!(parse_time("another input").is_none());
    // }

    // #[test]
    // fn test_none_ascii() {
    //     // How about none-ASCII characters?
    //     assert!(parse_time("Aௗ-1a0").is_none());
    // }

    use proptest::prelude::*;
    proptest! {
        #[test]
        /// Ensure the function doesn't crash on any condition.
        fn no_crash(s in "\\PC*") {
            parse_time(&s);
        }

        #[test]
        /// Ensure all valid times are parsed.
        fn all_valid_times(s in "[0-9]{2}:[0-9]{2}:[0-9]{2}") {
           parse_time(&s).unwrap();
        }
    }
}
