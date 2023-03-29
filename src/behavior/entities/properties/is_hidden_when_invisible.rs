/* Raw contents of is_hidden_when_invisible.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.is_hidden_when_invisible",
  "type": "object",
  "title": "Is Hidden When Invisible",
  "additionalProperties": false,
  "description": "Sets that this entity can hide from hostile mobs while invisible."
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsHiddenWhenInvisible {}
