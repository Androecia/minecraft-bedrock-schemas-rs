/* Raw contents of health.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.health",
  "description": "Sets the amount of health this mob has.",
  "$comment": "UNDOCUMENTED",
  "type": "object",
  "title": "Health",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "max": {
      "type": "integer",
      "description": "The maximum health the entity can heal.",
      "title": "Maximum"
    },
    "value": {
      "description": "Current health of the entity.",
      "title": "Value",
      "oneOf": [
        { "type": "integer", "default": 1 },
        {
          "type": "object",
          "additionalProperties": false,
          "required": ["range_min", "range_max"],
          "properties": {
            "range_min": {
              "title": "Range Minimum",
              "description": "The minimum amount of health this mob could have.",
              "$comment": "UNDOCUMENTED",
              "type": "number"
            },
            "range_max": {
              "title": "Range Maximum",
              "description": "The maximum amount of health this mob could have.",
              "$comment": "UNDOCUMENTED",
              "type": "number"
            }
          }
        }
      ]
    }
  },
  "examples": [{ "value": 1, "max": 0 }]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Health {}
