/* Raw contents of barter.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.barter",
  "type": "object",
  "title": "Barter",
  "description": "Enables the mob to barter for items that have been configured as barter currency. Must be used in combination with the barter component",
  "additionalProperties": false,
  "properties": {
    "priority": {
      "$ref": "types/priority.json"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Barter {}
