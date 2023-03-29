/* Raw contents of restrict_open_door.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.restrict_open_door",
  "description": "Allows the mob to stay indoors during night time.",
  "type": "object",
  "title": "Restrict Open Door",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": {
      "$ref": "types/priority.json"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RestrictOpenDoor {}
