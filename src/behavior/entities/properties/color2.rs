/* Raw contents of color2.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.color2",
  "type": "object",
  "title": "Color.0",
  "additionalProperties": false,
  "description": "Defines the entity's second texture color. Only works on vanilla entities that have a second predefined color values (tropical fish).",
  "required": [],
  "properties": {
    "value": {
      "type": "integer",
      "default": 0,
      "description": "The second Palette Color value of the entity.",
      "title": "Value"
    }
  },
  "examples": [
    {
      "value": 0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Color2 {}
