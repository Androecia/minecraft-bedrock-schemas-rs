/* Raw contents of celebrate_survive.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.celebrate_survive",
  "additionalProperties": false,
  "type": "object",
  "title": "Celebrate Survive",
  "description": "Allows this entity to celebrate surviving a raid by shooting fireworks.",
  "properties": {
    "priority": { "$ref": "types/priority.json" },
    "speed_multiplier": { "$ref": "./types/speed_multiplier.json" },
    "fireworks_interval": {
      "title": "Fireworks Interval",
      "$ref": "../types/range_number_type.json",
      "description": "Minimum and maximum time between firework (positive, in seconds).",
      "default": [10, 20]
    },
    "duration": {
      "title": "Duration",
      "description": "The duration in seconds that the celebration lasts for.",
      "type": "number",
      "default": 30.0
    },
    "on_celebration_end_event": {
      "title": "On Celebration End Event",
      "description": "The event to trigger when the goal's duration expires.",
      "$ref": "../types/event.json"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CelebrateSurvive {}
