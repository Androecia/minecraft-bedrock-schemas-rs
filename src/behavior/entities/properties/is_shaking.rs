/* Raw contents of is_shaking.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.is_shaking",
  "description": "Sets that this entity is currently shaking.",
  "type": "object",
  "title": "Is Shaking",
  "additionalProperties": false,
  "required": [],
  "properties": {}
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsShaking {}
