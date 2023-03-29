/* Raw contents of is_leashed_to.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.is_leashed_to",
  "type": "object",
  "title": "Is Leashed To",
  "description": "Returns true if the subject entity leashed to the calling entity.",
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
      "test": "is_leashed_to",
      "value": true
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsLeashedTo {}
