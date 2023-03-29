/* Raw contents of is_mark_variant.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.is_mark_variant",
  "type": "object",
  "title": "Is Mark Variant",
  "description": "Returns true if the subject entity is the mark variant number provided.",
  "additionalProperties": false,
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
      "type": "integer",
      "description": "The altitude value to compare with.",
      "title": "Value"
    }
  },
  "examples": [
    {
      "test": "is_mark_variant",
      "value": 0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsMarkVariant {}
