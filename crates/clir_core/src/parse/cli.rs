use std::{collections::HashMap, iter::Peekable};

use crate::{
    Cli, CliArgument, CliOption,
    error::ParseError,
    parse::{ParsedArg, ParsedArgs, ParsedOpt, ParsedOpts},
};

impl Cli {
    pub fn parse(&mut self, env_args: Vec<String>) -> (ParsedArgs, ParsedOpts) {
        let result = self.try_parse(env_args);

        match result {
            Ok((parsed_args, parsed_opts)) => (parsed_args, parsed_opts),
            Err(e) => {
                self.handle_parse_error(e);
                std::process::exit(1);
            }
        }
    }

    fn try_parse(&mut self, env_args: Vec<String>) -> Result<(ParsedArgs, ParsedOpts), ParseError> {
        let potential_cmd_name = env_args.first().map(String::as_str).unwrap_or("");

        // Handle help
        if potential_cmd_name == "help" {
            let command_name = env_args.get(1).map(String::as_str).unwrap_or("cli");

            if command_name != "cli" && !self.commands.iter().any(|cmd| cmd.name == command_name) {
                return Err(ParseError::InvalidCommand(command_name.to_owned()));
            }

            self.show_help(command_name);
            std::process::exit(0);
        }

        // Find command
        let command_def = if let Some(cmd) = self
            .commands
            .iter()
            .find(|cmd| cmd.name == potential_cmd_name)
        {
            self.used_command = Some(potential_cmd_name.to_owned());
            cmd
        } else if let Some(cmd) = self.commands.iter().find(|cmd| cmd.name == "cli") {
            self.used_command = Some("cli".to_owned());
            cmd
        } else {
            return Err(ParseError::InvalidCommand(potential_cmd_name.to_owned()));
        };

        // Skip command name when one was explicitly provided
        let args_start = if command_def.name == potential_cmd_name {
            1
        } else {
            0
        };

        let env_args = env_args.into_iter().skip(args_start).collect();

        let (parsed_args, parsed_opts) = Self::parse_args(
            env_args,
            command_def.arguments.clone(),
            command_def.options.clone(),
        )?;

        Ok((parsed_args, parsed_opts))
    }

    fn parse_args(
        env_args: Vec<String>,
        template_args: Vec<CliArgument>,
        template_opts: Vec<CliOption>,
    ) -> Result<(ParsedArgs, ParsedOpts), ParseError> {
        let mut parsed_args = Self::initialize_parsed_args(&template_args);
        let mut parsed_opts = Self::initialize_parsed_opts(&template_opts);

        let mut tokens = env_args.into_iter().peekable();

        let mut positional_idx = 0;

        while let Some(token) = tokens.next() {
            if Self::is_option_token(&token) {
                // Check if the option is included in the template
                if !template_opts.iter().any(|opt| opt.flags == token) {
                    return Err(ParseError::InvalidOptionFlag(token));
                }

                let opt_idx = template_opts
                    .iter()
                    .position(|opt| opt.flags == token)
                    .unwrap();
                let opt_def = &template_opts[opt_idx];

                // Option has no arguments = flag-only option
                if opt_def.args.is_empty() {
                    parsed_opts.insert(opt_def.name.clone(), ParsedOpt::Flag(true));
                    continue;
                }

                // Handle positional arguments for option
                let opt_args = &opt_def.args;
                let mut parsed_opt_args = Self::initialize_parsed_args(opt_args);
                let mut idx = 0;

                while idx < parsed_opt_args.len() {
                    if tokens.peek().is_none() || Self::is_option_token(tokens.peek().unwrap()) {
                        break;
                    }

                    let arg_def = &opt_args[idx];
                    let token = tokens.next().unwrap();
                    let parsed_value = Self::parse_arg(arg_def, token, &mut tokens)?;

                    // If the option only has one argument, insert the value into the option directly
                    if parsed_opt_args.len() == 1 {
                        parsed_opts.insert(
                            opt_def.name.clone(),
                            ParsedOpt::Value(parsed_value.as_value()),
                        );
                    } else {
                        parsed_opt_args.insert(arg_def.name.clone(), parsed_value);
                    }

                    idx += 1;
                }

                Self::check_for_missing_required_args(&opt_args, idx, Some(opt_idx))?;

                parsed_opts.insert(opt_def.name.clone(), ParsedOpt::Args(parsed_opt_args));
            } else {
                // Check if we've gone past the number of positional arguments
                if positional_idx >= template_args.len() {
                    let mut remaining_args = vec![token];
                    remaining_args.extend(tokens);
                    return Err(ParseError::TooManyArguments(remaining_args));
                }

                // Handle positional arguments
                let arg_def = &template_args[positional_idx];
                let parsed_value = Self::parse_arg(arg_def, token, &mut tokens)?;
                parsed_args.insert(arg_def.name.clone(), parsed_value);
                positional_idx += 1;
            }
        }

        Self::check_for_missing_required_args(&template_args, positional_idx, None)?;
        Self::check_for_missing_required_opts(&parsed_opts, &template_opts)?;

        Ok((parsed_args, parsed_opts))
    }

