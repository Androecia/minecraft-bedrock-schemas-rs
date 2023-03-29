/* Raw contents of horse.jump_strength.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.horse.jump_strength",
  "type": "object",
  "title": "Horse Jump Strength",
  "additionalProperties": false,
  "description": "Allows this mob to jump higher when being ridden by a player.",
  "$comment": "UNDOCUMENTED",
  "properties": {
    "value": {
      "description": "The multiplier to apply to the jumping height.",
      "$comment": "UNDOCUMENTED",
      "title": "Value",
      "oneOf": [
        {
          "type": "object",
          "additionalProperties": false,
          "properties": {
            "range_min": {
              "type": "number"
            },
            "range_max": {
              "type": "number"
            }
          }
        },
        {
          "type": "number"
        }
      ]
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Horse.jumpStrength {}
