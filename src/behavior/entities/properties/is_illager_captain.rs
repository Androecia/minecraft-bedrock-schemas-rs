/* Raw contents of is_illager_captain.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.is_illager_captain",
  "description": "Sets that this entity is an illager captain.",
  "type": "object",
  "title": "Is Illager Captain",
  "additionalProperties": false,
  "required": [],
  "properties": {}
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsIllagerCaptain {}
