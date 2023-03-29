/* Raw contents of movement.glide.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.movement.glide",
  "type": "object",
  "title": "Movement Glide",
  "description": "This is the move control for a flying mob that has a gliding movement.",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "max_turn": {
      "type": "number",
      "default": 30,
      "description": "The maximum number in degrees the mob can turn per tick.",
      "title": "Maximum Turn"
    },
    "start_speed": {
      "type": "number",
      "title": "Start Speed",
      "description": "UNDOCUMENTED.",
      "$comment": "UNDOCUMENTED"
    },
    "speed_when_turning": {
      "type": "number",
      "title": "Speed When Turning",
      "description": "UNDOCUMENTED.",
      "$comment": "UNDOCUMENTED"
    }
  },
  "examples": [
    {
      "start_speed": 1.0,
      "speed_when_turning": 2.0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Movement.glide {}
