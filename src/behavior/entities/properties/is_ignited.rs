/* Raw contents of is_ignited.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.is_ignited",
  "description": "Sets that this entity is currently on fire.",
  "type": "object",
  "title": "Is Ignited",
  "additionalProperties": false,
  "required": [],
  "properties": {}
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsIgnited {}
