/* Raw contents of door_interact.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.door_interact",
  "additionalProperties": false,
  "type": "object",
  "title": "Door Interact",
  "description": "Allows the mob to open and close doors.",
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DoorInteract {}
