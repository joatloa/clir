use crate::{Argument, CliDsl, CliOption, get_argument_values, get_option_values};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

pub fn generate_arg_def(arg: &Argument) -> TokenStream2 {
    let (name, desc, optional, variadic) = get_argument_values(arg);

    quote! {
        clir::clir_core::CliArgument::new(#name, #desc, #optional, #variadic)
    }
}

pub fn generate_opt_def(opt: &CliOption) -> TokenStream2 {
    let (opt_name, flags_expr, opt_desc, optional) = get_option_values(opt);
    let opt_arg_defs: Vec<TokenStream2> = opt.arguments.iter().map(generate_arg_def).collect();

    let add_arguments = if opt_arg_defs.is_empty() {
        quote! {}
    } else {
        quote! {
            opt #(.add_argument(#opt_arg_defs))*;
        }
    };

    quote! {
        {
            let mut opt = clir::clir_core::CliOption::new(
                #opt_name,
                #flags_expr,
                #opt_desc,
                #optional,
            );

            #add_arguments

            opt
        }
    }
}

pub fn generate_cli_setup(dsl: &CliDsl) -> TokenStream2 {
    let name = &dsl.name;
    let version = &dsl.version;
    let description = &dsl.description;

    let command_registrations: Vec<TokenStream2> = dsl
        .commands
        .iter()
        .map(|cmd| {
            let cmd_name = cmd.name.to_string();
            let cmd_desc = match &cmd.description {
                Some(d) => quote! { Some(#d) },
                None => quote! { None::<&str> },
            };

            let arg_defs: Vec<TokenStream2> = cmd.arguments.iter().map(generate_arg_def).collect();

            let opt_registrations: Vec<TokenStream2> =
                cmd.options.iter().map(generate_opt_def).collect();

            quote! {
                {
                    let cmd = cli.add_command(#cmd_name, #cmd_desc);
                    cmd
                        #(.add_argument(#arg_defs))*
                        #(.add_option(#opt_registrations))*;
                }
            }
        })
        .collect();

    quote! {
        let mut cli = clir::clir_core::Cli::new(#name, #version, #description);
        #(#command_registrations)*
    }
}
