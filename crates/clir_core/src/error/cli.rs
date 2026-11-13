use colored::Colorize;

use crate::{
    Cli,
    error::{ParseError, suggest_similar::suggest_similar},
};

impl Cli {
    pub fn handle_parse_error(&self, e: ParseError) {
        match e {
            ParseError::MissingCommand => {
                // Error Message
                eprintln!("{} Missing command\n", "error:".red());

                // Tip Message
                println!(
                    "{} Available commands: {}",
                    "tip:".yellow(),
                    self.commands
                        .iter()
                        .map(|cmd| cmd.name.clone())
                        .collect::<Vec<String>>()
                        .join(", ")
                );
                println!(
                    "( For more help on commands run: {} )",
                    format!("{} help", self.executable_name).blue()
                );
            }
            ParseError::UnknownCommand(command) => {
                // Error Message
                eprintln!("{} Unknown command '{}'\n", "error:".red(), command);

                // Tip Message
                println!(
                    "{} Available commands: {}",
                    "tip:".yellow(),
                    self.commands
                        .iter()
                        .map(|cmd| cmd.name.clone())
                        .collect::<Vec<String>>()
                        .join(", ")
                );
                println!(
                    "( For more help on commands run: {} )",
                    format!("{} help", self.executable_name).blue()
                );
            }
            ParseError::TooManyArguments(args) => {
                // Error Message
                eprintln!(
                    "{} Arguments: {:?} exceeded the maximum number of arguments\n",
                    "error:".red(),
                    args
                );

                // Tip Message
                let used_command = self.used_command.as_ref().unwrap();
                let command_def = self.get_command(used_command).unwrap();

                println!(
                    "{} Arguments for '{}' command are: {}",
                    "tip:".yellow(),
                    used_command,
                    command_def
                        .arguments
                        .iter()
                        .map(|arg| arg.reconstruct_name())
                        .collect::<Vec<String>>()
                        .join(", ")
                );
                println!(
                    "( For more help on arguments run: {} )",
                    format!("{} help {}", self.executable_name, used_command).blue()
                );
            }
            ParseError::MissingRequiredArguments(args) => {
                // Error Message
                eprintln!(
                    "{} Missing required arguments: {:?}\n",
                    "error:".red(),
                    args.join(" ")
                );

                // Tip Message
                let used_command = self.used_command.as_ref().unwrap();
                let command_def = self.get_command(used_command).unwrap();

                println!(
                    "{} Arguments for '{}' command are: {}",
                    "tip:".yellow(),
                    used_command,
                    command_def
                        .arguments
                        .iter()
                        .map(|arg| arg.reconstruct_name())
                        .collect::<Vec<String>>()
                        .join(", ")
                );
                println!(
                    "( For more help on required arguments run: {} )",
                    format!("{} help {}", self.executable_name, used_command).blue()
                );
            }
            ParseError::MissingRequiredOptions(opts) => {
                // Error Message
                eprintln!(
                    "{} Missing required options: {:?}\n",
                    "error:".red(),
                    opts.join(", ")
                );

                // Tip Message
                let used_command = self.used_command.as_ref().unwrap();
                let command_def = self.get_command(used_command).unwrap();

                println!(
                    "{} Options for '{}' command are: {}",
                    "tip:".yellow(),
                    used_command,
                    command_def
                        .options
                        .iter()
                        .map(|opt| format!("({})", opt.flags.to_string()))
                        .collect::<Vec<String>>()
                        .join(", ")
                );
                println!(
                    "( For more help on options run: {} )",
                    format!("{} help {}", self.executable_name, used_command).blue()
                );
            }
            ParseError::MissingRequiredArgumentsForOption(idx, args) => {
                // Error Message
                eprintln!(
                    "{} Missing required arguments for option: {:?}\n",
                    "error:".red(),
                    args.join(" ")
                );

                // Tip Message
                let used_command = self.used_command.as_ref().unwrap();
                let command_def = self.get_command(used_command).unwrap();

                let opt_def = &command_def.options[idx];

                println!(
                    "{} Option '{}' is defined as: {} {}",
                    "tip:".yellow(),
                    used_command,
                    opt_def.flags.to_string(),
                    opt_def
                        .args
                        .iter()
                        .map(|arg| arg.reconstruct_name())
                        .collect::<Vec<String>>()
                        .join(" ")
                );
                println!(
                    "( For more help on option arguments run: {} )",
                    format!("{} help {}", self.executable_name, used_command).blue()
                );
            }
            ParseError::InvalidOptionFlag(flag) => {
                // Error Message
                eprintln!("{} Invalid option flag: {:?}\n", "error:".red(), flag);

                // Tip Message
                let used_command = self.used_command.as_ref().unwrap();
                let command_def = self.get_command(used_command).unwrap();

                let mut long_flags = Vec::new();
                let mut short_flags = Vec::new();

                for opt in &command_def.options {
                    let flags = opt.flags.values();

                    if let Some(flag) = &flags[0] {
                        short_flags.push(flag.clone());
                    }

                    if let Some(flag) = &flags[1] {
                        long_flags.push(flag.clone());
                    }
                }

                if flag.starts_with("--") {
                    println!("{} {}", "tip:".yellow(), suggest_similar(flag, long_flags));
                } else {
                    println!(
                        "{} Available short flags for `{}` are: {}",
                        "tip:".yellow(),
                        used_command,
                        short_flags.join(", ")
                    );
                }
                println!(
                    "( For more help on options run: {})",
                    format!("{} help {}", self.executable_name, used_command).blue()
                );
            }
        }
        println!();
    }
}
