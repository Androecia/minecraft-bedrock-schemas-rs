/* Raw contents of ground_offset.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.ground_offset",
  "type": "object",
  "title": "Ground Offset",
  "additionalProperties": false,
  "description": "Sets the offset from the ground that the entity is actually at.",
  "required": [],
  "properties": {
    "value": {
      "type": "number",
      "default": 0.0,
      "description": "The value of the entity's offset from the terrain, in blocks.",
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
pub struct GroundOffset {}
