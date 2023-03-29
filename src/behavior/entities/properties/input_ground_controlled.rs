/* Raw contents of input_ground_controlled.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.input_ground_controlled",
  "description": "When configured as a rideable entity, the entity will be controlled using WASD controls.",
  "type": "object",
  "title": "Input Ground Controlled",
  "additionalProperties": false,
  "required": [],
  "properties": {}
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputGroundControlled {}
