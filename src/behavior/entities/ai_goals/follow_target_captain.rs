/* Raw contents of follow_target_captain.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.follow_target_captain",
  "type": "object",
  "title": "Follow Target Captain",
  "description": "Allows mob to move towards its current target captain.",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": { "$ref": "types/priority.json" },
    "speed_multiplier": { "$ref": "types/speed_multiplier.json" },
    "follow_distance": {
      "type": "number",
      "default": 0,
      "description": "Defines the distance in blocks the mob will stay from its target while following.",
      "title": "Follow Distance"
    },
    "within_radius": {
      "type": "number",
      "default": 0,
      "description": "Defines the maximum distance in blocks a mob can get from its target captain before giving up trying to follow it.",
      "title": "Within Radius"
    }
  },
  "examples": [
    {
      "follow_distance": 0,
      "within_radius": 0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FollowTargetCaptain {}
