/* Raw contents of work.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.work",
  "type": "object",
  "title": "Work",
  "description": "Allows the NPC to use the POI.",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": { "$ref": "./types/priority.json" },
    "speed_multiplier": { "$ref": "./types/speed_multiplier.json" },
    "active_time": {
      "title": "Active Time",
      "type": "integer",
      "default": 0,
      "description": "The amount of ticks the NPC will stay in their the work location.",
      "exclusiveMinimum": 0
    },
    "can_work_in_rain": {
      "title": "Can Work In Rain",
      "type": "boolean",
      "default": false,
      "description": "If true, this entity can work when their jobsite POI is being rained on."
    },
    "goal_cooldown": {
      "title": "Goal Cooldown",
      "type": "integer",
      "default": 0,
      "description": "The amount of ticks the goal will be on cooldown before it can be used again."
    },
    "on_arrival": {
      "title": "On Arrival",
      "$ref": "../types/trigger.json",
      "description": "Event to run when the mob reaches their jobsite."
    },
    "sound_delay_max": {
      "title": "Sound Delay Max",
      "type": "integer",
      "default": 0,
      "description": "The max interval in which a sound will play."
    },
    "sound_delay_min": {
      "title": "Sound Delay Min",
      "type": "integer",
      "default": 0,
      "description": "The min interval in which a sound will play."
    },
    "work_in_rain_tolerance": {
      "title": "Work In Rain Tolerance",
      "type": "integer",
      "default": -1,
      "description": "If \"can_work_in_rain\" is false, this is the maximum number of ticks left in the goal where rain will not interrupt the goal"
    }
  },
  "examples": [
    {
      "active_time": 0,
      "can_work_in_rain": false,
      "goal_cooldown": 0,
      "sound_delay_max": 0,
      "sound_delay_min": 0,
      "work_in_rain_tolerance": -1
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Work {}
