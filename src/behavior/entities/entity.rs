use std::collections::HashMap;

use crate::{general::{identifier::namespace::NamespaceIdentifier, version::Version}, molang};
/*

{
  "$schema": "http://json-schema.org/draft-07/schema",
  "$id": "blockception.minecraft.behavior.entities",
  "examples": [
    {
      "format_version": "1.19.0",
      "minecraft:entity": {
        "description": { "identifier": "namespace:entity", "is_spawnable": true, "is_summonable": true },
        "component_groups": {},
        "components": {},
        "events": {}
      }
    }
  ],
  "type": "object",
  "title": "Entity Behavior",
  "description": "The minecraft entity behavior specification.",
  "required": ["format_version", "minecraft:entity"],
  "additionalProperties": false,
  "properties": {
    "format_version": { "$ref": "../../general/format_version.json" },
    "minecraft:entity": { "$ref": "./format/minecraft.entity.json" }
  }
}


{
    "$id": "blockception.minecraft.behavior.entities.minecraft:entity",
    "title": "Entity",
    "required": ["description"],
    "dependencies": { "component_groups": ["events"] },
    "additionalProperties": false,
    "type": "object",
    "properties": {

        "component_groups": {
            "title": "Component Groups",
            "description": "Each group when add / remove the default components.",
            "uniqueItems": true,
            "type": "object",
            "propertyNames": { "examples": ["self:"] },
            "additionalProperties": {
                "$ref": "./components.json",
                "uniqueItems": true,
                "description": "The components that are added as the foundation of the entity.",
                "title": "Component"
            }
        },
        "components": {
            "$ref": "./components.json",
            "uniqueItems": true,
            "description": "The components that are added as the foundation of the entity.",
            "title": "Component"
        },
        "events": {
            "$ref": "./events.json",
            "uniqueItems": true,
            "description": "The events that the entity can run, these add or remove components_groups.",
            "title": "Events"
        }
    }
}
 */

use serde::{Deserialize, Serialize};

use super::event::{EventIdentifier, Event};

// TODO: Implement this, as this is just a placeholder
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct MinecraftEntityIdentifier(EntityIdentifier);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entities {
    format_version: Version,

    #[serde(rename = "minecraft:entity")]
    entity: Entity,
}

type ComponentGroupIdentifier = NamespaceIdentifier<ComponentGroup>;

#[derive(Debug, Clone, PartialEq)]
pub struct Entity {
    description: Description,

    component_groups: ComponentGroups,

    components: Components,

    events: HashMap<EventIdentifier, Event>
}

struct ComponentGroup(ComponentGroupIdentifier, Components);

struct ComponentGroups(pub Vec<ComponentGroup>);

// placeholder
struct Components {}

pub type EntityIdentifier = NamespaceIdentifier<Entities>;

