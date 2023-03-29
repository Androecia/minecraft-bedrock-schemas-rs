/* Raw contents of movement.generic.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.movement.generic",
  "description": "This move control allows a mob to fly, swim, climb, etc.",
  "type": "object",
  "title": "Movement Generic",
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
pub struct Movement.generic {}
