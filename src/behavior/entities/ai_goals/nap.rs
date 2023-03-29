/* Raw contents of nap.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.nap",
  "type": "object",
  "title": "Nap",
  "description": "Allows mobs to occassionally stop and take a nap under certain conditions.",
  "additionalProperties": false,
  "properties": {
    "priority": { "$ref": "types/priority.json" },
    "speed_multiplier": { "$ref": "types/speed_multiplier.json" },
    "cooldown_max": {
      "title": "Cooldown Max",
      "type": "number",
      "default": 0,
      "description": "Maximum time in seconds the mob has to wait before using the goal again."
    },
    "cooldown_min": {
      "title": "Cooldown Min",
      "type": "number",
      "default": 0,
      "description": "Minimum time in seconds the mob has to wait before using the goal again."
    },
    "mob_detect_dist": {
      "title": "Mob Detect Dist",
      "type": "number",
      "default": 6,
      "description": "The block distance in x and z that will be checked for mobs that this mob detects."
    },
    "mob_detect_height": {
      "title": "Mob Detect Height",
      "type": "number",
      "default": 6,
      "description": "The block distance in y that will be checked for mobs that this mob detects."
    },
    "can_nap_filters": {
      "title": "Can Nap Filters",
      "$ref": "../../filters/filters.json",
      "description": "The filters that need to be met for the nap to take place.",
      "$comment": "UNDOCUMENTED"
    },
    "wake_mob_exceptions": {
      "title": "Wake Mob Exceptions",
      "$ref": "../../filters/filters.json",
      "description": "Filters that can trigger the entity to wake up from it nap.",
      "$comment": "UNDOCUMENTED"
    }
  },
  "examples": [
    {
      "cooldown_max": 0,
      "cooldown_min": 0,
      "mob_detect_dist": 6,
      "mob_detect_height": 6
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Nap {}
