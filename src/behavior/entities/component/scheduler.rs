/* Raw contents of scheduler.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.scheduler",
  "description": "fires off scheduled mob events at time of day events.",
  "type": "object",
  "title": "Scheduler",
  "additionalProperties": false,
  "properties": {
    "min_delay_secs": {
      "title": "Minimum Delay Secs",
      "type": "number",
      "default": 0,
      "description": "The minimum the scheduler will be delayed.",
      "minimum": 0
    },
    "max_delay_secs": {
      "title": "Maximum Delay Secs",
      "type": "number",
      "default": 0,
      "description": "The maximum the scheduler will be delayed.",
      "minimum": 0
    },
    "scheduled_events": {
      "title": "Scheduled Events",
      "type": "array",
      "description": "The list of triggers that fire when the conditions match the given filter criteria. If any filter criteria overlap the first defined event will be picked.",
      "items": {
        "title": "Scheduled Events",
        "additionalProperties": false,
        "type": "object",
        "description": "A filter and event pair. The event runs when the filter criteria succeeds",
        "properties": {
          "filters": {
            "$ref": "../../filters/filters.json"
          },
          "event": {
            "$ref": "../types/event.json"
          }
        }
      }
    }
  },
  "examples": [
    {
      "min_delay_secs": 0,
      "max_delay_secs": 0,
      "scheduled_events": []
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scheduler {}
