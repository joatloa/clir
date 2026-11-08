use super::CliArgument;

// ----- CliOptionFlags

#[derive(Debug, Clone)]
pub enum CliOptionFlags {
    Short(char),
    Long(String),
    ShortAndLong(char, String),
}
impl PartialEq<String> for CliOptionFlags {
    fn eq(&self, other: &String) -> bool {
        match self {
            CliOptionFlags::Short(c) => ("-".to_owned() + &c.to_string()) == *other,
            CliOptionFlags::Long(s) => ("--".to_owned() + s) == *other,
            CliOptionFlags::ShortAndLong(c, s) => {
                ("-".to_owned() + &c.to_string()) == *other || ("--".to_owned() + s) == *other
            }
        }
    }
}
impl std::fmt::Display for CliOptionFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliOptionFlags::Short(c) => write!(f, "-{}", c),
            CliOptionFlags::Long(s) => write!(f, "--{}", s),
            CliOptionFlags::ShortAndLong(c, s) => write!(f, "-{}, --{}", c, s),
        }
    }
}

impl CliOptionFlags {
    pub fn values(&self) -> [Option<String>; 2] {
        match self {
            CliOptionFlags::Short(c) => [Some(c.to_string()), None],
            CliOptionFlags::Long(s) => [None, Some(s.to_string())],
            CliOptionFlags::ShortAndLong(c, s) => [Some(c.to_string()), Some(s.to_string())],
        }
    }
}

// ----- CliOption

#[derive(Debug, Clone)]
pub struct CliOption {
    pub(crate) name: String,
    pub(crate) flags: CliOptionFlags,
    pub(crate) description: Option<String>,
    pub(crate) optional: bool,
    pub(crate) args: Vec<CliArgument>,
}

impl CliOption {
    pub fn new(
        name: impl Into<String>,
        flags: CliOptionFlags,
        description: Option<impl Into<String>>,
        optional: bool,
    ) -> Self {
        Self {
            name: name.into(),
            flags,
            description: description.map(|d| d.into()),
            optional,
            args: Vec::new(),
        }
    }
    pub fn add_argument(&mut self, argument: CliArgument) -> &mut Self {
        self.args.push(argument);
        self
    }
}
