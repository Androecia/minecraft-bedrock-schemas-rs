/* Raw contents of is_brightness.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.is_brightness",
  "type": "object",
  "title": "Is Brightness",
  "description": "Tests the current brightness against a provided value in the range (0.0f, 1.0f).",
  "required": ["value"],
  "properties": {
    "test": {
      "type": "string",
      "title": "Test Property",
      "description": "Tests the current brightness against a provided value in the range (0.0f, 1.0f)."
    },
    "operator": {
      "$ref": "./types/operator.json"
    },
    "subject": {
      "$ref": "./types/subject.json"
    },
    "value": {
      "description": "The brightness value to compare with.",
      "type": "number",
      "minimum": 0.0,
      "maximum": 1.0,
      "title": "Value"
    }
  },
  "examples": [
    {
      "test": "is_brightness",
      "value": 0.0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsBrightness {}
