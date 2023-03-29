/* Raw contents of hide.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.hide",
  "type": "object",
  "title": "Hide",
  "description": "UNDOCUMENTED.",
  "$comment": "UNDOCUMENTED",
  "additionalProperties": false,
  "properties": {}
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hide {}
