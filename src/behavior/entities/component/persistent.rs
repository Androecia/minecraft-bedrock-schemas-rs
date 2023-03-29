/* Raw contents of persistent.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.persistent",
  "description": "Defines whether an entity should be persistent in the game world.",
  "type": "object",
  "title": "Persistent",
  "additionalProperties": false,
  "required": [],
  "properties": {}
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Persistent {}
