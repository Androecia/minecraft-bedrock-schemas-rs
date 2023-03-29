/* Raw contents of is_charged.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.is_charged",
  "description": "Sets that this entity is charged.",
  "type": "object",
  "title": "Is Charged",
  "additionalProperties": false,
  "required": [],
  "properties": {}
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsCharged {}
