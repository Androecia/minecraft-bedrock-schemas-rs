/* Raw contents of movement.hover.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.movement.hover",
  "title": "Movement Hover",
  "description": "This move control causes the mob to hover.",
  "type": "object",
  "additionalProperties": false,
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
pub struct Movement.hover {}
