/* Raw contents of flying_speed.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.flying_speed",
  "type": "object",
  "title": "Flying Speed",
  "additionalProperties": false,
  "description": "Speed in Blocks that this entity flies at.",
  "required": [],
  "properties": {
    "value": {
      "type": "number",
      "default": 0.02,
      "description": "Flying speed in blocks per tick.",
      "title": "Value"
    }
  },
  "examples": [
    {
      "value": 0.02
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlyingSpeed {}
