/* Raw contents of is_climbing.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.is_climbing",
  "type": "object",
  "title": "Is Climbing",
  "description": "Returns true if the subject entity is climbing.",
  "properties": {
    "test": {
      "type": "string",
      "title": "Test Property",
      "description": "Returns true if the subject entity is climbing."
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
      "test": "is_climbing",
      "value": true
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsClimbing {}
