/* Raw contents of random_look_around.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.random_look_around",
  "additionalProperties": false,
  "description": "Allows the mob to randomly look around.",
  "type": "object",
  "title": "Random Look Around",
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },
    "look_time": {
      "title": "Look Time",
      "$ref": "../../../../general/vectors/number2.json",
      "description": "The range of time in seconds the mob will stay looking in a random direction before looking elsewhere"
    },
    "max_angle_of_view_horizontal": {
      "title": "Max Angle Of View Horizontal",
      "type": "integer",
      "default": 30,
      "description": "The rightmost angle a mob can look at on the horizontal plane with respect to its initial facing direction."
    },
    "min_angle_of_view_horizontal": {
      "title": "Min Angle Of View Horizontal",
      "type": "integer",
      "default": -30,
      "description": "The leftmost angle a mob can look at on the horizontal plane with respect to its initial facing direction."
    }
  },
  "examples": [
    {
      "look_distance": 0.0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RandomLookAround {}
