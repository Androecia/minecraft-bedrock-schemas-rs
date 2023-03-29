/* Raw contents of random_look_around_and_sit.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.random_look_around_and_sit",
  "description": "Allows the mob to randomly sit and look around for a duration. Note: Must have a sitting animation set up to use this.",
  "type": "object",
  "title": "Random Look Around And Sit",
  "additionalProperties": false,
  "properties": {
    "priority": {
      "$ref": "types/priority.json"
    },
    "continue_if_leashed": {
      "title": "Continue If Leashed",
      "type": "boolean",
      "default": false,
      "description": "If the goal should continue to be used as long as the mob is leashed."
    },
    "max_angle_of_view_horizontal": {
      "title": "Max Angle Of View Horizontal",
      "type": "number",
      "default": 30.0,
      "description": "The rightmost angle a mob can look at on the horizontal plane with respect to its initial facing direction."
    },
    "max_look_count": {
      "title": "Max Look Count",
      "type": "integer",
      "default": 2,
      "description": "The max amount of unique looks a mob will have while looking around."
    },
    "max_look_time": {
      "title": "Max Look Time",
      "type": "integer",
      "default": 40,
      "description": "The max amount of time (in ticks) a mob will stay looking at a direction while looking around."
    },
    "min_angle_of_view_horizontal": {
      "title": "Min Angle Of View Horizontal",
      "type": "number",
      "default": -30.0,
      "description": "The leftmost angle a mob can look at on the horizontal plane with respect to its initial facing direction."
    },
    "min_look_count": {
      "title": "Min Look Count",
      "type": "integer",
      "default": 1,
      "description": "The min amount of unique looks a mob will have while looking around."
    },
    "min_look_time": {
      "title": "Min Look Time",
      "type": "integer",
      "default": 20,
      "description": "The min amount of time (in ticks) a mob will stay looking at a direction while looking around."
    },
    "probability": {
      "title": "Probability",
      "type": "number",
      "default": 0.02,
      "description": "The probability of randomly looking around/sitting."
    },
    "random_look_around_cooldown": {
      "title": "Random Look Around Cooldown",
      "type": "integer",
      "default": 0,
      "description": "The cooldown in seconds before the goal can be used again."
    }
  },
  "examples": [
    {
      "max_look_count": 2,
      "max_look_time": 40,
      "min_look_count": 1,
      "min_look_time": 20,
      "probability": 0.02
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RandomLookAroundAndSit {}
