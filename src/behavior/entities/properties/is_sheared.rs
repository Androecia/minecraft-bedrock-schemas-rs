/* Raw contents of is_sheared.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.is_sheared",
  "description": "Sets that this entity is currently sheared.",
  "type": "object",
  "title": "Is Sheared",
  "additionalProperties": false,
  "required": [],
  "properties": {}
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsSheared {}
