/* Raw contents of player.exhaustion.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.player.exhaustion",
  "additionalProperties": false,
  "type": "object",
  "title": "Player.exhaustion",
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
pub struct Player.exhaustion {}
