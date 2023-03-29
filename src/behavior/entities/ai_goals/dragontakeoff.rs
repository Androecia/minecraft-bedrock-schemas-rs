/* Raw contents of dragontakeoff.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.dragontakeoff",
  "additionalProperties": false,
  "type": "object",
  "title": "Dragontakeoff",
  "description": "Allows an entity to leave perch mode and go back to flying around. Note: This behavior can only be used by the ender_dragon entity type.",
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dragontakeoff {}
