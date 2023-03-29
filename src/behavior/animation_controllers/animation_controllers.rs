use crate::behavior::entities::event::Event;
pub use crate::shared::animation_controllers::*;

use crate::molang::Molang;
use crate::{
    general::{
        identifier::{
            animation_controller::AnimationControllerIdentifier, namespace::NamespaceIdentifier,
        },
        version::Version,
    },
    slash_command::SlashCommand,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnimationControllers {
    pub format_version: Version,
    pub animation_controllers: HashMap<AnimationControllerIdentifier, AnimationController>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnimationController {
    // Part of me feels like making this an this into a vector may improve performance
    /// The states of this animation controller.
    pub states: HashMap<String, AnimationState>,

    /// The state to start with, if not specified state at position 0 in the array is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_state: Option<String>,
}
///
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnimationState {
    /// The animations definition for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animations: Option<Vec<Animation>>,

    /// Events, commands or transitions to preform on entry of this state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_entry: Option<Vec<Executer>>,

    /// Events, commands or transitions to preform on exit of this state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_exit: Option<Vec<Executer>>,

    /// The transition definition for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transitions: Option<Vec<Animation>>,
}

/// Animation controller for behaviors.
#[derive(Debug, Clone, PartialEq)]
pub enum Executer {
    Event(EventExecuter),
    SlashCommand(SlashCommand),
    Molang(Molang),
}

impl Serialize for Executer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Executer::Event(event_executer) => event_executer.serialize(serializer),
            Executer::SlashCommand(slash_command) => slash_command.serialize(serializer),
            Executer::Molang(molang) => molang.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Executer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        if s.starts_with("/") {
            match SlashCommand::try_from(s) {
                Ok(slash_command) => Ok(Executer::SlashCommand(slash_command)),
                Err(e) => Err(serde::de::Error::custom(e.to_string())),
            }
        } else if s.trim().starts_with("@") {
            match EventExecuter::try_from(s) {
                Ok(event_executer) => Ok(Executer::Event(event_executer)),
                Err(e) => Err(serde::de::Error::custom(e.to_string())),
            }
        } else {
            match Molang::try_from(s) {
                Ok(molang) => Ok(Executer::Molang(molang)),
                Err(e) => Err(serde::de::Error::custom(e.to_string())),
            }
        }
    }
}

//TODO test if you can use more than just @s in event
#[derive(Debug, Clone, PartialEq)]
pub struct EventExecuter(NamespaceIdentifier<Event>);

impl Serialize for EventExecuter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug)]
pub enum EventExecuterError {
    InvalidEventExecuter(String),
}

impl std::fmt::Display for EventExecuterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventExecuterError::InvalidEventExecuter(s) => {
                write!(f, "Invalid event executer: {}", s)
            }
        }
    }
}

impl std::error::Error for EventExecuterError {}

impl ToString for EventExecuter {
    fn to_string(&self) -> String {
        format!("@s {}", self.0.to_string())
    }
}

impl TryFrom<String> for EventExecuter {
    type Error = EventExecuterError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.starts_with("@") {
            //trim the @s and trim the spaces

            let s = s.trim_start_matches("@s").trim();

            // check if the string is an identifier

            let identifier = match NamespaceIdentifier::try_from(s.to_string()) {
                Ok(identifier) => identifier,
                Err(_) => return Err(EventExecuterError::InvalidEventExecuter(s.to_string())),
            };
            Ok(EventExecuter(identifier))
        } else {
            Err(EventExecuterError::InvalidEventExecuter(s))
        }
    }
}

impl ToString for Executer {
    fn to_string(&self) -> String {
        match self {
            Executer::Event(event_executer) => event_executer.to_string(),
            Executer::SlashCommand(command) => format!("/{}", command.to_string()),
            Executer::Molang(molang) => molang.to_string(),
        }
    }
}
