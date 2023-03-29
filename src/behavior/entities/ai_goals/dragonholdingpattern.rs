/* Raw contents of dragonholdingpattern.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.dragonholdingpattern",
  "additionalProperties": false,
  "type": "object",
  "title": "Dragonholdingpattern",
  "description": "Allows the Dragon to fly around in a circle around the center podium. Note: This behavior can only be used by the ender_dragon entity type.",
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dragonholdingpattern {}
