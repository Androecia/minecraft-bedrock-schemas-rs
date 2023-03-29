/* Raw contents of enderman_leave_block.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.enderman_leave_block",
  "type": "object",
  "title": "Enderman Leave Block",
  "additionalProperties": false,
  "description": "Allows the enderman to drop a block they are carrying. Can only be used by Endermen.",
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndermanLeaveBlock {}
