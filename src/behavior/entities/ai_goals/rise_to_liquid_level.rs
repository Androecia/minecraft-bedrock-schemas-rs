/* Raw contents of rise_to_liquid_level.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.rise_to_liquid_level",
  "description": "Allows the mob to stay at a certain level when in liquid.",
  "type": "object",
  "title": "Rise To Liquid Level",
  "additionalProperties": false,
  "properties": {
    "priority": {
      "$ref": "types/priority.json"
    },
    "liquid_y_offset": {
      "type": "number",
      "title": "Liquid Y Offset",
      "description": "Vertical offset from the liquid."
    },
    "rise_delta": {
      "type": "number",
      "title": "Rise Delta",
      "description": "Displacement for how much the entity will move up in the vertical axis."
    },
    "sink_delta": {
      "type": "number",
      "title": "Sink Delta",
      "description": "Displacement for how much the entity will move down in the vertical axis."
    }
  },
  "examples": [
    {
      "liquid_y_offset": 0.0,
      "rise_delta": 0.0,
      "sink_delta": 0.0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RiseToLiquidLevel {}
