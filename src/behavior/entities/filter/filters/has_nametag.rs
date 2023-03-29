/* Raw contents of has_nametag.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.has_nametag",
  "type": "object",
  "title": "Has Equipment",
  "description": "Tests for the presence of a named item in the designated slot of the subject entity.",
  "required": ["value"],
  "properties": {
    "test": {
      "type": "string",
      "const": "has_nametag",
      "description": "Tests for the presence of a named item in the designated slot of the subject entity.",
      "title": "Test"
    },
    "domain": {
      "description": "The equipment location to test.",
      "default": "any",
      "enum": ["any", "armor", "feet", "hand", "head", "leg", "torso"],
      "title": "Domain"
    },
    "operator": {
      "$ref": "./types/operator.json"
    },
    "subject": {
      "$ref": "./types/subject.json"
    },
    "value": {
      "description": "The namtag to look for",
      "type": "boolean",
      "title": "Value"
    }
  },
  "examples": [
    { "test": "has_nametag", "value": false },
    { "test": "has_nametag", "value": true }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HasNametag {}
