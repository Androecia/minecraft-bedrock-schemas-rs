/* Raw contents of move_to_village.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.move_to_village",
  "type": "object",
  "title": "Move To Village",
  "additionalProperties": false,
  "description": "Allows the mob to move into a random location within a village.",
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },
    "speed_multiplier": {
      "$ref": "./types/speed_multiplier.json"
    },
    "cooldown_time": {
      "type": "number",
      "default": 0,
      "description": "Time in seconds the mob has to wait before using the goal again.",
      "title": "Cooldown Time"
    },
    "goal_radius": {
      "type": "number",
      "default": 0.5,
      "description": "Distance in blocks within the mob considers it has reached the goal. This is the `wiggle room` to stop the AI from bouncing back and forth trying to reach a specific spot",
      "title": "Goal Radius"
    },
    "search_range": {
      "type": "integer",
      "default": 0,
      "description": "The distance in blocks to search for villages. If <= 0, find the closest village regardless of distance.",
      "title": "Search Range"
    }
  },
  "examples": [
    {
      "cooldown_time": 0,
      "goal_radius": 0.5,
      "search_range": 0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoveToVillage {}
