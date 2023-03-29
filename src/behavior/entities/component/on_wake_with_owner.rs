/* Raw contents of on_wake_with_owner.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.on_wake_with_owner",
  "description": "Adds a trigger to call when this pet's owner awakes after sleeping with the pet.",
  "title": "On Wake With Owner",
  "$ref": "../types/trigger.json"
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OnWakeWithOwner {}
