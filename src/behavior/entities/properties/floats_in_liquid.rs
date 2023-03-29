/* Raw contents of floats_in_liquid.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.floats_in_liquid",
  "description": "Sets that this entity can float in liquid blocks.",
  "type": "object",
  "title": "Floats In Liquid",
  "additionalProperties": false,
  "required": [],
  "properties": {}
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FloatsInLiquid {}
