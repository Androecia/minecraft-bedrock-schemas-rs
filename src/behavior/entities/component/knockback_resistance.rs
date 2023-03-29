/* Raw contents of knockback_resistance.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.knockback_resistance",
  "type": "object",
  "title": "Knockback Resistance",
  "additionalProperties": false,
  "description": "Determines the amount of knockback resistance that the item has.",
  "$comment": "UNDOCUMENTED",
  "required": [],
  "properties": {
    "value": {
      "title": "Value",
      "type": "number",
      "default": 1.0,
      "description": "Percentage of knockback to reduce with 1.0 being 100% reduction.",
      "$comment": "UNDOCUMENTED",
      "maximum": 1
    }
  },
  "examples": [
    {
      "value": 1.0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KnockbackResistance {}
