/* Raw contents of on_death.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.on_death",
  "description": "Adds a trigger to call on this entity's death. minecraft:on_death can only be used by the `ender_dragon` entity.",
  "title": "On Death",
  "$ref": "../types/trigger.json"
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OnDeath {}
