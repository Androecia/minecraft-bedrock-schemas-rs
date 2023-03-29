/* Raw contents of is_variant.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.is_variant",
  "type": "object",
  "title": "Is Variant",
  "description": "Returns true if the subject entity is the variant number provided.",
  "required": ["value"],
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
      "test": "is_variant",
      "value": 0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsVariant {}
