/* Raw contents of is_waterlogged.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.is_waterlogged",
  "type": "object",
  "title": "Is Waterlogged",
  "description": "Tests if the subject block is submerged in water.",
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
      "description": "true or false.",
      "type": "boolean",
      "title": "Value"
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
pub struct IsWaterlogged {}
