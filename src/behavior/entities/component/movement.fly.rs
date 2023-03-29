/* Raw contents of movement.fly.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.movement.fly",
  "type": "object",
  "title": "Movement Fly",
  "description": "This move control causes the mob to fly.",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "max_turn": {
      "type": "number",
      "default": 30,
      "description": "The maximum number in degrees the mob can turn per tick.",
      "title": "Maximum Turn"
    }
  },
  "examples": [
    {
      "max_turn": 30
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Movement.fly {}
