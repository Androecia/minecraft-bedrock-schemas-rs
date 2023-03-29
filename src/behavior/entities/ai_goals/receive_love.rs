/* Raw contents of receive_love.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.receive_love",
  "description": "Allows the villager to stop so another villager can breed with it. Can only be used by a Villager.",
  "type": "object",
  "title": "Receive Love",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": {
      "$ref": "types/priority.json"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReceiveLove {}
