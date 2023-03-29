/* Raw contents of player.experience.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.player.experience",
  "additionalProperties": false,
  "type": "object",
  "title": "Player.experience",
  "description": "UNDOCUMENTED.",
  "$comment": "UNDOCUMENTED",
  "required": [],
  "properties": {
    "value": {
      "type": "number",
      "default": true,
      "title": "Value",
      "description": "UNDOCUMENTED.",
      "$comment": "UNDOCUMENTED"
    },
    "max": {
      "type": "number",
      "default": true,
      "title": "Maximum",
      "description": "UNDOCUMENTED.",
      "$comment": "UNDOCUMENTED"
    }
  },
  "examples": [
    {
      "value": true,
      "max": true
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player.experience {}
