/* Raw contents of fall_damage.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.fall_damage",
  "type": "object",
  "title": "Fall Damage",
  "additionalProperties": false,
  "description": "UNDOCUMENTED.",
  "$comment": "UNDOCUMENTED",
  "required": [],
  "properties": {
    "value": {
      "type": "number",
      "default": 1,
      "description": "UNDOCUMENTED: value.",
      "title": "Value"
    }
  },
  "examples": [
    {
      "value": 1
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FallDamage {}
