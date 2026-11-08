use clir::cli;

mod optional {
    use super::*;

    mod normal {
        use super::*;

        cli! {
            name "test_cli",
            version "1.0",
            description "Test CLI",

            cmd opt_test {
                arg value: Option<i32>,
            },
        }

        #[test]
        fn parses_optional_argument_present() {
            let parsed = parse_env(vec!["opt_test".into(), "42".into()]);

            match parsed {
                Command::OptTest(args, _) => {
                    assert_eq!(args.value, Some(42));
                }
            }
        }

        #[test]
        fn parses_optional_argument_missing() {
            let parsed = parse_env(vec!["opt_test".into()]);

            match parsed {
                Command::OptTest(args, _) => {
                    assert_eq!(args.value, None);
                }
            }
        }
    }

    mod with_default {
        use super::*;

        cli! {
            name "test_cli",
            version "1.0",
            description "Test CLI",

            cmd default_test {
                arg count: Option<i32> = 10,
            },
        }

        #[test]
        fn uses_default_when_missing() {
            let parsed = parse_env(vec!["default_test".into()]);

            match parsed {
                Command::DefaultTest(args, _) => {
                    assert_eq!(args.count, 10);
                }
            }
        }
    }
}

mod required {
    use super::*;

    mod normal {
        use super::*;

        cli! {
            name "test_cli",
            version "1.0",
            description "Test CLI",

            cmd greet {
                arg name: String,
            },
        }

        #[test]
        fn parses_required_argument() {
            let parsed = parse_env(vec!["greet".into(), "John".into()]);

            match parsed {
                Command::Greet(args, _) => {
                    assert_eq!(args.name, "John");
                }
            }
        }
    }

    mod variadic {
        use super::*;

        cli! {
            name "test_cli",
            version "1.0",
            description "Test CLI",

            cmd many {
                arg nums: Vec<i32>,
            },
        }

        #[test]
        fn parses_variadic() {
            let parsed = parse_env(vec!["many".into(), "1".into(), "2".into(), "3".into()]);

            match parsed {
                Command::Many(args, _) => {
                    assert_eq!(args.nums, vec![1, 2, 3]);
                }
            }
        }
    }
}
