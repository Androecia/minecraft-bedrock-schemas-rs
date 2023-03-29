/* Raw contents of has_silk_touch.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.has_silk_touch",
  "type": "object",
  "title": "Has Silk Touch",
  "description": "Tests if the subject is holding an item with silk touch.",
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
      "test": "has_silk_touch",
      "subject": "other",
      "value": true
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HasSilkTouch {}
