/* Raw contents of is_owner.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.is_owner",
  "type": "object",
  "title": "Is Owner",
  "description": "Returns true if the subject entity is the owner of the calling entity.",
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
      "test": "is_owner",
      "value": true
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsOwner {}
