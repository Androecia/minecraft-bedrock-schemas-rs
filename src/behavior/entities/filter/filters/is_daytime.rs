/* Raw contents of is_daytime.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.is_daytime",
  "type": "object",
  "title": "Is Daytime",
  "description": "Returns true during the daylight hours.",
  "properties": {
    "test": {
      "type": "string",
      "title": "Test Property",
      "description": "Returns true during the daylight hours."
    },
    "operator": {
      "$ref": "./types/operator.json"
    },
    "subject": {
      "$ref": "./types/subject.json"
    },
    "value": {
      "title": "Value",
      "description": "True or false.",
      "type": "boolean",
      "default": true
    }
  },
  "examples": [
    {
      "test": "is_daytime",
      "value": true
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsDaytime {}
