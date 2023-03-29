/* Raw contents of movement.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.movement",
  "type": "object",
  "title": "Movement",
  "description": "UNDOCUMENTED.",
  "$comment": "UNDOCUMENTED",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "value": {
      "title": "Value",
      "description": "UNDOCUMENTED.",
      "$comment": "UNDOCUMENTED",
      "oneOf": [
        {
          "type": "array",
          "items": [
            { "type": "number", "title": "Maximum" },
            { "type": "number", "title": "Maximum" }
          ]
        },
        {
          "type": "number"
        },
        {
          "type": "object",
          "additionalProperties": false,
          "title": "Range",
          "properties": {
            "range_min": {
              "type": "number",
              "title": "Range Minimum"
            },
            "range_max": {
              "type": "number",
              "title": "Range Maximum"
            }
          }
        }
      ]
    },
    "max": {
      "type": "number",
      "title": "Maximum",
      "description": "UNDOCUMENTED.",
      "$comment": "UNDOCUMENTED"
    }
  },
  "examples": [
    {
      "max": 0.0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Movement {}
