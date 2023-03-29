/* Raw contents of stay_while_sitting.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.stay_while_sitting",
  "description": "Allows the mob to stay put while it is in a sitting state instead of doing something else.",
  "type": "object",
  "title": "Stay While Sitting",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StayWhileSitting {}
