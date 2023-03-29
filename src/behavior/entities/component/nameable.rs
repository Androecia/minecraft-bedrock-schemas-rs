/* Raw contents of nameable.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.nameable",
  "type": "object",
  "title": "Nameable",
  "additionalProperties": false,
  "description": "Allows this entity to be named (e.g. using a name tag).",
  "definitions": {
    "name_action": {
      "type": "object",
      "additionalProperties": false,
      "description": "Describes the special names for this entity and the events to call when the entity acquires those names.",
      "title": "Name Action",
      "properties": {
        "name_filter": {
          "type": "string",
          "default": "",
          "description": "List of special names that will cause the events defined in `on_named` to fire.",
          "title": "Name Filter"
        },
        "on_named": {
          "$ref": "../types/event_object.json",
          "description": "Event to be called when this entity acquires the name specified in `name_filter'.",
          "title": "On Named"
        }
      }
    }
  },
  "properties": {
    "allow_name_tag_renaming": {
      "type": "boolean",
      "default": true,
      "description": "If true, this entity can be renamed with name tags.",
      "title": "Allow Name Tag Renaming"
    },
    "always_show": {
      "type": "boolean",
      "default": false,
      "description": "If true, the name will always be shown.",
      "title": "Always Show"
    },
    "default_trigger": {
      "$ref": "../types/trigger.json",
      "description": "Trigger to run when the entity gets named.",
      "title": "Default Trigger"
    },
    "name_actions": {
      "title": "Name Actions",
      "description": "Describes the special names for this entity and the events to call when the entity acquires those names.",
      "oneOf": [
        {
          "type": "array",
          "items": {
            "$ref": "#/definitions/name_action"
          }
        },
        {
          "type": "object",
          "$ref": "#/definitions/name_action"
        }
      ]
    }
  },
  "examples": [
    {
      "allow_name_tag_renaming": true,
      "always_show": false
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Nameable {}
