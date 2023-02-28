use crate::general::identifier::animation_controller::AnimationControllerIdentifier;
use crate::general::version::Version;
use crate::molang::molang::Molang;
use crate::{general::identifier::namespace::NamespaceIdentifier, slash_command::SlashCommand};
use std::{collections::HashMap, fmt};

use serde::{
    de::{self, MapAccess, Visitor},
    ser::SerializeMap,
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnimationControllers {
    pub format_version: Version,
    pub animation_controllers: HashMap<AnimationControllerIdentifier, AnimationController>,
}
/// if the molang is None then the output of the animation is the name of the animation itself. If the molang is Some then the output of the animation is the result of the molang in a key that is the name and the value which is the molang as a string.

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Animation {
    String(String),
    Molang(HashMap<String, Molang>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ParticleEffect {
    /// Set to false to have the effect spawned in the world without being bound to an actor (by default an effect is bound to the actor).
    pub bind_to_actor: Option<bool>,

    /// The name of a particle effect that should be played.
    pub effect: String,

    /// The name of a locator on the actor where the effect should be located.
    pub locator: String,

    /// A molang script that will be run when the particle emitter is initialized.
    pre_effect_script: Molang,
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
    pub on_entry: Option<Vec<Executers>>,

    /// Events, commands or transitions to preform on exit of this state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_exit: Option<Vec<Executers>>,

    /// The transition definition for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transitions: Option<Vec<HashMap<String, Molang>>>,
}

/// Animation controller for behaviors.
#[derive(Debug, Clone, PartialEq)]
pub enum Executers {
    Event(EventExecuter),
    SlashCommand(SlashCommand),
    Molang(Molang),
}

impl Serialize for Executers {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Executers::Event(event_executer) => event_executer.serialize(serializer),
            Executers::SlashCommand(slash_command) => slash_command.serialize(serializer),
            Executers::Molang(molang) => molang.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Executers {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        if s.starts_with("/") {
            match SlashCommand::try_from(s) {
                Ok(slash_command) => Ok(Executers::SlashCommand(slash_command)),
                Err(e) => Err(serde::de::Error::custom(e.to_string())),
            }
        } else if s.trim().starts_with("@") {
            match EventExecuter::try_from(s) {
                Ok(event_executer) => Ok(Executers::Event(event_executer)),
                Err(e) => Err(serde::de::Error::custom(e.to_string())),
            }
        } else {
            match Molang::try_from(s) {
                Ok(molang) => Ok(Executers::Molang(molang)),
                Err(e) => Err(serde::de::Error::custom(e.to_string())),
            }
        }
    }
}

//TODO test if you can use more than just @s in event
#[derive(Debug, Clone, PartialEq)]
pub enum EventExecuter {
    Identifier(NamespaceIdentifier),
    String(String),
}

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
        match self {
            EventExecuter::Identifier(identifier) => format!("@s {}", identifier.to_string()),
            EventExecuter::String(animation_identifier) => {
                format!("@s {}", animation_identifier.to_string())
            }
        }
    }
}

impl TryFrom<String> for EventExecuter {
    type Error = EventExecuterError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.starts_with("@") {
            //trim the @s and trim the spaces

            let s = s.trim_start_matches("@s").trim();

            // check if the string is an identifier

            if NamespaceIdentifier::try_from(s.to_string()).is_ok() {
                Ok(EventExecuter::Identifier(
                    NamespaceIdentifier::try_from(s.to_string()).unwrap(),
                ))
            } else {
                Ok(EventExecuter::String(s.to_string()))
            }
        } else {
            Err(EventExecuterError::InvalidEventExecuter(s))
        }
    }
}

impl ToString for Executers {
    fn to_string(&self) -> String {
        match self {
            Executers::Event(event_executer) => event_executer.to_string(),
            Executers::SlashCommand(command) => format!("/{}", command.to_string()),
            Executers::Molang(molang) => molang.to_string(),
        }
    }
}


#[test]
fn deserialize_debug() {
    let paths = vec![
        "./Minecraft-bedrock-json-schemas/test/files/correct/data_bp/animation_controllers"
            .to_string(),
        "./samples/behavior/animation_controllers".to_string(),
    ];
    let files = crate::utils::test_serde_json_from_files_in_path::<AnimationControllers>(paths);
}
