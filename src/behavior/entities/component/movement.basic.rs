/* Raw contents of movement.basic.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.movement.basic",
  "type": "object",
  "title": "Movement Basic",
  "description": "defines the movement of an entity.",
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
pub struct Movement.basic {}
