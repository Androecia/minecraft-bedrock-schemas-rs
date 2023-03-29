/* Raw contents of water_movement.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.water_movement",
  "description": "Defines the speed with which an entity can move through water.",
  "type": "object",
  "title": "Water Movement",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "drag_factor": {
      "type": "number",
      "default": 0.8,
      "description": "Drag factor to determine movement speed when in water.",
      "title": "Drag Factor"
    }
  },
  "examples": [{ "drag_factor": 0.8 }]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaterMovement {}
