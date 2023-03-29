/* Raw contents of strength.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.strength",
  "description": "Defines the entity's strength to carry items.",
  "type": "object",
  "title": "Strength",
  "additionalProperties": false,
  "properties": {
    "max": {
      "type": "integer",
      "default": 5,
      "description": "The maximum strength of this entity.",
      "title": "Maximum"
    },
    "value": {
      "type": "integer",
      "default": 1,
      "description": "The initial value of the strength.",
      "title": "Value"
    }
  },
  "examples": [
    {
      "max": 5,
      "value": 1
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Strength {}
