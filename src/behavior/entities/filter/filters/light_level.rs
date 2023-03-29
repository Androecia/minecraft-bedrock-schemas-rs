/* Raw contents of light_level.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.light_level",
  "type": "object",
  "title": "Light Level",
  "description": "Tests is the mob is outside of the specified light level range (0, 16).",
  "required": ["value"],
  "properties": {
    "test": {
      "type": "string",
      "title": "Test",
      "description": "The test property."
    },
    "operator": {
      "$ref": "./types/operator.json"
    },
    "subject": {
      "$ref": "./types/subject.json"
    },
    "value": {
      "description": "An integer value.",
      "type": "integer",
      "title": "Value",
      "minimum": 0,
      "maximum": 16
    }
  },
  "examples": [
    {
      "test": "light_level",
      "value": 0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LightLevel {}
