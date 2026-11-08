use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{Ident, parse_quote};

use crate::{
    Argument, CliDsl, CliOption, Command, generate_args_struct_name, generate_opts_struct_name,
    get_effective_type, is_optional_type, is_variadic_type, parse_flags, to_pascal_case,
};

pub fn generate_command_types(dsl: &CliDsl) -> (Vec<TokenStream2>, Vec<TokenStream2>) {
    dsl.commands
        .iter()
        .map(|cmd| {
            let cmd_name_pascal = to_pascal_case(&cmd.name.to_string());

            (
                generate_args_structs(&cmd.arguments, &cmd_name_pascal, cmd.span),
                generate_opts_structs(&cmd.options, &cmd_name_pascal, cmd.span),
            )
        })
        .collect()
}

pub fn generate_args_structs(
    args: &Vec<Argument>,
    pascal_prefix: &str,
    span: Span,
) -> TokenStream2 {
    let struct_name = Ident::new(
        &format!("{}", generate_args_struct_name(pascal_prefix)),
        span,
    );

    let fields: Vec<TokenStream2> = args
        .iter()
        .map(|arg| {
            let field_name = Ident::new(&arg.name.to_string(), arg.span);
            let field_type = get_effective_type(arg);
            quote! { pub #field_name: #field_type }
        })
        .collect();

    quote! {
        #[derive(Debug)]
        pub struct #struct_name {
            #(#fields),*
        }
    }
}

