/* Raw contents of on_target_acquired.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.on_target_acquired",
  "description": "Adds a trigger to call when this entity finds a target.",
  "title": "On Target Acquired",
  "$ref": "../types/trigger.json"
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OnTargetAcquired {}
