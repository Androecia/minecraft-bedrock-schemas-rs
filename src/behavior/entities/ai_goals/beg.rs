/* Raw contents of beg.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.beg",
  "type": "object",
  "title": "Beg",
  "additionalProperties": false,
  "description": "Allows this mob to look at and follow the player that holds food they like.",
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },
    "items": {
      "type": "array",
      "description": "List of items that this mob likes.",
      "items": {
        "type": "string",
        "description": "List of items that this mob likes.",
        "title": "Properties",
        "$ref": "../../../../general/item/descriptor.json"
      },
      "title": "Properties"
    },
    "look_distance": {
      "type": "number",
      "default": 8,
      "description": "Distance in blocks the mob will beg from.",
      "title": "Look Distance"
    },
    "look_time": {
      "description": "The range of time in seconds this mob will stare at the player holding a food they like, begging for it.",
      "$ref": "../types/range_number_type.json",
      "default": [2, 4],
      "title": "Look Time"
    }
  },
  "examples": [
    {
      "items": [],
      "look_distance": 8
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Beg {}
