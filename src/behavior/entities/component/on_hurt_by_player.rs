/* Raw contents of on_hurt_by_player.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.on_hurt_by_player",
  "description": "Adds a trigger to call when this entity is attacked by the player.",
  "title": "On Hurt By Player",
  "$ref": "../types/trigger.json"
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OnHurtByPlayer {}
