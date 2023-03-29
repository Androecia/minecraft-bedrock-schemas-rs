/* Raw contents of jump.dynamic.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.jump.dynamic",
  "description": "Defines a dynamic type jump control that will change jump properties based on the speed modifier of the mob.",
  "type": "object",
  "title": "Jump Dynamic",
  "additionalProperties": false,
  "required": [],
  "properties": {}
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Jump.dynamic {}
