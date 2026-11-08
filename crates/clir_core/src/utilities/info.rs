use crate::{Cli, CliArgument, CliCommand, CliOption};

pub struct Info {
    pub name: String,
    pub description: String,
}

impl Cli {
    pub fn commands_info(&self) -> Vec<Info> {
        self.commands.iter().map(|cmd| cmd.info()).collect()
    }
}

impl CliCommand {
    pub fn args_info(&self) -> Vec<Info> {
        self.arguments.iter().map(|arg| arg.info()).collect()
    }
    pub fn opts_info(&self) -> Vec<Info> {
        self.options.iter().map(|opt| opt.info()).collect()
    }
    pub fn info(&self) -> Info {
        let name = self.name.clone();
        let description = self.description.clone().unwrap_or_default();
        Info { name, description }
    }
}

impl CliArgument {
    pub fn reconstruct_name(&self) -> String {
        let mut name = self.name.clone();

        if self.variadic {
            name += "...";
        }

        if self.optional {
            name = format!("[{}]", name);
        } else {
            name = format!("<{}>", name);
        }

        name
    }
    // Takes in an argument an returns something like: (<name...>,description)
    pub fn info(&self) -> Info {
        let name = self.reconstruct_name();
        let description = self.description.clone().unwrap_or_default();
        Info { name, description }
    }
}

impl CliOption {
    pub fn reconstruct_arg_names(&self) -> String {
        self.args
            .iter()
            .map(|arg| arg.reconstruct_name())
            .collect::<Vec<String>>()
            .join(" ")
    }
    // Takes in an option and returns something like: (<flags> ...<name>,description)
    pub fn info(&self) -> Info {
        let flags = self.flags.to_string();

        let name = self.reconstruct_arg_names();

        let usage = [flags, name].join(" ");

        let description = self.description.clone().unwrap_or_default();

        Info {
            name: usage,
            description,
        }
    }
}
