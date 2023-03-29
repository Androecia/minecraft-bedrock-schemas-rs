/* Raw contents of follow_range.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.follow_range",
  "type": "object",
  "title": "Follow Range",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "value": {
      "type": "integer",
      "description": "UNDOCUMENTED.",
      "$comment": "UNDOCUMENTED",
      "title": "Value"
    },
    "max": {
      "type": "integer",
      "description": "UNDOCUMENTED.",
      "$comment": "UNDOCUMENTED",
      "title": "Max"
    }
  },
  "description": "UNDOCUMENTED.",
  "examples": [
    {
      "value": 0,
      "max": 0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FollowRange {}
