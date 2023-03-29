/* Raw contents of dragonlanding.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.dragonlanding",
  "additionalProperties": false,
  "type": "object",
  "title": "Dragonlanding",
  "description": "Allows the Dragon to stop flying and transition into perching mode. Note: This behavior can only be used by the ender_dragon entity type.",
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dragonlanding {}
