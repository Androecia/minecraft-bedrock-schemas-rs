/* Raw contents of collision_box.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.collision_box",
  "type": "object",
  "title": "Collision Box",
  "additionalProperties": false,
  "description": "Sets the width and height of the Entity's collision box.",
  "required": [],
  "properties": {
    "height": {
      "type": "number",
      "default": 1,
      "description": "Height of the collision box in blocks. A negative value will be assumed to be 0",
      "title": "Height"
    },
    "width": {
      "type": "number",
      "default": 1,
      "description": "Width and Depth of the collision box in blocks. A negative value will be assumed to be 0",
      "title": "Width"
    }
  },
  "examples": [
    {
      "height": 1,
      "width": 1
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollisionBox {}
