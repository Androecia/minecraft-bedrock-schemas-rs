/* Raw contents of move_towards_target.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.move_towards_target",
  "type": "object",
  "title": "Move Towards Target",
  "additionalProperties": false,
  "description": "Allows mob to move towards its current target.",
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },
    "speed_multiplier": {
      "$ref": "./types/speed_multiplier.json"
    },
    "within_radius": {
      "type": "number",
      "default": 0.0,
      "description": "Defines the radius in blocks that the mob tries to be from the target. A value of 0 means it tries to occupy the same block as the target",
      "title": "Within Radius"
    }
  },
  "examples": [
    {
      "within_radius": 0.0
    }
  ]
}*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoveTowardsTarget {}
