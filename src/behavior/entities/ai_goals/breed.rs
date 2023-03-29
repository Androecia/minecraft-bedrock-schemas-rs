/* Raw contents of breed.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.breed",
  "additionalProperties": false,
  "type": "object",
  "title": "Breed",
  "description": "Allows this mob to breed with other mobs.",
  "required": [],
  "properties": {
    "priority": {
      "$ref": "types/priority.json"
    },
    "speed_multiplier": {
      "$ref": "./types/speed_multiplier.json"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Breed {}
