/* Raw contents of move_to_land.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.move_to_land",
  "description": "Allows the mob to move back onto land when in water.",
  "type": "object",
  "title": "Move To Land",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },
    "speed_multiplier": {
      "$ref": "./types/speed_multiplier.json"
    },
    "goal_radius": {
      "type": "number",
      "default": 0.5,
      "description": "Distance in blocks within the mob considers it has reached the goal. This is the `wiggle room` to stop the AI from bouncing back and forth trying to reach a specific spot",
      "title": "Goal Radius"
    },
    "search_count": {
      "type": "integer",
      "default": 10,
      "description": "The number of blocks each tick that the mob will check within it's search range and height for a valid block to move to. A value of 0 will have the mob check every block within range in one tick",
      "title": "Search Count"
    },
    "search_height": {
      "type": "integer",
      "default": 1,
      "description": "Height in blocks the mob will look for land to move towards.",
      "title": "Search Height"
    },
    "search_range": {
      "type": "integer",
      "default": 0,
      "description": "The distance in blocks it will look for land to move towards.",
      "title": "Search Range"
    }
  },
  "examples": [
    {
      "goal_radius": 0.5,
      "search_count": 10,
      "search_height": 1,
      "search_range": 0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoveToLand {}
