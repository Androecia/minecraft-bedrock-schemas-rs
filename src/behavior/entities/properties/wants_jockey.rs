/* Raw contents of wants_jockey.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.wants_jockey",
  "description": "Sets that this entity wants to become a jockey.",
  "type": "object",
  "title": "Wants Jockey",
  "additionalProperties": false,
  "properties": {}
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WantsJockey {}
