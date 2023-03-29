/* Raw contents of hurt_when_wet.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.hurt_when_wet",
  "type": "object",
  "title": "Hurt When Wet",
  "additionalProperties": false,
  "required": [],
  "properties": {},
  "description": "Specifies if an actor is hurt when wet."
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HurtWhenWet {}
