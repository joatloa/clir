use crate::{Cli, CliCommand};

impl Cli {
    pub fn get_command(&self, command_name: &str) -> Option<&CliCommand> {
        self.commands.iter().find(|cmd| &cmd.name == command_name)
    }
}
