use crate::{command::Command, general::identifier::Identifier};
use crate::general::version::Version;
use crate::molang::molang::Molang;

use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Deserializer, Serialize, Serializer, ser::SerializeMap,
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

/// if the molang is None then the output of the animation is the name of the animation itself. If the molang is Some then the output of the animation is the result of the molang in a key that is the name and the value which is the molang as a string.
struct Animation {
    name: String,
    molang: Option<Molang>,
}

impl Serialize for Animation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(molang) = &self.molang {
            let mut map = serializer.serialize_map(Some(2))?;
            map.serialize_entry(&self.name, &molang.to_string())?;
            map.end()
        } else {
            serializer.serialize_str(&self.name)
        }
    }
}





impl Animation {
    pub fn new(name: String, molang: Option<Molang>) -> Self {
        Self { name, molang }
    }
}

struct Locator {}

struct ParticleEffect {
    /// Set to false to have the effect spawned in the world without being bound to an actor (by default an effect is bound to the actor).
    bind_to_actor: Option<bool>,

    /// The name of a particle effect that should be played.
    effect: String,

    /// The name of a locator on the actor where the effect should be located.
    locator: Locator,

    /// A molang script that will be run when the particle emitter is initialized.
    pre_effect_script: Molang,
}

struct AnimationController {
    /// The states of this animation controller.
    states: HashMap<String, AnimationState>,

    /// The state to start with, if not specified state at position 0 in the array is used.
    initial_state: Option<String>,
}

impl AnimationController {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            initial_state: None,
        }
    }

    pub fn set_initial_state(&mut self, initial_state: String) {
        self.initial_state = Some(initial_state);
    }

    pub fn set_state(&mut self, name: String, state: AnimationState) {
        self.states.insert(name, state);
    }

    pub fn get_state(&self, name: &str) -> Option<&AnimationState> {
        self.states.get(name)
    }
}

struct AnimationState {
    /// The animations definition for.
    animations: Vec<Animation>,

    /// Events, commands or transitions to preform on entry of this state.
    on_entry: Vec<Commands>,

    /// Events, commands or transitions to preform on exit of this state.
    on_exit: Vec<Commands>,

    /// The transition definition for.
    transitions: Vec<Transition>,
}
/// same as animation but with a name and a molang instead of just a name when it comes to serialization and deserialization.
struct Transition {
    name: String,
    molang: Molang,
}
impl Transition {
    pub fn new(name: String, molang: Molang) -> Self {
        Self { name, molang }
    }
}



impl Serialize for Transition {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
      S: Serializer,
  {
      let mut map = serializer.serialize_map(Some(2))?;
      map.serialize_entry(&self.name, &self.molang.to_string())?;
      map.end()

  }
}










use std::{collections::HashMap, fmt};
/// Animation controller for behaviors.
struct AnimationControllers {
    format_version: Version,

    /// The animation controllers schema for.
    /// The key is the name of the animation controller.
    animation_controllers: HashMap<String,AnimationController>,
}

impl AnimationControllers {
    /// # Arguments
    /// * `format_version` - The format version of the animation controller.
    pub fn new(format_version: Version) -> Self {
        Self {
            format_version,
            animation_controllers: HashMap::new(),
        }
    }

    /// # Arguments
    /// * `name` - The name of the animation controller without the `controller.animation.` prefix.
    /// * `animation_controller` - The animation controller.

    pub fn set_animation_controller(
        &mut self,
        name: String,
        animation_controller: AnimationController,
    ) {
        self.animation_controllers.insert(
            format!("controller.animation.{}", name),
            animation_controller,
        );
    }
    /// # Arguments
    /// * `name` - The name of the animation controller without the `controller.animation.` prefix.
    /// * `animation_controller` - The animation controller.
    pub fn remove_animation_controller(&mut self, name: String) {
      self.animation_controllers
      .remove(&format!("controller.animation.{}", name));
    }

    pub fn get_animation_controllers(&self) -> &HashMap<String,AnimationController> {
        &self.animation_controllers
    }

    pub fn get_animation_controller(&self, name: &str) -> Option<&AnimationController> {
        self.animation_controllers.get(name)
    }
}




enum Commands {
    Event(Identifier),
    Command(Command),
    Molang(Molang),
}

impl ToString for Commands {
    fn to_string(&self) -> String {
        match self {
            Commands::Event(identifier) => format!("@s {}", identifier.to_string()),
            Commands::Command(command) => format!("/{}", command.to_string()),
            Commands::Molang(molang) => molang.to_string(),
        }
    }
}
