/* Raw contents of can_power_jump.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.can_power_jump",
  "type": "object",
  "title": "Can Power Jump",
  "additionalProperties": false,
  "description": "Allows the entity to power jump like the horse does in vanilla.",
  "required": [],
  "properties": {}
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanPowerJump {}
