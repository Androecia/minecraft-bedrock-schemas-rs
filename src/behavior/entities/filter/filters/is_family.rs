/* Raw contents of is_family.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.is_family",
  "type": "object",
  "title": "Is Family",
  "description": "Returns true when the subject entity is a member of the named family.",
  "required": ["value"],
  "properties": {
    "test": {
      "type": "string",
      "title": "Test Property",
      "description": "Returns true when the subject entity is a member of the named family."
    },
    "operator": {
      "$ref": "./types/operator.json"
    },
    "subject": {
      "$ref": "./types/subject.json"
    },
    "value": {
      "description": "The Family name to look for.",
      "type": "string",
      "title": "Value"
    }
  },
  "examples": [
    {
      "test": "is_family",
      "value": "monster"
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsFamily {}
