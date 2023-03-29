/* Raw contents of restrict_sun.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.restrict_sun",
  "description": "Allows the mob to automatically start avoiding the sun when its a clear day out.",
  "type": "object",
  "title": "Restrict Sun",
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
pub struct RestrictSun {}
