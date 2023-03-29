/* Raw contents of has_container_open.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.has_container_open",
  "type": "object",
  "title": "Has Container Open",
  "description": "Returns true when the subject Player entity has opened a container.",
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
      "description": "(Optional) true or false.",
      "title": "Value",
      "type": "boolean",
      "default": true
    }
  },
  "examples": [
    {
      "test": "has_container_open",
      "value": true
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HasContainerOpen {}
