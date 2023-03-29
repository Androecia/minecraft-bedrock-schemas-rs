/* Raw contents of find_cover.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.find_cover",
  "type": "object",
  "title": "Find Cover",
  "description": "Allows the mob to seek shade.",
  "additionalProperties": false,
  "properties": {
    "priority": { "$ref": "types/priority.json" },
    "speed_multiplier": { "$ref": "types/speed_multiplier.json" },
    "cooldown_time": {
      "type": "number",
      "default": 0,
      "description": "Time in seconds the mob has to wait before using the goal again.",
      "title": "Cooldown Time"
    }
  },
  "examples": [
    {
      "cooldown_time": 0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FindCover {}
