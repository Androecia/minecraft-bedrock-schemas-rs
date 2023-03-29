/* Raw contents of is_chested.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.is_chested",
  "description": "Sets that this entity is currently carrying a chest.",
  "type": "object",
  "title": "Is Chested",
  "additionalProperties": false,
  "required": [],
  "properties": {}
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsChested {}
