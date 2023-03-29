/* Raw contents of run_around_like_crazy.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.run_around_like_crazy",
  "description": "Allows the mob to run around aimlessly.",
  "type": "object",
  "title": "Run Around Like Crazy",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": {
      "$ref": "types/priority.json"
    },
    "speed_multiplier": {
      "$ref": "types/speed_multiplier.json"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunAroundLikeCrazy {}
