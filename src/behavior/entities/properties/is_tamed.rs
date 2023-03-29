/* Raw contents of is_tamed.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.is_tamed",
  "description": "Sets that this entity is currently tamed.",
  "type": "object",
  "title": "Is Tamed",
  "additionalProperties": false,
  "required": [],
  "properties": {}
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsTamed {}
