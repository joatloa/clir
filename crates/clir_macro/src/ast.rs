use proc_macro2::{Span, TokenTree};
use syn::{
    Expr, Ident, LitStr, Token, Type, braced,
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

use crate::is_optional_type;

pub struct CliDsl {
    pub(crate) name: LitStr,
    pub(crate) version: LitStr,
    pub(crate) description: LitStr,
    pub(crate) commands: Vec<Command>,
}

pub struct Command {
    pub(crate) name: Ident,
    pub(crate) description: Option<LitStr>,
    pub(crate) arguments: Vec<Argument>,
    pub(crate) options: Vec<CliOption>,
    pub(crate) span: Span,
}

pub struct Argument {
    pub(crate) name: Ident,
    pub(crate) description: Option<LitStr>,
    pub(crate) ty: Type,
    pub(crate) default: Option<Expr>,
    pub(crate) span: Span,
}

pub struct CliOption {
    pub(crate) flags: LitStr,
    pub(crate) description: Option<LitStr>,
    pub(crate) arguments: Vec<Argument>,
    pub(crate) required: bool,
    pub(crate) span: Span,
}

// ----------------------------------------------------------------
// Parsing Implementation
// ----------------------------------------------------------------

impl Parse for CliDsl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse: name "value",
        let _: Ident = input.parse()?; // "name"
        let name: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;

        // Parse: version "value",
        let _: Ident = input.parse()?; // "version"
        let version: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;

        // Parse: description "value",
        let _: Ident = input.parse()?; // "description"
        let description: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;

        // Parse commands
        let mut commands = Vec::new();
        while !input.is_empty() {
            commands.push(input.parse()?);
        }

        Ok(CliDsl {
            name,
            version,
            description,
            commands,
        })
    }
}

impl Parse for Command {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse: cmd <name> ["description"] { ... }
        let cmd_keyword: Ident = input.parse()?;
        if cmd_keyword != "cmd" {
            return Err(syn::Error::new(
                cmd_keyword.span(),
                format!(
                    "Write 'cmd' instead of '{}' to create a command.",
                    cmd_keyword.to_string()
                ),
            ));
        }

        let name: Ident = input.parse()?;

        // Optional description
        let description = if input.peek(LitStr) {
            Some(input.parse()?)
        } else {
            None
        };

        // Parse body
        let content;
        braced!(content in input);

        let mut arguments = Vec::new();
        let mut options = Vec::new();

        while !content.is_empty() {
            let keyword: Ident = content.parse()?;
            let span = keyword.span();
            match keyword.to_string().as_str() {
                "arg" => {
                    arguments.push(parse_argument(&content, true, true, span)?);
                }
                "opt" => {
                    options.push(parse_option(&content, false, span)?);
                }
                "req_opt" => {
                    options.push(parse_option(&content, true, span)?);
                }
                _ => {
                    return Err(syn::Error::new(
                        keyword.span(),
                        format!(
                            "Encountered unexpected keyword '{}'. Please use one of the following: 'arg' / 'opt' / 'req_opt'",
                            keyword
                        ),
                    ));
                }
            }
        }

        // Optional trailing comma after the command block
        let _ = input.parse::<Token![,]>();

        Ok(Command {
            name,
            description,
            arguments,
            options,
            span: cmd_keyword.span(),
        })
    }
}

fn parse_argument(
    input: ParseStream,
    is_positional: bool,
    is_ctx_required: bool,
    keyword_span: Span,
) -> syn::Result<Argument> {
    // arg <name> ["description"] [: type] [= <default>],
    let name: Ident = input.parse()?;

    // Optional description
    let description = if input.peek(LitStr) {
        if !is_positional {
            return Err(syn::Error::new(
                input.span(),
                "For arguments in options, a description is not supported. Consider adding info about arguments in the option description directly.",
            ));
        }
        Some(input.parse()?)
    } else {
        None
    };

    // Parse type (default is String)
    // Parse type (default is String)
    let ty: Type = if input.is_empty() || input.peek(Token![,]) || input.peek(Token![=]) {
        syn::parse_quote!(String)
    } else if input.peek(Token![:]) {
        input.parse::<Token![:]>()?;
        input.parse()?
    } else {
        let unexpected: TokenTree = input.parse()?;
        return Err(syn::Error::new(
            unexpected.span(),
            format!("Expected ':', got '{}'.", unexpected),
        ));
    };
    let ty_is_option = is_optional_type(&ty);

    // Optional default value
    let default = if input.peek(Token![=]) {
        let asignment = input.parse::<Token![=]>()?;

        if !ty_is_option && is_ctx_required {
            return Err(syn::Error::new(
                asignment.span(),
                "Default values are only allowed for *Option<T>* types or for arguments *inside optional options*.",
            ));
        }

        Some(input.parse()?)
    } else {
        None
    };

    // Optional trailing comma
    let _ = input.parse::<Token![,]>();

    Ok(Argument {
        name,
        description,
        ty,
        default,
        span: keyword_span,
    })
}

fn parse_option(input: ParseStream, required: bool, keyword_span: Span) -> syn::Result<CliOption> {
    // opt|req_opt "<flags>" ["description"] [{args}],
    let flags: LitStr = input.parse()?;
    validate_flags(&flags)?;

    // Optional description
    let description = if input.peek(LitStr) {
        Some(input.parse()?)
    } else {
        None
    };

    // Optional arguments block
    let arguments = if input.peek(syn::token::Brace) {
        let content;
        braced!(content in input);
        let mut args = Vec::new();
        while !content.is_empty() {
            let keyword: Ident = content.parse()?;
            let span = keyword.span();
            if keyword == "arg" {
                args.push(parse_argument(&content, false, required, span)?);
            } else {
                return Err(syn::Error::new(
                    keyword.span(),
                    format!(
                        "Write 'arg' instead of '{}' to create an option argument.",
                        keyword.to_string()
                    ),
                ));
            }
        }
        args
    } else {
        Vec::new()
    };

    // Optional trailing comma
    let _ = input.parse::<Token![,]>();

    Ok(CliOption {
        flags,
        description,
        arguments,
        required,
        span: keyword_span,
    })
}

// ----- Validation

fn validate_flags(flags: &LitStr) -> Result<(), syn::Error> {
    let flags_value = flags.value();

    let is_valid = {
        let parts: Vec<&str> = flags_value.split(", ").collect();

        match parts.as_slice() {
            [flag] => {
                flag.len() == 2 && flag.starts_with('-') || flag.starts_with("--") && flag.len() > 2
            }
            [short, long] => {
                short.len() == 2
                    && short.starts_with('-')
                    && short.as_bytes()[1].is_ascii_alphanumeric()
                    && long.starts_with("--")
                    && long.len() > 2
                    && long[2..]
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '-')
            }
            _ => false,
        }
    };

    if !is_valid {
        return Err(syn::Error::new(
            flags.span(),
            "invalid flags format; expected `-s`, `--something`, or `-s, --something`",
        ));
    }

    Ok(())
}
