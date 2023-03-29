/* Raw contents of follow_mob.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.follow_mob",
  "type": "object",
  "title": "Follow Mob",
  "description": "Allows the mob to follow other mobs.",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": {
      "$ref": "types/priority.json"
    },
    "speed_multiplier": {
      "$ref": "./types/speed_multiplier.json"
    },
    "search_range": {
      "type": "integer",
      "default": 0,
      "description": "The distance in blocks it will look for a mob to follow.",
      "title": "Search Range"
    },
    "stop_distance": {
      "type": "number",
      "default": 2,
      "description": "The distance in blocks this mob stops from the mob it is following.",
      "title": "Stop Distance"
    }
  },
  "examples": [
    {
      "search_range": 0,
      "stop_distance": 2
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FollowMob {}
