use colored::Colorize;

use crate::{Cli, utilities::Info};

impl Cli {
    pub fn show_help(&self, command_name: &str) {
        if command_name == "cli" {
            self.show_cli_help();
        } else {
            self.show_command_help(command_name);
        }
    }

    fn show_cli_help(&self) {
        let mut commands = self.commands_info();

        commands.push(Info {
            name: "help [command]".into(),
            description: "Show help for a command".into(),
        });

        let max_width = Self::info_max_width(&commands);

        println!("\n{} - v{}", self.name, self.version);

        println!("\n{}", self.description);

        println!(
            "\n{} {} <command> [options]",
            "Usage:".green(),
            self.executable_name
        );

        Self::print_section("Commands", &commands, max_width);

        println!();
    }

    fn show_command_help(&self, command_name: &str) {
        let Some(command) = self.commands.iter().find(|cmd| cmd.name == command_name) else {
            unreachable!()
        };

        let args = command.args_info();
        let opts = command.opts_info();

        let max_width = Self::info_max_width(&args).max(Self::info_max_width(&opts));

        let usage = Self::build_usage(&self.executable_name, command_name, &args, !opts.is_empty());

        println!("\n{} {}", "Usage:".green(), usage);

        println!(
            "\n{}",
            command
                .description
                .as_deref()
                .unwrap_or("No description available")
        );

        Self::print_section("Arguments", &args, max_width);
        Self::print_section("Options", &opts, max_width);

        println!();
    }
}

impl Cli {
    fn info_max_width(items: &[Info]) -> usize {
        items.iter().map(|i| i.name.len()).max().unwrap_or(0) + 2
    }

    fn build_usage(executable: &str, command: &str, args: &[Info], has_options: bool) -> String {
        let mut usage = format!("{} {}", executable, command);

        let args_string = args
            .iter()
            .map(|i| i.name.as_str())
            .collect::<Vec<_>>()
            .join(" ");

        if !args_string.is_empty() {
            usage.push_str(&format!(" {}", args_string));
        }

        if has_options {
            usage.push_str(" [options]");
        }

        usage
    }
}

impl Cli {
    fn print_section(title: &str, items: &[Info], width: usize) {
        if items.is_empty() {
            return;
        }

        println!("\n{}:", title.green());

        for Info { name, description } in items {
            println!("  {:<width$}{}", name.blue(), description, width = width);
        }
    }
}
