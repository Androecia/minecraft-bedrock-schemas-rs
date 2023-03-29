/* Raw contents of can_climb.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.can_climb",
  "type": "object",
  "title": "Can Climb",
  "additionalProperties": false,
  "description": "Allows this entity to climb up ladders.",
  "required": [],
  "properties": {}
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanClimb {}
