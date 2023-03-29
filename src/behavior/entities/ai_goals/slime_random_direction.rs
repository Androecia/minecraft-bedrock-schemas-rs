/* Raw contents of slime_random_direction.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.slime_random_direction",
  "additionalProperties": false,
  "description": "Can only be used by Slimes and Magma Cubes. Allows the mob to move in random directions like a slime.",
  "type": "object",
  "title": "Slime Random Direction",
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },
    "add_random_time_range": {
      "title": "Add Random Time Range",
      "type": "integer",
      "default": 3,
      "description": "Additional time (in whole seconds), chosen randomly in the range of [0, \"add_random_time_range\"], to add to \"min_change_direction_time\"."
    },
    "min_change_direction_time": {
      "title": "Min Change Direction Time",
      "type": "number",
      "default": 2,
      "description": "Constant minimum time (in seconds) to wait before choosing a new direction."
    },
    "turn_range": {
      "title": "Turn Range",
      "type": "integer",
      "default": 360,
      "description": "Maximum rotation angle range (in degrees) when randomly choosing a new direction."
    }
  },
  "examples": [
    {
      "add_random_time_range": 0.0,
      "turn_range": 0.0,
      "min_change_direction_time": 0.0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SlimeRandomDirection {}
