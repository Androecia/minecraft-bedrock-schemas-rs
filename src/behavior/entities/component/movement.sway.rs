/* Raw contents of movement.sway.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.movement.sway",
  "title": "Movement Sway",
  "description": "This move control causes the mob to sway side to side giving the impression it is swimming.",
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "max_turn": {
      "type": "number",
      "default": 30,
      "description": "The maximum number in degrees the mob can turn per tick.",
      "title": "Maximum Turn"
    },
    "sway_amplitude": {
      "type": "number",
      "description": "Strength of the sway movement.",
      "title": "Sway Amplitude",
      "default": 0.05
    },
    "sway_frequency": {
      "type": "number",
      "description": "Multiplier for the frequency of the sway movement.",
      "title": "Sway Amplitude",
      "default": 0.5
    }
  },
  "examples": [
    {
      "max_turn": 30,
      "sway_amplitude": 0.0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Movement.sway {}
