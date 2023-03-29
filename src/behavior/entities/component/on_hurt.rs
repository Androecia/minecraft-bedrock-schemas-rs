/* Raw contents of on_hurt.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.on_hurt",
  "description": "Adds a trigger to call when this entity takes damage.",
  "title": "On Hurt",
  "$ref": "../types/trigger.json"
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OnHurt {}