pub fn generate_opts_structs(
    opts: &Vec<CliOption>,
    pascal_prefix: &str,
    span: Span,
) -> TokenStream2 {
    let struct_name = Ident::new(
        &format!("{}", generate_opts_struct_name(pascal_prefix)),
        span,
    );

    let mut nested_structs = Vec::new();
    let mut fields = Vec::new();

    for opt in opts {
        let (_, _, opt_name) = parse_flags(&opt.flags.value());
        let field_name = Ident::new(&opt_name, opt.span);

        match opt.arguments.len() {
            0 => fields.push(quote! { pub #field_name: bool }),
            1 => {
                let arg = &opt.arguments[0];
                // Option type should be optional if the option itself is optional,
                // regardless of whether the argument type is non-Option.
                //
                // Defaults always produce a concrete value, so unwrap Option<T> when a default exists.
                let field_type = if arg.default.is_some() {
                    get_effective_type(arg)
                } else if opt.required || is_optional_type(&arg.ty) {
                    arg.ty.clone()
                } else {
                    let ty = arg.ty.clone();
                    parse_quote!(Option<#ty>)
                };

                fields.push(quote! { pub #field_name: #field_type });
            }
            _ => {
                let pascal_prefix = format!("{}{}", pascal_prefix, to_pascal_case(&opt_name));
                let nested_struct_name = Ident::new(
                    &format!("{}", generate_args_struct_name(&pascal_prefix)),
                    opt.span,
                );

                let nested_fields: Vec<TokenStream2> = opt
                    .arguments
                    .iter()
                    .map(|arg| {
                        let field_name = Ident::new(&arg.name.to_string(), arg.span);
                        let field_type = if arg.default.is_some() {
                            get_effective_type(arg)
                        } else if opt.required || is_optional_type(&arg.ty) {
                            arg.ty.clone()
                        } else {
                            let ty = arg.ty.clone();
                            parse_quote!(Option<#ty>)
                        };
                        quote! { pub #field_name: #field_type }
                    })
                    .collect();

                if !opt.required {
                    nested_structs.push(quote! {
                        #[derive(Debug, Default)]
                        pub struct #nested_struct_name {
                            #(#nested_fields),*
                        }
                    })
                } else {
                    nested_structs.push(quote! {
                        #[derive(Debug)]
                        pub struct #nested_struct_name {
                            #(#nested_fields),*
                        }
                    })
                }

                fields.push(quote! { pub #field_name: #nested_struct_name });
            }
        }
    }

    quote! {
        #(#nested_structs)*

        #[derive(Debug)]
        pub struct #struct_name {
            #(#fields),*
        }
    }
}

// Generate the fields for the command enum
pub fn generate_fields_for_command_enum(commands: &Vec<Command>) -> Vec<TokenStream2> {
    commands
        .iter()
        .map(|cmd| {
            let cmd_name_pascal = to_pascal_case(&cmd.name.to_string());
            let cmd_name_str = format_ident!("{}", cmd_name_pascal);
            let args_struct = format_ident!("{}", generate_args_struct_name(&cmd_name_pascal));
            let opts_struct = format_ident!("{}", generate_opts_struct_name(&cmd_name_pascal));

            quote! { #cmd_name_str(#args_struct, #opts_struct) }
        })
        .collect()
}

pub fn generate_match_arms(dsl: &CliDsl) -> Vec<TokenStream2> {
    let mut match_arms: Vec<TokenStream2> = dsl
        .commands
        .iter()
        .filter_map(|cmd| {
            let cmd_name_str = cmd.name.to_string();

            // We need to add the cli command last since it is a "_" pattern which will match for everything
            // including our existing commands
            if cmd_name_str == "cli" {
                return None;
            }

            let cmd_name_pascal = to_pascal_case(&cmd_name_str);

            let cmd_ident = format_ident!("{}", cmd_name_pascal);

            let args_struct = format_ident!("{}", generate_args_struct_name(&cmd_name_pascal));
            let opts_struct = format_ident!("{}", generate_opts_struct_name(&cmd_name_pascal));

            // Commands without actions print debug info about parsed args/opts
            Some(quote! {
                #cmd_name_str => {
                    Command::#cmd_ident(
                        #args_struct::from_parsed(parsed_args),
                        #opts_struct::from_parsed(parsed_opts)
                    )
                }
            })
        })
        .collect();

    if dsl.commands.iter().any(|cmd| cmd.name.to_string() == "cli") {
        match_arms.push(quote! {
            _ => {
                return Command::Cli(
                    CliArgs::from_parsed(parsed_args),
                    CliOpts::from_parsed(parsed_opts)
                );
            }
        });
    }

    match_arms
}

// -------------------------------------------------------------------------------------------

pub fn generate_impls(dsl: &CliDsl) -> (Vec<TokenStream2>, Vec<TokenStream2>) {
    let args_from_parsed: Vec<TokenStream2> = dsl
        .commands
        .iter()
        .map(generate_from_parsed_impl_for_args)
        .collect();

    let opts_from_parsed: Vec<TokenStream2> = dsl
        .commands
        .iter()
        .map(generate_from_parsed_impl_for_opts)
        .collect();

    (args_from_parsed, opts_from_parsed)
}

pub fn generate_from_parsed_impl_for_args(cmd: &Command) -> TokenStream2 {
    let struct_name = format_ident!(
        "{}",
        generate_args_struct_name(&to_pascal_case(&cmd.name.to_string()))
    );

    let field_extractions: Vec<TokenStream2> = cmd
        .arguments
        .iter()
        .map(|arg| {
            let field_name = &arg.name;

            let field_name_str = field_name.to_string();

            generate_arg_extraction(field_name, &field_name_str, arg, false)
        })
        .collect();

    let field_names: Vec<&Ident> = cmd.arguments.iter().map(|arg| &arg.name).collect();

    quote! {
        impl clir::clir_core::FromParsedArgs for #struct_name {
            fn from_parsed(mut parsed: clir::clir_core::ParsedArgs) -> Self {
                #(#field_extractions)*
                Self {
                    #(#field_names),*
                }
            }
        }
    }
}

pub fn generate_from_parsed_impl_for_opts(cmd: &Command) -> TokenStream2 {
    let struct_name = format_ident!("{}Opts", to_pascal_case(&cmd.name.to_string()));
    let cmd_pascal = to_pascal_case(&cmd.name.to_string());

    let mut nested_impls = Vec::new();
    let mut field_extractions = Vec::new();
    let mut field_names = Vec::new();

    for opt in &cmd.options {
        let (_, _, opt_name) = parse_flags(&opt.flags.value());
        let field_name = format_ident!("{}", opt_name);
        field_names.push(field_name.clone());

        let arg_count = opt.arguments.len();
        let is_optional = !opt.required;

        match (arg_count, is_optional) {
            (0, false) => {
                field_extractions.push(quote! {
                    let #field_name: bool = {
                        let val = parsed.remove(#opt_name).unwrap();
                        val.as_flag()
                    };
                });
            }
            (0, true) => {
                field_extractions.push(quote! {
                    let #field_name: bool = {
                        let val = parsed.remove(#opt_name).unwrap();
                        if !val.is_none() {
                            val.as_flag()
                        } else {
                            false
                        }
                    };
                });
            }
            (1, is_scope_optional) => {
                let arg = &opt.arguments[0];
                let field_name = &Ident::new(&opt_name, Span::call_site());
                let field_name_str = opt_name;
                field_extractions.push(generate_arg_extraction(
                    field_name,
                    &field_name_str,
                    arg,
                    is_scope_optional,
                ));
            }
            (_, is_scope_optional) => {
                // Multiple arguments - use nested struct
                let nested_prefix = format!("{}{}", cmd_pascal, to_pascal_case(&opt_name));
                let nested_struct_name =
                    format_ident!("{}", generate_args_struct_name(&nested_prefix));

                // Generate FromParsed for nested struct
                let nested_field_extractions: Vec<TokenStream2> = opt
                    .arguments
                    .iter()
                    .map(|arg| {
                        let arg_field_name = &arg.name;
                        let arg_field_name_str = arg_field_name.to_string();

                        generate_arg_extraction(
                            arg_field_name,
                            &arg_field_name_str,
                            arg,
                            is_scope_optional,
                        )
                    })
                    .collect();

                let nested_field_names: Vec<&Ident> =
                    opt.arguments.iter().map(|arg| &arg.name).collect();

                nested_impls.push(quote! {
                    impl clir::clir_core::FromParsedArgs for #nested_struct_name {
                        fn from_parsed(mut parsed: clir::clir_core::ParsedArgs) -> Self {
                            #(#nested_field_extractions)*
                            Self {
                                #(#nested_field_names),*
                            }
                        }
                    }
                });

                field_extractions.push(quote! {
                    let #field_name: #nested_struct_name = {
                        let val = parsed.remove(#opt_name).unwrap();
                        #nested_struct_name::from_parsed(val.as_args())
                    };
                });
            }
        }
    }

    quote! {
        #(#nested_impls)*

        impl clir::clir_core::FromParsedOpts for #struct_name {
            fn from_parsed(mut parsed: clir::clir_core::ParsedOpts) -> Self {
                #(#field_extractions)*
                Self {
                    #(#field_names),*
                }
            }
        }
    }
}

fn generate_arg_extraction(
    field_name: &syn::Ident,
    field_name_str: &str,
    arg: &Argument,
    scope_optional: bool,
) -> TokenStream2 {
    let is_optional = scope_optional || is_optional_type(&arg.ty);
    let is_variadic = is_variadic_type(&arg.ty);
    let has_default = arg.default.is_some();

    let ty = get_effective_type(arg);
    let field_type = if scope_optional && !has_default {
        syn::parse_quote!(Option<#ty>)
    } else {
        ty
    };

    match (is_optional, is_variadic, has_default) {
        // T
        (false, false, false) => {
            return quote! {
                let #field_name: #field_type = {
                    let val = parsed.remove(#field_name_str).unwrap();
                    val.as_value().parse().unwrap()
                };
            };
        }
        // Option<T>
        (true, false, false) => {
            return quote! {
                let #field_name: #field_type = {
                    let val = parsed.remove(#field_name_str).unwrap();
                    if !val.is_none() {
                        Some(val.as_value().parse().unwrap())
                    } else {
                        None
                    }
                };
            };
        }
        // Vec<T>
        (false, true, false) => {
            return quote! {
                let #field_name: #field_type = {
                    let val = parsed.remove(#field_name_str).unwrap();
                    let vec_val = val.as_variadic();
                    vec_val.iter().map(|s| s.parse().unwrap()).collect()
                };
            };
        }
        // Option<Vec<T>>
        (true, true, false) => {
            return quote! {
                let #field_name: #field_type = {
                    let val = parsed.remove(#field_name_str).unwrap();
                    if !val.is_none() {
                        let vec_val = val.as_variadic();
                        Some(vec_val.iter().map(|s| s.parse().unwrap()).collect())
                    } else {
                        None
                    }
                };
            };
        }
        // Option<Vec<T>> with default
        (true, true, true) => {
            let default_val = arg.default.as_ref().unwrap();
            return quote! {
                let #field_name: #field_type = {
                    let val = parsed.remove(#field_name_str).unwrap();
                    if !val.is_none() {
                        let vec_val = val.as_variadic();
                        vec_val.iter().map(|s| s.parse().unwrap()).collect()
                    } else {
                        #default_val
                    }
                };
            };
        }
        // Option<T> with default
        (true, false, true) => {
            let default_val = arg.default.as_ref().unwrap();
            return quote! {
                let #field_name: #field_type = {
                    let val = parsed.remove(#field_name_str).unwrap();
                    if !val.is_none() {
                        val.as_value().parse().unwrap()
                    } else {
                        #default_val
                    }
                };
            };
        }
        _ => unreachable!(),
    }
}
