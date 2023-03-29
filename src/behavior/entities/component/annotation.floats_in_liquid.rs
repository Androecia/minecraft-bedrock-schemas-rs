/* Raw contents of annotation.floats_in_liquid.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.annotation.floats_in_liquid",
  "additionalProperties": false,
  "description": "Sets that this entity can float in liquid blocks.",
  "type": "object",
  "title": "Annotation.floats In Liquid",
  "required": [],
  "properties": {}
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Annotation.floatsInLiquid {}
