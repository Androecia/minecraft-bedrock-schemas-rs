/* Raw contents of is_humid.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.is_humid",
  "type": "object",
  "title": "Is Humid",
  "description": "Tests whether the Subject is in an area with humidity.",
  "properties": {
    "test": {
      "type": "string",
      "title": "Test Property",
      "description": "Tests whether the Subject is in an area with humidity."
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
      "test": "is_humid",
      "value": true
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsHumid {}
