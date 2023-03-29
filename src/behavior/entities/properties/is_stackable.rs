/* Raw contents of is_stackable.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.is_stackable",
  "description": "Sets that this entity can be stacked.",
  "type": "object",
  "title": "Is Stackable",
  "additionalProperties": false,
  "properties": {
    "value": {
      "title": "Value",
      "description": "UNDOCUMENTED.",
      "type": "boolean"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsStackable {}
