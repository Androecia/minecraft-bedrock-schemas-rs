/* Raw contents of on_start_landing.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.on_start_landing",
  "description": "Only usable by the Ender Dragon. Adds a trigger to call when this entity lands.",
  "title": "On Start Landing",
  "$ref": "../types/trigger.json"
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OnStartLanding {}
