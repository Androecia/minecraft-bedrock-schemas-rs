/* Raw contents of has_target.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.has_target",
  "type": "object",
  "title": "Has Target",
  "description": "Returns true if the subject entity has a valid target.",
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
      "description": "True or false.",
      "type": "boolean",
      "default": true,
      "title": "Value"
    }
  },
  "examples": [
    {
      "test": "has_target",
      "value": true
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HasTarget {}
