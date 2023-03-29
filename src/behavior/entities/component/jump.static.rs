/* Raw contents of jump.static.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.jump.static",
  "description": "Gives the entity the ability to jump.",
  "type": "object",
  "title": "Jump Static",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "jump_power": {
      "type": "number",
      "default": 0.42,
      "description": "The initial vertical velocity for the jump.",
      "title": "Jump Power"
    }
  },
  "examples": [
    {
      "jump_power": 0.42
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Jump.static {}
