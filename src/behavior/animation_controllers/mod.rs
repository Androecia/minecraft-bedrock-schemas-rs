use crate::general::version::Version;
use crate::molang::molang::Molang;
use crate::{slash_command::SlashCommand, general::identifier::Identifier};

use serde::{
    de::{self, MapAccess, Visitor},
    ser::SerializeMap,
    Deserialize, Deserializer, Serialize, Serializer,
};

/*

{
  "$schema": "http://json-schema.org/draft-07/schema",
  "$id": "blockception.minecraft.behavior.animation_controller",
  "examples": [
    {
      "format_version": "1.19.0",
      "animation_controllers": {
        "controller.animation.example": {
          "initial_state": "default",
          "states": { "default": { "transitions": [{ "state_1": "query.is_baby" }] }, "state_1": {} }
        }
      }
    }
  ],
  "definitions": {
    "animationspec": {
      "anyOf": [
        {
          "title": "Animation Specification",
          "description": "A single string that specifies which animation there are.",
          "type": "string"
        },
        {
          "type": "object",
          "title": "Animation Specification",
          "description": "A object specification on when to animate.",
          "maxProperties": 1,
          "minProperties": 1,
          "additionalProperties": {
            "$ref": "../../molang/string.json"
          }
        }
      ]
    },
    "particle_effect_spec": {
      "additionalProperties": false,
      "type": "object",
      "required": ["effect"],
      "properties": {
        "bind_to_actor": {
          "type": "boolean",
          "title": "Bind To Actor",
          "description": "Set to false to have the effect spawned in the world without being bound to an actor (by default an effect is bound to the actor).",
          "const": false
        },
        "effect": {
          "type": "string",
          "title": "Effect",
          "description": "The name of a particle effect that should be played."
        },
        "locator": {
          "type": "string",
          "title": "Locator",
          "description": "The name of a locator on the actor where the effect should be located."
        },
        "pre_effect_script": {
          "type": "string",
          "title": "Pre Effect Script",
          "description": "A molang script that will be run when the particle emitter is initialized."
        }
      }
    },
    "commands": {
      "type": "string",
      "description": "The event or commands to execute.",
      "title": "Commands",
      "oneOf": [
        { "pattern": "^@s .+$", "title": "Event" },
        { "pattern": "^/.+$", "title": "Command" },
        { "pattern": "^.+;$", "title": "Molang" }
      ]
    }
  },
  "type": "object",
  "title": "Animation Controller",
  "description": "Animation controller for behaviors.",
  "required": ["format_version", "animation_controllers"],
  "additionalProperties": false,
  "properties": {
    "format_version": { "$ref": "../../general/format_version.json" },
    "animation_controllers": {
      "type": "object",
      "title": "Animation Controllers",
      "description": "The animation controllers schema for.",
      "propertyNames": {
        "pattern": "^controller\\.animation\\.[a-z\\.]+",
        "examples": ["controller.animation.example", "controller.animation.example.foo"]
      },
      "additionalProperties": {
        "additionalProperties": false,
        "type": "object",
        "title": "Animation Controller",
        "description": "A single animation controller.",
        "required": ["states"],
        "minProperties": 1,
        "properties": {
          "states": {
            "title": "States",
            "description": "The states of this animation controller.",
            "propertyNames": { "pattern": "[a-z\\.]+" },
            "minProperties": 1,
            "type": "object",
            "additionalProperties": {
              "additionalProperties": false,
              "title": "Animation State",
              "description": "Animation state.",
              "type": "object",
              "examples": [
                {
                  "animations": ["anim.idle"],
                  "transitions": [{ "example": "query.is_sheared" }]
                }
              ],
              "properties": {
                "animations": {
                  "title": "Animations",
                  "description": "The animations definition for.",
                  "type": "array",
                  "items": {
                    "$ref": "#/definitions/animationspec",
                    "description": "The key definition of an animation to play, defined in the entity.",
                    "title": "Animations"
                  }
                },
                "on_entry": {
                  "type": "array",
                  "description": "Events, commands or transitions to preform on entry of this state.",
                  "title": "On Entry",
                  "items": {
                    "$ref": "#/definitions/commands"
                  }
                },
                "on_exit": {
                  "type": "array",
                  "description": "Events, commands or transitions to preform on exit of this state.",
                  "title": "On Exit",
                  "items": {
                    "$ref": "#/definitions/commands"
                  }
                },
                "transitions": {
                  "title": "Transition",
                  "description": "The transition definition for.",
                  "minProperties": 1,
                  "type": "array",
                  "items": {
                    "title": "Transition",
                    "description": "A transition to another state.",
                    "type": "object",
                    "maxProperties": 1,
                    "minProperties": 1,
                    "examples": [{ "default": "query.is_chested" }],
                    "additionalProperties": {
                      "$ref": "../../molang/string.json"
                    }
                  }
                }
              }
            }
          },
          "initial_state": {
            "title": "Initial State",
            "description": "The state to start with, if not specified state at position 0 in the array is used.",
            "type": "string",
            "examples": ["default"]
          }
        }
      }
    }
  }
}
 */
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnimationControllers {
    pub format_version: Version,

    /// The animation controllers schema for.
    /// The key is the name of the animation controller.
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
    /// The states of this animation controller.
    pub states: HashMap<String, AnimationState>,

    /// The state to start with, if not specified state at position 0 in the array is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_state: Option<String>,
}

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

