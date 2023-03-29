/* Raw contents of scale.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.scale",
  "type": "object",
  "title": "Scale",
  "additionalProperties": false,
  "description": "Sets the entity's visual size.",
  "required": [],
  "properties": {
    "value": {
      "type": "number",
      "default": 0.0,
      "description": "The value of the scale. 1.0 means the entity will appear at the scale they are defined in their model. Higher numbers make the entity bigger",
      "title": "Value"
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
pub struct Scale {}
