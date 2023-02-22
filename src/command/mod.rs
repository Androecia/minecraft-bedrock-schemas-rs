
/// impl from string
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Command {
    command: String,
    arguments: Vec<String>,
}
impl ToString for Command {
    fn to_string(&self) -> String {
        let mut command = self.command.clone();
        for argument in &self.arguments {
            command.push_str(&format!(" {}", argument));
        }
        command
    }
}
// FIXME this isnt right but it works for now
impl  From<String > for Command {
    fn from(s: String) -> Self {
        let mut split = s.split(" ");
        let command = split.next().unwrap().to_string();
        let mut arguments = Vec::new();
        for argument in split {
            arguments.push(argument.to_string());
        }
        Self { command, arguments }
    }
}

use serde::{Deserialize, Serialize};

impl Serialize for Command {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Command {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self::from(s))
    }
}
