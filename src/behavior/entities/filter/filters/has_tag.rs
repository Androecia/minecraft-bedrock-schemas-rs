/* Raw contents of has_tag.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.has_tag",
  "type": "object",
  "title": "Has Tag",
  "description": "Returns true if the subject entity has the tag provided.",
  "required": ["value"],
  "properties": {
    "test": {
      "type": "string",
      "title": "Test Property",
      "description": "Returns true if the subject entity has the tag provided."
    },
    "operator": {
      "$ref": "./types/operator.json"
    },
    "subject": {
      "$ref": "./types/subject.json"
    },
    "value": {
      "type": "string",
      "description": "The tag as a string.",
      "pattern": "[a-zA-Z0-9_]+",
      "title": "Value"
    }
  },
  "examples": [
    {
      "test": "has_tag",
      "value": "example"
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HasTag {}
