/* Raw contents of underwater_movement.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.underwater_movement",
  "description": "UNDOCUMENTED.",
  "$comment": "UNDOCUMENTED",
  "type": "object",
  "title": "Underwater Movement",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "value": {
      "type": "number",
      "title": "Value",
      "description": "UNDOCUMENTED.",
      "$comment": "UNDOCUMENTED"
    }
  },
  "examples": [
    {
      "value": 0.0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnderwaterMovement {}