use std::{collections::HashMap, fmt};
/// Animation controller for behaviors.



#[derive( Debug, Clone, PartialEq,Hash,Eq)]
pub struct AnimationControllerIdentifier(String);

impl fmt::Display for AnimationControllerIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "controller.animation.{}", self.0)
    }
}

impl Serialize for AnimationControllerIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}


impl<'de> Deserialize<'de> for AnimationControllerIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.starts_with("controller.animation.") {
            Ok(Self(s[21..].to_string()))
        } else {
            Err(serde::de::Error::custom(format!(
                "expected animation controller identifier but got {}",
                s
            )))
        }
    }
}



#[derive( Debug, Clone, PartialEq)]

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

          match  SlashCommand::try_from(s) {
            Ok(slash_command) => Ok(Executers::SlashCommand(slash_command)),
            Err(e) => Err(serde::de::Error::custom(e.to_string()))
          }



        } else if s.trim().starts_with("@") {
          match EventExecuter::try_from(s) {
            Ok(event_executer) => Ok(Executers::Event(event_executer)),
            Err(e) => Err(serde::de::Error::custom(e.to_string()))
          }



        } else {


          match Molang::try_from(s) {
            Ok(molang) => Ok(Executers::Molang(molang)),
            Err(e) => Err(serde::de::Error::custom(e.to_string()))
          }




        }



    }
}






//TODO test if you can use more than just @s in event
#[derive( Debug, Clone, PartialEq)]
pub enum EventExecuter {
  Identifier (Identifier),
  String(String)
}


impl Serialize for EventExecuter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}



#[derive( Debug)]
pub enum EventExecuterError {
  InvalidEventExecuter(String)
}

impl std::fmt::Display for EventExecuterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventExecuterError::InvalidEventExecuter(s) => write!(f, "Invalid event executer: {}", s),
        }
    }
}

impl std::error::Error for EventExecuterError {}





impl ToString for EventExecuter {
    fn to_string(&self) -> String {
        match self {
            EventExecuter::Identifier(identifier) => format!("@s {}", identifier.to_string()),
            EventExecuter::String(animation_identifier) =>  format!("@s {}", animation_identifier.to_string()),
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

      if Identifier::try_from(s.to_string()).is_ok() {
        Ok(EventExecuter::Identifier(Identifier::try_from(s.to_string()).unwrap()))
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





use std::fs;
#[test]
fn deserialize_debug() {
    let paths = vec![
        "./Minecraft-bedrock-json-schemas/test/files/correct/data_bp/animation_controllers"
            .to_string(),"./samples/behavior/animation_controllers".to_string()
    ];
    let files =crate::test_utils::test_serde_json_from_files_in_path::<AnimationControllers>(paths);

/*
    // write files to output folder with names incrementing by 1

    let mut i = 1;

    for file in files {

      let file_name = format!("test_output/{}.json", i);
      /// also write raw rust debug output to file with the same name but with .txt extension
      ///
      let file_name_debug = format!("test_output/{}.rs", i);

      fs::write(file_name_debug, format!("{:#?}", file)).unwrap();



      fs::write(file_name, serde_json::to_string_pretty(&file).unwrap()).unwrap();
      i += 1;

    }

*/




}
