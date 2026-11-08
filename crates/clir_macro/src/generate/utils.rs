use crate::{Argument, CliOption};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Type;

/// Parses `Ast Argument` fields into values that can be used to create a CliArgument
pub fn get_argument_values(arg: &Argument) -> (String, TokenStream2, bool, bool) {
    let name = arg.name.to_string();
    let desc = match &arg.description {
        Some(d) => quote! { Some(#d) },
        None => quote! { None::<&str> },
    };
    let optional = is_optional_type(&arg.ty);
    let variadic = is_variadic_type(&arg.ty);

    (name, desc, optional, variadic)
}

/// Parses `Ast Option` fields into values that can be used to create a CliOption
pub fn get_option_values(opt: &CliOption) -> (String, TokenStream2, TokenStream2, bool) {
    let flags_str = opt.flags.value();
    let (short, long, opt_name) = parse_flags(&flags_str);

    let flags_expr = match (short, long.as_ref()) {
        (Some(s), Some(l)) => {
            quote! { clir::clir_core::CliOptionFlags::ShortAndLong(#s, #l.to_string()) }
        }
        (Some(s), None) => quote! { clir::clir_core::CliOptionFlags::Short(#s) },
        (None, Some(l)) => quote! { clir::clir_core::CliOptionFlags::Long(#l.to_string()) },
        (None, None) => unreachable!(),
    };

    let opt_desc = match &opt.description {
        Some(d) => quote! { Some(#d) },
        None => quote! { None::<&str> },
    };

    let optional = !opt.required;

    (opt_name, flags_expr, opt_desc, optional)
}

/// Convert snake_case to PascalCase
pub fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(c) => c.to_uppercase().chain(chars).collect(),
                None => String::new(),
            }
        })
        .collect()
}

/// Parse flags like "-t, --times" into (Option<char>, Option<String>, name)
pub fn parse_flags(flags_str: &str) -> (Option<char>, Option<String>, String) {
    let parts: Vec<&str> = flags_str.split(',').map(|s| s.trim()).collect();
    let mut short = None;
    let mut long = None;

    for part in parts {
        if part.starts_with("--") {
            long = Some(part.trim_start_matches("--").to_string());
        } else if part.starts_with('-') {
            short = part.trim_start_matches('-').chars().next();
        }
    }

    let name = long
        .clone()
        .or_else(|| short.map(|c| c.to_string()))
        .unwrap_or_default()
        .replace('-', "_");

    (short, long, name)
}

/// Check if a type is Option<T> or Option<Vec<T>>
pub fn is_optional_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.first() {
            return segment.ident == "Option";
        }
    }
    false
}

/// Check if a type is Vec<T> or Option<Vec<T>>
pub fn is_variadic_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.first() {
            if segment.ident == "Vec" {
                return true;
            }
            if segment.ident == "Option" {
                // Check inner type for Vec
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        return is_variadic_type(inner_ty);
                    }
                }
            }
        }
    }
    false
}

/// Get the inner type from Option<T> - returns T
pub fn get_inner_option_type(ty: &Type) -> Option<&Type> {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.first() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
                        return Some(inner);
                    }
                }
            }
        }
    }
    None
}

/// Get the effective type (unwrap Option if default is provided)
pub fn get_effective_type(arg: &Argument) -> Type {
    if arg.default.is_some() {
        if let Some(inner) = get_inner_option_type(&arg.ty) {
            return inner.clone();
        }
    }
    arg.ty.clone()
}

/// Generate a struct name
pub fn generate_args_struct_name(prefix: &str) -> String {
    format!("{}Args", prefix)
}

/// Generate a struct name
pub fn generate_opts_struct_name(prefix: &str) -> String {
    format!("{}Opts", prefix)
}
