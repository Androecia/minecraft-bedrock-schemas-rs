/* Raw contents of open_door.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.open_door",
  "type": "object",
  "title": "Open Door",
  "additionalProperties": false,
  "description": "Allows the mob to open doors. Requires the mob to be able to path through doors, otherwise the mob won't even want to try opening them.",
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },
    "close_door_after": {
      "type": "boolean",
      "default": true,
      "description": "If true, the mob will close the door after opening it and going through it.",
      "title": "Close Door After"
    }
  },
  "examples": [
    {
      "close_door_after": true
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenDoor {}
