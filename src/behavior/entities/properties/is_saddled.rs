/* Raw contents of is_saddled.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.is_saddled",
  "description": "Sets that this entity is currently saddled.",
  "type": "object",
  "title": "Is Saddled",
  "additionalProperties": false,
  "required": [],
  "properties": {}
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsSaddled {}
