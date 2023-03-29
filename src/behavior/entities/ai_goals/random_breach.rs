/* Raw contents of random_breach.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.random_breach",
  "description": "Allows the mob to randomly break surface of the water.",
  "type": "object",
  "title": "Random Breach",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": { "$ref": "types/priority.json" },
    "speed_multiplier": { "$ref": "types/speed_multiplier.json" },
    "cooldown_time": {
      "type": "number",
      "default": 0,
      "description": "Time in seconds the mob has to wait before using the goal again.",
      "title": "Cooldown Time"
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
  "examples": [
    {
      "cooldown_time": 0,
      "interval": 120,
      "xz_dist": 10,
      "y_dist": 7
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RandomBreach {}
