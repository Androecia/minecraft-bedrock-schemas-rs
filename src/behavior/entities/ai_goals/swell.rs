/* Raw contents of swell.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.swell",
  "description": "Allows the creeper to swell up when a player is nearby. It can only be used by Creepers.",
  "title": "Swell",
  "type": "object",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },
    "start_distance": {
      "type": "number",
      "default": 10,
      "description": "This mob starts swelling when a target is at least this many blocks away.",
      "title": "Start Distance"
    },
    "stop_distance": {
      "type": "number",
      "default": 2,
      "description": "This mob stops swelling when a target has moved away at least this many blocks.",
      "title": "Stop Distance"
    }
  },
  "examples": [
    {
      "start_distance": 10,
      "stop_distance": 2
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Swell {}
