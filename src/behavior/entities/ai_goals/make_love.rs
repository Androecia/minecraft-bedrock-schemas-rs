/* Raw contents of make_love.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.make_love",
  "description": "Allows the villager to look for a mate to spawn other villagers with. Can only be used by Villagers.",
  "type": "object",
  "title": "Make Love",
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
pub struct MakeLove {}
