/* Raw contents of roar.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.roar",
  "additionalProperties": false,
  "type": "object",
  "title": "Roar",
  "description": "[EXPERIMENTAL BEHAVIOR] Plays the provided sound and activates the `ROARING` actor flag during the specified duration",
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },
    "duration": {
      "title": "Duration",
      "type": "number",
      "default": 0.0,
      "description": "Goal duration in seconds."
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Roar {}
