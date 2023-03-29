/* Raw contents of enderman_take_block.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.enderman_take_block",
  "type": "object",
  "title": "Enderman Take Block",
  "description": "Allows the enderman to take a block and carry it around. Can only be used by Endermen.",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndermanTakeBlock {}
