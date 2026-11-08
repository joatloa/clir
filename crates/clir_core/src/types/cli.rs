use super::CliCommand;

pub struct Cli {
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) description: String,
    pub(crate) commands: Vec<CliCommand>,
    // Used for error messages
    pub(crate) executable_name: String,
    pub(crate) used_command: Option<String>,
}

impl Cli {
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        let name = name.into();
        let executable_name = executable_name().unwrap_or_else(|_| name.clone());

        Self {
            name,
            version: version.into(),
            description: description.into(),
            commands: Vec::new(),
            executable_name,
            used_command: None,
        }
    }
    pub fn add_command(
        &mut self,
        command_name: impl Into<String>,
        description: Option<impl Into<String>>,
    ) -> &mut CliCommand {
        let command = CliCommand::new(command_name, description);
        self.commands.push(command);
        self.commands.last_mut().unwrap()
    }
}

fn executable_name() -> Result<String, String> {
    let exe_path = std::env::current_exe().map_err(|e| e.to_string())?;

    let name = exe_path
        .file_stem()
        .ok_or("Could not determine executable name")?
        .to_string_lossy()
        .into_owned();

    if name.is_empty() {
        Err("Executable name is empty".to_string())
    } else {
        Ok(name)
    }
}
