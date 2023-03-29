/* Raw contents of in_water.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.in_water",
  "type": "object",
  "title": "In Water",
  "description": "Returns true when the subject entity is in water.",
  "properties": {
    "test": {
      "type": "string",
      "title": "Test Property",
      "description": "Returns true when the subject entity is in water."
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
      "test": "in_water",
      "value": true
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InWater {}
