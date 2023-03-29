/* Raw contents of is_stunned.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.is_stunned",
  "description": "Sets that this entity is currently stunned.",
  "type": "object",
  "title": "Is Stunned",
  "additionalProperties": false,
  "required": [],
  "properties": {}
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsStunned {}