    fn parse_arg(
        arg_def: &CliArgument,
        current_token: String,
        tokens: &mut Peekable<std::vec::IntoIter<String>>,
    ) -> Result<ParsedArg, ParseError> {
        if arg_def.variadic {
            let mut values = vec![current_token];
            while tokens.peek().is_some() && !Self::is_option_token(tokens.peek().unwrap()) {
                values.push(tokens.next().unwrap());
            }

            Ok(ParsedArg::Variadic(values))
        } else {
            Ok(ParsedArg::Value(current_token))
        }
    }
}

// ----- Validation

impl Cli {
    fn check_for_missing_required_opts(
        parsed_opts: &ParsedOpts,
        template_opts: &Vec<CliOption>,
    ) -> Result<(), ParseError> {
        let required_opts = template_opts.iter().filter(|opt| !opt.optional);

        let missing_required_opts: Vec<String> = required_opts
            .filter_map(|opt| {
                let is_missing = parsed_opts.get(&opt.name).unwrap().is_none();

                if is_missing {
                    Some(format!("({})", opt.flags))
                } else {
                    None
                }
            })
            .collect();

        if !missing_required_opts.is_empty() {
            return Err(ParseError::MissingRequiredOptions(missing_required_opts));
        }

        Ok(())
    }
    fn check_for_missing_required_args(
        template_args: &Vec<CliArgument>,
        positional_idx: usize,
        opt_idx: Option<usize>,
    ) -> Result<(), ParseError> {
        // Required args can only preceed other required args.
        // This is essentially the head where required args end.
        let required_args_count = template_args.iter().filter(|arg| !arg.optional).count();

        if positional_idx < required_args_count {
            let missing_args = template_args[positional_idx..required_args_count - 1]
                .iter()
                .map(|arg| arg.reconstruct_name())
                .collect::<Vec<String>>();

            if let Some(opt_idx) = opt_idx {
                return Err(ParseError::MissingRequiredArgumentsForOption(
                    opt_idx,
                    missing_args,
                ));
            } else {
                return Err(ParseError::MissingRequiredArguments(missing_args));
            }
        }

        Ok(())
    }
}

// ----- Utils

impl Cli {
    fn is_option_token(token: &str) -> bool {
        token.starts_with('-') && token != "-"
    }
    fn initialize_parsed_args(template_args: &Vec<CliArgument>) -> ParsedArgs {
        let mut parsed_args: ParsedArgs = HashMap::new();
        for arg in template_args {
            parsed_args.insert(arg.name.clone(), ParsedArg::None);
        }
        parsed_args
    }
    fn initialize_parsed_opts(template_opts: &Vec<CliOption>) -> ParsedOpts {
        let mut parsed_opts: ParsedOpts = HashMap::new();
        for opt in template_opts {
            if opt.optional && opt.args.len() > 1 {
                parsed_opts.insert(
                    opt.name.clone(),
                    ParsedOpt::Args(Self::initialize_parsed_args(&opt.args)),
                );
            } else {
                parsed_opts.insert(opt.name.clone(), ParsedOpt::None);
            }
        }
        parsed_opts
    }
}
