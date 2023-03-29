/* Raw contents of dig.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.dig",
  "additionalProperties": false,
  "type": "object",
  "title": "Dig",
  "description": "[EXPERIMENTAL BEHAVIOR] Activates the `DIGGING` actor flag during the specified duration. Currently only Warden can use the Dig goal",
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },
    "allow_dig_when_named": {
      "title": "Allow Dig When Named",
      "type": "boolean",
      "default": false,
      "description": "If true, this behavior can run when this entity is named. Otherwise not."
    },
    "digs_in_daylight": {
      "title": "Digs In Daylight",
      "type": "boolean",
      "default": false,
      "description": "Indicates that the actor should start digging when it sees daylight."
    },
    "duration": {
      "title": "Duration",
      "type": "number",
      "default": 0.0,
      "description": "Goal duration in seconds."
    },
    "idle_time": {
      "title": "Idle Time",
      "type": "number",
      "description": "The minimum idle time in seconds between the last detected disturbance to the start of digging."
    },
    "suspicion_is_disturbance": {
      "title": "Suspicion Is Disturbance",
      "type": "boolean",
      "default": false,
      "description": "If true, finding new suspicious locations count as disturbances that may delay the start of this goal."
    },
    "vibration_is_disturbance": {
      "title": "Vibration Is Disturbance",
      "type": "boolean",
      "default": false,
      "description": "If true, vibrations count as disturbances that may delay the start of this goal."
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dig {}
