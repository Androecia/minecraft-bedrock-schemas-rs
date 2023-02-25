/// impl from string
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct SlashCommand {
    command: String,
}

#[derive(Debug)]
pub enum SlashCommandError {
    InvalidCommand(String),
}

impl Display for SlashCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SlashCommandError::InvalidCommand(s) => write!(f, "Invalid Slash Command: {}", s),
        }
    }
}

use std::error::Error;

impl Error for SlashCommandError {}

// FIXME this isnt right but it works for now
impl TryFrom<String> for SlashCommand {
    type Error = SlashCommandError;

    fn try_from(command: String) -> Result<Self, Self::Error> {
        Ok(Self { command })
    }
}

use std::fmt::Display;

use serde::{Deserialize, Serialize};

impl Serialize for SlashCommand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.command)
    }
}

impl<'de> Deserialize<'de> for SlashCommand {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        Ok(Self { command: s })
    }
}

impl ToString for SlashCommand {
    fn to_string(&self) -> String {
        self.command.clone()
    }
}
