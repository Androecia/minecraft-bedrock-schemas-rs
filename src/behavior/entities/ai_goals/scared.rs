/* Raw contents of scared.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.scared",
  "description": "Allows the a mob to become scared when the weather outside is thundering.",
  "type": "object",
  "title": "Scared",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },
    "sound_interval": {
      "type": "integer",
      "default": 0,
      "description": "The interval in which a sound will play when active in a 1/delay chance to kick off.",
      "title": "Sound Interval"
    }
  },
  "examples": [
    {
      "sound_interval": 0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scared {}
