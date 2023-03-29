/* Raw contents of in_clouds.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.in_clouds",
  "type": "object",
  "title": "In Clouds",
  "description": "Returns true when the subject entity is in the clouds.",
  "properties": {
    "test": {
      "type": "string",
      "title": "Test Property",
      "description": "Returns true when the subject entity is in the clouds."
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
      "test": "in_clouds",
      "value": true
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InClouds {}
