/* Raw contents of out_of_control.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.out_of_control",
  "type": "object",
  "title": "Out Of Control",
  "additionalProperties": false,
  "description": "defines the entity's `out of control` state.",
  "required": [],
  "properties": {}
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutOfControl {}
