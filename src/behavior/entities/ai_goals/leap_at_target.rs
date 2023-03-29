/* Raw contents of leap_at_target.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.leap_at_target",
  "description": "Allows monsters to jump at and attack their target. Can only be used by hostile mobs.",
  "type": "object",
  "title": "Leap At Target",
  "additionalProperties": false,
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },
    "must_be_on_ground": {
      "type": "boolean",
      "default": true,
      "description": "If true, the mob will only jump at its target if its on the ground. Setting it to false will allow it to jump even if its already in the air",
      "title": "Must Be On Ground"
    },
    "set_persistent": {
      "type": "boolean",
      "default": false,
      "description": "Allows the actor to be set to persist upon targeting a player.",
      "title": "Set Persistent"
    },
    "yd": {
      "type": "number",
      "default": 0.0,
      "description": "The height in blocks the mob jumps when leaping at its target.",
      "title": "Yd"
    },
    "target_dist": {
      "type": "number",
      "default": 0.3,
      "description": "Distance in blocks the mob jumps when leaping at its target.",
      "title": "Target Dist"
    }
  },
  "examples": [
    {
      "must_be_on_ground": true,
      "set_persistent": false,
      "yd": 0,
      "target_dist": 0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeapAtTarget {}
