/* Raw contents of on_friendly_anger.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.on_friendly_anger",
  "description": "Adds a trigger that will run when a nearby entity of the same type as this entity becomes Angry.",
  "title": "On Friendly Anger",
  "$ref": "../types/trigger.json"
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OnFriendlyAnger {}
