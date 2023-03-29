/* Raw contents of fire_immune.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.fire_immune",
  "title": "Fire Immune",
  "additionalProperties": false,
  "description": "Sets that this entity doesn't take damage from fire.",
  "type": "object",
  "required": [],
  "properties": {}
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FireImmune {}
