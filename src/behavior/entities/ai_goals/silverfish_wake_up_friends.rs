/* Raw contents of silverfish_wake_up_friends.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.silverfish_wake_up_friends",
  "description": "Allows the mob to alert mobs in nearby blocks to come out. Currently it can only be used by Silverfish.",
  "type": "object",
  "title": "Silverfish Wake Up Friends",
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
pub struct SilverfishWakeUpFriends {}
