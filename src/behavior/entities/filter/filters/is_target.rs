/* Raw contents of is_target.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.is_target",
  "type": "object",
  "title": "Is Target",
  "description": "Returns true if the subject entity is the target of the calling entity.",
  "properties": {
    "test": {
      "type": "string",
      "title": "Test Property",
      "description": "The test property."
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
      "test": "is_target",
      "value": true
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsTarget {}
