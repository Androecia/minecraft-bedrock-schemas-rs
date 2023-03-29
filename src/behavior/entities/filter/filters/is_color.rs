/* Raw contents of is_color.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.is_color",
  "type": "object",
  "title": "Is Color",
  "description": "Returns true if the subject entity is the named color.",
  "required": ["value"],
  "properties": {
    "test": {
      "type": "string",
      "title": "Test Property",
      "description": "Returns true if the subject entity is the named color."
    },
    "operator": {
      "$ref": "./types/operator.json"
    },
    "subject": {
      "$ref": "./types/subject.json"
    },
    "value": {
      "description": "The Palette Color to test.",
      "type": "string",
      "enum": ["black", "blue", "brown", "cyan", "gray", "green", "light_blue", "light_green", "magenta", "orange", "pink", "purple", "red", "silver", "white", "yellow"],
      "title": "Value"
    }
  },
  "examples": [
    {
      "test": "is_color",
      "value": "black"
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsColor {}
