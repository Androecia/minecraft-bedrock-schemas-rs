/* Raw contents of share_items.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.share_items",
  "description": "Allows the mob to give items it has to others.",
  "type": "object",
  "title": "Share Items",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": { "$ref": "types/priority.json" },
    "speed_multiplier": { "$ref": "types/speed_multiplier.json" },
    "entity_types": {
      "$ref": "../types/entity_types.json",
      "description": "List of entities this mob will share items with.",
      "title": "Entity Types"
    },
    "goal_radius": {
      "type": "number",
      "default": 0.5,
      "description": "Distance in blocks within the mob considers it has reached the goal. This is the `wiggle room` to stop the AI from bouncing back and forth trying to reach a specific spot",
      "title": "Goal Radius"
    },
    "max_dist": {
      "type": "number",
      "default": 0,
      "description": "Maximum distance in blocks this mob will look for entities to share items with.",
      "title": "Maximum Distance"
    }
  },
  "examples": [
    {
      "goal_radius": 0.5,
      "max_dist": 0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShareItems {}
