/* Raw contents of follow_owner.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.follow_owner",
  "type": "object",
  "title": "Follow Owner",
  "description": "Allows the mob to follow the player that owns them.",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": {
      "$ref": "types/priority.json"
    },
    "speed_multiplier": {
      "$ref": "./types/speed_multiplier.json"
    },
    "can_teleport": {
      "title": "Can Teleport",
      "type": "boolean",
      "default": true,
      "description": "Specify if the mob can teleport to the player if it is too far away."
    },
    "ignore_vibration": {
      "title": "Ignore Vibration",
      "type": "boolean",
      "default": true,
      "description": "Specify if the mob will follow the owner if it has heard a vibration lately."
    },
    "max_distance": {
      "title": "Max Distance",
      "type": "number",
      "default": 60.0,
      "description": "The maximum distance in blocks this mob can be from its owner to start following, only used when canTeleport is false."
    },
    "start_distance": {
      "title": "Start Distance",
      "type": "number",
      "default": 10.0,
      "description": "The distance in blocks that the owner can be away from this mob before it starts following it."
    },
    "stop_distance": {
      "title": "Stop Distance",
      "type": "number",
      "default": 2.0,
      "description": "The distance in blocks this mob will stop from its owner while following it."
    }
  },
  "examples": [
    {
      "start_distance": 10,
      "stop_distance": 2
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FollowOwner {}
