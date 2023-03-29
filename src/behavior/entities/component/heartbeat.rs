/* Raw contents of heartbeat.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.heartbeat",
  "description": "UNDOCUMENTED.",
  "$comment": "UNDOCUMENTED",
  "type": "object",
  "title": "Heartbeat",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "interval": {
      "title": "Interval",
      "description": "UNDOCUMENTED.",
      "$comment": "UNDOCUMENTED",
      "$ref": "../../../../molang/number.json"
    }
  },
  "examples": [{}]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Heartbeat {}
