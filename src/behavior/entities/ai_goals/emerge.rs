/* Raw contents of emerge.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.emerge",
  "type": "object",
  "title": "Emerge",
  "description": "[EXPERIMENTAL BEHAVIOR] Activates the `EMERGING` actor flag during the specified duration and triggers `on_done` at the end",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": { "$ref": "./types/priority.json" },
    "cooldown_time": {
      "title": "Cooldown Time",
      "type": "integer",
      "default": 0.5,
      "description": "Time in seconds the mob has to wait before using the goal again."
    },
    "duration": {
      "title": "Duration",
      "type": "number",
      "default": 5.0,
      "description": "Goal duration in seconds."
    },
    "on_done": {
      "title": "On Done",
      "$ref": "../types/trigger.json",
      "description": "Trigger to be executed when the goal execution is about to end."
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Emerge {}
