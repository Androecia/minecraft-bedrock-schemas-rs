/* Raw contents of on_ignite.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.on_ignite",
  "description": "Adds a trigger to call when this entity is set on fire.",
  "title": "On Ignite",
  "$ref": "../types/trigger.json"
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OnIgnite {}
