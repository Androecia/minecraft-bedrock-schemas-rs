/* Raw contents of on_target_escape.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.on_target_escape",
  "description": "Adds a trigger to call when this entity loses the target it currently has.",
  "title": "On Target Escape",
  "$ref": "../types/trigger.json"
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OnTargetEscape {}
