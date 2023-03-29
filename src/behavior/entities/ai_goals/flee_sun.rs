/* Raw contents of flee_sun.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.flee_sun",
  "type": "object",
  "title": "Flee Sun",
  "description": "Allows the mob to run away from direct sunlight and seek shade.",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": {
      "$ref": "types/priority.json"
    },
    "speed_multiplier": {
      "$ref": "types/speed_multiplier.json"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FleeSun {}
