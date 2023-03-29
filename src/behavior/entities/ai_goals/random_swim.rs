/* Raw contents of random_swim.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.random_swim",
  "type": "object",
  "title": "Random Swim",
  "description": "Allows an entity to randomly move through water.",
  "required": [],
  "properties": {
    "priority": { "$ref": "types/priority.json" },
    "speed_multiplier": { "$ref": "types/speed_multiplier.json" },
    "avoid_surface": {
      "type": "boolean",
      "default": true,
      "description": "If true, the mob will avoid surface water blocks by swimming below them.",
      "title": "Avoid Surface"
    },
    "interval": {
      "type": "integer",
      "default": 120,
      "description": "A random value to determine when to randomly move somewhere. This has a 1/interval chance to choose this goal",
      "title": "Interval"
    },
    "xz_dist": {
      "type": "integer",
      "default": 10,
      "description": "Distance in blocks on ground that the mob will look for a new spot to move to. Must be at least 1",
      "title": "XZ Distance"
    },
    "y_dist": {
      "type": "integer",
      "default": 7,
      "description": "Distance in blocks that the mob will look up or down for a new spot to move to. Must be at least 1",
      "title": "Y Distance"
    }
  },
  "additionalProperties": false,
  "examples": [
    {
      "avoid_surface": true,
      "interval": 120,
      "xz_dist": 10,
      "y_dist": 7
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RandomSwim {}
