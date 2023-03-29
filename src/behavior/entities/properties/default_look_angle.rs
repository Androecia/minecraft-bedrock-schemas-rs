/* Raw contents of default_look_angle.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.default_look_angle",
  "type": "object",
  "title": "Default Look Angle",
  "additionalProperties": false,
  "description": "Sets this entity's default head rotation angle.",
  "required": [],
  "properties": {
    "value": {
      "type": "number",
      "default": 0.0,
      "description": "Angle in degrees.",
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
pub struct DefaultLookAngle {}
