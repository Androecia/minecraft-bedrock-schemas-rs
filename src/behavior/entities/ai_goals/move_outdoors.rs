/* Raw contents of move_outdoors.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.move_outdoors",
  "additionalProperties": false,
  "type": "object",
  "title": "Move Outdoors",
  "$comment": "UNDOCUMENTED",
  "description": "Forces the entity to move `outside`, whatever that means.",
  "properties": {
    "priority": { "$ref": "types/priority.json" },
    "speed_multiplier": { "$ref": "./types/speed_multiplier.json" },
    "goal_radius": {
      "title": "Goal Radius",
      "type": "number",
      "default": 0.5,
      "description": "The radius away from the target block to count as reaching the goal."
    },
    "search_count": {
      "title": "Search Count",
      "type": "integer",
      "default": 0.0,
      "description": "The amount of times to try finding a random outdoors position before failing."
    },
    "search_height": {
      "title": "Search Height",
      "type": "integer",
      "default": 0.0,
      "description": "The y range to search for an outdoors position for."
    },
    "search_range": {
      "title": "Search Range",
      "type": "integer",
      "default": 0.0,
      "description": "The x and z range to search for an outdoors position for."
    },
    "timeout_cooldown": {
      "title": "Timeout Cooldown",
      "type": "number",
      "default": 8000000,
      "description": "The cooldown time in seconds before the goal can be reused after pathfinding fails."
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoveOutdoors {}
