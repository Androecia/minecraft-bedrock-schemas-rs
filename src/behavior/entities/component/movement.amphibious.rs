/* Raw contents of movement.amphibious.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.movement.amphibious",
  "type": "object",
  "title": "Movement Amphibious",
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
  "description": "This move control allows the mob to swim in water and walk on land.",
  "examples": [
    {
      "max_turn": 30
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Movement.amphibious {}
