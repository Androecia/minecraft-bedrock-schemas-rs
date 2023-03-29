/* Raw contents of squid_idle.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.squid_idle",
  "description": "Allows the squid to swim in place idly. Can only be used by the Squid.",
  "type": "object",
  "title": "Squid Idle",
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
pub struct SquidIdle {}
