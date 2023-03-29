/* Raw contents of squid_flee.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.squid_flee",
  "description": "Allows the squid to swim away. Can only be used by the Squid.",
  "type": "object",
  "title": "Squid Flee",
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
pub struct SquidFlee {}
