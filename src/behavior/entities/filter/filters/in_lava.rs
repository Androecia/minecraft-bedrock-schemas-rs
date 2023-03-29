/* Raw contents of in_lava.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.in_lava",
  "type": "object",
  "title": "In Lava",
  "description": "Returns true when the subject entity is in lava.",
  "properties": {
    "test": {
      "type": "string",
      "title": "Test Property",
      "description": "Returns true when the subject entity is in lava."
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
      "test": "in_lava",
      "value": true
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InLava {}