/*"description": {
    "required": ["identifier"],
    "title": "Description",
    "description": "The description of the this entity.",
    "properties": {
        "animations": {
            "title": "Animations",
            "description": "Sets the mapping of internal animation / animation controllers references to actual animations. This is a JSON Object of name/animation pairs",
            "type": "object",
            "additionalProperties": {
                "title": "Animation / Controller",
                "description": "The name of the animation controller / animation.",
                "type": "string",
                "examples": ["animation.", "controller."]
            }
        },
        "identifier": {
            "$ref": "../../../general/entity/identifier.json",
            "description": "Sets the identifier for this entity's description.",
            "title": "Identifier"
        },
        "is_spawnable": {
            "type": "boolean",
            "title": "Is Spawnable",
            "description": "Sets whether or not this entity has a spawn egg in the creative ui.",
            "default": false
        },
        "is_summonable": {
            "type": "boolean",
            "title": "Is Summonable Property",
            "description": "Sets whether or not we can summon this entity using commands such as /summon.",
            "default": true
        },
        "is_experimental": {
            "type": "boolean",
            "title": "Is Experimental",
            "description": "Sets whether or not this entity is experimental. Experimental entities are only enabled when the experimental toggle is enabled.",
            "default": false
        },
        "properties": {
            "title": "Experimental",
            "description": "Experimental",
            "type": "object",
            "additionalProperties": {
                "type": "object",
                "required": ["type"],
                "properties": {
                    "type": {
                        "title": "Type",
                        "description": "The type of the property.",
                        "enum": ["bool", "int", "float", "enum"]
                    }
                },
                "oneOf": [
                    {
                        "properties": {
                            "type": { "const": "bool" },
                            "default": {
                                "title": "Default",
                                "description": "The default value of the property.",
                                "$ref": "../../../molang/boolean.json"
                            }
                        }
                    },
                    {
                        "properties": {
                            "type": { "const": "int" },
                            "default": {
                                "title": "Default",
                                "description": "The default value of the property.",
                                "$ref": "../../../molang/number.json"
                            },
                            "range": {
                                "title": "Range",
                                "description": "The range of the property.",
                                "type": "array",
                                "items": [
                                    {
                                        "title": "Min",
                                        "type": "integer",
                                        "description": "The minimum value of the property."
                                    },
                                    {
                                        "title": "Max",
                                        "type": "integer",
                                        "description": "The minimum value of the property."
                                    }
                                ]
                            }
                        }
                    },
                    {
                        "properties": {
                            "type": { "const": "float" },
                            "default": {
                                "title": "Default",
                                "description": "The default value of the property.",
                                "$ref": "../../../molang/number.json"
                            },
                            "range": {
                                "title": "Range",
                                "description": "The range of the property.",
                                "type": "array",
                                "items": [
                                    {
                                        "title": "Min",
                                        "type": "number",
                                        "description": "The minimum value of the property."
                                    },
                                    {
                                        "title": "Max",
                                        "type": "number",
                                        "description": "The minimum value of the property."
                                    }
                                ]
                            }
                        }
                    },
                    {
                        "properties": {
                            "type": { "const": "enum" },
                            "default": { "type": "string" },
                            "client_sync": {
                                "title": "Client Sync",
                                "description": "Sets whether or not the property is synced to the client.",
                                "type": "boolean",
                                "default": false
                            },
                            "values": {
                                "title": "Values",
                                "description": "The values of the property.",
                                "type": "array",
                                "minItems": 1,
                                "items": { "type": "string" }
                            }
                        }
                    }
                ]
            }
        },
        "runtime_identifier": {
            "type": "string",
            "title": "Runtime Identifier",
            "description": "Sets the name for the Vanilla Minecraft identifier this entity will use to build itself from.",
            "examples": ["minecraft."]
        },
        "scripts": {
            "type": "object",
            "title": "Scripts",
            "description": "Sets the mapping of internal animation controller references to actual animation controller. This is a JSON Array of name/animation-controller pairs",
            "properties": {
                "animate": {
                    "type": "array",
                    "title": "Animate",
                    "description": "Tells minecraft to run which animation / animation controllers and under what conditions.",
                    "items": {
                        "oneOf": [
                            {
                                "type": "string",
                                "title": "Animation",
                                "description": "The name of an animation controller referenced in animations."
                            },
                            {
                                "type": "object",
                                "title": "Conditional Animation",
                                "description": "A conditional statement to run the animation under a specified condition.",
                                "$comment": "UNDOCUMENTED",
                                "additionalProperties": { "type": "string", "title": "Animation" }
                            }
                        ]
                    }
                }
            }
        }
    }
}, */

use crate::general::identifier::{
    animation::AnimationIdentifier, animation_controller::AnimationControllerIdentifier,
};

