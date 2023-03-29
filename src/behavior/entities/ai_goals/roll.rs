/* Raw contents of roll.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.roll",
  "description": "This allows the mob to roll forward.",
  "type": "object",
  "title": "Roll",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": {
      "$ref": "types/priority.json"
    },
    "probability": {
      "type": "number",
      "minimum": 0,
      "description": "The probability that the mob will use the goal.",
      "title": "Probability"
    }
  },
  "examples": [
    {
      "probability": 0.0
    }
  ]
}*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Roll {}
