use dsl_cli::cli;

cli! {
    name "string_utils",
    version "0.1.0",
    description "A simple CLI for string utilities",

    cmd split "Split a string by a separator" {
        arg string "The string to split",
        req_opt "-s, --separator" "The separator to use" {
            arg string
        },
    },
}

fn main() {
    let parsed = parse_env(std::env::args().skip(1).collect());

    match parsed {
        Command::Split(args, opts) => {
            println!(
                "Split string: {}",
                args.string
                    .split(&opts.separator)
                    .collect::<Vec<&str>>()
                    .join(" ")
            );
        }
    }
}