/// The description of the an entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Description {
    /// Sets the mapping of internal animation / animation controllers references to actual animations. This is a JSON Object of name/animation pairs
    pub animations: Option<HashMap<Animate,AnimationsType>>,
    /// Sets the identifier for this entity's description.
    pub identifier: EntityIdentifier,
    /// Sets whether or not this entity has a spawn egg in the creative ui.
    pub is_spawnable: Option<bool>,
    /// Sets whether or not we can summon this entity using commands such as /summon.
    pub is_summonable: Option<bool>,
    /// Sets whether or not this entity is experimental. Experimental entities are only enabled when the experimental toggle is enabled.
    pub is_experimental: Option<bool>,
    /// Experimental
    pub properties: HashMap<PropertyIdentifier,Property>,
    /// Sets the name for the Vanilla Minecraft identifier this entity will use to build itself from.
    pub runtime_identifier: Option<MinecraftEntityIdentifier>,
    /// Sets the mapping of internal animation controller references to actual animation controller. This is a JSON Array of name/animation-controller pairs
    pub scripts: Option<Scripts>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnimationsType {
    Animation(AnimationIdentifier),
    Controller(AnimationControllerIdentifier),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scripts {
    /// Tells minecraft to run which animation / animation controllers and under what conditions.
    pub animate: Option<Vec<Animate>>,
}

/*
 {
                                "type": "string",
                                "title": "Animation",
                                "description": "The name of an animation controller referenced in animations."
                            },
                            {
                                "type": "object",
                                "title": "Conditional Animation",
                                "description": "A conditional statement to run the animation under a specified condition.",
                                "$comment": "UNDOCUMENTED",
                                "additionalProperties": { "type": "string", "title": "Animation" }
                            }

 */
/// Sets the mapping of internal animation controller references to actual animation controller.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum Animate  {
    Animation(AnimateIdentifier),
    // TODO Find an example of this, as this is undocumented. I am guessing it is a HashMap<AnimateIdentifier, AnimationIdentifier>
    ConditionalAnimation(HashMap<AnimateIdentifier, AnimationIdentifier>),
}




type AnimateIdentifier = NamespaceIdentifier<Animate>;
/*"properties": {
            "title": "Experimental",
            "description": "Experimental",
            "type": "object",
            "additionalProperties": {
                "type": "object",
                "required": ["type"],
                "properties": {
                    "type": {
                        "title": "Type",
                        "description": "The type of the property.",
                        "enum": ["bool", "int", "float", "enum"]
                    }
                },
                "oneOf": [
                    {
                        "properties": {
                            "type": { "const": "bool" },
                            "default": {
                                "title": "Default",
                                "description": "The default value of the property.",
                                "$ref": "../../../molang/boolean.json"
                            }
                        }
                    },
                    {
                        "properties": {
                            "type": { "const": "int" },
                            "default": {
                                "title": "Default",
                                "description": "The default value of the property.",
                                "$ref": "../../../molang/number.json"
                            },
                            "range": {
                                "title": "Range",
                                "description": "The range of the property.",
                                "type": "array",
                                "items": [
                                    {
                                        "title": "Min",
                                        "type": "integer",
                                        "description": "The minimum value of the property."
                                    },
                                    {
                                        "title": "Max",
                                        "type": "integer",
                                        "description": "The minimum value of the property."
                                    }
                                ]
                            }
                        }
                    },
                    {
                        "properties": {
                            "type": { "const": "float" },
                            "default": {
                                "title": "Default",
                                "description": "The default value of the property.",
                                "$ref": "../../../molang/number.json"
                            },
                            "range": {
                                "title": "Range",
                                "description": "The range of the property.",
                                "type": "array",
                                "items": [
                                    {
                                        "title": "Min",
                                        "type": "number",
                                        "description": "The minimum value of the property."
                                    },
                                    {
                                        "title": "Max",
                                        "type": "number",
                                        "description": "The minimum value of the property."
                                    }
                                ]
                            }
                        }
                    },
                    {
                        "properties": {
                            "type": { "const": "enum" },
                            "default": { "type": "string" },
                            "client_sync": {
                                "title": "Client Sync",
                                "description": "Sets whether or not the property is synced to the client.",
                                "type": "boolean",
                                "default": false
                            },
                            "values": {
                                "title": "Values",
                                "description": "The values of the property.",
                                "type": "array",
                                "minItems": 1,
                                "items": { "type": "string" }
                            }
                        }
                    }
                ]
            }
        }, */

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Property<T=String> {
    Bool {
        client_sync: Option<bool>,
        default: molang::Bool,
    },
    Int {
        client_sync: Option<bool>,
        default: molang::Number,
        range: Option<[i32; 2]>,
    },
    Float {
        client_sync: Option<bool>,
        default: molang::Number,
        range: Option<[f32; 2]>,
    },
    // TODO: maybe type this enum?
    Enum {
        client_sync: Option<bool>,
        default: T,
        values: Vec<T>,
    },
}

pub type PropertyIdentifier = NamespaceIdentifier<Property>;
