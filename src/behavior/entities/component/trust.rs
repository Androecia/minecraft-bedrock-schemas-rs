/* Raw contents of trust.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.trust",
  "type": "object",
  "title": "Trust",
  "description": "Allows this entity to trust multiple players.",
  "required": [],
  "additionalProperties": false,
  "properties": {}
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trust {}
