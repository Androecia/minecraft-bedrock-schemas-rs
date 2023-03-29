/* Raw contents of look_at_player.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.look_at_player",
  "type": "object",
  "title": "Look At Player",
  "description": "Allows the mob to look at the player when the player is nearby.",
  "additionalProperties": false,
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },
    "angle_of_view_vertical": {
      "title": "Angle Of View Vertical",
      "type": "integer",
      "default": 360,
      "description": "The angle in degrees that the mob can see in the X-axis (left-right)."
    },
    "angle_of_view_horizontal": {
      "title": "Angle Of View Horizontal",
      "type": "integer",
      "default": 360,
      "description": "The angle in degrees that the mob can see in the Y-axis (up-down)."
    },
    "look_distance": {
      "title": "Look Distance",
      "type": "number",
      "default": 8.0,
      "description": "The distance in blocks from which the entity will look at."
    },
    "probability": {
      "title": "Probability",
      "type": "number",
      "default": 0.02,
      "minimum": 0,
      "description": "The probability of looking at the target. A value of 1.00 is 100%"
    },
    "look_time": {
      "title": "Look Time",
      "type": "array",
      "default": [2, 4],
      "description": "Time range to look at the entity.",
      "items": [
        { "type": "number", "title": "Minimum", "description": "The minimum amount of time to look." },
        { "type": "number", "title": "Maximum", "description": "The maximum amount of time to look." }
      ]
    },
    "target_distance": {
      "title": "Target Distance",
      "type": "number",
      "default": 6.0,
      "description": "The distance in blocks from which the entity will choose a target."
    }
  },
  "examples": [
    {
      "angle_of_view_vertical": 360,
      "angle_of_view_horizontal": 360,
      "look_distance": 8.0,
      "probability": 0.02,
      "look_time": [],
      "target_distance": 6.0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LookAtPlayer {}
