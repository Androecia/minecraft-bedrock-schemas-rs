/* Raw contents of move_indoors.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.move_indoors",
  "description": "Can only be used by Villagers. Allows them to seek shelter indoors.",
  "type": "object",
  "title": "Move Indoors",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },
    "speed_multiplier": {
      "$ref": "./types/speed_multiplier.json"
    },
    "timeout_cooldown": {
      "type": "number",
      "default": 8,
      "description": "The cooldown time in seconds before the goal can be reused after pathfinding fails.",
      "title": "Timeout Cooldown"
    }
  },
  "examples": [
    {
      "timeout_cooldown": 8
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoveIndoors {}
