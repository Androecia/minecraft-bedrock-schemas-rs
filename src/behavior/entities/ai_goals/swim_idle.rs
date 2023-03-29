/* Raw contents of swim_idle.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.swim_idle",
  "description": "Allows the entity go idle, if swimming. Entity must be in water.",
  "type": "object",
  "title": "Swim Idle",
  "additionalProperties": false,
  "properties": {
    "priority": {
      "$ref": "types/priority.json"
    },
    "idle_time": {
      "type": "number",
      "title": "Idle Time",
      "default": 5,
      "description": "Amount of time (in seconds) to stay idle."
    },
    "success_rate": {
      "type": "number",
      "title": "Succes Rate",
      "default": 0.1,
      "description": "Percent chance this entity will go idle, 1.0 = 100%."
    }
  },
  "examples": [
    {
      "idle_time": 0.0,
      "success_rate": 0.0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwimIdle {}
