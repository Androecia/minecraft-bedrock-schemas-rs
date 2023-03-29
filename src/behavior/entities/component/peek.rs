/* Raw contents of peek.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.peek",
  "description": "Defines the entity's `peek` behavior, defining the events that should be called during it.",
  "type": "object",
  "title": "Peek",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "on_close": {
      "$ref": "../types/event_object.json",
      "description": "Event to call when the entity is done peeking.",
      "title": "On Close"
    },
    "on_open": {
      "$ref": "../types/event_object.json",
      "description": "Event to call when the entity starts peeking.",
      "title": "On Open"
    },
    "on_target_open": {
      "$ref": "../types/event_object.json",
      "description": "Event to call when the entity's target entity starts peeking.",
      "title": "On Target Open"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Peek {}
