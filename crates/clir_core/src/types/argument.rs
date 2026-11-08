#[derive(Debug, Clone)]
pub struct CliArgument {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) optional: bool,
    pub(crate) variadic: bool,
}

impl CliArgument {
    pub fn new(
        arg_name: String,
        description: Option<impl Into<String>>,
        optional: bool,
        variadic: bool,
    ) -> Self {
        Self {
            name: arg_name,
            description: description.map(|d| d.into()),
            optional: optional,
            variadic: variadic,
        }
    }
}
