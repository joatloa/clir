use super::{CliArgument, CliOption};

#[derive(Debug, Clone)]
pub struct CliCommand {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) options: Vec<CliOption>,
    pub(crate) arguments: Vec<CliArgument>,
}

impl CliCommand {
    pub fn new(name: impl Into<String>, description: Option<impl Into<String>>) -> Self {
        Self {
            name: name.into(),
            description: description.map(|s| s.into()),
            options: Vec::new(),
            arguments: Vec::new(),
        }
    }
    pub fn add_argument(&mut self, argument: CliArgument) -> &mut Self {
        self.arguments.push(argument);
        self
    }
    pub fn add_option(&mut self, option: CliOption) -> &mut Self {
        self.options.push(option);
        self
    }
}
