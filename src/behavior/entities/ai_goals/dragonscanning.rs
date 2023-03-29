/* Raw contents of dragonscanning.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.dragonscanning",
  "additionalProperties": false,
  "type": "object",
  "title": "Dragonscanning",
  "description": "Allows an entity to look around for a player to attack while in perch mode. Note: This behavior can only be used by the ender_dragon entity type.",
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dragonscanning {}
