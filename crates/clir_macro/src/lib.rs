use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod generate;
use generate::*;

mod ast;
use ast::*;

#[proc_macro]
pub fn cli(input: TokenStream) -> TokenStream {
    let dsl = parse_macro_input!(input as CliDsl);

    let cli_setup = generate_cli_setup(&dsl);

    let (args_structs, opts_structs) = generate_command_types(&dsl);
    let (args_from_parsed_impl, opts_from_parsed_impl) = generate_impls(&dsl);
    let command_fields = generate_fields_for_command_enum(&dsl.commands);
    let match_arms = generate_match_arms(&dsl);

    quote! {
        // These structs are generated for each command so we can parse into them later
        #(#args_structs)*
        #(#opts_structs)*

        // Generated Commands enum
        pub enum Command {
            #(#command_fields),*
        }

        #[allow(non_local_definitions)]
        pub fn parse_env(env_args: Vec<String>) -> Command {
            // CLI setup
            #cli_setup

            // FromParsed implementations
            use clir::clir_core::{FromParsedArgs, FromParsedOpts};

            #(#args_from_parsed_impl)*
            #(#opts_from_parsed_impl)*

            // Command matching and parsing
            let command_name = env_args.first().map(|s| s.as_str()).unwrap_or("").to_string();
            let (parsed_args, parsed_opts) = cli.parse(env_args);

            match command_name.as_str() {
                #(#match_arms),*
                _ => unreachable!()
            }
        }
    }
    .into()
}
