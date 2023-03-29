/* Raw contents of dash.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.dash",
  "type": "object",
  "title": "Dash",
  "additionalProperties": false,
  "description": "Ability for a ridable entity to dash.",
  "required": [],
  "properties": {
    "cooldown_time": {
      "title": "cooldown time",
      "type": "number",
      "default": 1.0,
      "description": "The dash cooldown in seconds."
    },
    "horizontal_momentum": {
      "title": "horizoontal momentum",
      "type": "number",
      "default": 1.0,
      "description": "Horizontal momentum of the dash."
    },
    "vertical_momentum": {
      "title": "vertical momentum",
      "type": "number",
      "default": 1.0,
      "description": "Vertical momentum of the dash."
    }
  },
  "examples": [
    {
      "value": 0.0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dash {}
