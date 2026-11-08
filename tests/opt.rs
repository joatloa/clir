use clir::cli;

mod optional {
    use super::*;

    mod flag {
        use super::*;

        cli! {
            name "test_cli",
            version "1.0",
            description "Test CLI",

            cmd flag_test {
                opt "-v, --verbose",
            },
        }

        #[test]
        fn parses_flag_present() {
            let parsed = parse_env(vec!["flag_test".into(), "--verbose".into()]);

            match parsed {
                Command::FlagTest(_, opts) => {
                    assert_eq!(opts.verbose, true);
                }
            }
        }

        #[test]
        fn parses_flag_absent() {
            let parsed = parse_env(vec!["flag_test".into()]);

            match parsed {
                Command::FlagTest(_, opts) => {
                    assert_eq!(opts.verbose, false);
                }
            }
        }
    }

    mod with_arg {
        use super::*;

        cli! {
            name "test_cli",
            version "1.0",
            description "Test CLI",

            cmd split {
                opt "-s, --separator" {
                    arg separator: String
                }
            },
        }

        #[test]
        fn parses_option_argument_present() {
            let parsed = parse_env(vec!["split".into(), "-s".into(), ",".into()]);

            match parsed {
                Command::Split(_, opts) => {
                    assert_eq!(opts.separator, Some(",".to_string()));
                }
            }
        }
    }

    mod with_args {
        use super::*;

        cli! {
            name "test_cli",
            version "1.0",
            description "Test CLI",

            cmd multi {
                opt "--range" {
                    arg start: i32,
                    arg end: i32,
                },
            },
        }

        #[test]
        fn parses_multi_arg_option() {
            let parsed = parse_env(vec![
                "multi".into(),
                "--range".into(),
                "1".into(),
                "5".into(),
            ]);

            match parsed {
                Command::Multi(_, opts) => {
                    assert_eq!(opts.range.start, Some(1));
                    assert_eq!(opts.range.end, Some(5));
                }
            }
        }
    }
}

mod required {
    use super::*;

    mod flag {
        use super::*;

        cli! {
            name "test_cli",
            version "1.0",
            description "Test CLI",

            cmd flag_required {
                req_opt "-v, --verbose",
            },
        }

        #[test]
        fn parses_required_flag_present() {
            let parsed = parse_env(vec!["flag_required".into(), "--verbose".into()]);

            match parsed {
                Command::FlagRequired(_, opts) => {
                    assert!(opts.verbose);
                }
            }
        }
    }

    mod with_arg {
        use super::*;

        cli! {
            name "test_cli",
            version "1.0",
            description "Test CLI",

            cmd split_required {
                req_opt "-s, --separator" {
                    arg separator: String
                }
            },
        }

        #[test]
        fn parses_required_option_argument() {
            let parsed = parse_env(vec!["split_required".into(), "-s".into(), ",".into()]);

            match parsed {
                Command::SplitRequired(_, opts) => {
                    assert_eq!(opts.separator, ",".to_string());
                }
            }
        }
    }

    mod with_args {
        use super::*;

        cli! {
            name "test_cli",
            version "1.0",
            description "Test CLI",

            cmd multi_required {
                req_opt "--range" {
                    arg start: i32,
                    arg end: i32,
                },
            },
        }

        #[test]
        fn parses_required_multi_arg_option() {
            let parsed = parse_env(vec![
                "multi_required".into(),
                "--range".into(),
                "1".into(),
                "5".into(),
            ]);

            match parsed {
                Command::MultiRequired(_, opts) => {
                    assert_eq!(opts.range.start, 1);
                    assert_eq!(opts.range.end, 5);
                }
            }
        }
    }
}
