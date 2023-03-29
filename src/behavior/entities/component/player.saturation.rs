/* Raw contents of player.saturation.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.player.saturation",
  "additionalProperties": false,
  "type": "object",
  "title": "Player.saturation",
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
    }
  },
  "examples": [
    {
      "value": true
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player.saturation {}
