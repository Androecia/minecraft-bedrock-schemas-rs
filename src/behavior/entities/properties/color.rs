/* Raw contents of color.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.color",
  "type": "object",
  "title": "Color",
  "additionalProperties": false,
  "description": "Defines the entity's color. Only works on vanilla entities that have predefined color values (sheep, llama, shulker).",
  "required": [],
  "properties": {
    "value": {
      "type": "integer",
      "default": 0,
      "description": "The Palette Color value of the entity.",
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
pub struct Color {}
