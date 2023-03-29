/* Raw contents of rail_movement.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.rail_movement",
  "type": "object",
  "title": "Rail Movement",
  "description": "Defines the entity's movement on the rails. An entity with this component is only allowed to move on the rail.",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "max_speed": {
      "type": "number",
      "default": 0.4,
      "description": "Maximum speed that this entity will move at when on the rail.",
      "title": "Maximum Speed"
    }
  },
  "examples": [
    {
      "max_speed": 0.4
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RailMovement {}
